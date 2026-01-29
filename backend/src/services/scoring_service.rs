use crate::domain::question::{Question, QuestionType};
use crate::domain::test_attempt::{AnswerResult, SubmitAnswer};

pub struct ScoringService;

impl ScoringService {
    pub fn score_test(questions: &[Question], answers: &[SubmitAnswer]) -> (f32, Vec<AnswerResult>) {
        let mut correct_count = 0;
        let mut results = Vec::new();
        let total = questions.len();

        if total == 0 {
            return (0.0, Vec::new());
        }

        for question in questions {
            let submitted = answers.iter().find(|a| a.question_id == question.id);
            let is_correct = match submitted {
                Some(ans) => Self::check_answer(&question.question_type, &question.correct_answer, &ans.response),
                None => false,
            };

            if is_correct {
                correct_count += 1;
            }

            results.push(AnswerResult {
                question_id: question.id,
                is_correct,
                correct_answer: question.correct_answer.clone(),
                explanation: question.explanation.clone(),
            });
        }

        let score = (correct_count as f32 / total as f32) * 100.0;
        (score, results)
    }

    fn check_answer(q_type: &QuestionType, correct: &str, submitted: &str) -> bool {
        let clean = |s: &str| s.trim().to_lowercase();

        match q_type {
            QuestionType::Single => clean(correct) == clean(submitted),
            QuestionType::Blank => clean(correct) == clean(submitted),
            QuestionType::Multiple => {
                // simple contains set logic if comma separated, or exact match
                // for MVP, let's strict sort and compare if structure is unknown
                // assuming comma separated for now
                let mut correct_parts: Vec<_> = correct.split(',').map(clean).collect();
                let mut submitted_parts: Vec<_> = submitted.split(',').map(clean).collect();
                correct_parts.sort();
                submitted_parts.sort();
                correct_parts == submitted_parts
            }
        }
    }
}
