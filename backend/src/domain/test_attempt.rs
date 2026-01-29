use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAttempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question_set_id: Uuid,
    pub started_at: OffsetDateTime,
    pub submitted_at: Option<OffsetDateTime>,
    pub score_percent: Option<f32>,
    pub feedback_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub response: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub struct SubmitTestRequest {
    pub question_set_id: Uuid,
    pub answers: Vec<SubmitAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitAnswer {
    pub question_id: Uuid,
    pub response: String,
}

#[derive(Debug, Serialize)]
pub struct TestResultResponse {
    pub attempt_id: Uuid,
    pub score_percent: f32,
    pub feedback: String,
    pub correct_answers: Vec<AnswerResult>,
}

#[derive(Debug, Serialize)]
pub struct AnswerResult {
    pub question_id: Uuid,
    pub is_correct: bool,
    pub correct_answer: String,
    pub explanation: Option<String>,
}
