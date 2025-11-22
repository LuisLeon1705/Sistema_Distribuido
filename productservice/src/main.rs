use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod errors;
mod handlers;
mod models;
mod state;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "productservice=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Initialize application state
    let app_state = match AppState::new() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to initialize application state: {}", e);
            return;
        }
    };

    // Define application routes
    let app = Router::new()
        // Public routes (still require a valid JWT, but no specific role)
        .route("/products", get(handlers::list_products))
        .route("/products/:id", get(handlers::get_product))
        // Admin-only routes
        .route("/inventory", get(handlers::list_inventory))
        .route("/inventory/increase", post(handlers::increase_stock))
        .route("/inventory/decrease", post(handlers::decrease_stock))
        .with_state(app_state)
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
