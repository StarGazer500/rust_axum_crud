use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Sophisticated Error Type using thiserror
#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Database error: {0}")]
    Database(sqlx::Error),
    
    // #[error("Validation failed: {message}")]
    Validation { message: String },
    
    // #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    // #[error("Conflict: {message}")]
    Conflict { message: String },
    
    // #[error("Authentication failed: {reason}")]
    Authentication { reason: String },
    
    // #[error("Authorization failed: {reason}")]
    Authorization { reason: String },
    
    // #[error("Password hashing failed")]
    PasswordHashing(#[from] bcrypt::BcryptError),
    
    // #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },
    
    // #[error("Internal server error: {message}")]
    Internal { message: String },
}

// Error Response for JSON API
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

// Custom error conversion for database constraints
impl AppError {
    fn from_database_error(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                match db_err.constraint() {
                    Some("credentials_email_key") => AppError::Conflict {
                        message: "Email address already exists".to_string(),
                    },
                    Some("credentials_email_check") => AppError::Validation {
                        message: "Email format is invalid".to_string(),
                    },
                    _ => AppError::Database(err),
                }
            }
            sqlx::Error::RowNotFound => AppError::NotFound {
                resource: "Credentials".to_string(),
            },
            _ => AppError::Database(err),
        }
    }
}

// Convert sqlx::Error to AppError with custom logic
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::from_database_error(err)
    }
}

// Implement IntoResponse for AppError
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message, details) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred",
                    None,
                )
            }
            AppError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                message.as_str(),
                None,
            ),
            AppError::NotFound { resource } => (
                StatusCode::NOT_FOUND,
                "RESOURCE_NOT_FOUND",
                format!("{} not found", resource).as_str(),
                None,
            ),
            AppError::Conflict { message } => (
                StatusCode::CONFLICT,
                "RESOURCE_CONFLICT",
                message.as_str(),
                None,
            ),
            AppError::Authentication { reason } => (
                StatusCode::UNAUTHORIZED,
                "AUTHENTICATION_FAILED",
                reason.as_str(),
                None,
            ),
            AppError::Authorization { reason } => (
                StatusCode::FORBIDDEN,
                "AUTHORIZATION_FAILED",
                reason.as_str(),
                None,
            ),
            AppError::PasswordHashing(err) => {
                tracing::error!("Password hashing error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PASSWORD_HASHING_ERROR",
                    "Password processing failed",
                    None,
                )
            }
            AppError::InvalidEmail { email } => (
                StatusCode::BAD_REQUEST,
                "INVALID_EMAIL",
                "Invalid email format",
                Some(serde_json::json!({ "email": email })),
            ),
            AppError::Internal { message } => {
                tracing::error!("Internal error: {}", message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    "An internal server error occurred",
                    None,
                )
            }
        };

        let body = Json(ErrorResponse {
            error: error_code.to_string(),
            message: message.to_string(),
            details,
        });

        (status, body).into_response()
    }
}

// Convenience constructors for AppError
impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
        }
    }

    pub fn authentication(reason: impl Into<String>) -> Self {
        Self::Authentication {
            reason: reason.into(),
        }
    }

    pub fn authorization(reason: impl Into<String>) -> Self {
        Self::Authorization {
            reason: reason.into(),
        }
    }

    pub fn invalid_email(email: impl Into<String>) -> Self {
        Self::InvalidEmail {
            email: email.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

// Custom Result type alias
pub type AppResult<T> = Result<T, AppError>;

// Email validation with detailed error
fn validate_email(email: &str) -> AppResult<()> {
    if email.is_empty() {
        return Err(AppError::validation("Email cannot be empty"));
    }
    
    if !email.contains('@') {
        return Err(AppError::invalid_email(email));
    }
    
    if !email.contains('.') {
        return Err(AppError::invalid_email(email));
    }
    
    // More sophisticated validation could go here
    Ok(())
}

// Password validation
fn validate_password(password: &str) -> AppResult<()> {
    if password.len() < 8 {
        return Err(AppError::validation(
            "Password must be at least 8 characters long"
        ));
    }
    
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(AppError::validation(
            "Password must contain at least one uppercase letter"
        ));
    }
    
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(AppError::validation(
            "Password must contain at least one lowercase letter"
        ));
    }
    
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::validation(
            "Password must contain at least one digit"
        ));
    }
    
    Ok(())
}

// Updated Repository Layer
pub async fn save_credential_repository(
    input: RequestCredentials,
    pool: &PgPool,
) -> AppResult<ResponseCredentials> {
    let record = sqlx::query!(
        r#"
        INSERT INTO credentials (email, password)
        VALUES ($1, $2)
        RETURNING id, email, password, created_at
        "#,
        input.email,
        input.password
    )
    .fetch_one(pool)
    .await?;

    Ok(ResponseCredentials {
        email: record.email,
        password: "[REDACTED]".to_string(), // Never return actual password
    })
}

pub async fn get_credentials_by_mail_repository(
    email: &str,
    pool: &PgPool,
) -> AppResult<Option<ResponseCredentials>> {
    let record = sqlx::query!(
        "SELECT id, email, password, created_at FROM credentials WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| ResponseCredentials {
        email: r.email,
        password: "[REDACTED]".to_string(),
    }))
}

// Updated Service Layer with comprehensive validation
pub async fn save_credentials_service(
    input: RequestCredentials,
    pool: &PgPool,
) -> AppResult<ResponseCredentials> {
    // Validate input
    validate_email(&input.email)?;
    validate_password(&input.password)?;

    // Hash password
    use bcrypt::{hash, DEFAULT_COST};
    let hashed_password = hash(&input.password, DEFAULT_COST)?;

    let hashed_input = RequestCredentials {
        email: input.email.to_lowercase().trim().to_string(), // Normalize email
        password: hashed_password,
    };

    // Save to database
    save_credential_repository(hashed_input, pool).await
}

pub async fn get_credentials_by_email_service(
    email: &str,
    pool: &PgPool,
) -> AppResult<ResponseCredentials> {
    validate_email(email)?;
    
    let normalized_email = email.to_lowercase().trim();
    
    match get_credentials_by_mail_repository(normalized_email, pool).await? {
        Some(credentials) => Ok(credentials),
        None => Err(AppError::not_found("User")),
    }
}

// Updated Handlers - much cleaner now!
#[axum::debug_handler]
pub async fn save_credentials_handler(
    State(state): State<AppState>,
    Json(body): Json<RequestCredentials>,
) -> AppResult<impl IntoResponse> {
    let response = save_credentials_service(body, &state.db).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[axum::debug_handler]
pub async fn get_credentials_by_email_json_handler(
    State(state): State<AppState>,
    Json(body): Json<GetByEmailRequest>,
) -> AppResult<impl IntoResponse> {
    let credentials = get_credentials_by_email_service(&body.email, &state.db).await?;
    Ok((StatusCode::OK, Json(credentials)))
}

// Bonus: Middleware for logging errors
pub async fn error_logging_middleware<B>(
    request: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let response = next.run(request).await;
    
    // Log if response is an error
    if response.status().is_client_error() || response.status().is_server_error() {
        tracing::warn!(
            status = %response.status(),
            "Request resulted in error"
        );
    }
    
    response
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_email_validation() {
//         assert!(validate_email("test@example.com").is_ok());
//         assert!(validate_email("invalid-email").is_err());
//         assert!(validate_email("").is_err());
//         assert!(validate_email("test@").is_err());
//     }

//     #[test]
//     fn test_password_validation() {
//         assert!(validate_password("StrongPass1").is_ok());
//         assert!(validate_password("weak").is_err());
//         assert!(validate_password("nouppercase1").is_err());
//         assert!(validate_password("NOLOWERCASE1").is_err());
//         assert!(validate_password("NoNumbers").is_err());
//     }
// }