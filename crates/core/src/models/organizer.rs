use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organizer {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub theme: serde_json::Value,
    pub custom_domain: Option<String>,
    pub has_custom_spa: bool,
    pub spa_updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganizerRequest {
    pub name: Option<String>,
    pub theme: Option<serde_json::Value>,
    pub custom_domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizerPublic {
    pub slug: String,
    pub name: String,
    pub theme: serde_json::Value,
    pub custom_domain: Option<String>,
}
