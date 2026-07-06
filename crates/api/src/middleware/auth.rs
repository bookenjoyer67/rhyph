use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::ApiError;
use crate::services::auth;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
    pub is_admin: bool,
}

pub struct RequireAdmin(pub AuthUser);

impl FromRequestParts<Arc<PgPool>> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<PgPool>) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get("authorization") {
            let value = auth_header.to_str().map_err(|_| ApiError::Unauthorized)?;
            if let Some(token) = value.strip_prefix("Bearer ") {
                let claims = auth::validate_token(token)?;
                let user_id = Uuid::parse_str(&claims.sub).map_err(|_| ApiError::Unauthorized)?;
                return Ok(AuthUser {
                    user_id,
                    email: claims.email,
                    is_admin: claims.is_admin,
                });
            }
        }

        if let Some(api_key_header) = parts.headers.get("x-api-key") {
            let api_key = api_key_header.to_str().map_err(|_| ApiError::Unauthorized)?;
            let device = auth::validate_api_key(state, api_key).await?;
            return Ok(AuthUser {
                user_id: device.organizer_id,
                email: String::new(),
                is_admin: false,
            });
        }

        Err(ApiError::Unauthorized)
    }
}

impl FromRequestParts<Arc<PgPool>> for RequireAdmin {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &Arc<PgPool>) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if !user.is_admin {
            return Err(ApiError::Forbidden);
        }
        Ok(RequireAdmin(user))
    }
}
