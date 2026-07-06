use axum::{Router, routing::get};
use rhyph_api::routes::{cart, checkin, events, orders};
use sqlx::PgPool;
use std::sync::Arc;

pub fn build(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(cart::routes(pool.clone()))
        .merge(orders::routes(pool.clone()))
        .merge(events::routes(pool.clone()))
        .merge(checkin::routes(pool.clone()))
}

async fn health() -> &'static str {
    "ok"
}
