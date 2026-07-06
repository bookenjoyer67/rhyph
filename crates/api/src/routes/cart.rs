use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, post},
    Json, Router,
};
use rhyph_core::services::cart::{self};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub item_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub answers: Option<serde_json::Value>,
    pub seat_id: Option<Uuid>,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/api/v1/organizers/{org_slug}/events/{event_slug}/cart",
            post(add_to_cart).get(get_cart_handler),
        )
        .route(
            "/api/v1/organizers/{org_slug}/events/{event_slug}/cart/{position_id}",
            delete(remove_from_cart_handler),
        )
        .with_state(pool)
}

fn get_or_create_session(headers: &HeaderMap) -> (String, Option<String>) {
    let cookie_header = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    for part in cookie_header.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("cart_session=") {
            return (value.to_string(), None);
        }
    }

    let new_id = Uuid::new_v4().to_string();
    let cookie = format!(
        "cart_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=1800",
        new_id
    );
    (new_id, Some(cookie))
}

fn insert_set_cookie(response: &mut Response, cookie: &str) {
    response
        .headers_mut()
        .insert("set-cookie", cookie.parse().unwrap());
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

async fn add_to_cart(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, event_slug)): Path<(String, String)>,
    headers: HeaderMap,
    Json(body): Json<AddToCartRequest>,
) -> Result<Response, ApiError> {
    let event_id = lookup_event_by_slug(&pool, &event_slug).await?;
    let (session_key, new_cookie) = get_or_create_session(&headers);

    let quantity = body.quantity.unwrap_or(1);
    let answers = body.answers.unwrap_or(serde_json::Value::Null);

    let positions = cart::add_to_cart(
        &pool, event_id, body.item_id, body.variation_id,
        &session_key, quantity, answers, body.seat_id,
    )
    .await
    .map_err(map_cart_error)?;

    let mut resp = Json(positions).into_response();
    if let Some(cookie) = new_cookie {
        insert_set_cookie(&mut resp, &cookie);
    }
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn get_cart_handler(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, event_slug)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let event_id = lookup_event_by_slug(&pool, &event_slug).await?;
    let (session_key, new_cookie) = get_or_create_session(&headers);

    let positions = cart::get_cart(&pool, &session_key, event_id)
        .await
        .map_err(map_cart_error)?;

    let mut resp = Json(positions).into_response();
    if let Some(cookie) = new_cookie {
        insert_set_cookie(&mut resp, &cookie);
    }
    Ok(resp)
}

async fn remove_from_cart_handler(
    State(pool): State<Arc<PgPool>>,
    Path((_org_slug, _event_slug, position_id)): Path<(String, String, Uuid)>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let (session_key, new_cookie) = get_or_create_session(&headers);

    cart::remove_from_cart(&pool, position_id, &session_key)
        .await
        .map_err(map_cart_error)?;

    let mut resp = Json(serde_json::json!({ "removed": true })).into_response();
    if let Some(cookie) = new_cookie {
        insert_set_cookie(&mut resp, &cookie);
    }
    Ok(resp)
}

fn map_cart_error(e: cart::CartError) -> ApiError {
    match e {
        cart::CartError::ItemNotFound => ApiError::NotFound("item not found".into()),
        cart::CartError::QuotaExceeded(name) => ApiError::Conflict(format!("quota exceeded: {name}")),
        cart::CartError::NotAvailable => ApiError::Validation("item not available".into()),
        cart::CartError::MaxPerOrderExceeded => ApiError::Validation("max per order exceeded".into()),
        cart::CartError::Database(e) => ApiError::Internal(e.to_string()),
    }
}
