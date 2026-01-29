use axum::{
    extract::{Path, State},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{middleware::auth::require_user_id, router::AppState},
    domain::error::AppError,
    infra::repositories::GenerationRepository,
    services::generation_service::{build_generation_config, GenerationJob, GenerationService},
};

#[derive(Debug, Deserialize)]
pub struct GenerationRequest {
    pub material_id: Uuid,
    pub mcq_single_count: i32,
    pub mcq_multi_count: i32,
    pub fill_blank_count: i32,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GenerationJobResponse {
    pub id: Uuid,
    pub status: String,
    pub question_set_id: Option<Uuid>,
    pub error_message: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_job))
        .route("/{job_id}", get(get_job))
        .route("/sets/{set_id}/questions", get(get_questions))
}

async fn get_questions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(set_id): Path<Uuid>,
) -> Result<Json<Vec<crate::domain::question::Question>>, AppError> {
    require_user_id(&headers, &state.config.jwt_secret)?; // Ensure authenticated, though maybe ownership check needed too

    let repo = GenerationRepository::new(&state.pool);
    let questions = repo.find_questions_by_set_id(set_id).await.map_err(|_| AppError::Internal)?;
    
    Ok(Json(questions))
}

async fn create_job(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GenerationRequest>,
) -> Result<Json<GenerationJobResponse>, AppError> {
    let user_id = require_user_id(&headers, &state.config.jwt_secret)?;

    if !state.rate_limiter.allow(&user_id.to_string()).await {
        return Err(AppError::RateLimited);
    }

    let config = build_generation_config(
        user_id,
        payload.material_id,
        payload.mcq_single_count,
        payload.mcq_multi_count,
        payload.fill_blank_count,
        payload.language,
    );

    let service = GenerationService::new(state.ai_client.as_ref(), GenerationRepository::new(&state.pool), state.generation_jobs.clone());
    let job = service.create_job(config).await?;

    Ok(Json(to_job_response(job)))
}

async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<GenerationJobResponse>, AppError> {
    let service = GenerationService::new(state.ai_client.as_ref(), GenerationRepository::new(&state.pool), state.generation_jobs.clone());
    let job = service.get_job(job_id).await.ok_or(AppError::NotFound)?;

    Ok(Json(to_job_response(job)))
}

fn to_job_response(job: GenerationJob) -> GenerationJobResponse {
    GenerationJobResponse {
        id: job.id,
        status: format!("{:?}", job.status).to_lowercase(),
        question_set_id: job.question_set_id,
        error_message: job.error_message,
    }
}
