use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: Uuid,
    pub organizer_id: Uuid,
    pub name: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
}
