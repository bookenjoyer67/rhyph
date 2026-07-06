use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use rhyph_core::services::{cart, orders};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

mod app;
mod spa_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Handle subcommands
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "hashpw" {
        let password = args.get(2).expect("usage: rhyph-server hashpw <password>");
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("hash failed: {e}"))?
            .to_string();
        println!("{hash}");
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("../../crates/core/src/db/migrations")
        .run(&pool)
        .await?;

    // Background cleanup task: expire carts + pending orders every 60s
    let cleanup_pool = pool.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            if let Err(e) = cart::clear_expired(&cleanup_pool).await {
                tracing::warn!("cart cleanup failed: {e}");
            }
            if let Err(e) = orders::expire_pending(&cleanup_pool).await {
                tracing::warn!("order expiry failed: {e}");
            }
        }
    });

    let pool = Arc::new(pool);
    let app = app::build(pool)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Rhyph listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
