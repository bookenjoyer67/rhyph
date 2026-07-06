use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn build(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(pool)
}

async fn health() -> &'static str {
    "ok"
}
