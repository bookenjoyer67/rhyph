use axum::{
    extract::{Path, State},
    routing::{get, patch},
    Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use rhyph_core::{Organizer, OrganizerPublic, UpdateOrganizerRequest};

use crate::error::ApiError;
use crate::middleware::auth::RequireAdmin;

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/api/v1/admin/organizers/{slug}",
            patch(update_organizer),
        )
        // Public: fetch organizer brand config
        .route(
            "/api/v1/organizers/{slug}",
            get(get_organizer_public),
        )
        .with_state(pool)
}

async fn get_organizer_public(
    State(pool): State<Arc<PgPool>>,
    Path(slug): Path<String>,
) -> Result<Json<OrganizerPublic>, ApiError> {
    let org = sqlx::query_as::<_, Organizer>(
        "SELECT id, slug, name, theme, custom_domain, created_at, updated_at
         FROM organizers WHERE slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("organizer not found".into()))?;

    Ok(Json(OrganizerPublic {
        slug: org.slug,
        name: org.name,
        theme: org.theme,
        custom_domain: org.custom_domain,
    }))
}

async fn update_organizer(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
    Json(body): Json<UpdateOrganizerRequest>,
) -> Result<Json<Organizer>, ApiError> {
    // Fetch existing
    let existing = sqlx::query_as::<_, Organizer>(
        "SELECT id, slug, name, theme, custom_domain, created_at, updated_at
         FROM organizers WHERE slug = $1",
    )
    .bind(&slug)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("organizer not found".into()))?;

    let name = body.name.unwrap_or(existing.name);
    let theme = body.theme.unwrap_or(existing.theme);
    let custom_domain = body.custom_domain.or(existing.custom_domain);

    let updated = sqlx::query_as::<_, Organizer>(
        "UPDATE organizers
         SET name = $1, theme = $2, custom_domain = $3, updated_at = NOW()
         WHERE id = $4
         RETURNING id, slug, name, theme, custom_domain, created_at, updated_at",
    )
    .bind(&name)
    .bind(&theme)
    .bind(&custom_domain)
    .bind(existing.id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict("custom_domain already in use".into())
        } else {
            ApiError::Internal(msg)
        }
    })?;

    Ok(Json(updated))
}

/// Public organizer lookup helper — also exported for use by other modules
pub async fn lookup_organizer_public(
    pool: &PgPool,
    slug: &str,
) -> Result<OrganizerPublic, ApiError> {
    let org = sqlx::query_as::<_, Organizer>(
        "SELECT id, slug, name, theme, custom_domain, created_at, updated_at
         FROM organizers WHERE slug = $1",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("organizer not found".into()))?;

    Ok(OrganizerPublic {
        slug: org.slug,
        name: org.name,
        theme: org.theme,
        custom_domain: org.custom_domain,
    })
}
