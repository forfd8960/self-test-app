use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("rate limit exceeded")]
    RateLimited,
    #[error("not found")]
    NotFound,
    #[error("user not found")]
    UserNotFound,
    #[error("internal error")]
    Internal,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", self.to_string()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request", self.to_string()),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "rate_limited", self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found", self.to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found", self.to_string()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", self.to_string()),
        };

        let body = Json(ErrorResponse {
            code: code.to_string(),
            message,
        });

        (status, body).into_response()
    }
}
