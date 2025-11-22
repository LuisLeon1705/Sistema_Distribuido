use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

//========= Enums ===========
#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[sqlx(type_name = "order_status")]
pub enum OrderStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

//========= Models from Database ===========
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Stock {
    pub id: i32,
    pub product_id: i32,
    pub quantity: i32,
    #[serde(skip_deserializing)]
    pub last_updated: DateTime<Utc>,
    pub warehouse_location: Option<String>,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price_at_time_of_purchase: f64,
}


//========= Models for API requests/responses ===========

#[derive(Debug, Deserialize)]
pub struct UpdateStockRequest {
    pub product_id: i32,
    pub amount: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrderItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrderRequest {
    pub items: Vec<CreateOrderItemRequest>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemporalOrder {
    pub id: String,
    pub user_id: i32,
    pub order_details: CreateOrderRequest,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl TemporalOrder {
    pub fn new(user_id: i32, order_details: CreateOrderRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            order_details,
            created_at: now,
            last_updated: now,
        }
    }

    pub fn touch(&mut self) {
        self.last_updated = Utc::now();
    }
}
