


use crate::crud::model::ResponseCredentials;
use crate::crud::dto::RequestCredentials;
use crate::crud::error_traits::AppResult;
use sqlx::PgPool;

// pub async fn save_credential_repository(
//     input: RequestCredentials, 
//     pool: &PgPool

// )->Result<ResponseCredentials, sqlx::Error>{

//     // Using sqlx::query! macro for compile-time SQL checking
//     let record = sqlx::query!(
//         r#"
//         INSERT INTO credentials (email, password)
//         VALUES ($1, $2)
//         RETURNING id, email, password, created_at
//         "#,
//         input.email,
//         input.password
//     )
//     .fetch_one(pool)
//     .await?;
    
//     // The macro automatically infers types and provides compile-time safety
//     let response = ResponseCredentials {
//         email: record.email,
//         password: record.password,
//     };
    
//     Ok(response)

// }

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


// pub async fn get_credentials_by_mail_repository(
//     email: &str,
//     pool: &PgPool,
// ) -> Result<Option<ResponseCredentials>, sqlx::Error> {
//     let record = sqlx::query!(
//         "SELECT id, email, password, created_at FROM credentials WHERE email = $1",
//         email
//     )
//     .fetch_optional(pool)
//     .await?;
    
//     Ok(record.map(|r| ResponseCredentials {
//         email: r.email,
//         password: r.password, // In production, don't return passwords!
//     }))
// }

// O


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