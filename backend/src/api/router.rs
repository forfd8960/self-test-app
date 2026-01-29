use std::{collections::HashMap, sync::Arc};

use axum::{http::Method, Router};
use sqlx::PgPool;
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    api::{auth, generation, materials, tests, middleware},
    config::AppConfig,
    infra::{ai_client::AiClient, rate_limit::RateLimiter, storage::FileStorage},
    services::generation_service::GenerationJob,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub pool: PgPool,
    pub storage: Arc<FileStorage>,
    pub ai_client: Arc<AiClient>,
    pub rate_limiter: Arc<RateLimiter>,
    pub generation_jobs: Arc<Mutex<HashMap<Uuid, GenerationJob>>>,
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    let protected_routes = Router::new()
        .nest("/materials", materials::router())
        .nest("/generation", generation::router())
        .nest("/tests", tests::router())
        .layer(axum::middleware::from_fn_with_state(state.clone(), middleware::auth::auth_middleware));

    Router::new()
        .nest("/auth", auth::router())
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
