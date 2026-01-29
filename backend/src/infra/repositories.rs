use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::{
    generation_config::GenerationConfig,
    learning_material::{ExtractStatus, LearningMaterial},
    question::{Question, QuestionSet, QuestionSetStatus, QuestionType},
    test_attempt::{Answer, TestAttempt},
    user::{User, UserStatus},
};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, username, password_hash, created_at, updated_at, status FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                password_hash: row.try_get("password_hash")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                status: match row.try_get::<String, _>("status")?.as_str() {
                    "active" => UserStatus::Active,
                    _ => UserStatus::Disabled,
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, username, password_hash, created_at, updated_at, status) VALUES ($1,$2,$3,$4,$5,$6)",
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(match user.status {
            UserStatus::Active => "active",
            UserStatus::Disabled => "disabled",
        })
        .execute(self.pool)
        .await?;
        Ok(())
    }
}

pub struct MaterialRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> MaterialRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_material(&self, material: &LearningMaterial) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO learning_materials (id, user_id, original_filename, storage_path, file_type, file_size_bytes, uploaded_at, extracted_text_status, extracted_text) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
        )
        .bind(material.id)
        .bind(material.user_id)
        .bind(&material.original_filename)
        .bind(&material.storage_path)
        .bind(&material.file_type)
        .bind(material.file_size_bytes)
        .bind(material.uploaded_at)
        .bind(match material.extracted_text_status {
            ExtractStatus::Pending => "pending",
            ExtractStatus::Processing => "processing",
            ExtractStatus::Ready => "ready",
            ExtractStatus::Failed => "failed",
        })
        .bind(&material.extracted_text)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<LearningMaterial>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, user_id, original_filename, storage_path, file_type, file_size_bytes, uploaded_at, extracted_text_status, extracted_text FROM learning_materials WHERE user_id = $1 ORDER BY uploaded_at DESC",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;

        let mut materials = Vec::new();
        for row in rows {
            materials.push(LearningMaterial {
                id: row.try_get("id")?,
                user_id: row.try_get("user_id")?,
                original_filename: row.try_get("original_filename")?,
                storage_path: row.try_get("storage_path")?,
                file_type: row.try_get("file_type")?,
                file_size_bytes: row.try_get("file_size_bytes")?,
                uploaded_at: row.try_get("uploaded_at")?,
                extracted_text_status: match row.try_get::<String, _>("extracted_text_status")?.as_str() {
                    "pending" => ExtractStatus::Pending,
                    "processing" => ExtractStatus::Processing,
                    "ready" => ExtractStatus::Ready,
                    _ => ExtractStatus::Failed,
                },
                extracted_text: row.try_get("extracted_text")?,
            });
        }
        Ok(materials)
    }
}

pub struct GenerationRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> GenerationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_config(&self, config: &GenerationConfig) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO generation_configs (id, user_id, material_id, mcq_single_count, mcq_multi_count, fill_blank_count, language, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(config.id)
        .bind(config.user_id)
        .bind(config.material_id)
        .bind(config.mcq_single_count)
        .bind(config.mcq_multi_count)
        .bind(config.fill_blank_count)
        .bind(&config.language)
        .bind(config.created_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_question_set(&self, set: &QuestionSet) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO question_sets (id, user_id, material_id, config_id, status, created_at, completed_at, raw_ai_response) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(set.id)
        .bind(set.user_id)
        .bind(set.material_id)
        .bind(set.config_id)
        .bind(match set.status {
            QuestionSetStatus::Queued => "queued",
            QuestionSetStatus::Generating => "generating",
            QuestionSetStatus::Ready => "ready",
            QuestionSetStatus::Failed => "failed",
        })
        .bind(set.created_at)
        .bind(set.completed_at)
        .bind(&set.raw_ai_response)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_question(&self, question: &Question) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO questions (id, question_set_id, type, prompt, options, correct_answer, explanation, order_index) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
        )
        .bind(question.id)
        .bind(question.question_set_id)
        .bind(match question.question_type {
            QuestionType::Single => "single",
            QuestionType::Multiple => "multiple",
            QuestionType::Blank => "blank",
        })
        .bind(&question.prompt)
        .bind(&question.options)
        .bind(&question.correct_answer)
        .bind(&question.explanation)
        .bind(question.order_index)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_questions_by_set_id(&self, set_id: Uuid) -> Result<Vec<Question>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, question_set_id, type, prompt, options, correct_answer, explanation, order_index FROM questions WHERE question_set_id = $1 ORDER BY order_index ASC",
        )
        .bind(set_id)
        .fetch_all(self.pool)
        .await?;

        let mut questions = Vec::new();
        for row in rows {
            questions.push(Question {
                id: row.try_get("id")?,
                question_set_id: row.try_get("question_set_id")?,
                question_type: match row.try_get::<String, _>("type")?.as_str() {
                    "single" => QuestionType::Single,
                    "multiple" => QuestionType::Multiple,
                    "blank" => QuestionType::Blank,
                    _ => QuestionType::Single, // Fallback
                },
                prompt: row.try_get("prompt")?,
                options: row.try_get("options")?,
                correct_answer: row.try_get("correct_answer")?,
                explanation: row.try_get("explanation")?,
                order_index: row.try_get("order_index")?,
            });
        }
        Ok(questions)
    }
}

pub struct TestAttemptRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TestAttemptRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_attempt(&self, attempt: &TestAttempt) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO test_attempts (id, user_id, question_set_id, started_at, submitted_at, score_percent, feedback_summary) VALUES ($1,$2,$3,$4,$5,$6,$7)",
        )
        .bind(attempt.id)
        .bind(attempt.user_id)
        .bind(attempt.question_set_id)
        .bind(attempt.started_at)
        .bind(attempt.submitted_at)
        .bind(attempt.score_percent)
        .bind(&attempt.feedback_summary)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_answers(&self, answers: &[Answer]) -> Result<(), sqlx::Error> {
        // Bulk insert would be better, but loop is fine for MVP
        for answer in answers {
            sqlx::query(
                "INSERT INTO answers (id, attempt_id, question_id, response, is_correct) VALUES ($1,$2,$3,$4,$5)",
            )
            .bind(answer.id)
            .bind(answer.attempt_id)
            .bind(answer.question_id)
            .bind(&answer.response)
            .bind(answer.is_correct)
            .execute(self.pool)
            .await?;
        }
        Ok(())
    }
}

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
