use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use self_test_backend::api::router::{create_router, AppState};
use self_test_backend::config::AppConfig;
use self_test_backend::infra::{ai_client::AiClient, rate_limit::RateLimiter, storage::FileStorage};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[sqlx::test]
async fn test_submit_answers(pool: PgPool) {
    let config = Arc::new(AppConfig {
        jwt_secret: "test_secret".to_string(),
        jwt_expiration_hours: 24,
        upload_dir: "test_uploads".to_string(),
        database_url: "postgres://postgres:postgres@localhost:5432/self_test_db".to_string(), // Ignored in test
        ai_api_key: "test_key".to_string(),
        ai_model: "test_model".to_string(),
        ai_base_url: "http://localhost:8080".to_string(),
        server_port: 3000,
        enable_registration: true,
        max_upload_bytes: 10 * 1024 * 1024,
        jwt_refresh_secret: "refresh_secret".to_string(),
        jwt_refresh_expiration_days: 7,
    });

    let storage = Arc::new(FileStorage::new(&config.upload_dir));
    let ai_client = Arc::new(AiClient::new(&config.ai_base_url, &config.ai_api_key, &config.ai_model));
    let rate_limiter = Arc::new(RateLimiter::new(10));
    let generation_jobs = Arc::new(Mutex::new(HashMap::new()));

    let state = AppState {
        config: config.clone(),
        pool: pool.clone(),
        storage,
        ai_client,
        rate_limiter,
        generation_jobs,
    };

    let _app = create_router(state);

    // TODO: Create user, material, question set
    // TODO: Submit answers
    // TODO: Assert score and feedback

    // Placeholder assertion until implementation
    assert!(true);
}
