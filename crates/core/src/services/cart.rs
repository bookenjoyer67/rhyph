// Cart service — add/remove items, validate quotas.
// Ported from pretix's cart.py service layer.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// A pre-order cart position. Reserves quota slot temporarily.
/// Expires after event.reservation_time minutes.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CartPosition {
    pub id: Uuid,
    pub event_id: Uuid,
    pub item_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub session_key: String,
    pub price: BigDecimal,
    pub expires: DateTime<Utc>,
    pub answers: serde_json::Value,
    pub seat_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum CartError {
    #[error("Item not found")]
    ItemNotFound,
    #[error("Quota exceeded for {0}")]
    QuotaExceeded(String),
    #[error("Item not available (sale period)")]
    NotAvailable,
    #[error("Max per order exceeded")]
    MaxPerOrderExceeded,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Add an item to the cart. Validates quotas first.
pub async fn add_to_cart(
    pool: &PgPool,
    event_id: Uuid,
    item_id: Uuid,
    variation_id: Option<Uuid>,
    session_key: &str,
    quantity: i32,
    answers: serde_json::Value,
    seat_id: Option<Uuid>,
) -> Result<Vec<CartPosition>, CartError> {
    // 1. Fetch item and verify it exists + is active
    let item = sqlx::query_as::<_, ItemRow>(
        "SELECT id, event_id, name, active, default_price,
                available_from, available_until, max_per_order, min_per_order
         FROM items WHERE id = $1 AND event_id = $2",
    )
    .bind(item_id)
    .bind(event_id)
    .fetch_optional(pool)
    .await?
    .ok_or(CartError::ItemNotFound)?;

    if !item.active {
        return Err(CartError::ItemNotFound);
    }

    // 2. Check sale window
    let now = Utc::now();
    if let Some(from) = item.available_from {
        if now < from {
            return Err(CartError::NotAvailable);
        }
    }
    if let Some(until) = item.available_until {
        if now > until {
            return Err(CartError::NotAvailable);
        }
    }

    // 3. Check max per order
    if let Some(max) = item.max_per_order {
        let current_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM cart_positions
             WHERE session_key = $1 AND event_id = $2 AND item_id = $3",
        )
        .bind(session_key)
        .bind(event_id)
        .bind(item_id)
        .fetch_one(pool)
        .await?;

        if current_count.0 + quantity as i64 > max as i64 {
            return Err(CartError::MaxPerOrderExceeded);
        }
    }

    // 4. If variation specified, fetch it for price override
    let price = if let Some(vid) = variation_id {
        let variation: (Option<BigDecimal>,) = sqlx::query_as(
            "SELECT default_price FROM item_variations WHERE id = $1 AND item_id = $2",
        )
        .bind(vid)
        .bind(item_id)
        .fetch_optional(pool)
        .await?
        .ok_or(CartError::ItemNotFound)?;

        variation.0.unwrap_or(item.default_price)
    } else {
        item.default_price
    };

    // 5. Check quotas
    check_quotas(pool, event_id, item_id, variation_id, quantity).await?;

    // 6. Get event reservation time
    let reservation_time: (i32,) =
        sqlx::query_as("SELECT reservation_time FROM events WHERE id = $1")
            .bind(event_id)
            .fetch_one(pool)
            .await?;

    let expires = now + Duration::minutes(reservation_time.0 as i64);

    // 7. Create cart positions
    let mut positions = Vec::new();
    for _ in 0..quantity {
        let position: CartPosition = sqlx::query_as(
            "INSERT INTO cart_positions (event_id, item_id, variation_id, session_key, price, expires, answers, seat_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, event_id, item_id, variation_id, session_key, price, expires, answers, seat_id, created_at",
        )
        .bind(event_id)
        .bind(item_id)
        .bind(variation_id)
        .bind(session_key)
        .bind(&price)
        .bind(expires)
        .bind(&answers)
        .bind(seat_id)
        .fetch_one(pool)
        .await?;

        positions.push(position);
    }

    Ok(positions)
}

/// Remove a cart position. Only the session that created it can remove it.
pub async fn remove_from_cart(
    pool: &PgPool,
    position_id: Uuid,
    session_key: &str,
) -> Result<(), CartError> {
    let result = sqlx::query("DELETE FROM cart_positions WHERE id = $1 AND session_key = $2")
        .bind(position_id)
        .bind(session_key)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(CartError::ItemNotFound);
    }

    Ok(())
}

/// Get all cart positions for a session.
pub async fn get_cart(
    pool: &PgPool,
    session_key: &str,
    event_id: Uuid,
) -> Result<Vec<CartPosition>, CartError> {
    let positions = sqlx::query_as::<_, CartPosition>(
        "SELECT id, event_id, item_id, variation_id, session_key, price, expires, answers, seat_id, created_at
         FROM cart_positions
         WHERE session_key = $1 AND event_id = $2 AND expires > NOW()
         ORDER BY created_at",
    )
    .bind(session_key)
    .bind(event_id)
    .fetch_all(pool)
    .await?;

    Ok(positions)
}

/// Clear expired cart positions. Returns number of positions cleared.
pub async fn clear_expired(pool: &PgPool) -> Result<u64, CartError> {
    let result = sqlx::query("DELETE FROM cart_positions WHERE expires < NOW()")
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

// ── Internal helpers ──

#[derive(Debug, sqlx::FromRow)]
struct ItemRow {
    #[allow(dead_code)]
    id: Uuid,
    #[allow(dead_code)]
    event_id: Uuid,
    #[allow(dead_code)]
    name: String,
    active: bool,
    default_price: BigDecimal,
    available_from: Option<DateTime<Utc>>,
    available_until: Option<DateTime<Utc>>,
    max_per_order: Option<i32>,
    #[allow(dead_code)]
    min_per_order: Option<i32>,
}

/// Check all quotas that apply to this item+variation combination.
async fn check_quotas(
    pool: &PgPool,
    _event_id: Uuid,
    item_id: Uuid,
    variation_id: Option<Uuid>,
    quantity: i32,
) -> Result<(), CartError> {
    let quotas: Vec<QuotaRow> = if let Some(vid) = variation_id {
        sqlx::query_as(
            "SELECT q.id, q.name, q.size
             FROM quotas q
             JOIN quota_items qi ON qi.quota_id = q.id
             LEFT JOIN quota_variations qv ON qv.quota_id = q.id
             WHERE qi.item_id = $1
               AND (qv.variation_id = $2 OR qv.variation_id IS NULL)",
        )
        .bind(item_id)
        .bind(vid)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT q.id, q.name, q.size
             FROM quotas q
             JOIN quota_items qi ON qi.quota_id = q.id
             WHERE qi.item_id = $1",
        )
        .bind(item_id)
        .fetch_all(pool)
        .await?
    };

    for quota in &quotas {
        if let Some(size) = quota.size {
            // Count sold tickets (order positions, not canceled)
            let sold: (i64,) = sqlx::query_as(
                "SELECT COALESCE(COUNT(*), 0) FROM order_positions op
                 JOIN orders o ON o.id = op.order_id
                 WHERE op.item_id = $1
                   AND NOT op.canceled
                   AND o.status IN ('paid', 'pending')
                   AND (op.variation_id = $2 OR $2 IS NULL)",
            )
            .bind(item_id)
            .bind(variation_id)
            .fetch_one(pool)
            .await?;

            // Count current cart reservations
            let reserved: (i64,) = sqlx::query_as(
                "SELECT COALESCE(COUNT(*), 0) FROM cart_positions
                 WHERE item_id = $1
                   AND (variation_id = $2 OR $2 IS NULL)
                   AND expires > NOW()",
            )
            .bind(item_id)
            .bind(variation_id)
            .fetch_one(pool)
            .await?;

            let total = sold.0 + reserved.0 + quantity as i64;
            if total > size as i64 {
                return Err(CartError::QuotaExceeded(quota.name.clone()));
            }
        }
    }

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct QuotaRow {
    #[allow(dead_code)]
    id: Uuid,
    name: String,
    size: Option<i32>,
}
