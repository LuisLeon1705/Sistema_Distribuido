use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use dotenvy::dotenv;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    // Load .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
