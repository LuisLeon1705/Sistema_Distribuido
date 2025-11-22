use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::{
    auth::{is_admin, AuthenticatedUser},
    errors::ApiError,
    models::{Stock, UpdateStockRequest},
    state::AppState,
};

/// Lists the full inventory status. Accessible only by admin users.
pub async fn list_inventory(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<Stock>>, ApiError> {
    is_admin(&user)?;

    let inventory = sqlx::query_as!(Stock, "SELECT * FROM stock")
        .fetch_all(&state.db_pool)
        .await?;

    Ok(Json(inventory))
}


/// Increases the stock of a product. Accessible only by admin users.
pub async fn increase_stock(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<UpdateStockRequest>,
) -> Result<StatusCode, ApiError> {
    is_admin(&user)?;

    let rows_affected = sqlx::query!(
        "UPDATE stock SET quantity = quantity + $1, last_updated = NOW() WHERE product_id = $2",
        payload.amount,
        payload.product_id
    )
    .execute(&state.db_pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "Product not found in inventory"));
    }

    Ok(StatusCode::OK)
}

/// Decreases the stock of a product. Accessible only by admin users.
pub async fn decrease_stock(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<UpdateStockRequest>,
) -> Result<StatusCode, ApiError> {
    is_admin(&user)?;

    // Check for sufficient stock before decreasing
    let current_stock = sqlx::query!("SELECT quantity FROM stock WHERE product_id = $1", payload.product_id)
        .fetch_optional(&state.db_pool)
        .await?
        .map(|record| record.quantity)
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "Product not found in inventory"))?;

    if current_stock < payload.amount {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Not enough stock to decrease"));
    }

    let rows_affected = sqlx::query!(
        "UPDATE stock SET quantity = quantity - $1, last_updated = NOW() WHERE product_id = $2",
        payload.amount,
        payload.product_id
    )
    .execute(&state.db_pool)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        // This case should theoretically not be reached due to the check above, but is good for safety.
        return Err(ApiError::new(StatusCode::NOT_FOUND, "Product not found in inventory"));
    }

    Ok(StatusCode::OK)
}
