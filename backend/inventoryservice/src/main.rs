use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use chrono::Utc;

mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod orders;
mod state;

const ORDER_TTL_SECONDS: i64 = 3600; // 1 hour

fn start_cleanup_task(app_state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300)); // Check every 5 minutes
        loop {
            interval.tick().await;
            tracing::debug!("Running cleanup task for temporary orders...");
            let now = Utc::now();
            app_state.temporal_orders.retain(|_, order| {
                let elapsed = now.signed_duration_since(order.last_updated).num_seconds();
                if elapsed > ORDER_TTL_SECONDS {
                    tracing::info!("Removing expired temporary order {}", order.id);
                    false
                } else {
                    true
                }
            });
        }
    });
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "inventoryservice=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Initialize application state
    let app_state = match AppState::new().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to initialize application state: {}", e);
            return;
        }
    };

    // Start the cleanup task
    start_cleanup_task(app_state.clone());

    // Define application routes
    let app = Router::new()
        // Inventory routes
        .route("/inventory", get(handlers::list_inventory))
        .route("/inventory/increase", post(handlers::increase_stock))
        .route("/inventory/decrease", post(handlers::decrease_stock))
        // Order routes
        .route("/orders", post(orders::create_temporal_order).get(orders::get_my_orders))
        .route("/orders/confirm/:id", post(orders::confirm_order))
        .route("/orders/:id", get(orders::get_order_details))
        .route("/admin/orders", get(orders::list_all_orders))
        .with_state(app_state)
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
