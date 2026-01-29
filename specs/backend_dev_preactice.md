# Backend Development Rules & Best Practices
**Rust + Axum + SQLx**

## Project Structure & Organization

Organize your project with clear module boundaries and separation of concerns:

```
src/
├── main.rs           # Application entry point
├── config/           # Configuration management
├── routes/           # HTTP route handlers
├── handlers/         # Business logic for routes
├── models/           # Data models and domain types
├── db/               # Database operations and queries
├── middleware/       # Custom middleware (auth, logging, etc.)
├── services/         # Business logic services
├── utils/            # Helper functions and utilities
├── error.rs          # Error types and handling
└── lib.rs            # Library exports (optional)
```

Keep each module focused on a single responsibility. If a file exceeds 400-500 lines, consider splitting it into smaller, more focused modules.

## Application Architecture

Use a layered architecture to separate concerns:

**Routes Layer** handles HTTP-specific concerns like request parsing and response formatting. Keep routes thin and delegate to handlers.

**Handlers Layer** orchestrates the business logic, calling services and managing transactions. This is where you coordinate multiple operations.

**Services Layer** contains pure business logic, independent of HTTP concerns. These functions should be testable without spinning up a web server.

**Database Layer** handles all database interactions through repositories or query modules. Abstract database specifics behind trait interfaces when possible.

## Type Safety & Error Handling

Leverage Rust's type system to make illegal states unrepresentable:

```rust
// Good: Types prevent invalid states
pub struct UserId(uuid::Uuid);
pub struct Email(String); // Validated email

// Avoid: Primitive obsession
pub struct User {
    id: String,  // Could be invalid UUID
    email: String,  // Could be invalid email
}
```

Create custom error types using `thiserror` for clear error handling:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, &self.to_string()),
            AppError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };
        
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

Use `Result<T, E>` throughout your application. Never use `unwrap()` or `expect()` in production code except in truly unreachable situations. Prefer `?` operator for error propagation.

## Database Management with SQLx

Use compile-time checked queries with SQLx's `query!` and `query_as!` macros:

```rust
// Compile-time verified query
let user = sqlx::query_as!(
    User,
    r#"
    SELECT id, email, created_at
    FROM users
    WHERE email = $1
    "#,
    email
)
.fetch_optional(&pool)
.await?;
```

Set up SQLx with the offline mode for CI/CD environments where the database isn't available:

```bash
cargo sqlx prepare
```

This generates `.sqlx/` directory with query metadata for compile-time checking without a running database.

Always use parameterized queries to prevent SQL injection. SQLx's macros handle this automatically, but if you must use raw queries, never concatenate user input into SQL strings.

## Connection Pool Management

Configure your connection pool appropriately based on your workload:

```rust
let pool = PgPoolOptions::new()
    .max_connections(20)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(&database_url)
    .await?;
```

The general rule: `max_connections` should be less than your database's max connections. For web applications, start with `(available_cores * 2) + effective_spindle_count` and tune based on monitoring.

Don't create new pools per request. Share a single pool across your application using Axum's state management.

## Database Migrations

Use SQLx's migration system for schema management:

```bash
sqlx migrate add create_users_table
```

Keep migrations small and focused. Each migration should represent a single logical change. Never modify existing migrations that have been deployed to production.

Write both `up` and `down` migrations when possible. This allows you to roll back changes if needed.

```sql
-- migrations/20240101_create_users.up.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

## Request Handling & Routing

Organize routes logically using Axum's nested routing:

```rust
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .layer(middleware::from_fn(logging_middleware))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", user_routes())
        .nest("/posts", post_routes())
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
```

Use extractors to parse and validate request data:

```rust
use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8))]
    password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    payload.validate()?;
    
    let user = state.user_service
        .create_user(&payload.email, &payload.password)
        .await?;
    
    Ok(Json(UserResponse::from(user)))
}
```

## Validation & Input Sanitization

Validate all input at the API boundary using the `validator` crate or custom validation logic:

```rust
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    name: Option<String>,
    
    #[validate(custom = "validate_age")]
    age: Option<i32>,
    
    #[validate(url)]
    website: Option<String>,
}

fn validate_age(age: &i32) -> Result<(), ValidationError> {
    if *age >= 18 && *age <= 120 {
        Ok(())
    } else {
        Err(ValidationError::new("age must be between 18 and 120"))
    }
}
```

Never trust user input. Sanitize data before storing it in the database and before rendering it in responses. Use prepared statements (which SQLx does automatically) to prevent SQL injection.

## Authentication & Authorization

Implement JWT-based authentication with proper security practices:

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // Subject (user ID)
    exp: usize,   // Expiration time
    iat: usize,   // Issued at
}

pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::TokenCreation)
}
```

Create middleware for authentication:

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;
    
    let claims = decode_jwt(token, &state.config.jwt_secret)?;
    
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}
```

Hash passwords using `argon2` or `bcrypt`. Never store plaintext passwords:

```rust
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::PasswordHash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::PasswordHash)?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
```

## Configuration Management

Use environment variables and the `config` crate for configuration:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
```

Never hardcode secrets. Use environment variables or secret management systems. Validate configuration at startup to fail fast if misconfigured.

## Logging & Observability

Set up structured logging with `tracing`:

```rust
use tracing::{info, error, warn, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

#[instrument(skip(state))]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!(email = %payload.email, "Creating new user");
    
    match state.user_service.create_user(&payload.email, &payload.password).await {
        Ok(user) => {
            info!(user_id = %user.id, "User created successfully");
            Ok(Json(UserResponse::from(user)))
        }
        Err(e) => {
            error!(error = ?e, "Failed to create user");
            Err(e)
        }
    }
}
```

Log important events, errors, and performance metrics. Use appropriate log levels: `trace` for fine-grained details, `debug` for developer info, `info` for general events, `warn` for concerning situations, `error` for failures.

Implement request ID tracking to correlate logs across a request:

```rust
use uuid::Uuid;

pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    
    let response = next.run(request).await;
    
    response
}
```

## Middleware & Request/Response Processing

Create reusable middleware for cross-cutting concerns:

```rust
use axum::middleware;
use std::time::Instant;

pub async fn timing_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let path = request.uri().path().to_owned();
    let method = request.method().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    info!(
        method = %method,
        path = %path,
        status = %response.status(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );
    
    response
}
```

Apply middleware in the correct order. Generally: logging → CORS → authentication → business logic.

## Testing Strategy

Write unit tests for business logic and integration tests for API endpoints:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    
    #[tokio::test]
    async fn test_create_user_success() {
        let app = create_test_app().await;
        
        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/users")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"email":"test@example.com","password":"securepass123"}"#,
            ))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
    }
    
    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let app = create_test_app().await;
        
        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/users")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"email":"invalid-email","password":"securepass123"}"#,
            ))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
```

Use `sqlx::test` macro for database integration tests:

```rust
#[sqlx::test]
async fn test_user_repository_create(pool: PgPool) {
    let repo = UserRepository::new(pool);
    
    let user = repo
        .create("test@example.com", "hashed_password")
        .await
        .unwrap();
    
    assert_eq!(user.email, "test@example.com");
}
```

Mock external dependencies and test edge cases. Focus on testing behavior rather than implementation details.

## Performance Optimization

Use async operations efficiently. Don't block the runtime with CPU-intensive tasks:

```rust
use tokio::task;

// For CPU-intensive work, use spawn_blocking
pub async fn process_image(data: Vec<u8>) -> Result<Vec<u8>, AppError> {
    task::spawn_blocking(move || {
        // CPU-intensive image processing
        process_image_sync(data)
    })
    .await
    .map_err(|_| AppError::TaskJoin)?
}
```

Batch database operations when possible:

```rust
// Instead of multiple queries
for user_id in user_ids {
    let user = get_user(user_id).await?;
    users.push(user);
}

// Use a single query
let users = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = ANY($1)",
    &user_ids
)
.fetch_all(&pool)
.await?;
```

Use database indexes appropriately. Index foreign keys and columns frequently used in WHERE clauses, but don't over-index as it slows writes.

Implement pagination for list endpoints:

```rust
#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    page: i64,
    
    #[serde(default = "default_page_size")]
    page_size: i64,
}

fn default_page() -> i64 { 1 }
fn default_page_size() -> i64 { 20 }

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<User>>, AppError> {
    let offset = (params.page - 1) * params.page_size;
    
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        params.page_size,
        offset
    )
    .fetch_all(&state.pool)
    .await?;
    
    Ok(Json(PaginatedResponse { data: users, page: params.page }))
}
```

## Security Best Practices

Implement rate limiting to prevent abuse:

```rust
use tower::limit::RateLimitLayer;
use std::time::Duration;

let app = Router::new()
    .route("/api/login", post(login))
    .layer(RateLimitLayer::new(5, Duration::from_secs(60)));
```

Set security headers:

```rust
use axum::middleware;
use tower_http::set_header::SetResponseHeaderLayer;

let app = Router::new()
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ));
```

Validate content types and implement CORS properly:

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin("https://example.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
```

Never expose internal error details to clients in production. Log detailed errors but return generic messages.

## Database Transactions

Use transactions for operations that must be atomic:

```rust
pub async fn transfer_funds(
    pool: &PgPool,
    from_account: i64,
    to_account: i64,
    amount: Decimal,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;
    
    sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
        amount,
        from_account
    )
    .execute(&mut *tx)
    .await?;
    
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        amount,
        to_account
    )
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(())
}
```

Keep transactions short. Don't perform I/O operations or external API calls within transactions.

## Graceful Shutdown

Implement graceful shutdown to finish in-flight requests:

```rust
use tokio::signal;

pub async fn run_server(app: Router, port: u16) -> Result<(), AppError> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    
    info!("Server listening on {}", addr);
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    info!("Shutdown signal received, starting graceful shutdown");
}
```

## Documentation

Document public APIs with doc comments:

```rust
/// Creates a new user with the provided email and password.
///
/// # Arguments
/// * `email` - Valid email address
/// * `password` - Plain text password (will be hashed)
///
/// # Returns
/// * `Ok(User)` - Successfully created user
/// * `Err(AppError::Validation)` - Invalid input
/// * `Err(AppError::Database)` - Database error
///
/// # Example
/// ```rust
/// let user = create_user("user@example.com", "securepass").await?;
/// ```
pub async fn create_user(
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    // Implementation
}
```

Keep a CHANGELOG.md documenting API changes, especially breaking changes.

## Deployment Considerations

Build optimized release binaries:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

Use multi-stage Docker builds to minimize image size:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api-server /usr/local/bin/
CMD ["api-server"]
```

Implement health check endpoints:

```rust
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn readiness_check(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => (StatusCode::OK, "Ready"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "Not ready"),
    }
}
```

---

These practices represent current best practices for Rust backend development. The ecosystem evolves rapidly, so stay updated with Axum, SQLx, and Rust releases. Prioritize correctness, safety, and maintainability over premature optimization. Let the compiler help you write robust code.