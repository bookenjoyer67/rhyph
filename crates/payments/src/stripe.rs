// Stripe payment provider implementation.
// Requires STRIPE_SECRET_KEY and STRIPE_WEBHOOK_SECRET env vars.

use async_trait::async_trait;
use rhyph_core::services::orders;
use uuid::Uuid;

use crate::{PaymentError, PaymentEvent, PaymentProvider, PaymentStatus};

pub struct StripeProvider {
    client: reqwest::Client,
    secret_key: String,
    webhook_secret: String,
}

impl StripeProvider {
    pub fn new(secret_key: String, webhook_secret: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            secret_key,
            webhook_secret,
        }
    }

    /// Create a Stripe Checkout Session and return the redirect URL.
    async fn create_checkout(
        &self,
        amount_cents: i64,
        currency: &str,
        order_id: Uuid,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<String, PaymentError> {
        let resp = self
            .client
            .post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(&self.secret_key, Some(""))
            .form(&[
                ("mode", "payment"),
                ("success_url", success_url),
                ("cancel_url", cancel_url),
                (
                    "line_items[0][price_data][currency]",
                    currency,
                ),
                (
                    "line_items[0][price_data][product_data][name]",
                    "Event Ticket",
                ),
                (
                    "line_items[0][price_data][unit_amount]",
                    &amount_cents.to_string(),
                ),
                ("line_items[0][quantity]", "1"),
                (
                    "metadata[order_id]",
                    &order_id.to_string(),
                ),
            ])
            .send()
            .await
            .map_err(|e| PaymentError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(PaymentError::Provider(body));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| PaymentError::Provider(e.to_string()))?;

        json["url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| PaymentError::Provider("no checkout URL in response".into()))
    }

    /// Verify a Stripe webhook signature.
    pub fn verify_signature(
        &self,
        payload: &str,
        sig_header: &str,
    ) -> Result<PaymentEvent, PaymentError> {
        // Use a simple HMAC comparison for now — in production use stripe::Webhook::construct_event
        let timestamp = sig_header
            .split(',')
            .find(|p| p.starts_with("t="))
            .and_then(|p| p.strip_prefix("t="))
            .unwrap_or("0");

        let signature = sig_header
            .split(',')
            .find(|p| p.starts_with("v1="))
            .and_then(|p| p.strip_prefix("v1="));

        if signature.is_none() {
            return Err(PaymentError::InvalidSignature);
        }

        // Compute expected signature
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let signed_payload = format!("{}.{}", timestamp, payload);
        let mut mac =
            HmacSha256::new_from_slice(self.webhook_secret.as_bytes())
                .map_err(|_| PaymentError::InvalidSignature)?;
        mac.update(signed_payload.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());

        if expected != signature.unwrap() {
            return Err(PaymentError::InvalidSignature);
        }

        // Parse the event JSON
        let event: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| PaymentError::Provider(e.to_string()))?;

        let event_type = event["type"].as_str().unwrap_or("");
        let order_id_str = event["data"]["object"]["metadata"]["order_id"]
            .as_str()
            .unwrap_or("");

        let order_id = Uuid::parse_str(order_id_str)
            .map_err(|_| PaymentError::Provider("invalid order_id in webhook".into()))?;

        let status = match event_type {
            "checkout.session.completed" => PaymentStatus::Completed,
            "checkout.session.expired" => PaymentStatus::Failed,
            _ => return Err(PaymentError::Provider(format!("unhandled event: {event_type}"))),
        };

        Ok(PaymentEvent {
            order_id,
            status,
            transaction_id: event["data"]["object"]["id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        })
    }
}

#[async_trait]
impl PaymentProvider for StripeProvider {
    async fn create_checkout_session(
        &self,
        order_id: Uuid,
        amount: bigdecimal::BigDecimal,
        currency: &str,
    ) -> Result<String, PaymentError> {
        // Convert to cents
        let amount_cents = (amount * bigdecimal::BigDecimal::from(100))
            .to_string()
            .parse::<i64>()
            .unwrap_or(0);

        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
        let success_url = format!("{base_url}/orders/{order_id}?paid=true");
        let cancel_url = format!("{base_url}/orders/{order_id}?canceled=true");

        self.create_checkout(amount_cents, currency, order_id, &success_url, &cancel_url)
            .await
    }

    async fn verify_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<PaymentEvent, PaymentError> {
        self.verify_signature(payload, signature)
    }
}

/// Process a payment webhook: verify, then mark order as paid.
pub async fn handle_stripe_webhook(
    pool: &sqlx::PgPool,
    provider: &StripeProvider,
    payload: &str,
    signature: &str,
) -> Result<(), PaymentError> {
    let event = provider.verify_webhook(payload, signature).await?;

    match event.status {
        PaymentStatus::Completed => {
            orders::mark_paid(pool, event.order_id, "stripe")
                .await
                .map_err(|e| PaymentError::Provider(e.to_string()))?;
            tracing::info!(order_id = %event.order_id, txn = %event.transaction_id, "payment completed");
        }
        PaymentStatus::Failed => {
            tracing::warn!(order_id = %event.order_id, "payment failed");
        }
        _ => {}
    }

    Ok(())
}
