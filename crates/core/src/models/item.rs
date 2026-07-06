use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidityMode {
    EventDefault,
    Fixed,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaPolicy {
    NoPolicy,
    ReusableMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub event_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub default_price: BigDecimal,
    pub tax_rate: BigDecimal,
    pub active: bool,
    pub admission: bool,
    pub personalized: bool,
    pub max_per_order: Option<i32>,
    pub min_per_order: Option<i32>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub require_voucher: bool,
    pub hide_without_voucher: bool,
    pub require_approval: bool,
    pub generate_giftcard: bool,
    pub checkin_attention: bool,
    pub validity_mode: String,
    pub validity_fixed_from: Option<DateTime<Utc>>,
    pub validity_fixed_until: Option<DateTime<Utc>>,
    pub validity_dynamic_duration_minutes: Option<i32>,
    pub media_policy: String,
    pub picture_id: Option<Uuid>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemVariation {
    pub id: Uuid,
    pub item_id: Uuid,
    pub value: String,
    pub default_price: Option<BigDecimal>,
    pub active: bool,
    pub position: i32,
    pub require_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemCategory {
    pub id: Uuid,
    pub event_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub is_addon: bool,
}
