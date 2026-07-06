use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckinType {
    Entry,
    Exit,
}

impl CheckinType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckinType::Entry => "entry",
            CheckinType::Exit => "exit",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "entry" => Some(CheckinType::Entry),
            "exit" => Some(CheckinType::Exit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CheckinList {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub all_products: bool,
    pub include_pending: bool,
    pub allow_multiple_entries: bool,
    pub allow_entry_after_exit: bool,
    pub rules: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Checkin {
    pub id: Uuid,
    pub position_id: Uuid,
    pub list_id: Uuid,
    pub datetime: DateTime<Utc>,
    #[serde(rename = "type")]
    pub type_: String,
    pub successful: bool,
    pub error_reason: Option<String>,
    pub device_id: Option<Uuid>,
    pub gate_id: Option<Uuid>,
    pub nonce: Option<String>,
    pub forced: bool,
}
