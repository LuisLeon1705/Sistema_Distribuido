use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    auth::{is_admin, AuthenticatedUser},
    errors::ApiError,
    models::{InventoryResponse, Product, UpdateStockRequest},
    state::AppState,
};

/// Lists all products. Accessible by any authenticated user.
pub async fn list_products(
    State(state): State<AppState>,
    _user: AuthenticatedUser, // This ensures the user is authenticated
) -> Result<Json<Vec<Product>>, ApiError> {
    let products_map = state.products.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock products mutex"))?;
    let products: Vec<Product> = products_map.values().cloned().collect();
    Ok(Json(products))
}

/// Gets a single product by its ID. Accessible by any authenticated user.
pub async fn get_product(
    State(state): State<AppState>,
    Path(product_id): Path<i32>,
    _user: AuthenticatedUser,
) -> Result<Json<Product>, ApiError> {
    let products_map = state.products.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock products mutex"))?;
    match products_map.get(&product_id) {
        Some(product) => Ok(Json(product.clone())),
        None => Err(ApiError::new(StatusCode::NOT_FOUND, "Product not found")),
    }
}

/// Lists the full inventory status. Accessible only by admin users.
pub async fn list_inventory(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<InventoryResponse>>, ApiError> {
    is_admin(&user)?;

    let products_map = state.products.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock products mutex"))?;
    let inventory_map = state.inventory.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock inventory mutex"))?;

    let response: Vec<InventoryResponse> = inventory_map
        .values()
        .map(|stock| {
            let product_name = products_map
                .get(&stock.product_id)
                .map_or_else(|| "Unknown Product".to_string(), |p| p.name.clone());

            InventoryResponse {
                product_id: stock.product_id,
                product_name,
                quantity: stock.quantity,
                last_updated: stock.last_updated,
                warehouse_location: stock.warehouse_location.clone(),
            }
        })
        .collect();

    Ok(Json(response))
}


/// Increases the stock of a product. Accessible only by admin users.
pub async fn increase_stock(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<UpdateStockRequest>,
) -> Result<StatusCode, ApiError> {
    is_admin(&user)?;

    let mut inventory_map = state.inventory.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock inventory mutex"))?;
    
    if let Some(stock) = inventory_map.get_mut(&payload.product_id) {
        stock.quantity += payload.amount;
        stock.last_updated = chrono::Utc::now();
        Ok(StatusCode::OK)
    } else {
        Err(ApiError::new(StatusCode::NOT_FOUND, "Product not found in inventory"))
    }
}

/// Decreases the stock of a product. Accessible only by admin users.
pub async fn decrease_stock(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<UpdateStockRequest>,
) -> Result<StatusCode, ApiError> {
    is_admin(&user)?;

    let mut inventory_map = state.inventory.lock().map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to lock inventory mutex"))?;

    if let Some(stock) = inventory_map.get_mut(&payload.product_id) {
        if stock.quantity < payload.amount {
            return Err(ApiError::new(StatusCode::BAD_REQUEST, "Not enough stock to decrease"));
        }
        stock.quantity -= payload.amount;
        stock.last_updated = chrono::Utc::now();
        Ok(StatusCode::OK)
    } else {
        Err(ApiError::new(StatusCode::NOT_FOUND, "Product not found in inventory"))
    }
}
