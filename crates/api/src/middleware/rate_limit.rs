// Rate limiting middleware using a simple in-memory counter.
// Production should use Redis, but this is enough for a single-instance venue.

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// A simple token-bucket-ish rate limiter.
pub struct RateLimiter {
    /// Map of client key -> (window_start, count)
    windows: Mutex<HashMap<String, (Instant, u32)>>,
    max_requests: u32,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            windows: Mutex::new(HashMap::new()),
            max_requests,
            window_secs,
        }
    }

    /// Check if a request should be allowed. Returns Ok(()) or 429 response.
    pub fn check(&self, key: &str) -> Result<(), Response> {
        let mut windows = self.windows.lock().unwrap();
        let now = Instant::now();
        let window_duration = std::time::Duration::from_secs(self.window_secs);

        let entry = windows.entry(key.to_string()).or_insert((now, 0));

        if now.duration_since(entry.0) > window_duration {
            // Reset window
            *entry = (now, 1);
            Ok(())
        } else if entry.1 >= self.max_requests {
            let retry_after = window_duration
                .saturating_sub(now.duration_since(entry.0))
                .as_secs();
            let mut resp = axum::Json(serde_json::json!({
                "error": "rate limit exceeded",
                "retry_after_secs": retry_after
            }))
            .into_response();
            *resp.status_mut() = StatusCode::TOO_MANY_REQUESTS;
            resp.headers_mut().insert(
                "retry-after",
                retry_after.to_string().parse().unwrap(),
            );
            Err(resp)
        } else {
            entry.1 += 1;
            Ok(())
        }
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        // Each clone gets a fresh counter — the original is shared via Arc
        Self {
            windows: Mutex::new(HashMap::new()),
            max_requests: self.max_requests,
            window_secs: self.window_secs,
        }
    }
}

/// Axum middleware layer for rate limiting.
pub async fn rate_limit_middleware(
    axum::extract::State(limiter): axum::extract::State<std::sync::Arc<RateLimiter>>,
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, Response> {
    // Derive client key from X-Forwarded-For or remote addr
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    limiter.check(client_ip)?;

    Ok(next.run(req).await)
}
