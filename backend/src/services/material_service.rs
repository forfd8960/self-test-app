use std::path::PathBuf;

use tokio::fs;
use uuid::Uuid;

use crate::{
    domain::{
        error::AppError,
        learning_material::{ExtractStatus, LearningMaterial},
    },
    infra::{repositories::{now_utc, MaterialRepository}, storage::FileStorage},
    services::extraction_service::ExtractionService,
};

const MAX_UPLOAD_BYTES: usize = 5 * 1024 * 1024;

pub struct MaterialService<'a> {
    pub storage: &'a FileStorage,
    pub materials: MaterialRepository<'a>,
}

impl<'a> MaterialService<'a> {
    pub fn new(storage: &'a FileStorage, materials: MaterialRepository<'a>) -> Self {
        Self { storage, materials }
    }

    pub async fn save_material(
        &self,
        user_id: Uuid,
        filename: &str,
        content: Vec<u8>,
    ) -> Result<LearningMaterial, AppError> {
        if content.len() > MAX_UPLOAD_BYTES {
            return Err(AppError::BadRequest("file too large".to_string()));
        }

        let extension = PathBuf::from(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let extraction_service = ExtractionService::new();
        if !extraction_service.is_supported(&extension) {
            return Err(AppError::BadRequest("unsupported file type".to_string()));
        }

        fs::create_dir_all(self.storage.base_dir())
            .await
            .map_err(|e| {
                tracing::error!("Failed to create material dir: {:?}", e);
                AppError::Internal
            })?;

        let material_id = Uuid::new_v4();
        let file_path = self.storage.material_path(&material_id.to_string(), &extension);
        let file_size_bytes = content.len() as i64;

        fs::write(&file_path, &content)
            .await
            .map_err(|e| {
                tracing::error!("Failed to write material file: {:?}", e);
                AppError::Internal
            })?;

        let (extracted_text, status) = match extraction_service
            .extract_text(&extension, &file_path)
            .await
        {
            Ok(Some(text)) => (Some(text), ExtractStatus::Ready),
            Ok(None) => (None, ExtractStatus::Pending),
            Err(e) => {
                tracing::error!("Extraction failed for {:?}: {:?}", file_path, e);
                (None, ExtractStatus::Failed)
            }
        };

        let material = LearningMaterial {
            id: material_id,
            user_id,
            original_filename: filename.to_string(),
            storage_path: file_path.to_string_lossy().to_string(),
            file_type: extension,
            file_size_bytes,
            uploaded_at: now_utc(),
            extracted_text_status: status,
            extracted_text,
        };

        self.materials
            .create_material(&material)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create material in DB: {:?}", e);
                AppError::Internal
            })?;

        Ok(material)
    }

    pub async fn list_user_materials(&self, user_id: Uuid) -> Result<Vec<LearningMaterial>, AppError> {
        self.materials
            .find_all_by_user_id(user_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to list materials: {:?}", e);
                AppError::Internal
            })
    }
}
