// Integration tests for cart + order pipeline.
// Requires DATABASE_URL pointing to a test PostgreSQL instance.
// Run: DATABASE_URL=postgres://... cargo test -- --nocapture

use bigdecimal::BigDecimal;
use rhyph_core::services::cart::{self, CartError};
use rhyph_core::services::orders;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup(pool: &PgPool) -> (Uuid, Uuid, Uuid, Uuid) {
    // Clean slate
    sqlx::query("DELETE FROM cart_positions").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM order_positions").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM orders").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM quota_items").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM quotas").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM items").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM events").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM organizers").execute(pool).await.unwrap();

    let org: (Uuid,) = sqlx::query_as(
        "INSERT INTO organizers (slug, name) VALUES ('test', 'Test Org') RETURNING id"
    ).fetch_one(pool).await.unwrap();

    let event: (Uuid,) = sqlx::query_as(
        "INSERT INTO events (organizer_id, slug, name, date_from, ap_url, reservation_time)
         VALUES ($1, 'test', 'Test Event', NOW(), $2, 30) RETURNING id"
    ).bind(org.0).bind(format!("https://test/events/{}", Uuid::new_v4()))
    .fetch_one(pool).await.unwrap();

    let item: (Uuid,) = sqlx::query_as(
        "INSERT INTO items (event_id, name, default_price, active, admission)
         VALUES ($1, 'GA', $2, true, true) RETURNING id"
    ).bind(event.0).bind(BigDecimal::from(25))
    .fetch_one(pool).await.unwrap();

    let quota: (Uuid,) = sqlx::query_as(
        "INSERT INTO quotas (event_id, name, size) VALUES ($1, 'Total', 5) RETURNING id"
    ).bind(event.0).fetch_one(pool).await.unwrap();

    sqlx::query("INSERT INTO quota_items (quota_id, item_id) VALUES ($1, $2)")
        .bind(quota.0).bind(item.0).execute(pool).await.unwrap();

    (org.0, event.0, item.0, quota.0)
}

// ── Cart tests ──

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn cart_add_within_quota() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    let result = cart::add_to_cart(&pool, event, item, None, "s1", 3, serde_json::json!([]), None).await;
    assert!(result.is_ok(), "3 within quota of 5 should succeed");
    assert_eq!(result.unwrap().len(), 3);
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn cart_quota_exceeded() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    // Fill quota: 5 items
    cart::add_to_cart(&pool, event, item, None, "s1", 5, serde_json::json!([]), None).await.unwrap();

    // Try 1 more
    let result = cart::add_to_cart(&pool, event, item, None, "s1", 1, serde_json::json!([]), None).await;
    assert!(matches!(result, Err(CartError::QuotaExceeded(_))));
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn cart_max_per_order() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, _item, _quota) = setup(&pool).await;

    // Create item with max_per_order=2
    let limited: (Uuid,) = sqlx::query_as(
        "INSERT INTO items (event_id, name, default_price, active, admission, max_per_order)
         VALUES ($1, 'VIP', $2, true, true, 2) RETURNING id"
    ).bind(event).bind(BigDecimal::from(50))
    .fetch_one(&pool).await.unwrap();

    // Add quota for it
    let q: (Uuid,) = sqlx::query_as(
        "INSERT INTO quotas (event_id, name, size) VALUES ($1, 'VIP Quota', 100) RETURNING id"
    ).bind(event).fetch_one(&pool).await.unwrap();
    sqlx::query("INSERT INTO quota_items (quota_id, item_id) VALUES ($1, $2)")
        .bind(q.0).bind(limited.0).execute(&pool).await.unwrap();

    // 2 should work
    cart::add_to_cart(&pool, event, limited.0, None, "s1", 2, serde_json::json!([]), None).await.unwrap();

    // 3rd should fail
    let result = cart::add_to_cart(&pool, event, limited.0, None, "s1", 1, serde_json::json!([]), None).await;
    assert!(matches!(result, Err(CartError::MaxPerOrderExceeded)));
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn cart_remove_and_verify() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    let positions = cart::add_to_cart(&pool, event, item, None, "s1", 2, serde_json::json!([]), None).await.unwrap();
    assert_eq!(positions.len(), 2);

    cart::remove_from_cart(&pool, positions[0].id, "s1").await.unwrap();

    let remaining = cart::get_cart(&pool, "s1", event).await.unwrap();
    assert_eq!(remaining.len(), 1);
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn cart_remove_wrong_session_denied() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    let positions = cart::add_to_cart(&pool, event, item, None, "alice", 1, serde_json::json!([]), None).await.unwrap();

    let result = cart::remove_from_cart(&pool, positions[0].id, "bob").await;
    assert!(result.is_err());
}

// ── Order tests ──

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn order_create_clears_cart() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    cart::add_to_cart(&pool, event, item, None, "s1", 2, serde_json::json!([]), None).await.unwrap();

    let result = orders::create_order(&pool, event, "s1", Some("fan@test.com"), "en").await.unwrap();
    assert_eq!(result.position_count, 2);

    // Cart empty after order
    let cart = cart::get_cart(&pool, "s1", event).await.unwrap();
    assert!(cart.is_empty());
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn order_cancel_updates_status() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, item, _quota) = setup(&pool).await;

    cart::add_to_cart(&pool, event, item, None, "s1", 1, serde_json::json!([]), None).await.unwrap();
    let result = orders::create_order(&pool, event, "s1", Some("fan@test.com"), "en").await.unwrap();

    orders::cancel_order(&pool, result.order_id).await.unwrap();

    let order = orders::get_order(&pool, event, &result.order_code).await.unwrap();
    assert_eq!(order.order.status, "canceled");
}

#[tokio::test]
#[ignore = "requires DATABASE_URL"]
async fn order_empty_cart_fails() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let (_org, event, _item, _quota) = setup(&pool).await;

    let result = orders::create_order(&pool, event, "empty-session", None, "en").await;
    assert!(result.is_err());
}
