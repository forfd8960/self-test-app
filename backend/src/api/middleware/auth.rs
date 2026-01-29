use axum::{http::{header, HeaderMap}, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::domain::{auth::Claims, error::AppError};

pub fn require_user_id(headers: &HeaderMap, jwt_secret: &str) -> Result<Uuid, AppError> {
    // Check Authorization header first
    if let Some(auth_header) = headers.get(header::AUTHORIZATION).and_then(|h| h.to_str().ok()) {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            let validation = Validation::default();
            
            if let Ok(token_data) = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &validation,
            ) {
                if let Ok(uuid) = Uuid::parse_str(&token_data.claims.sub) {
                    return Ok(uuid);
                }
            }
        }
    }

    // Fallback to x-user-id
    let value = headers
        .get("x-user-id")

        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    Uuid::parse_str(value).map_err(|_| AppError::Unauthorized)
}

#[allow(dead_code)]
pub async fn require_auth(request: axum::extract::Request, next: Next) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}
