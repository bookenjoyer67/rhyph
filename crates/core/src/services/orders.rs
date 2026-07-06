// Order service — cart → order transition, payment, cancellation.
// Ported from pretix's orders.py service layer.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use rand::Rng;

#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("Cart is empty")]
    EmptyCart,
    #[error("Quota exceeded for {0}")]
    QuotaExceeded(String),
    #[error("Order not found")]
    NotFound,
    #[error("Invalid transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Result of creating an order from a cart.
#[derive(Debug)]
pub struct CreateOrderResult {
    pub order_id: Uuid,
    pub order_code: String,
    pub order_secret: String,
    pub total: BigDecimal,
    pub position_count: usize,
}

/// Create an order from a cart session.
///
/// This is the critical transaction:
/// 1. Fetch all non-expired cart positions for this session
/// 2. Verify quotas again (double-check under a potential race)
/// 3. Generate order code and secrets
/// 4. Insert order + order_positions in a transaction
/// 5. Clear the cart
pub async fn create_order(
    pool: &PgPool,
    event_id: Uuid,
    session_key: &str,
    email: Option<&str>,
    locale: &str,
) -> Result<CreateOrderResult, OrderError> {
    let mut tx = pool.begin().await?;

    // 1. Fetch cart positions
    let cart_positions = sqlx::query_as::<_, CartPositionRow>(
        "SELECT cp.id, cp.item_id, cp.variation_id, cp.price, cp.answers, cp.seat_id,
                i.name as item_name
         FROM cart_positions cp
         JOIN items i ON i.id = cp.item_id
         WHERE cp.session_key = $1 AND cp.event_id = $2 AND cp.expires > NOW()
         ORDER BY cp.created_at
         FOR UPDATE",
    )
    .bind(session_key)
    .bind(event_id)
    .fetch_all(&mut *tx)
    .await?;

    if cart_positions.is_empty() {
        tx.rollback().await?;
        return Err(OrderError::EmptyCart);
    }

    // 2. Verify quotas for every position
    for pos in &cart_positions {
        let available = check_quota_available(
            pool,
            pos.item_id,
            pos.variation_id,
        ).await?;

        if !available {
            tx.rollback().await?;
            return Err(OrderError::QuotaExceeded(pos.item_name.clone()));
        }
    }

    // 3. Generate order code (5-char alphanumeric, uppercase)
    let order_code = generate_order_code();
    let order_secret = generate_secret(32);
    let now = Utc::now();

    // Calculate total
    let total: BigDecimal = cart_positions.iter()
        .fold(BigDecimal::from(0), |acc, p| acc + p.price.clone());

    // 4. Insert order
    let order_id: (Uuid,) = sqlx::query_as(
        "INSERT INTO orders (event_id, code, status, secret, email, locale, total, datetime, payment_state, sales_channel)
         VALUES ($1, $2, 'pending', $3, $4, $5, $6, $7, 'created', 'web')
         RETURNING id",
    )
    .bind(event_id)
    .bind(&order_code)
    .bind(&order_secret)
    .bind(email)
    .bind(locale)
    .bind(&total)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;

    // 5. Insert order positions
    let mut position_count = 0;
    for (i, pos) in cart_positions.iter().enumerate() {
        let position_secret = generate_secret(64);
        let pseudonymization_id = generate_secret(32);

        sqlx::query(
            "INSERT INTO order_positions
             (order_id, positionid, item_id, variation_id, price, tax_rate, tax_value,
              secret, answers, seat_id, pseudonymization_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(order_id.0)
        .bind((i + 1) as i32)
        .bind(pos.item_id)
        .bind(pos.variation_id)
        .bind(&pos.price)
        .bind(BigDecimal::from(0))   // tax_rate — will be calculated later
        .bind(BigDecimal::from(0))   // tax_value
        .bind(&position_secret)
        .bind(&pos.answers)
        .bind(pos.seat_id)
        .bind(&pseudonymization_id)
        .execute(&mut *tx)
        .await?;

        position_count += 1;
    }

    // 6. Clear cart
    sqlx::query("DELETE FROM cart_positions WHERE session_key = $1 AND event_id = $2")
        .bind(session_key)
        .bind(event_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(CreateOrderResult {
        order_id: order_id.0,
        order_code,
        order_secret,
        total,
        position_count,
    })
}

/// Get an order by its event and code (human-readable).
pub async fn get_order(
    pool: &PgPool,
    event_id: Uuid,
    code: &str,
) -> Result<OrderWithPositions, OrderError> {
    let order = sqlx::query_as::<_, OrderRow>(
        "SELECT id, event_id, code, status, secret, email, phone, locale,
                total, datetime, expires, payment_provider, payment_state,
                testmode, require_approval, valid_if_pending, sales_channel,
                created_at, updated_at
         FROM orders
         WHERE event_id = $1 AND code = $2",
    )
    .bind(event_id)
    .bind(code)
    .fetch_optional(pool)
    .await?
    .ok_or(OrderError::NotFound)?;

    let positions = sqlx::query_as::<_, PositionRow>(
        "SELECT op.id, op.order_id, op.positionid, op.item_id, op.variation_id,
                op.price, op.secret, op.attendee_name, op.attendee_email,
                op.answers, op.seat_id, op.pseudonymization_id,
                op.canceled, op.valid_from, op.valid_until,
                i.name as item_name
         FROM order_positions op
         JOIN items i ON i.id = op.item_id
         WHERE op.order_id = $1
         ORDER BY op.positionid",
    )
    .bind(order.id)
    .fetch_all(pool)
    .await?;

    Ok(OrderWithPositions { order, positions })
}

/// Cancel an order. Releases quota slots.
pub async fn cancel_order(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<(), OrderError> {
    let result = sqlx::query(
        "UPDATE orders SET status = 'canceled', updated_at = NOW()
         WHERE id = $1 AND status IN ('pending', 'paid')",
    )
    .bind(order_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(OrderError::NotFound);
    }

    // Mark all positions as canceled
    sqlx::query(
        "UPDATE order_positions SET canceled = true WHERE order_id = $1",
    )
    .bind(order_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark order as paid (called by payment webhook).
pub async fn mark_paid(
    pool: &PgPool,
    order_id: Uuid,
    payment_provider: &str,
) -> Result<(), OrderError> {
    let result = sqlx::query(
        "UPDATE orders SET status = 'paid', payment_state = 'confirmed',
         payment_provider = $2, updated_at = NOW()
         WHERE id = $1 AND status = 'pending'",
    )
    .bind(order_id)
    .bind(payment_provider)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(OrderError::NotFound);
    }

    Ok(())
}

// ── Internal helpers ──

#[derive(Debug, sqlx::FromRow)]
struct CartPositionRow {
    id: Uuid,
    item_id: Uuid,
    variation_id: Option<Uuid>,
    price: BigDecimal,
    answers: serde_json::Value,
    seat_id: Option<Uuid>,
    item_name: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OrderRow {
    pub id: Uuid,
    pub event_id: Uuid,
    pub code: String,
    pub status: String,
    pub secret: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub locale: String,
    pub total: BigDecimal,
    pub datetime: DateTime<Utc>,
    pub expires: Option<DateTime<Utc>>,
    pub payment_provider: Option<String>,
    pub payment_state: String,
    pub testmode: bool,
    pub require_approval: bool,
    pub valid_if_pending: bool,
    pub sales_channel: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PositionRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub positionid: i32,
    pub item_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub price: BigDecimal,
    pub secret: String,
    pub attendee_name: Option<String>,
    pub attendee_email: Option<String>,
    pub answers: serde_json::Value,
    pub seat_id: Option<Uuid>,
    pub pseudonymization_id: String,
    pub canceled: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub item_name: String,
}

/// Order with its positions, for API responses.
#[derive(Debug, Serialize)]
pub struct OrderWithPositions {
    pub order: OrderRow,
    pub positions: Vec<PositionRow>,
}

/// Check whether a quota has remaining capacity for one more sale.
async fn check_quota_available(
    pool: &PgPool,
    item_id: Uuid,
    variation_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let quotas: Vec<QuotaCheckRow> = if let Some(vid) = variation_id {
        sqlx::query_as(
            "SELECT q.id, q.size
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
            "SELECT q.id, q.size
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

            if sold.0 >= size as i64 {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn generate_order_code() -> String {
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..5).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

fn generate_secret(len: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..len).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

#[derive(Debug, sqlx::FromRow)]
struct QuotaCheckRow {
    #[allow(dead_code)]
    id: Uuid,
    size: Option<i32>,
}
