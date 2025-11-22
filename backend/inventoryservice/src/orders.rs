use axum::{
    extract::{Path, State, TypedHeader},
    http::StatusCode,
    Json,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use crate::{
    auth::{is_admin, AuthenticatedUser},
    errors::ApiError,
    models::{CreateOrderRequest, Order, OrderItem, OrderResponse, TemporalOrder},
    state::AppState,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Product {
    id: i32,
    name: String,
    price: f64,
    // other fields are not needed for order creation
}

pub async fn create_temporal_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<TemporalOrder>, ApiError> {
    let temporal_order = TemporalOrder::new(user.user_id, payload);
    
    // The clone here is cheap, as the inner fields of TemporalOrder are either small or behind an Arc
    state.temporal_orders.insert(temporal_order.id.clone(), temporal_order.clone());

    Ok(Json(temporal_order))
}

pub async fn confirm_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(temporal_order_id): Path<String>,
) -> Result<Json<OrderResponse>, ApiError> {
    let temporal_order = state.temporal_orders.remove(&temporal_order_id)
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Temporal order not found or expired"))?
        .1; // Get the value from the DashMap remove

    // Check if the user is authorized to confirm this order
    if temporal_order.user_id != user.user_id {
        // Re-insert the temporal order as it was not the user's intention to consume it.
        state.temporal_orders.insert(temporal_order_id, temporal_order);
        return Err(ApiError::new(StatusCode::FORBIDDEN, "You are not authorized to confirm this order"));
    }

    let payload = temporal_order.order_details;
    let mut tx = state.db_pool.begin().await?;

    let mut total_price = 0.0;
    let mut order_items_to_create = Vec::new();
    let client = reqwest::Client::new();
    
    let token = auth.token();

    for item in &payload.items {
        // 1. Get product details from inventoryservice
        let product_url = format!("http://inventoryservice:3001/products/{}", item.product_id);
        let product_res = client.get(&product_url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to inventory service"))?;
        
        if !product_res.status().is_success() {
            return Err(ApiError::new(StatusCode::NOT_FOUND, format!("Product with id {} not found in inventory service", item.product_id)));
        }

        let product: Product = product_res.json().await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse product data"))?;

        // 2. Check stock in our own database
        let stock_record = sqlx::query!("SELECT quantity FROM stock WHERE product_id = $1", item.product_id)
            .fetch_optional(&mut *tx)
            .await?;
            
        let stock_quantity = stock_record.and_then(|r| r.quantity).unwrap_or(0);

        if stock_quantity < item.quantity {
            return Err(ApiError::new(StatusCode::BAD_REQUEST, format!("Not enough stock for product with id {}", item.product_id)));
        }

        total_price += product.price * item.quantity as f64;
        order_items_to_create.push((item.product_id, item.quantity, product.price));
    }

    let order_id = sqlx::query!(
        "INSERT INTO orders (user_id, total_price) VALUES ($1, $2) RETURNING id",
        user.user_id,
        total_price
    )
    .fetch_one(&mut *tx)
    .await?
    .id;

    for (product_id, quantity, price_at_time_of_purchase) in &order_items_to_create {
        sqlx::query!(
            "INSERT INTO order_items (order_id, product_id, quantity, price_at_time_of_purchase) VALUES ($1, $2, $3, $4)",
            order_id,
            product_id,
            quantity,
            price_at_time_of_purchase
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "UPDATE stock SET quantity = quantity - $1 WHERE product_id = $2",
            quantity,
            product_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let created_order = sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = $1", order_id)
        .fetch_one(&state.db_pool)
        .await?;
    
    let items = sqlx::query_as!(OrderItem, "SELECT * FROM order_items WHERE order_id = $1", order_id)
        .fetch_all(&state.db_pool)
        .await?;

    let response = OrderResponse {
        id: created_order.id,
        user_id: created_order.user_id,
        total_price: created_order.total_price,
        status: created_order.status,
        created_at: created_order.created_at,
        items,
    };

    Ok(Json(response))
}

pub async fn get_my_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = sqlx::query_as!(Order, "SELECT * FROM orders WHERE user_id = $1", user.user_id)
        .fetch_all(&state.db_pool)
        .await?;

    Ok(Json(orders))
}

pub async fn get_order_details(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(order_id): Path<i32>,
) -> Result<Json<OrderResponse>, ApiError> {
    let order = sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = $1 AND user_id = $2", order_id, user.user_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Order not found"))?;

    let items = sqlx::query_as!(OrderItem, "SELECT * FROM order_items WHERE order_id = $1", order_id)
        .fetch_all(&state.db_pool)
        .await?;

    let response = OrderResponse {
        id: order.id,
        user_id: order.user_id,
        total_price: order.total_price,
        status: order.status,
        created_at: order.created_at,
        items,
    };

    Ok(Json(response))
}

pub async fn list_all_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<Order>>, ApiError> {
    is_admin(&user)?;

    let orders = sqlx::query_as!(Order, "SELECT * FROM orders")
        .fetch_all(&state.db_pool)
        .await?;

    Ok(Json(orders))
}
