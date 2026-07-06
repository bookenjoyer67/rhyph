use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Quota availability states.
/// Stored as TEXT in PostgreSQL. Values: "gone", "ordered", "reserved", "ok"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaAvailability {
    Gone = 0,
    Ordered = 10,
    Reserved = 20,
    Ok = 100,
}

impl QuotaAvailability {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuotaAvailability::Gone => "gone",
            QuotaAvailability::Ordered => "ordered",
            QuotaAvailability::Reserved => "reserved",
            QuotaAvailability::Ok => "ok",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gone" => Some(QuotaAvailability::Gone),
            "ordered" => Some(QuotaAvailability::Ordered),
            "reserved" => Some(QuotaAvailability::Reserved),
            "ok" => Some(QuotaAvailability::Ok),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quota {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub size: Option<i32>,
    pub subevent_id: Option<Uuid>,
    pub close_when_sold_out: bool,
    pub ignore_for_event_availability: bool,
}
