use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use rhyph_core::Device;

use crate::error::ApiError;
use crate::middleware::auth::RequireAdmin;
use crate::services::auth as auth_svc;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDeviceRequest {
    pub organizer_id: Uuid,
    pub name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateDeviceResponse {
    pub id: Uuid,
    pub api_key: String,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/admin/devices", get(list_devices).post(create_device))
        .with_state(pool)
}

async fn login_handler(
    State(pool): State<Arc<PgPool>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<auth_svc::LoginResponse>, ApiError> {
    let response = auth_svc::login(&pool, &body.email, &body.password).await?;
    Ok(Json(response))
}

async fn create_device(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Json(body): Json<CreateDeviceRequest>,
) -> Result<Json<CreateDeviceResponse>, ApiError> {
    let api_key = auth_svc::generate_api_key();
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO devices (id, organizer_id, name, api_key, created_at) VALUES ($1, $2, $3, $4, NOW())",
    )
    .bind(id)
    .bind(body.organizer_id)
    .bind(&body.name)
    .bind(&api_key)
    .execute(&*pool)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict("api key collision".into())
        } else {
            ApiError::Internal(msg)
        }
    })?;

    Ok(Json(CreateDeviceResponse { id, api_key }))
}

async fn list_devices(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
) -> Result<Json<Vec<Device>>, ApiError> {
    let devices = sqlx::query_as::<_, Device>(
        "SELECT id, organizer_id, name, api_key, created_at FROM devices ORDER BY created_at DESC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(devices))
}
