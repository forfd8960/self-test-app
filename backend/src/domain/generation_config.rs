use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub material_id: Uuid,
    pub mcq_single_count: i32,
    pub mcq_multi_count: i32,
    pub fill_blank_count: i32,
    pub language: Option<String>,
    pub created_at: OffsetDateTime,
}
