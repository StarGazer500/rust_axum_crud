

use axum::{
 
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Serialize};
use thiserror::Error;

// Sophisticated Error Type using thiserror
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(sqlx::Error),
    
    #[error("Validation failed: {message}")]
    Validation { message: String },
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Conflict: {message}")]
    Conflict { message: String },
    
    // #[error("Authentication failed: {reason}")]
    // Authentication { reason: String },
    
    // #[error("Authorization failed: {reason}")]
    // Authorization { reason: String },
    
    #[error("Password hashing failed")]
    PasswordHashing(#[from] bcrypt::BcryptError),
    
    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },
    
    // #[error("Internal server error: {message}")]
    // Internal { message: String },
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
        let (status, error_code, message, details) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "A database error occurred".to_string(),
                    None,
                )
            }
            AppError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                message,
                None,
            ),
            AppError::NotFound { resource } => (
                StatusCode::NOT_FOUND,
                "RESOURCE_NOT_FOUND".to_string(),
                format!("{} not found", resource), // Now owned String
                None,
            ),
            AppError::Conflict { message } => (
                StatusCode::CONFLICT,
                "RESOURCE_CONFLICT".to_string(),
                message,
                None,
            ),
            // AppError::Authentication { reason } => (
            //     StatusCode::UNAUTHORIZED,
            //     "AUTHENTICATION_FAILED".to_string(),
            //     reason,
            //     None,
            // ),
            // AppError::Authorization { reason } => (
            //     StatusCode::FORBIDDEN,
            //     "AUTHORIZATION_FAILED".to_string(),
            //     reason,
            //     None,
            // ),
            AppError::PasswordHashing(err) => {
                tracing::error!("Password hashing error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PASSWORD_HASHING_ERROR".to_string(),
                    "Password processing failed".to_string(),
                    None,
                )
            }
            AppError::InvalidEmail { email } => (
                StatusCode::BAD_REQUEST,
                "INVALID_EMAIL".to_string(),
                "Invalid email format".to_string(),
                Some(serde_json::json!({ "email": email })),
            ),
            // AppError::Internal { message } => {
            //     tracing::error!("Internal error: {}", message);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "INTERNAL_SERVER_ERROR".to_string(),
            //         "An internal server error occurred".to_string(),
            //         None,
            //     )
            // }
        };

        let body = Json(ErrorResponse {
            error: error_code,
            message,
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

    // pub fn conflict(message: impl Into<String>) -> Self {
    //     Self::Conflict {
    //         message: message.into(),
    //     }
    // }

    // pub fn authentication(reason: impl Into<String>) -> Self {
    //     Self::Authentication {
    //         reason: reason.into(),
    //     }
    // }

    // pub fn authorization(reason: impl Into<String>) -> Self {
    //     Self::Authorization {
    //         reason: reason.into(),
    //     }
    // }

    pub fn invalid_email(email: impl Into<String>) -> Self {
        Self::InvalidEmail {
            email: email.into(),
        }
    }

    // pub fn internal(message: impl Into<String>) -> Self {
    //     Self::Internal {
    //         message: message.into(),
    //     }
    // }
}

// Custom Result type alias
pub type AppResult<T> = Result<T, AppError>;

