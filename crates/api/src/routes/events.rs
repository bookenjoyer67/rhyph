use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EventResponse {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub date_from: DateTime<Utc>,
    pub date_to: Option<DateTime<Utc>>,
    pub timezone: String,
    pub live: bool,
    pub presale_start: Option<DateTime<Utc>>,
    pub presale_end: Option<DateTime<Utc>>,
    pub currency: String,
    pub locale: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub max_items_per_order: i32,
    pub reservation_time: i32,
    pub show_remaining_capacity: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date_from: DateTime<Utc>,
    pub date_to: Option<DateTime<Utc>>,
    pub timezone: String,
    pub currency: Option<String>,
    pub locale: Option<String>,
    pub presale_start: Option<DateTime<Utc>>,
    pub presale_end: Option<DateTime<Utc>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub max_items_per_order: Option<i32>,
    pub reservation_time: Option<i32>,
    pub show_remaining_capacity: Option<bool>,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/v1/organizers/{org_slug}/events", get(list_events).post(create_event))
        .route("/api/v1/organizers/{org_slug}/events/{slug}", get(get_event))
        .with_state(pool)
}

async fn lookup_organizer_by_slug(pool: &PgPool, slug: &str) -> Result<Uuid, ApiError> {
    let (id,): (Uuid,) =
        sqlx::query_as("SELECT id FROM organizers WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::NotFound("organizer not found".into()))?;
    Ok(id)
}

async fn list_events(
    State(pool): State<Arc<PgPool>>,
    Path(org_slug): Path<String>,
) -> Result<Json<Vec<EventResponse>>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    tracing::info!(organizer_id = %organizer_id, "listing events");

    let events = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity
         FROM events WHERE organizer_id = $1 ORDER BY date_from DESC",
    )
    .bind(organizer_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(events))
}

async fn create_event(
    State(pool): State<Arc<PgPool>>,
    Path(org_slug): Path<String>,
    Json(body): Json<CreateEventRequest>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    tracing::info!(organizer_id = %organizer_id, slug = %body.slug, name = %body.name, "creating event");

    let id = Uuid::new_v4();
    let ap_url = format!("https://rhyph.example/@{org_slug}/events/{slug}", org_slug = org_slug, slug = body.slug);

    sqlx::query(
        "INSERT INTO events (id, organizer_id, slug, name, description, location,
         date_from, date_to, timezone, live, presale_start, presale_end,
         currency, locale, lat, lon, max_items_per_order, reservation_time,
         ap_url, show_remaining_capacity)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, false, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)",
    )
    .bind(id).bind(organizer_id).bind(&body.slug).bind(&body.name)
    .bind(body.description.as_deref().unwrap_or("")).bind(body.location.as_deref().unwrap_or(""))
    .bind(body.date_from).bind(body.date_to).bind(&body.timezone)
    .bind(body.presale_start).bind(body.presale_end)
    .bind(body.currency.as_deref().unwrap_or("USD")).bind(body.locale.as_deref().unwrap_or("en"))
    .bind(body.lat).bind(body.lon).bind(body.max_items_per_order.unwrap_or(10))
    .bind(body.reservation_time.unwrap_or(30)).bind(&ap_url)
    .bind(body.show_remaining_capacity.unwrap_or(false))
    .execute(&*pool)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict(format!("event slug '{}' already exists", body.slug))
        } else {
            ApiError::Internal(msg)
        }
    })?;

    let event = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity FROM events WHERE id = $1",
    )
    .bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut resp = Json(event).into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn get_event(
    State(pool): State<Arc<PgPool>>,
    Path((org_slug, slug)): Path<(String, String)>,
) -> Result<Json<EventResponse>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    tracing::info!(org_slug = %org_slug, slug = %slug, "fetching event");

    let event = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity FROM events WHERE slug = $1 AND organizer_id = $2",
    )
    .bind(&slug).bind(organizer_id)
    .fetch_optional(&*pool).await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("event not found".into()))?;

    Ok(Json(event))
}
