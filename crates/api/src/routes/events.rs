use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch},
    Json, Router,
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;

// ── Event types ──

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

#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub live: Option<bool>,
    pub presale_start: Option<DateTime<Utc>>,
    pub presale_end: Option<DateTime<Utc>>,
    pub currency: Option<String>,
    pub max_items_per_order: Option<i32>,
    pub reservation_time: Option<i32>,
    pub show_remaining_capacity: Option<bool>,
}

// ── Item types ──

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ItemResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub default_price: BigDecimal,
    pub tax_rate: BigDecimal,
    pub active: bool,
    pub admission: bool,
    pub max_per_order: Option<i32>,
    pub position: i32,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub default_price: BigDecimal,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub admission: Option<bool>,
    pub max_per_order: Option<i32>,
    pub category_id: Option<Uuid>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
}

// ── Quota types ──

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QuotaResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub size: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuotaRequest {
    pub name: String,
    pub size: Option<i32>,
    pub item_ids: Vec<Uuid>,
}

// ── Checkin list types ──

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CheckinListResponse {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub all_products: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckinListRequest {
    pub name: String,
    pub all_products: Option<bool>,
}

// ── Routes ──

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        // Event CRUD
        .route("/api/v1/organizers/{org_slug}/events", get(list_events).post(create_event))
        .route("/api/v1/organizers/{org_slug}/events/{slug}", get(get_event).patch(update_event))
        // Items
        .route("/api/v1/organizers/{org_slug}/events/{slug}/items", get(list_items).post(create_item))
        .route("/api/v1/organizers/{org_slug}/events/{slug}/items/{id}", patch(update_item))
        // Quotas
        .route("/api/v1/organizers/{org_slug}/events/{slug}/quotas", get(list_quotas).post(create_quota))
        .route("/api/v1/organizers/{org_slug}/events/{slug}/quotas/{id}", axum::routing::delete(delete_quota))
        // Checkin lists
        .route("/api/v1/organizers/{org_slug}/events/{slug}/checkin-lists", get(list_checkin_lists).post(create_checkin_list))
        // Public
        .route("/api/v1/events/{org_slug}/{event_slug}", get(get_public_event))
        .with_state(pool)
}

// ── Helpers ──

async fn lookup_organizer_by_slug(pool: &PgPool, slug: &str) -> Result<Uuid, ApiError> {
    let (id,): (Uuid,) =
        sqlx::query_as("SELECT id FROM organizers WHERE slug = $1")
            .bind(slug).fetch_optional(pool).await
            .map_err(|e| ApiError::Internal(e.to_string()))?
            .ok_or_else(|| ApiError::NotFound("organizer not found".into()))?;
    Ok(id)
}

async fn lookup_event(pool: &PgPool, organizer_id: Uuid, slug: &str) -> Result<(Uuid, EventResponse), ApiError> {
    let event = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity
         FROM events WHERE slug = $1 AND organizer_id = $2",
    )
    .bind(slug).bind(organizer_id)
    .fetch_optional(pool).await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("event not found".into()))?;
    Ok((event.id, event))
}

// ── Event handlers ──

async fn list_events(
    State(pool): State<Arc<PgPool>>, Path(org_slug): Path<String>,
) -> Result<Json<Vec<EventResponse>>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let events = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity
         FROM events WHERE organizer_id = $1 ORDER BY date_from DESC",
    ).bind(organizer_id).fetch_all(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(events))
}

async fn create_event(
    State(pool): State<Arc<PgPool>>, Path(org_slug): Path<String>,
    Json(body): Json<CreateEventRequest>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
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
    .execute(&*pool).await.map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict(format!("event slug '{}' already exists", body.slug))
        } else { ApiError::Internal(msg) }
    })?;

    let event = sqlx::query_as::<_, EventResponse>(
        "SELECT id, organizer_id, slug, name, description, location,
                date_from, date_to, timezone, live, presale_start, presale_end,
                currency, locale, lat, lon, max_items_per_order, reservation_time,
                show_remaining_capacity FROM events WHERE id = $1",
    ).bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut resp = Json(event).into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn get_event(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
) -> Result<Json<EventResponse>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (_, event) = lookup_event(&pool, organizer_id, &slug).await?;
    Ok(Json(event))
}

async fn update_event(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
    Json(body): Json<UpdateEventRequest>,
) -> Result<Json<EventResponse>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    if let Some(name) = &body.name {
        sqlx::query("UPDATE events SET name = $1 WHERE id = $2").bind(name).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(desc) = &body.description {
        sqlx::query("UPDATE events SET description = $1 WHERE id = $2").bind(desc).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(loc) = &body.location {
        sqlx::query("UPDATE events SET location = $1 WHERE id = $2").bind(loc).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(df) = body.date_from {
        sqlx::query("UPDATE events SET date_from = $1 WHERE id = $2").bind(df).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if body.date_to.is_some() {
        sqlx::query("UPDATE events SET date_to = $1 WHERE id = $2").bind(body.date_to).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(tz) = &body.timezone {
        sqlx::query("UPDATE events SET timezone = $1 WHERE id = $2").bind(tz).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(live) = body.live {
        sqlx::query("UPDATE events SET live = $1 WHERE id = $2").bind(live).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(ps) = body.presale_start {
        sqlx::query("UPDATE events SET presale_start = $1 WHERE id = $2").bind(ps).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if body.presale_end.is_some() {
        sqlx::query("UPDATE events SET presale_end = $1 WHERE id = $2").bind(body.presale_end).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(cur) = &body.currency {
        sqlx::query("UPDATE events SET currency = $1 WHERE id = $2").bind(cur).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(mpo) = body.max_items_per_order {
        sqlx::query("UPDATE events SET max_items_per_order = $1 WHERE id = $2").bind(mpo).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(rt) = body.reservation_time {
        sqlx::query("UPDATE events SET reservation_time = $1 WHERE id = $2").bind(rt).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }
    if let Some(src) = body.show_remaining_capacity {
        sqlx::query("UPDATE events SET show_remaining_capacity = $1 WHERE id = $2").bind(src).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }

    let (_, event) = lookup_event(&pool, organizer_id, &slug).await?;
    Ok(Json(event))
}

async fn get_public_event(
    State(pool): State<Arc<PgPool>>, Path((org_slug, event_slug)): Path<(String, String)>,
) -> Result<Json<EventResponse>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (_, event) = lookup_event(&pool, organizer_id, &event_slug).await?;
    Ok(Json(event))
}

// ── Item handlers ──

async fn list_items(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
) -> Result<Json<Vec<ItemResponse>>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    let items = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, event_id, category_id, name, description, default_price, tax_rate,
                active, admission, max_per_order, position, available_from, available_until
         FROM items WHERE event_id = $1 ORDER BY position",
    ).bind(event_id).fetch_all(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(items))
}

async fn create_item(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
    Json(body): Json<CreateItemRequest>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO items (id, event_id, category_id, name, description, default_price,
         active, admission, max_per_order, available_from, available_until)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(id).bind(event_id).bind(body.category_id).bind(&body.name)
    .bind(body.description.as_deref().unwrap_or(""))
    .bind(&body.default_price)
    .bind(body.active.unwrap_or(true)).bind(body.admission.unwrap_or(true))
    .bind(body.max_per_order).bind(body.available_from).bind(body.available_until)
    .execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let item = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, event_id, category_id, name, description, default_price, tax_rate,
                active, admission, max_per_order, position, available_from, available_until
         FROM items WHERE id = $1",
    ).bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut resp = Json(item).into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn update_item(
    State(pool): State<Arc<PgPool>>,
    Path((org_slug, slug, id)): Path<(String, String, Uuid)>,
    Json(body): Json<CreateItemRequest>,
) -> Result<Json<ItemResponse>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    sqlx::query(
        "UPDATE items SET name = $1, default_price = $2, description = $3,
         active = COALESCE($4, active), max_per_order = $5, category_id = $6,
         available_from = $7, available_until = $8
         WHERE id = $9 AND event_id = $10",
    )
    .bind(&body.name).bind(&body.default_price).bind(body.description.as_deref().unwrap_or(""))
    .bind(body.active).bind(body.max_per_order).bind(body.category_id)
    .bind(body.available_from).bind(body.available_until)
    .bind(id).bind(event_id)
    .execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let item = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, event_id, category_id, name, description, default_price, tax_rate,
                active, admission, max_per_order, position, available_from, available_until
         FROM items WHERE id = $1",
    ).bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(item))
}

// ── Quota handlers ──

async fn list_quotas(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
) -> Result<Json<Vec<QuotaResponse>>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    let quotas = sqlx::query_as::<_, QuotaResponse>(
        "SELECT id, event_id, name, size FROM quotas WHERE event_id = $1 ORDER BY name",
    ).bind(event_id).fetch_all(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(quotas))
}

async fn create_quota(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
    Json(body): Json<CreateQuotaRequest>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO quotas (id, event_id, name, size) VALUES ($1, $2, $3, $4)")
        .bind(id).bind(event_id).bind(&body.name).bind(body.size)
        .execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    for item_id in &body.item_ids {
        sqlx::query("INSERT INTO quota_items (quota_id, item_id) VALUES ($1, $2)")
            .bind(id).bind(item_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    }

    let quota = sqlx::query_as::<_, QuotaResponse>(
        "SELECT id, event_id, name, size FROM quotas WHERE id = $1",
    ).bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut resp = Json(quota).into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}

async fn delete_quota(
    State(pool): State<Arc<PgPool>>,
    Path((org_slug, slug, id)): Path<(String, String, Uuid)>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    sqlx::query("DELETE FROM quota_items WHERE quota_id = $1").bind(id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    let rows = sqlx::query("DELETE FROM quotas WHERE id = $1 AND event_id = $2").bind(id).bind(event_id).execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    if rows.rows_affected() == 0 {
        return Err(ApiError::NotFound("quota not found".into()));
    }

    let mut resp = Json(serde_json::json!({"deleted": true})).into_response();
    *resp.status_mut() = StatusCode::OK;
    Ok(resp)
}

// ── Checkin list handlers ──

async fn list_checkin_lists(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
) -> Result<Json<Vec<CheckinListResponse>>, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;

    let lists = sqlx::query_as::<_, CheckinListResponse>(
        "SELECT id, event_id, name, all_products FROM checkin_lists WHERE event_id = $1 ORDER BY name",
    ).bind(event_id).fetch_all(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(lists))
}

async fn create_checkin_list(
    State(pool): State<Arc<PgPool>>, Path((org_slug, slug)): Path<(String, String)>,
    Json(body): Json<CreateCheckinListRequest>,
) -> Result<Response, ApiError> {
    let organizer_id = lookup_organizer_by_slug(&pool, &org_slug).await?;
    let (event_id, _) = lookup_event(&pool, organizer_id, &slug).await?;
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO checkin_lists (id, event_id, name, all_products) VALUES ($1, $2, $3, $4)",
    )
    .bind(id).bind(event_id).bind(&body.name).bind(body.all_products.unwrap_or(true))
    .execute(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, CheckinListResponse>(
        "SELECT id, event_id, name, all_products FROM checkin_lists WHERE id = $1",
    ).bind(id).fetch_one(&*pool).await.map_err(|e| ApiError::Internal(e.to_string()))?;

    let mut resp = Json(list).into_response();
    *resp.status_mut() = StatusCode::CREATED;
    Ok(resp)
}
