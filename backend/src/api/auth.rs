use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    api::router::AppState,
    domain::error::AppError,
    infra::repositories::UserRepository,
    services::auth_service::{AuthResponse, AuthService},
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthTokens>, AppError> {
    let service = AuthService::new(&state.config, UserRepository::new(&state.pool));
    let AuthResponse { access_token, refresh_token } =
        service.register(&payload.username, &payload.password).await?;

    Ok(Json(AuthTokens {
        access_token,
        refresh_token,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthTokens>, AppError> {
    let service = AuthService::new(&state.config, UserRepository::new(&state.pool));
    let AuthResponse { access_token, refresh_token } =
        service.login(&payload.username, &payload.password).await?;

    Ok(Json(AuthTokens {
        access_token,
        refresh_token,
    }))
}
