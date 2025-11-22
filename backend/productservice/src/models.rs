use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//========= Models from JSON ===========
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub image_url: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub id: i32,
    pub product_id: i32,
    pub quantity: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_updated: DateTime<Utc>,
    pub warehouse_location: String,
}

//========= Models for API requests/responses ===========

#[derive(Debug, Deserialize)]
pub struct UpdateStockRequest {
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub product_id: i32,
    pub product_name: String,
    pub quantity: i32,
    pub last_updated: DateTime<Utc>,
    pub warehouse_location: String,
}
