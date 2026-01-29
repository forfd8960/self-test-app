use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    domain::{auth::Claims, error::AppError, user::{User, UserStatus}},
    infra::repositories::{now_utc, UserRepository},
};

#[derive(Debug, serde::Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthService<'a> {
    pub config: &'a AppConfig,
    pub users: UserRepository<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(config: &'a AppConfig, users: UserRepository<'a>) -> Self {
        Self { config, users }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<AuthResponse, AppError> {
        if self.users.find_by_username(username).await.map_err(|_| AppError::Internal)?.is_some() {
            return Err(AppError::BadRequest("username already exists".to_string()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal)?
            .to_string();

        let now = now_utc();
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            password_hash,
            created_at: now,
            updated_at: now,
            status: UserStatus::Active,
        };

        self.users.create_user(&user).await.map_err(|_| AppError::Internal)?;

        self.issue_tokens(&user)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<AuthResponse, AppError> {
        let user = self
            .users
            .find_by_username(username)
            .await
            .map_err(|_| AppError::Internal)?
            // We consciously reveal user existence here as per requirements
            .ok_or(AppError::UserNotFound)?;

        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::Unauthorized)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized)?;

        self.issue_tokens(&user)
    }

    fn issue_tokens(&self, user: &User) -> Result<AuthResponse, AppError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let exp = now + (self.config.jwt_exp_hours as i64 * 3600);

        let claims = Claims {
            sub: user.id.to_string(),
            iat: now as usize,
            exp: exp as usize,
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal)?;

        let refresh_token = Uuid::new_v4().to_string();

        Ok(AuthResponse {
            access_token,
            refresh_token,
        })
    }
}
