use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};
use uuid::Uuid;

use crate::{
    domain::{error::AppError, generation_config::GenerationConfig},
    infra::{ai_client::AiClient, repositories::{now_utc, GenerationRepository}},
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
    pub repo: GenerationRepository<'a>,
    pub jobs: Arc<Mutex<HashMap<Uuid, GenerationJob>>>,
}

impl<'a> GenerationService<'a> {
    pub fn new(
        ai_client: &'a AiClient,
        repo: GenerationRepository<'a>,
        jobs: Arc<Mutex<HashMap<Uuid, GenerationJob>>>,
    ) -> Self {
        Self { ai_client, repo, jobs }
    }

    pub async fn create_job(&self, config: GenerationConfig) -> Result<GenerationJob, AppError> {
        self.repo
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

        tokio::spawn(async move {
            let mut jobs_guard = jobs.lock().await;
            if let Some(entry) = jobs_guard.get_mut(&job_id) {
                entry.status = JobStatus::Generating;
            }
            drop(jobs_guard);

            let result = generate_with_retry(&ai_client, "generate questions").await;

            let mut jobs_guard = jobs.lock().await;
            if let Some(entry) = jobs_guard.get_mut(&job_id) {
                match result {
                    Ok(_) => entry.status = JobStatus::Ready,
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

async fn generate_with_retry(client: &AiClient, prompt: &str) -> Result<String, AppError> {
    let mut attempt = 0;
    let mut delay = Duration::from_millis(200);

    loop {
        attempt += 1;
        match client.generate(prompt).await {
            Ok(response) => return Ok(response),
            Err(_) if attempt < 3 => {
                sleep(delay).await;
                delay *= 2;
            }
            Err(_) => return Err(AppError::Internal),
        }
    }
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
