use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{PgPool};
use uuid::Uuid;
// Nota: Se ha eliminado `use serde_json::json;` si no se usa.

use crate::{
    // Se unifica la importación de tipos a `models`. Esto resuelve E0308.
    models::{CreateOrder, Order, CreateStock, UpdateStock, Stock, TempOrder, UpdateOrderStatusPayload, OrderItem}, 
    // Se importa OrderManager. Esto resuelve E0433/E0432.
    order_logic::{OrderManager}, 
    // Se mantiene la importación necesaria para la gestión de stock
    stock_logic::{StockManager, StockManagement}, 
    db::Db, // Alias del Pool de la DB
    seed, // Import the seed module
};

// ----------------------------------------------------------------------
// Seeding Handler
// ----------------------------------------------------------------------

pub async fn seed_stock_handler(State(db): State<Db>) -> Result<StatusCode, impl IntoResponse> {
    match seed::seed_stock(&db).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            eprintln!("Failed to seed database: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to seed database: {}", e)))
        }
    }
}

// ----------------------------------------------------------------------
// Order Handlers
// ----------------------------------------------------------------------

pub async fn get_orders(State(db): State<Db>) -> Result<Json<Vec<Order>>, StatusCode> {
    let orders = sqlx::query_as::<_, Order>(
        r#"
        SELECT id, user_id, total_price, status, created_at
        FROM orders
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&db)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch orders: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(orders))
}

pub async fn create_order(
    State(db): State<Db>,
    Json(payload): Json<CreateOrder>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut tx = db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = OrderManager::create_order_in_db(&mut tx, payload.user_id).await;

    match result {
        Ok(order) => {
            tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::CREATED, Json(order)))
        }
        Err(e) => { 
            eprintln!("Failed to create order: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_order_by_id_handler(
    State(pool): State<Db>,
    Path(order_id): Path<i32>,
) -> Result<Json<Order>, StatusCode> {
    match OrderManager::get_order_by_id(&pool, order_id).await {
        Ok(Some(order)) => Ok(Json(order)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_e) => {
            eprintln!("Error getting order by ID: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_orders_by_user_id_handler(
    State(pool): State<Db>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    match OrderManager::get_orders_by_user_id(&pool, user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(_e) => {
            eprintln!("Error getting orders by user ID: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_order_status_handler(
    State(db): State<Db>,
    Json(payload): Json<UpdateOrderStatusPayload>,
) -> Result<Json<Order>, StatusCode> {
    match OrderManager::update_order_status(&db, payload.order_id, payload.new_status).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => {
            eprintln!("Failed to update order status: {}", e);
            // Map specific errors to appropriate HTTP status codes
            if e.to_string().contains("not found") {
                Err(StatusCode::NOT_FOUND)
            } else if e.to_string().contains("Cannot change status") {
                Err(StatusCode::BAD_REQUEST)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn get_order_items_handler(
    State(pool): State<Db>,
    Path(order_id): Path<i32>,
) -> Result<Json<Vec<OrderItem>>, StatusCode> {
    match OrderManager::get_order_items_by_order_id(&pool, order_id).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            eprintln!("Error getting order items: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------------------------------------------------
// Stock Handlers
// ----------------------------------------------------------------------

pub async fn add_stock_handler(
    State(pool): State<PgPool>,
    Json(create_stock): Json<CreateStock>,
) -> Result<Json<Stock>, StatusCode> {
    // Usa el tipo `CreateStock` importado de `models`
    match StockManager::add_stock(&pool, create_stock).await {
        Ok(stock) => Ok(Json(stock)),
        Err(_e) => {
            eprintln!("Error adding stock: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_stock_handler(
    State(pool): State<PgPool>,
    Path(product_id): Path<Uuid>,
    Json(update_stock): Json<UpdateStock>,
) -> Result<Json<Stock>, StatusCode> {
    // Try to update existing stock
    match StockManager::update_stock(&pool, product_id, update_stock.clone()).await {
        Ok(Some(stock)) => Ok(Json(stock)),
        Ok(None) => {
            // Stock doesn't exist, create it
            let create_stock = CreateStock {
                product_id,
                quantity: update_stock.quantity.unwrap_or(0),
                warehouse_location: update_stock.warehouse_location.unwrap_or_else(|| "Almacén Principal".to_string()),
            };
            
            match StockManager::add_stock(&pool, create_stock).await {
                Ok(stock) => Ok(Json(stock)),
                Err(e) => {
                    eprintln!("Error creating stock: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(_e) => {
            eprintln!("Error updating stock: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_stock_handler(
    State(pool): State<PgPool>,
    Path(product_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match StockManager::delete_stock(&pool, product_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_e) => {
            eprintln!("Error deleting stock: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_stock_handler(
    State(pool): State<PgPool>,
    product_id: Option<Path<Uuid>>,
) -> Result<Json<Vec<Stock>>, StatusCode> {
    let id = product_id.map(|Path(id)| id);
    match StockManager::get_stock(&pool, id).await {
        Ok(stocks) => Ok(Json(stocks)),
        Err(_e) => {
            eprintln!("Error getting stock: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------------------------------------------------
// Temp Order Handlers
// ----------------------------------------------------------------------

use crate::temp_order_logic;

pub async fn add_temp_order_handler(
    State(db): State<Db>,
    Json(payload): Json<CreateOrder>,
) -> Result<impl IntoResponse, StatusCode> {
    match temp_order_logic::add_temp_order(&db, payload).await {
        Ok(temp_order) => Ok((StatusCode::CREATED, Json(temp_order))),
        Err(e) => {
            eprintln!("Failed to add temporary order: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_temp_orders_by_user_id_handler(
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<TempOrder>>, StatusCode> {
    match temp_order_logic::get_temp_orders_by_user_id(user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => {
            eprintln!("Failed to retrieve temporary orders: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
