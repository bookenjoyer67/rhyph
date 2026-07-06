use axum::{middleware, Router, routing::get};
use rhyph_api::middleware::rate_limit::{self, RateLimiter};
use rhyph_api::routes::{auth, cart, checkin, events, orders};
use sqlx::PgPool;
use std::sync::Arc;

pub fn build(pool: Arc<PgPool>) -> Router {
    let limiter = Arc::new(RateLimiter::new(200, 60));

    Router::new()
        .route("/health", get(health))
        .merge(cart::routes(pool.clone()))
        .merge(orders::routes(pool.clone()))
        .merge(events::routes(pool.clone()))
        .merge(checkin::routes(pool.clone()))
        .merge(auth::routes(pool.clone()))
        .layer(middleware::from_fn_with_state(
            limiter,
            rate_limit::rate_limit_middleware,
        ))
}

async fn health() -> &'static str {
    "ok"
}
