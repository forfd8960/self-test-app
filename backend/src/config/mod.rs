use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_exp_hours: u64,
    pub ai_api_key: String,
    pub ai_base_url: String,
    pub ai_model: String,
    pub upload_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok();

        let jwt_exp_hours = std::env::var("JWT_EXP_HOURS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(6);

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            jwt_refresh_secret: std::env::var("JWT_REFRESH_SECRET")?,
            jwt_exp_hours,
            ai_api_key: std::env::var("AI_API_KEY")?,
            ai_base_url: std::env::var("AI_BASE_URL")?,
            ai_model: std::env::var("AI_MODEL")?,
            upload_dir: std::env::var("UPLOAD_DIR")?,
        })
    }
}
