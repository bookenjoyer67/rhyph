use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use rand::rngs::OsRng;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use rhyph_core::{Device, User};

use crate::error::ApiError;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub is_admin: bool,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "rhyph-dev-secret-change-me".to_string())
}

pub async fn login(pool: &PgPool, email: &str, password: &str) -> Result<LoginResponse, ApiError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or(ApiError::Unauthorized)?;

    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|e| ApiError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Unauthorized)?;

    let exp = (Utc::now() + chrono::Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        is_admin: user.is_admin,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(LoginResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            is_admin: user.is_admin,
        },
    })
}

pub fn validate_token(token: &str) -> Result<Claims, ApiError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?;

    Ok(token_data.claims)
}

pub async fn validate_api_key(pool: &PgPool, api_key: &str) -> Result<Device, ApiError> {
    sqlx::query_as::<_, Device>(
        "SELECT id, organizer_id, name, api_key, created_at FROM devices WHERE api_key = $1",
    )
    .bind(api_key)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or(ApiError::Unauthorized)
}

pub fn generate_api_key() -> String {
    let bytes: [u8; 16] = rand::thread_rng().gen();
    hex::encode(bytes)
}

/// Check if any users exist — if not, the instance needs initial setup.
pub async fn needs_setup(pool: &PgPool) -> Result<bool, ApiError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(count == 0)
}

/// Create the first admin user. Refuses if users already exist.
pub async fn create_admin(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<LoginResponse, ApiError> {
    // Guard: only allow if no users exist
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    if count > 0 {
        return Err(ApiError::Conflict("instance already set up".into()));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .to_string();

    // Insert admin user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, is_admin)
         VALUES ($1, $2, TRUE)
         RETURNING id, email, password_hash, is_admin, created_at, updated_at",
    )
    .bind(email)
    .bind(&hash)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict("email already exists".into())
        } else {
            ApiError::Internal(msg)
        }
    })?;

    // Generate JWT so user is logged in immediately after setup
    let exp = (Utc::now() + chrono::Duration::days(7)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        is_admin: true,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(LoginResponse {
        token,
        user: UserInfo {
            id: user.id,
            email: user.email,
            is_admin: true,
        },
    })
}
