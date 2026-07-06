use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SeatingPlan {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub name: String,
    pub layout: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Seat {
    pub id: Uuid,
    pub event_id: Uuid,
    pub seating_plan_id: Uuid,
    pub seat_guid: String,
    pub zone_name: String,
    pub row_name: String,
    pub seat_number: String,
    pub x: f64,
    pub y: f64,
    pub category: Option<String>,
    pub blocked: bool,
    pub order_position_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SeatCategoryMapping {
    pub id: Uuid,
    pub event_id: Uuid,
    pub category_name: String,
    pub item_id: Uuid,
    pub price: BigDecimal,
}
