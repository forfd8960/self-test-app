#![allow(dead_code)]

mod api;
mod config;
mod domain;
mod infra;
mod services;

use std::{net::SocketAddr, sync::Arc};

use api::router::{create_router, AppState};
use infra::{ai_client::AiClient, db::init_pool, rate_limit::RateLimiter, storage::FileStorage};
use services::generation_service::GenerationJob;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "self_test_backend=debug,tower_http=debug".into()),
        )
        .init();

    let config = config::AppConfig::from_env()?;
    info!("Configuration loaded");


    let pool = init_pool(&config).await?;
    let storage = FileStorage::new(&config.upload_dir);
    let ai_client = AiClient::new(&config);
    let rate_limiter = RateLimiter::new(10);

    let state = AppState {
        config: Arc::new(config),
        pool,
        storage: Arc::new(storage),
        ai_client: Arc::new(ai_client),
        rate_limiter: Arc::new(rate_limiter),
        generation_jobs: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::<uuid::Uuid, GenerationJob>::new())),
    };

    let app = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
