use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rhyph_core::services::orders::{self, OrderWithPositions};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub email: Option<String>,
    pub locale: Option<String>,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/api/v1/organizers/{org_slug}/events/{event_slug}/orders",
            post(create_order),
        )
        .route(
            "/api/v1/organizers/{org_slug}/events/{event_slug}/orders/{code}",
            get(get_order),
        )
        .route(
            "/api/v1/organizers/{org_slug}/events/{event_slug}/orders/{code}/cancel",
            post(cancel_order),
        )
        .with_state(pool)
}

fn get_session_from_cookie(headers: &HeaderMap) -> Result<String, ApiError> {
    let cookie_header = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    for part in cookie_header.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("cart_session=") {
            return Ok(value.to_string());
        }
    }

    Err(ApiError::Validation("no cart session found — add items to cart first".into()))
}

async fn lookup_event_by_slug(pool: &PgPool, slug: &str) -> Result<Uuid, ApiError> {
    let (id,): (Uuid,) = sqlx::query_as("SELECT id FROM events WHERE slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("event not found".into()))?;
    Ok(id)
}

async fn create_order(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, event_slug)): Path<(String, String)>,
    headers: HeaderMap,
    Json(body): Json<CreateOrderRequest>,
) -> Result<Response, ApiError> {
    let event_id = lookup_event_by_slug(&pool, &event_slug).await?;
    let session_key = get_session_from_cookie(&headers)?;
    let email = body.email.as_deref();
    let locale = body.locale.as_deref().unwrap_or("en");

    tracing::info!(event_id = %event_id, session = %session_key, email = ?email, "creating order");

    let result = orders::create_order(&pool, event_id, &session_key, email, locale)
        .await
        .map_err(map_order_error)?;

    let mut resp = Json(serde_json::json!({
        "order_id": result.order_id,
        "code": result.order_code,
        "secret": result.order_secret,
        "total": result.total.to_string(),
        "position_count": result.position_count,
    }))
    .into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn get_order(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, event_slug, code)): Path<(String, String, String)>,
) -> Result<Json<OrderWithPositions>, ApiError> {
    let event_id = lookup_event_by_slug(&pool, &event_slug).await?;
    tracing::info!(event_id = %event_id, code = %code, "fetching order");

    let order = orders::get_order(&pool, event_id, &code)
        .await
        .map_err(map_order_error)?;

    Ok(Json(order))
}

async fn cancel_order(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, event_slug, code)): Path<(String, String, String)>,
) -> Result<Response, ApiError> {
    let event_id = lookup_event_by_slug(&pool, &event_slug).await?;
    let order = orders::get_order(&pool, event_id, &code)
        .await
        .map_err(map_order_error)?;

    tracing::info!(order_id = %order.order.id, code = %code, "cancelling order");

    orders::cancel_order(&pool, order.order.id)
        .await
        .map_err(map_order_error)?;

    let mut resp = Json(serde_json::json!({ "status": "canceled", "order_id": order.order.id }))
        .into_response();
    *resp.status_mut() = StatusCode::OK;
    Ok(resp)
}

fn map_order_error(e: orders::OrderError) -> ApiError {
    match e {
        orders::OrderError::EmptyCart => ApiError::Validation("cart is empty".into()),
        orders::OrderError::QuotaExceeded(name) => ApiError::Conflict(format!("quota exceeded: {name}")),
        orders::OrderError::NotFound => ApiError::NotFound("order not found".into()),
        orders::OrderError::InvalidTransition { from, to } => {
            ApiError::Validation(format!("invalid status transition from {from} to {to}"))
        }
        orders::OrderError::Database(e) => ApiError::Internal(e.to_string()),
    }
}
