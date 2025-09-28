
use sqlx::{PgPool};



pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}