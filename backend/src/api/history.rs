use axum::{
    extract::{Path, State, Extension},
    Json, Router, routing::get,
};
use uuid::Uuid;

use crate::{
    api::{middleware::auth::AuthUser, router::AppState},
    domain::{
        error::AppError,
        test_attempt::{TestAttempt, TestAttemptDetail},
    },
    services::history_service::HistoryService,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_history))
        .route("/{id}", get(get_history_detail))
}

async fn list_history(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Vec<TestAttempt>>, AppError> {
    let attempts = HistoryService::get_user_history(&state.pool, user.id).await?;
    Ok(Json(attempts))
}

async fn get_history_detail(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<TestAttemptDetail>, AppError> {
    let detail = HistoryService::get_attempt_detail(&state.pool, id, user.id).await?;
    Ok(Json(detail))
}
