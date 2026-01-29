use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::error::AppError;
use crate::domain::test_attempt::{AttemptAnswerDetail, TestAttempt, TestAttemptDetail};
use crate::infra::repositories::{GenerationRepository, TestAttemptRepository};

pub struct HistoryService;

impl HistoryService {
    pub async fn get_user_history(pool: &PgPool, user_id: Uuid) -> Result<Vec<TestAttempt>, AppError> {
        let repo = TestAttemptRepository::new(pool);
        repo.find_by_user_id(user_id)
            .await
            .map_err(|_| AppError::Internal)
    }

    pub async fn get_attempt_detail(
        pool: &PgPool,
        attempt_id: Uuid,
        user_id: Uuid,
    ) -> Result<TestAttemptDetail, AppError> {
        let attempt_repo = TestAttemptRepository::new(pool);
        let gen_repo = GenerationRepository::new(pool);

        // 1. Fetch attempt
        let attempt = attempt_repo
            .find_by_id(attempt_id)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::NotFound)?; // Fixed: NotFound is unit

        // Verify ownership
        if attempt.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        // 2. Fetch answers (user responses)
        let user_answers = attempt_repo
            .find_answers_by_attempt_id(attempt_id)
            .await
            .map_err(|_| AppError::Internal)?;

        // 3. Fetch questions (for correct answer, prompt, explanation)
        let questions = gen_repo
            .find_questions_by_set_id(attempt.question_set_id)
            .await
            .map_err(|_| AppError::Internal)?;

        // 4. Combine
        let mut details = Vec::new();
        for q in questions {
            let user_ans = user_answers
                .iter()
                .find(|a| a.question_id == q.id);

            details.push(AttemptAnswerDetail {
                question_id: q.id,
                user_response: user_ans
                    .map(|a| a.response.clone())
                    .unwrap_or_else(|| "".to_string()),
                is_correct: user_ans.map(|a| a.is_correct).unwrap_or(false),
                correct_answer: q.correct_answer,
                explanation: q.explanation,
                prompt: q.prompt,
            });
        }

        Ok(TestAttemptDetail {
            attempt,
            answers: details,
        })
    }
}
