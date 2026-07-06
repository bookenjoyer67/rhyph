pub trait PaymentProvider: Send + Sync {
    async fn create_checkout_session(
        &self,
        order_id: uuid::Uuid,
        amount: rust_decimal::Decimal,
        currency: &str,
    ) -> Result<String, PaymentError>;

    async fn verify_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<PaymentEvent, PaymentError>;
}

#[derive(Debug)]
pub struct PaymentEvent {
    pub order_id: uuid::Uuid,
    pub status: PaymentStatus,
    pub transaction_id: String,
}

#[derive(Debug)]
pub enum PaymentStatus {
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Provider error: {0}")]
    Provider(String),
    #[error("Invalid signature")]
    InvalidSignature,
}
