use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Paid,
    Expired,
    Canceled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Expired => "expired",
            OrderStatus::Canceled => "canceled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(OrderStatus::Pending),
            "paid" => Some(OrderStatus::Paid),
            "expired" => Some(OrderStatus::Expired),
            "canceled" => Some(OrderStatus::Canceled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentState {
    Created,
    Pending,
    Confirmed,
    Failed,
    Refunded,
}

impl PaymentState {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentState::Created => "created",
            PaymentState::Pending => "pending",
            PaymentState::Confirmed => "confirmed",
            PaymentState::Failed => "failed",
            PaymentState::Refunded => "refunded",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "created" => Some(PaymentState::Created),
            "pending" => Some(PaymentState::Pending),
            "confirmed" => Some(PaymentState::Confirmed),
            "failed" => Some(PaymentState::Failed),
            "refunded" => Some(PaymentState::Refunded),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub event_id: Uuid,
    pub code: String,
    pub status: String,
    pub secret: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub locale: String,
    pub total: BigDecimal,
    pub datetime: DateTime<Utc>,
    pub expires: Option<DateTime<Utc>>,
    pub payment_provider: Option<String>,
    pub payment_state: String,
    pub customer_id: Option<Uuid>,
    pub testmode: bool,
    pub require_approval: bool,
    pub valid_if_pending: bool,
    pub comment: Option<String>,
    pub sales_channel: String,
    pub invoice_name: Option<String>,
    pub invoice_company: Option<String>,
    pub invoice_street: Option<String>,
    pub invoice_city: Option<String>,
    pub invoice_zip: Option<String>,
    pub invoice_country: Option<String>,
    pub invoice_vat_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderPosition {
    pub id: Uuid,
    pub order_id: Uuid,
    pub positionid: i32,
    pub item_id: Uuid,
    pub variation_id: Option<Uuid>,
    pub price: BigDecimal,
    pub tax_rate: BigDecimal,
    pub tax_value: BigDecimal,
    pub secret: String,
    pub attendee_name: Option<String>,
    pub attendee_email: Option<String>,
    pub answers: JsonValue,
    pub seat_id: Option<Uuid>,
    pub pseudonymization_id: String,
    pub canceled: bool,
    pub blocked: Option<JsonValue>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub voucher_budget_use: Option<BigDecimal>,
}
