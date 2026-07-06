use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use rhyph_core::services::tickets::{self, CheckinStats, TicketValidation};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub secret: String,
    pub nonce: String,
    pub device_id: Option<Uuid>,
    pub gate_id: Option<Uuid>,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/v1/checkin/lists/{list_id}/scan", post(validate_ticket))
        .route("/api/v1/checkin/lists/{list_id}/stats", get(get_checkin_stats))
        .with_state(pool)
}

async fn validate_ticket(
    State(pool): State<Arc<PgPool>>,
    Path(list_id): Path<Uuid>,
    Json(body): Json<ScanRequest>,
) -> Result<Json<TicketValidation>, ApiError> {
    tracing::info!(list_id = %list_id, device_id = ?body.device_id, "validating ticket");

    let result = tickets::validate_ticket(
        &pool, &body.secret, list_id, body.device_id, body.gate_id, &body.nonce,
    )
    .await
    .map_err(map_ticket_error)?;

    Ok(Json(result))
}

async fn get_checkin_stats(
    State(pool): State<Arc<PgPool>>,
    Path(list_id): Path<Uuid>,
) -> Result<Json<CheckinStats>, ApiError> {
    tracing::info!(list_id = %list_id, "fetching checkin stats");

    let stats = tickets::get_checkin_stats(&pool, list_id)
        .await
        .map_err(map_ticket_error)?;

    Ok(Json(stats))
}

fn map_ticket_error(e: tickets::TicketError) -> ApiError {
    match e {
        tickets::TicketError::NotFound => ApiError::NotFound("ticket not found".into()),
        tickets::TicketError::Canceled => ApiError::Validation("ticket is canceled".into()),
        tickets::TicketError::Unpaid => ApiError::Validation("order not paid".into()),
        tickets::TicketError::Blocked(reason) => ApiError::Validation(format!("ticket blocked: {reason}")),
        tickets::TicketError::NotYetValid => ApiError::Validation("ticket not yet valid".into()),
        tickets::TicketError::Expired => ApiError::Validation("ticket expired".into()),
        tickets::TicketError::Database(e) => ApiError::Internal(e.to_string()),
    }
}
