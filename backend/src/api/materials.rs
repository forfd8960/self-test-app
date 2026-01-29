use axum::{
    extract::{Multipart, State},
    http::HeaderMap,
    routing::post,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    api::{middleware::auth::require_user_id, router::AppState},
    domain::error::AppError,
    infra::repositories::MaterialRepository,
    services::material_service::MaterialService,
};

#[derive(Debug, Serialize)]
pub struct MaterialResponse {
    pub id: Uuid,
    pub original_filename: String,
    pub file_type: String,
    pub file_size_bytes: i64,
    pub uploaded_at: String,
    pub extracted_text_status: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(upload_material).get(list_materials))
}

async fn upload_material(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<MaterialResponse>, AppError> {
    let user_id = require_user_id(&headers, &state.config.jwt_secret)?;

    let mut filename: Option<String> = None;
    let mut bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| AppError::BadRequest("invalid multipart".to_string()))? {
        if field.name() == Some("file") {
            filename = field.file_name().map(|name| name.to_string());
            bytes = Some(field.bytes().await.map_err(|_| AppError::BadRequest("invalid file".to_string()))?.to_vec());
            break;
        }
    }

    let filename = filename.ok_or(AppError::BadRequest("missing file".to_string()))?;
    let bytes = bytes.ok_or(AppError::BadRequest("missing file".to_string()))?;

    let service = MaterialService::new(state.storage.as_ref(), MaterialRepository::new(&state.pool));
    let material = service.save_material(user_id, &filename, bytes).await?;

    Ok(Json(MaterialResponse {
        id: material.id,
        original_filename: material.original_filename,
        file_type: material.file_type,
        file_size_bytes: material.file_size_bytes,
        uploaded_at: material.uploaded_at.to_string(),
        extracted_text_status: format!("{:?}", material.extracted_text_status).to_lowercase(),
    }))
}

async fn list_materials(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<MaterialResponse>>, AppError> {
    let user_id = require_user_id(&headers, &state.config.jwt_secret)?;

    let service = MaterialService::new(state.storage.as_ref(), MaterialRepository::new(&state.pool));
    let materials = service.list_user_materials(user_id).await?;

    Ok(Json(
        materials
            .into_iter()
            .map(|m| MaterialResponse {
                id: m.id,
                original_filename: m.original_filename,
                file_type: m.file_type,
                file_size_bytes: m.file_size_bytes,
                uploaded_at: m.uploaded_at.to_string(),
                extracted_text_status: format!("{:?}", m.extracted_text_status).to_lowercase(),
            })
            .collect(),
    ))
}
