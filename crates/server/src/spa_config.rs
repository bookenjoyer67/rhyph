use axum::{
    body::Body,
    http::{HeaderValue, Request, Response},
};
use http_body_util::BodyExt;
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

/// Injects `window.RHYPH = {...}` config into HTML responses.
/// Used when serving custom SPAs so they know which organizer they belong to.
#[derive(Clone)]
pub struct SpaConfigLayer {
    script: String,
}

impl SpaConfigLayer {
    pub fn new(organizer_slug: &str, organizer_name: &str, theme_json: &str) -> Self {
        let config = format!(
            r#"{{"apiUrl":"/api/v1","organizer":{{"slug":"{}","name":"{}"}},"theme":{}}}"#,
            organizer_slug, organizer_name, theme_json
        );
        Self {
            script: format!("<script>window.RHYPH={};</script>", config),
        }
    }
}

impl<S> Layer<S> for SpaConfigLayer {
    type Service = SpaConfigService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SpaConfigService {
            inner,
            script: self.script.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SpaConfigService<S> {
    inner: S,
    script: String,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SpaConfigService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone + Send + 'static,
    S::Future: Send + 'static,
    ResBody: http_body::Body<Data = bytes::Bytes> + Send + 'static,
    ResBody::Error: std::error::Error + Send + Sync + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();
        let script = self.script.clone();
        Box::pin(async move {
            let resp: Response<ResBody> = inner.call(req).await?;

            let is_html = resp
                .headers()
                .get(http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|ct| ct.starts_with("text/html"))
                .unwrap_or(false);

            if !is_html {
                let (parts, body) = resp.into_parts();
                let body = Body::from_stream(body.into_data_stream());
                return Ok(Response::from_parts(parts, body));
            }

            let (parts, body) = resp.into_parts();
            let bytes = body.collect().await.map(|b| b.to_bytes()).unwrap_or_default();
            let html = String::from_utf8_lossy(&bytes);

            let injected = if let Some(pos) = html.find("</head>") {
                let mut r = String::with_capacity(html.len() + script.len());
                r.push_str(&html[..pos]);
                r.push_str(&script);
                r.push_str(&html[pos..]);
                r
            } else {
                format!("{}{}", script, html)
            };

            let mut response = Response::from_parts(parts, Body::from(injected));
            // Remove Content-Length — size changed after injection
            response.headers_mut().remove(http::header::CONTENT_LENGTH);
            response.headers_mut().insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            );
            Ok(response)
        })
    }
}
