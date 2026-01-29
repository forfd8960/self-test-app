use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionSet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub material_id: Uuid,
    pub config_id: Uuid,
    pub status: QuestionSetStatus,
    pub created_at: OffsetDateTime,
    pub completed_at: Option<OffsetDateTime>,
    pub raw_ai_response: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionSetStatus {
    Queued,
    Generating,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub question_set_id: Uuid,
    pub question_type: QuestionType,
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Single,
    Multiple,
    Blank,
}
