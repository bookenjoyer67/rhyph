use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
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
    pub is_local: bool,
    pub ap_url: String,
    pub maximum_attendee_capacity: Option<i32>,
    pub remaining_attendee_capacity: Option<i32>,
    pub show_remaining_capacity: bool,
}
