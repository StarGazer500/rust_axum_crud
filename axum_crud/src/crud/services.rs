
use crate::crud::model::ResponseCredentials;
use crate::crud::dto::RequestCredentials;
use crate::crud::error_traits::{AppResult,AppError};
use sqlx::PgPool;
use crate::crud::repository::{save_credential_repository,get_credentials_by_mail_repository};

// pub async fn save_credentials_service(
//     input: RequestCredentials, 
//     pool: &PgPool
// ) -> Result<ResponseCredentials, sqlx::Error> {
//     // Using sqlx::query! macro for compile-time SQL checking
//     let response_credentials = save_credential_repository(input, pool).await?; 
//     Ok(response_credentials)
    
// }

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



// pub async fn get_credentials_by_email_service(
//     email: &str,
//     pool: &PgPool
// ) -> Result<Option<ResponseCredentials>, sqlx::Error> {
//     let data = get_credentials_by_mail_repository(email, pool).await?;
//     Ok(data)
    
// }

pub async fn get_credentials_by_email_service(
    email: &str,
    pool: &PgPool,
) -> AppResult<ResponseCredentials> {
    validate_email(email)?;
    
    let normalized_email = email.to_lowercase().trim().to_string();
    
     match get_credentials_by_mail_repository(&normalized_email, pool).await? {
        Some(credentials) => Ok(credentials),
        None => Err(AppError::not_found("User")),
    }
}
