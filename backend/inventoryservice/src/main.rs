mod handlers;
mod models;
mod routes;
mod db;
mod order_logic;
mod stock_logic;
mod seed; // New module declaration

use db::Db;
use routes::create_router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: Db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");

    // Call the seeding function
    seed::seed_db(&db).await;

    let app = create_router(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
