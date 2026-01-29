use axum::{
    extract::State,
    http::HeaderMap,
    Json, Router, routing::post,
};
use uuid::Uuid;

use crate::{
    api::{middleware::auth::require_user_id, router::AppState},
    domain::{
        error::AppError,
        test_attempt::{Answer, SubmitTestRequest, TestAttempt, TestResultResponse},
    },
    infra::repositories::{GenerationRepository, TestAttemptRepository, now_utc},
    services::{feedback_service::FeedbackService, scoring_service::ScoringService},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/submit", post(submit_test))
}

async fn submit_test(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SubmitTestRequest>,
) -> Result<Json<TestResultResponse>, AppError> {
    let user_id = require_user_id(&headers, &state.config.jwt_secret)?;

    let gen_repo = GenerationRepository::new(&state.pool);
    let attempt_repo = TestAttemptRepository::new(&state.pool);

    // 1. Fetch questions
    let questions = gen_repo
        .find_questions_by_set_id(payload.question_set_id)
        .await
        .map_err(|_| AppError::Internal)?;

    if questions.is_empty() {
        return Err(AppError::BadRequest("Question set not found or empty".to_string()));
    }

    // 2. Score
    let (score_percent, answer_results) = ScoringService::score_test(&questions, &payload.answers);

    // 3. Generate Feedback
    let feedback_service = FeedbackService::new(state.ai_client.clone());
    let feedback = feedback_service
        .generate_feedback(score_percent, &answer_results, &questions)
        .await
        .unwrap_or_else(|_| "Feedback generation unavailable.".to_string());

    // 4. Create Attempt Record
    let attempt_id = Uuid::new_v4();
    let attempt = TestAttempt {
        id: attempt_id,
        user_id,
        question_set_id: payload.question_set_id,
        started_at: now_utc(), // Ideally passed from FE, but using submitting time for MVP
        submitted_at: Some(now_utc()),
        score_percent: Some(score_percent),
        feedback_summary: Some(feedback.clone()),
    };

    attempt_repo.create_attempt(&attempt).await.map_err(|_| AppError::Internal)?;

    // 5. Create Answer Records
    let answers: Vec<Answer> = answer_results.iter().map(|res| {
        let submitted_ans = payload.answers.iter().find(|a| a.question_id == res.question_id);
        Answer {
            id: Uuid::new_v4(),
            attempt_id,
            question_id: res.question_id,
            response: submitted_ans.map(|a| a.response.clone()).unwrap_or_default(),
            is_correct: res.is_correct,
        }
    }).collect();

    attempt_repo.create_answers(&answers).await.map_err(|_| AppError::Internal)?;

    // 6. Return Response
    Ok(Json(TestResultResponse {
        attempt_id,
        score_percent,
        feedback,
        correct_answers: answer_results,
    }))
}
