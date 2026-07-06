// Ticket service — secret generation, validation, and QR code content.
// Ported from pretix's tickets.py service layer.

use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::PgPool;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("Ticket not found")]
    NotFound,
    #[error("Ticket is canceled")]
    Canceled,
    #[error("Order not paid")]
    Unpaid,
    #[error("Ticket is blocked: {0}")]
    Blocked(String),
    #[error("Ticket not yet valid")]
    NotYetValid,
    #[error("Ticket expired")]
    Expired,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Generate a ticket secret — the content of the QR code.
///
/// Format: {position_id}:{hmac_signature}
/// The HMAC is computed over position_id + order_secret using the instance's signing key.
/// This allows offline validation: scanner checks the HMAC and matches against
/// a cached list of valid secrets.
pub fn generate_ticket_secret(position_id: Uuid, order_secret: &str, signing_key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(signing_key).expect("HMAC can take any key size");
    mac.update(position_id.to_string().as_bytes());
    mac.update(b":");
    mac.update(order_secret.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    format!("{}:{}", position_id, &signature[..16])
}

/// Validate a ticket at the door.
///
/// Steps:
/// 1. Look up the OrderPosition by its secret (QR code content)
/// 2. Verify the order is paid (or valid_if_pending)
/// 3. Verify the position is not canceled
/// 4. Verify the position is not blocked
/// 5. Verify validity window (valid_from/until)
/// 6. Check if already scanned on this list (unless allow_multiple_entries)
/// 7. Record the checkin
pub async fn validate_ticket(
    pool: &PgPool,
    secret: &str,
    list_id: Uuid,
    _device_id: Option<Uuid>,
    _gate_id: Option<Uuid>,
    nonce: &str,
) -> Result<TicketValidation, TicketError> {
    // 1. Look up position by secret
    let position = sqlx::query_as::<_, PositionCheckRow>(
        "SELECT op.id, op.order_id, op.item_id, op.secret, op.canceled, op.blocked,
                op.valid_from, op.valid_until, op.attendee_name, op.attendee_email,
                op.seat_id,
                o.status as order_status, o.valid_if_pending, o.code as order_code,
                i.name as item_name
         FROM order_positions op
         JOIN orders o ON o.id = op.order_id
         JOIN items i ON i.id = op.item_id
         WHERE op.secret = $1",
    )
    .bind(secret)
    .fetch_optional(pool)
    .await?
    .ok_or(TicketError::NotFound)?;

    // 2. Check order status
    if position.order_status != "paid" && !(position.order_status == "pending" && position.valid_if_pending) {
        return Ok(TicketValidation::denied("unpaid", &position));
    }

    // 3. Check if canceled
    if position.canceled {
        return Ok(TicketValidation::denied("canceled", &position));
    }

    // 4. Check if blocked
    if let Some(ref blocked) = position.blocked {
        if !blocked.is_null() {
            return Ok(TicketValidation::denied("blocked", &position));
        }
    }

    // 5. Check validity window
    let now = Utc::now();
    if let Some(valid_from) = position.valid_from {
        if now < valid_from {
            return Ok(TicketValidation::denied("invalid_time", &position));
        }
    }
    if let Some(valid_until) = position.valid_until {
        if now > valid_until {
            return Ok(TicketValidation::denied("invalid_time", &position));
        }
    }

    // 6. Check if already scanned on this list (unless multiple entries allowed)
    let list = sqlx::query_as::<_, CheckinListRow>(
        "SELECT id, allow_multiple_entries, allow_entry_after_exit FROM checkin_lists WHERE id = $1",
    )
    .bind(list_id)
    .fetch_optional(pool)
    .await?
    .ok_or(TicketError::NotFound)?;

    if !list.allow_multiple_entries {
        let already_scanned: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM checkins
             WHERE position_id = $1 AND list_id = $2 AND type = 'entry' AND successful = true",
        )
        .bind(position.id)
        .bind(list_id)
        .fetch_one(pool)
        .await?;

        // If already entered and re-entry not allowed
        if already_scanned.0 > 0 && !list.allow_entry_after_exit {
            return Ok(TicketValidation::denied("already_redeemed", &position));
        }
    }

    // 7. Duplicate nonce check (prevent accidental double-scans)
    let nonce_dup: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM checkins WHERE nonce = $1",
    )
    .bind(nonce)
    .fetch_one(pool)
    .await?;

    if nonce_dup.0 > 0 {
        // Nonce already used — this is a duplicate, silently succeed
        return Ok(TicketValidation {
            success: true,
            position_id: position.id,
            attendee_name: position.attendee_name.clone(),
            attendee_email: position.attendee_email.clone(),
            item_name: position.item_name.clone(),
            order_code: position.order_code.clone(),
            seat: position.seat_id.map(|s| s.to_string()),
            message: None,
            error_reason: None,
        });
    }

    // 8. Record the checkin
    sqlx::query(
        "INSERT INTO checkins (position_id, list_id, type, successful, device_id, gate_id, nonce)
         VALUES ($1, $2, 'entry', true, $3, $4, $5)",
    )
    .bind(position.id)
    .bind(list_id)
    .bind(_device_id)
    .bind(_gate_id)
    .bind(nonce)
    .execute(pool)
    .await?;

    Ok(TicketValidation {
        success: true,
        position_id: position.id,
        attendee_name: position.attendee_name.clone(),
        attendee_email: position.attendee_email.clone(),
        item_name: position.item_name.clone(),
        order_code: position.order_code.clone(),
        seat: position.seat_id.map(|s| s.to_string()),
        message: None,
        error_reason: None,
    })
}

/// Result of a ticket validation (door scan).
#[derive(Debug, serde::Serialize)]
pub struct TicketValidation {
    pub success: bool,
    pub position_id: Uuid,
    pub attendee_name: Option<String>,
    pub attendee_email: Option<String>,
    pub item_name: String,
    pub order_code: String,
    pub seat: Option<String>,
    pub message: Option<String>,
    pub error_reason: Option<String>,
}

impl TicketValidation {
    fn denied(reason: &str, position: &PositionCheckRow) -> Self {
        // Record failed checkin for analytics
        // (In a real system, we'd do this asynchronously)
        TicketValidation {
            success: false,
            position_id: position.id,
            attendee_name: position.attendee_name.clone(),
            attendee_email: position.attendee_email.clone(),
            item_name: position.item_name.clone(),
            order_code: position.order_code.clone(),
            seat: position.seat_id.map(|s| s.to_string()),
            message: None,
            error_reason: Some(reason.to_string()),
        }
    }
}

/// Get checkin statistics for a list.
pub async fn get_checkin_stats(
    pool: &PgPool,
    list_id: Uuid,
) -> Result<CheckinStats, TicketError> {
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM checkins WHERE list_id = $1 AND successful = true",
    )
    .bind(list_id)
    .fetch_one(pool)
    .await?;

    let inside: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT position_id) FROM checkins
         WHERE list_id = $1 AND type = 'entry' AND successful = true
         AND position_id NOT IN (
             SELECT position_id FROM checkins
             WHERE list_id = $1 AND type = 'exit' AND successful = true
         )",
    )
    .bind(list_id)
    .fetch_one(pool)
    .await?;

    Ok(CheckinStats {
        total_scans: total.0,
        currently_inside: inside.0,
    })
}

#[derive(Debug, serde::Serialize)]
pub struct CheckinStats {
    pub total_scans: i64,
    pub currently_inside: i64,
}

// ── Internal row types ──

#[derive(Debug, sqlx::FromRow)]
struct PositionCheckRow {
    id: Uuid,
    #[allow(dead_code)]
    order_id: Uuid,
    #[allow(dead_code)]
    item_id: Uuid,
    #[allow(dead_code)]
    secret: String,
    canceled: bool,
    blocked: Option<serde_json::Value>,
    valid_from: Option<DateTime<Utc>>,
    valid_until: Option<DateTime<Utc>>,
    attendee_name: Option<String>,
    attendee_email: Option<String>,
    seat_id: Option<Uuid>,
    order_status: String,
    valid_if_pending: bool,
    order_code: String,
    item_name: String,
}

#[derive(Debug, sqlx::FromRow)]
struct CheckinListRow {
    #[allow(dead_code)]
    id: Uuid,
    allow_multiple_entries: bool,
    allow_entry_after_exit: bool,
}
