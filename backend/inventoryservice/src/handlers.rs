use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{PgPool};
// ⚠️ Nota: Se ha eliminado `use serde_json::json;` si no se usa.

use crate::{
    // ✅ Se unifica la importación de tipos a `models`. Esto resuelve E0308.
    models::{CreateOrder, Order, CreateStock, UpdateStock, Stock}, 
    // ✅ Se importa OrderManager. Esto resuelve E0433/E0432.
    order_logic::{OrderManager}, 
    // Se mantiene la importación necesaria para la gestión de stock
    stock_logic::{StockManager, StockManagement}, 
    db::Db // Alias del Pool de la DB
};

// ----------------------------------------------------------------------
// Order Handlers
// ----------------------------------------------------------------------

pub async fn get_orders(State(db): State<Db>) -> Result<Json<Vec<Order>>, StatusCode> {
    let orders = sqlx::query_as!(
        Order,
        r#"
        SELECT id as "id!",
               user_id as "user_id!",
               total_price as "total_price!",
               status as "status!: _",
               created_at as "created_at!"
        FROM orders
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

    // ✅ La llamada ahora usa order_logic::OrderManager, que acabamos de definir.
    let result = OrderManager::create_order_in_db(&mut tx, payload.user_id, &payload.items).await;

    match result {
        Ok(order) => {
            tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::CREATED, Json(order)))
        }
        Err(e) => {
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
        Err(e) => {
            eprintln!("Error getting order by ID: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_orders_by_user_id_handler(
    State(pool): State<Db>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    match OrderManager::get_orders_by_user_id(&pool, user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => {
            eprintln!("Error getting orders by user ID: {:?}", e);
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
    // ✅ Usa el tipo `CreateStock` importado de `models`
    match StockManager::add_stock(&pool, create_stock).await {
        Ok(stock) => Ok(Json(stock)),
        Err(e) => {
            eprintln!("Error adding stock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_stock_handler(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
    Json(update_stock): Json<UpdateStock>,
) -> Result<Json<Stock>, StatusCode> {
    // ✅ Usa el tipo `UpdateStock` importado de `models`
    match StockManager::update_stock(&pool, product_id, update_stock).await {
        Ok(Some(stock)) => Ok(Json(stock)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error updating stock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_stock_handler(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match StockManager::delete_stock(&pool, product_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error deleting stock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_stock_handler(
    State(pool): State<PgPool>,
    product_id: Option<Path<i32>>,
) -> Result<Json<Vec<Stock>>, StatusCode> {
    let id = product_id.map(|Path(id)| id);
    match StockManager::get_stock(&pool, id).await {
        Ok(stocks) => Ok(Json(stocks)),
        Err(e) => {
            eprintln!("Error getting stock: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}