use std::sync::Arc;
use crate::domain::error::AppError;
use crate::domain::question::Question;
use crate::domain::test_attempt::AnswerResult;
use crate::infra::ai_client::AiClient;

pub struct FeedbackService {
    ai_client: Arc<AiClient>,
}

impl FeedbackService {
    pub fn new(ai_client: Arc<AiClient>) -> Self {
        Self { ai_client }
    }

    pub async fn generate_feedback(&self, score: f32, results: &[AnswerResult], questions: &[Question]) -> Result<String, AppError> {
        let mut prompt = format!(
            "The student scored {:.1}%. Provide a brief, encouraging feedback summary highlighting strengths and areas for improvement based on these results:\n\n",
            score
        );

        for (i, result) in results.iter().enumerate() {
            let question = questions.iter().find(|q| q.id == result.question_id);
            if let Some(q) = question {
                let status = if result.is_correct { "Correct" } else { "Incorrect" };
                prompt.push_str(&format!("{}. [{}]: {}\n", i + 1, status, q.prompt));
            }
        }

        prompt.push_str("\nKeep it concise (under 100 words).");

        let feedback = self.ai_client.generate(&prompt).await.map_err(|_| {
            AppError::Internal // map AI error
        })?;

        Ok(feedback)
    }
}
