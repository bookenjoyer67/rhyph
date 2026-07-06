use axum::{
    extract::{Multipart, Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use rhyph_core::{Organizer, OrganizerImage, OrganizerImageResponse, OrganizerPublic, UpdateOrganizerRequest};

use crate::error::ApiError;
use crate::middleware::auth::RequireAdmin;

const PUBLIC_PAGES_DIR: &str = "public_pages";
const MAX_UPLOAD_SIZE: u64 = 5 * 1024 * 1024; // 5MB
const MAX_SPA_UPLOAD_SIZE: u64 = 50 * 1024 * 1024; // 50MB
const ALLOWED_TYPES: &[&str] = &["image/png", "image/jpeg", "image/webp", "image/svg+xml"];

pub fn routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/api/v1/admin/organizers/{slug}",
            patch(update_organizer),
        )
        // Image management
        .route(
            "/api/v1/admin/organizers/{slug}/images",
            get(list_images).post(upload_image),
        )
        .route(
            "/api/v1/admin/organizers/{slug}/images/{image_id}",
            delete(delete_image),
        )
        // Custom SPA management
        .route(
            "/api/v1/admin/organizers/{slug}/spa",
            get(get_spa_status).post(upload_spa).delete(delete_spa),
        )
        .route(
            "/api/v1/admin/organizers/{slug}/spa/folder",
            post(upload_spa_folder),
        )
        // Public: fetch organizer brand config
        .route(
            "/api/v1/organizers/{slug}",
            get(get_organizer_public),
        )
        .with_state(pool)
}

// ── Image upload ──

async fn upload_image(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<OrganizerImageResponse>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| ApiError::BadRequest(format!("multipart error: {e}")))?
    {
        let name = field.name().unwrap_or("file").to_string();
        if name != "file" {
            continue;
        }

        let original_name = field
            .file_name()
            .unwrap_or("upload")
            .to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        if !ALLOWED_TYPES.contains(&content_type.as_str()) {
            return Err(ApiError::BadRequest(format!(
                "unsupported file type: {content_type}. allowed: png, jpeg, webp, svg"
            )));
        }

        let data = field.bytes().await
            .map_err(|e| ApiError::BadRequest(format!("read error: {e}")))?;

        if data.len() as u64 > MAX_UPLOAD_SIZE {
            return Err(ApiError::BadRequest(format!(
                "file too large: {} bytes (max {})", data.len(), MAX_UPLOAD_SIZE
            )));
        }

        // Determine file extension
        let ext = extension_from_content_type(&content_type);
        let image_id = Uuid::new_v4();
        let filename = format!("{image_id}.{ext}");

        // Create directory
        let dir = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("images");
        fs::create_dir_all(&dir).await
            .map_err(|e| ApiError::Internal(format!("failed to create directory: {e}")))?;

        let filepath = dir.join(&filename);

        // Write file
        let mut f = fs::File::create(&filepath).await
            .map_err(|e| ApiError::Internal(format!("failed to create file: {e}")))?;
        f.write_all(&data).await
            .map_err(|e| ApiError::Internal(format!("failed to write file: {e}")))?;

        // Save to DB
        let record = sqlx::query_as::<_, OrganizerImage>(
            "INSERT INTO organizer_images (id, organizer_id, filename, original_name, content_type, size_bytes)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, organizer_id, filename, original_name, content_type, size_bytes, created_at",
        )
        .bind(image_id)
        .bind(org.id)
        .bind(&filename)
        .bind(&original_name)
        .bind(&content_type)
        .bind(data.len() as i64)
        .fetch_one(&*pool)
        .await
        .map_err(|e| ApiError::Internal(format!("db error: {e}")))?;

        let url = format!("/public_pages/{slug}/images/{filename}");

        return Ok(Json(OrganizerImageResponse {
            id: record.id,
            url,
            original_name: record.original_name,
            content_type: record.content_type,
            size_bytes: record.size_bytes,
            created_at: record.created_at,
        }));
    }

    Err(ApiError::BadRequest("no file field found in upload".into()))
}

async fn list_images(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
) -> Result<Json<Vec<OrganizerImageResponse>>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

    let images = sqlx::query_as::<_, OrganizerImage>(
        "SELECT id, organizer_id, filename, original_name, content_type, size_bytes, created_at
         FROM organizer_images WHERE organizer_id = $1
         ORDER BY created_at DESC",
    )
    .bind(org.id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| ApiError::Internal(format!("db error: {e}")))?;

    let responses: Vec<OrganizerImageResponse> = images
        .into_iter()
        .map(|img| OrganizerImageResponse {
            id: img.id,
            url: format!("/public_pages/{slug}/images/{}", img.filename),
            original_name: img.original_name,
            content_type: img.content_type,
            size_bytes: img.size_bytes,
            created_at: img.created_at,
        })
        .collect();

    Ok(Json(responses))
}

async fn delete_image(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path((slug, image_id)): Path<(String, Uuid)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

    let img = sqlx::query_as::<_, OrganizerImage>(
        "SELECT id, organizer_id, filename, original_name, content_type, size_bytes, created_at
         FROM organizer_images WHERE id = $1 AND organizer_id = $2",
    )
    .bind(image_id)
    .bind(org.id)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| ApiError::Internal(format!("db error: {e}")))?
    .ok_or_else(|| ApiError::NotFound("image not found".into()))?;

    // Delete file
    let filepath = PathBuf::from(PUBLIC_PAGES_DIR)
        .join(&slug)
        .join("images")
        .join(&img.filename);
    let _ = fs::remove_file(&filepath).await;

    // Delete DB record
    sqlx::query("DELETE FROM organizer_images WHERE id = $1")
        .bind(image_id)
        .execute(&*pool)
        .await
        .map_err(|e| ApiError::Internal(format!("db error: {e}")))?;

    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Custom SPA upload ──

async fn get_spa_status(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;
    let spa_dir = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("spa");
    let index_exists = spa_dir.join("index.html").exists();
    Ok(Json(serde_json::json!({
        "has_custom_spa": org.has_custom_spa,
        "index_exists": index_exists,
        "spa_updated_at": org.spa_updated_at,
    })))
}

async fn upload_spa(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| ApiError::BadRequest(format!("multipart error: {e}")))?
    {
        if field.name().unwrap_or("") != "file" { continue; }

        let data = field.bytes().await
            .map_err(|e| ApiError::BadRequest(format!("read error: {e}")))?;

        if data.len() as u64 > MAX_SPA_UPLOAD_SIZE {
            return Err(ApiError::BadRequest(format!(
                "file too large: {} bytes (max {})", data.len(), MAX_SPA_UPLOAD_SIZE
            )));
        }

        // Write to temp file
        let tmp_path = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("_upload.tar.gz");
        let parent = tmp_path.parent().unwrap();
        fs::create_dir_all(parent).await
            .map_err(|e| ApiError::Internal(format!("mkdir: {e}")))?;

        let mut f = fs::File::create(&tmp_path).await
            .map_err(|e| ApiError::Internal(format!("create file: {e}")))?;
        f.write_all(&data).await
            .map_err(|e| ApiError::Internal(format!("write: {e}")))?;
        drop(f);

        // Validate: must contain index.html
        let output = tokio::process::Command::new("tar")
            .args(["tzf", tmp_path.to_str().unwrap()])
            .output().await
            .map_err(|e| ApiError::BadRequest(format!("tar list failed: {e}")))?;

        let listing = String::from_utf8_lossy(&output.stdout);
        let has_index = listing.lines().any(|l| l.ends_with("index.html") || l.contains("/index.html"));
        if !has_index {
            let _ = fs::remove_file(&tmp_path).await;
            return Err(ApiError::BadRequest("archive must contain index.html at root or in a subdirectory".into()));
        }

        // Check for path traversal
        if listing.lines().any(|l| l.contains("..")) {
            let _ = fs::remove_file(&tmp_path).await;
            return Err(ApiError::BadRequest("archive contains path traversal".into()));
        }

        // Extract to spa dir
        let spa_dir = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("spa");
        if spa_dir.exists() {
            fs::remove_dir_all(&spa_dir).await
                .map_err(|e| ApiError::Internal(format!("cleanup: {e}")))?;
        }
        fs::create_dir_all(&spa_dir).await
            .map_err(|e| ApiError::Internal(format!("mkdir spa: {e}")))?;

        let status = tokio::process::Command::new("tar")
            .args(["xzf", tmp_path.to_str().unwrap(), "-C", spa_dir.to_str().unwrap(), "--strip-components=1"])
            .output().await
            .map_err(|e| ApiError::BadRequest(format!("tar extract failed: {e}")))?;

        if !status.status.success() {
            let _ = fs::remove_file(&tmp_path).await;
            return Err(ApiError::BadRequest("tar extraction failed".into()));
        }

        // Clean up temp
        let _ = fs::remove_file(&tmp_path).await;

        // Count files
        let file_count = listing.lines().count();

        // Update DB
        sqlx::query("UPDATE organizers SET has_custom_spa = TRUE, spa_updated_at = NOW() WHERE id = $1")
            .bind(org.id)
            .execute(&*pool)
            .await
            .map_err(|e| ApiError::Internal(format!("db: {e}")))?;

        return Ok(Json(serde_json::json!({
            "ok": true,
            "file_count": file_count,
            "path": format!("public_pages/{}/spa/", slug),
        })));
    }

    Err(ApiError::BadRequest("no file field in upload".into()))
}

async fn delete_spa(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

    let spa_dir = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("spa");
    if spa_dir.exists() {
        fs::remove_dir_all(&spa_dir).await
            .map_err(|e| ApiError::Internal(format!("remove: {e}")))?;
    }

    sqlx::query("UPDATE organizers SET has_custom_spa = FALSE, spa_updated_at = NULL WHERE id = $1")
        .bind(org.id)
        .execute(&*pool)
        .await
        .map_err(|e| ApiError::Internal(format!("db: {e}")))?;

    Ok(Json(serde_json::json!({"ok": true})))
}

// ── Folder upload (accepts a directory of files) ──

async fn upload_spa_folder(
    State(pool): State<Arc<PgPool>>,
    _admin: RequireAdmin,
    Path(slug): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;
    let spa_dir = PathBuf::from(PUBLIC_PAGES_DIR).join(&slug).join("spa");

    // Clear existing
    if spa_dir.exists() {
        fs::remove_dir_all(&spa_dir).await
            .map_err(|e| ApiError::Internal(format!("cleanup: {e}")))?;
    }
    fs::create_dir_all(&spa_dir).await
        .map_err(|e| ApiError::Internal(format!("mkdir: {e}")))?;

    let mut total_size: u64 = 0;
    let mut file_count: usize = 0;
    let mut has_index = false;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| ApiError::BadRequest(format!("multipart: {e}")))?
    {
        let filename = field.file_name().unwrap_or("").to_string();
        // webkitdirectory sends relative paths like "build/index.html" or "index.html"
        let rel_path = filename.strip_prefix("build/").unwrap_or(&filename).to_string();
        if rel_path.is_empty() || rel_path.ends_with('/') { continue; }

        // Path traversal check
        if rel_path.contains("..") || rel_path.starts_with('/') {
            return Err(ApiError::BadRequest("path traversal in filename".into()));
        }

        let data = field.bytes().await
            .map_err(|e| ApiError::BadRequest(format!("read: {e}")))?;

        total_size += data.len() as u64;
        if total_size > MAX_SPA_UPLOAD_SIZE {
            return Err(ApiError::BadRequest(format!(
                "total size {} exceeds max {}", total_size, MAX_SPA_UPLOAD_SIZE
            )));
        }

        let dest = spa_dir.join(&rel_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| ApiError::Internal(format!("mkdir: {e}")))?;
        }

        let mut f = fs::File::create(&dest).await
            .map_err(|e| ApiError::Internal(format!("create: {e}")))?;
        f.write_all(&data).await
            .map_err(|e| ApiError::Internal(format!("write: {e}")))?;

        if rel_path == "index.html" || rel_path.ends_with("/index.html") {
            has_index = true;
        }
        file_count += 1;
    }

    if file_count == 0 {
        return Err(ApiError::BadRequest("no files in upload".into()));
    }
    if !has_index {
        return Err(ApiError::BadRequest("upload must contain index.html".into()));
    }

    sqlx::query("UPDATE organizers SET has_custom_spa = TRUE, spa_updated_at = NOW() WHERE id = $1")
        .bind(org.id)
        .execute(&*pool)
        .await
        .map_err(|e| ApiError::Internal(format!("db: {e}")))?;

    Ok(Json(serde_json::json!({
        "ok": true,
        "file_count": file_count,
        "path": format!("public_pages/{}/spa/", slug),
    })))
}

fn extension_from_content_type(ct: &str) -> &str {
    match ct {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        _ => "bin",
    }
}

async fn lookup_organizer(pool: &PgPool, slug: &str) -> Result<Organizer, ApiError> {
    sqlx::query_as::<_, Organizer>(
        "SELECT id, slug, name, theme, custom_domain, has_custom_spa, spa_updated_at, created_at, updated_at
         FROM organizers WHERE slug = $1",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("organizer not found".into()))
}

// ── Organizer public / update (existing) ──

async fn get_organizer_public(
    State(pool): State<Arc<PgPool>>,
    Path(slug): Path<String>,
) -> Result<Json<OrganizerPublic>, ApiError> {
    let org = lookup_organizer(&pool, &slug).await?;

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
    let existing = lookup_organizer(&pool, &slug).await?;

    let name = body.name.unwrap_or(existing.name);
    let theme = body.theme.unwrap_or(existing.theme);
    let custom_domain = body.custom_domain.or(existing.custom_domain);

    let updated = sqlx::query_as::<_, Organizer>(
        "UPDATE organizers
         SET name = $1, theme = $2, custom_domain = $3, updated_at = NOW()
         WHERE id = $4
         RETURNING id, slug, name, theme, custom_domain, has_custom_spa, spa_updated_at, created_at, updated_at",
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
