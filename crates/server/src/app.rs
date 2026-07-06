use axum::{middleware, Router, routing::get};
use rhyph_api::middleware::rate_limit::{self, RateLimiter};
use rhyph_api::routes::{auth, cart, checkin, events, orders, admin};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use crate::spa_config::SpaConfigLayer;

pub fn build(pool: Arc<PgPool>) -> Router {
    let limiter = Arc::new(RateLimiter::new(200, 60));

    // Resolve paths relative to project root (server crate is in crates/server/)
    let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap().to_path_buf();

    // Config injection for the default organizer's custom SPA
    let spa_dir = project_root.join("public_pages/default/spa");
    let spa_dir_str = spa_dir.to_str().unwrap();
    let spa_layer = SpaConfigLayer::new("default", "The Neon Cathedral", "{\"primary_color\":\"#FF1493\",\"accent_color\":\"#00E676\"}");
    let spa_service = tower::ServiceBuilder::new()
        .layer(spa_layer)
        .service(ServeDir::new(&spa_dir).fallback(ServeFile::new(format!("{}/index.html", spa_dir_str))));

    Router::new()
        .route("/health", get(health))
        .merge(cart::routes(pool.clone()))
        .merge(orders::routes(pool.clone()))
        .merge(events::routes(pool.clone()))
        .merge(checkin::routes(pool.clone()))
        .merge(auth::routes(pool.clone()))
        .merge(admin::routes(pool.clone()))
        // Serve uploaded organizer images
        .nest_service("/public_pages", ServeDir::new(project_root.join("public_pages")))
        // Custom SPA as the main frontend (catch-all for non-API routes)
        .fallback_service(spa_service)
        .layer(middleware::from_fn_with_state(
            limiter,
            rate_limit::rate_limit_middleware,
        ))
}

async fn health() -> &'static str {
    "ok"
}
