use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use sqlx::PgPool;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::{
        error::AppError,
        generation_config::GenerationConfig,
        question::{Question, QuestionSet, QuestionSetStatus, QuestionType},
    },
    infra::{
        ai_client::AiClient,
        repositories::{now_utc, GenerationRepository, MaterialRepository},
    },
};

#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerationJob {
    pub id: Uuid,
    pub status: JobStatus,
    pub question_set_id: Option<Uuid>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Generating,
    Ready,
    Failed,
}

pub struct GenerationService<'a> {
    pub ai_client: &'a AiClient,
    pub pool: PgPool,
    pub jobs: Arc<Mutex<HashMap<Uuid, GenerationJob>>>,
}

#[derive(Debug, Deserialize)]
struct AiQuestion {
    prompt: String,
    #[serde(rename = "type")]
    q_type: String,
    options: Option<Vec<String>>,
    correct: String,
    explanation: String,
}

#[derive(Debug, Deserialize)]
struct AiResponse {
    questions: Vec<AiQuestion>,
}

impl<'a> GenerationService<'a> {
    pub fn new(
        ai_client: &'a AiClient,
        pool: PgPool,
        jobs: Arc<Mutex<HashMap<Uuid, GenerationJob>>>,
    ) -> Self {
        Self { ai_client, pool, jobs }
    }

    pub async fn create_job(&self, config: GenerationConfig) -> Result<GenerationJob, AppError> {
        let repo = GenerationRepository::new(&self.pool);
        repo
            .create_config(&config)
            .await
            .map_err(|_| AppError::Internal)?;

        let job_id = Uuid::new_v4();
        let job = GenerationJob {
            id: job_id,
            status: JobStatus::Queued,
            question_set_id: None,
            error_message: None,
        };

        self.jobs.lock().await.insert(job_id, job.clone());

        let ai_client = self.ai_client.clone();
        let jobs = Arc::clone(&self.jobs);
        let pool = self.pool.clone();
        let config_clone = config.clone();

        tokio::spawn(async move {
            let mut jobs_guard = jobs.lock().await;
            if let Some(entry) = jobs_guard.get_mut(&job_id) {
                entry.status = JobStatus::Generating;
            }
            drop(jobs_guard);

            let result = run_generation_task(ai_client, pool, config_clone).await;

            let mut jobs_guard = jobs.lock().await;
            if let Some(entry) = jobs_guard.get_mut(&job_id) {
                match result {
                    Ok(set_id) => {
                        entry.status = JobStatus::Ready;
                        entry.question_set_id = Some(set_id);
                    }
                    Err(err) => {
                        entry.status = JobStatus::Failed;
                        entry.error_message = Some(err.to_string());
                    }
                }
            }
        });

        Ok(job)
    }

    pub async fn get_job(&self, job_id: Uuid) -> Option<GenerationJob> {
        self.jobs.lock().await.get(&job_id).cloned()
    }
}

async fn run_generation_task(
    ai_client: AiClient,
    pool: PgPool,
    config: GenerationConfig,
) -> Result<Uuid, AppError> {
    let material_repo = MaterialRepository::new(&pool);
    let gen_repo = GenerationRepository::new(&pool);

    // 1. Fetch material content
    let material = material_repo
        .find_by_id(config.material_id)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;
    
    let text = material.extracted_text.ok_or(AppError::BadRequest("Material has no text".into()))?;

    // 2. Construct Prompt
    let prompt = format!(
        r#"
You are an expert exam generator. Create a test based on the following text.
Return the result as a strictly valid JSON object.
Do not wrap it in markdown code blocks.
The JSON structure must be:
{{
  "questions": [
    {{
      "type": "single" | "multiple" | "blank",
      "prompt": "Question text here",
      "options": ["Option A", "Option B", ...], (Required for single/multiple, optional for blank)
      "correct": "Correct Answer String",
      "explanation": "Explanation of why this is correct"
    }}
  ]
}}

Requirements:
- {mcq_single} Single Choice Questions (type: "single")
- {mcq_multi} Multiple Choice Questions (type: "multiple", correct answer can be comma separated if needed, but here just put the string representation of correct options)
- {fill_blank} Fill in the Blank Questions (type: "blank", options can be null or empty list)

Text:
{text_snippet}
        "#,
        mcq_single = config.mcq_single_count,
        mcq_multi = config.mcq_multi_count,
        fill_blank = config.fill_blank_count,
        text_snippet = text.chars().take(8000).collect::<String>() // Limit text to avoid context limit overflow for MVP
    );

    // 3. Call AI
    let raw_response = ai_client
        .generate(&prompt)
        .await
        .map_err(|e| {
             println!("AI Error: {:?}", e);
             AppError::Internal
        })?;

    // 4. Parse JSON
    // Clean potential markdown blocks just in case
    let clean_json = raw_response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let ai_data: AiResponse = serde_json::from_str(clean_json).map_err(|e| {
        println!("JSON Parse Error: {:?} \nInput: {}", e, clean_json);
        AppError::Internal
    })?;

    // 5. Create QuestionSet
    let set_id = Uuid::new_v4();
    let question_set = QuestionSet {
        id: set_id,
        user_id: config.user_id,
        material_id: config.material_id,
        config_id: config.id,
        status: QuestionSetStatus::Ready,
        created_at: now_utc(),
        completed_at: None,
        raw_ai_response: Some(raw_response),
    };

    gen_repo.create_question_set(&question_set).await.map_err(|_| AppError::Internal)?;

    // 6. Save Questions
    for (idx, q) in ai_data.questions.into_iter().enumerate() {
        let q_type = match q.q_type.to_lowercase().as_str() {
            "single" => QuestionType::Single,
            "multiple" => QuestionType::Multiple,
            "blank" => QuestionType::Blank,
            _ => QuestionType::Single, // Fallback
        };

        let question_entity = Question {
            id: Uuid::new_v4(),
            question_set_id: set_id,
            question_type: q_type,
            prompt: q.prompt,
            options: q.options.unwrap_or_default(),
            correct_answer: q.correct,
            explanation: Some(q.explanation),
            order_index: idx as i32,
        };

        gen_repo.create_question(&question_entity).await.map_err(|_| AppError::Internal)?;
    }

    Ok(set_id)
}

pub fn build_generation_config(
    user_id: Uuid,
    material_id: Uuid,
    mcq_single_count: i32,
    mcq_multi_count: i32,
    fill_blank_count: i32,
    language: Option<String>,
) -> GenerationConfig {
    GenerationConfig {
        id: Uuid::new_v4(),
        user_id,
        material_id,
        mcq_single_count,
        mcq_multi_count,
        fill_blank_count,
        language,
        created_at: now_utc(),
    }
}
