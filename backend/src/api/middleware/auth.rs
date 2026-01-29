use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::{api::router::AppState, domain::{auth::Claims, error::AppError}};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let headers = request.headers();
    let jwt_secret = &state.config.jwt_secret;

    // Check Authorization header first
    let user_id = if let Some(auth_header) = headers.get(header::AUTHORIZATION).and_then(|h| h.to_str().ok()) {
        let mut found_id = None;
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            let validation = Validation::default();
            
            if let Ok(token_data) = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &validation,
            ) {
                if let Ok(uuid) = Uuid::parse_str(&token_data.claims.sub) {
                    found_id = Some(uuid);
                }
            }
        }
        
        match found_id {
            Some(id) => id,
            None => extract_user_id_from_header(headers)?,
        }
    } else {
        extract_user_id_from_header(headers)?
    };

    request.extensions_mut().insert(AuthUser { id: user_id });
    Ok(next.run(request).await)
}

fn extract_user_id_from_header(headers: &axum::http::HeaderMap) -> Result<Uuid, AppError> {
    let value = headers
        .get("x-user-id")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    Uuid::parse_str(value).map_err(|_| AppError::Unauthorized)
}

#[allow(dead_code)]
pub fn require_user_id(headers: &axum::http::HeaderMap, jwt_secret: &str) -> Result<Uuid, AppError> {
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
     extract_user_id_from_header(headers)
}
