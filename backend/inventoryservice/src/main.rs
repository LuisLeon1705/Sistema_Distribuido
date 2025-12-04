mod handlers;
mod models;
mod routes;
mod db;
mod order_logic;
mod stock_logic;
mod seed; // New module declaration
mod temp_order_logic;

use db::Db;
use routes::create_router;
use std::net::SocketAddr;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{self, Duration};

const CLEANUP_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: Db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");

    // Spawn the cleanup task
    tokio::spawn(async move {
        let mut interval = time::interval(CLEANUP_INTERVAL);
        loop {
            interval.tick().await;
            if let Err(e) = temp_order_logic::cleanup_expired_orders() {
                eprintln!("Error during cleanup: {}", e);
            }
        }
    });

    // The seeding function is now called via an API endpoint
    // if let Err(e) = seed::seed_stock(&db).await {
    //     eprintln!("Failed to seed database: {}", e);
    // }

    let app = create_router(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
