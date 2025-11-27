use crate::{
    db::Db,
    handlers::{create_order, get_orders, add_stock_handler, update_stock_handler, delete_stock_handler, get_stock_handler, get_order_by_id_handler, get_orders_by_user_id_handler},
};
use axum::{
    routing::{get, post, put, delete},
    Router,
};

pub fn create_router(db: Db) -> Router {
    Router::new()
        .route("/orders", get(get_orders).post(create_order))
        .route("/orders/:order_id", get(crate::handlers::get_order_by_id_handler))
        .route("/orders/user/:user_id", get(crate::handlers::get_orders_by_user_id_handler))
        .route("/stock", post(crate::handlers::add_stock_handler).get(crate::handlers::get_stock_handler))
        .route("/stock/:product_id", get(crate::handlers::get_stock_handler).put(crate::handlers::update_stock_handler).delete(crate::handlers::delete_stock_handler))
        .with_state(db)
}
