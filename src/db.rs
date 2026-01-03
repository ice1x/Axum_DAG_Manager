use sqlx::PgPool;
use std::env;

pub async fn setup_database() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/dag_service".to_string());

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
