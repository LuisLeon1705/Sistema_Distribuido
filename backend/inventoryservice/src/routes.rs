use crate::{
    db::Db,
    handlers::{create_order, get_orders},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/orders", get(get_orders).post(create_order))
        .route("/orders/:order_id", get(crate::handlers::get_order_by_id_handler))
        .route("/orders/:order_id/items", get(crate::handlers::get_order_items_handler))
        .route("/orders/user/:user_id", get(crate::handlers::get_orders_by_user_id_handler))
        .route("/orders/status", post(crate::handlers::update_order_status_handler))
        .route("/stock", post(crate::handlers::add_stock_handler).get(crate::handlers::get_stock_handler))
        .route("/stock/:product_id", get(crate::handlers::get_stock_handler).put(crate::handlers::update_stock_handler).delete(crate::handlers::delete_stock_handler))
        .route("/temp_orders", post(crate::handlers::add_temp_order_handler))
        .route("/temp_orders/user/:user_id", get(crate::handlers::get_temp_orders_by_user_id_handler))
        .route("/seed", post(crate::handlers::seed_stock_handler)) // Add this route
        .with_state(db)
}
