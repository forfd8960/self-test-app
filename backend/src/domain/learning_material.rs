use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMaterial {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_filename: String,
    pub storage_path: String,
    pub file_type: String,
    pub file_size_bytes: i64,
    pub uploaded_at: OffsetDateTime,
    pub extracted_text_status: ExtractStatus,
    pub extracted_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractStatus {
    Pending,
    Processing,
    Ready,
    Failed,
}
