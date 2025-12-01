use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Order {
    pub id: i32,
    pub user_id: Uuid,
    pub total_price: Decimal,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price_at_time_of_purchase: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Stock {
    pub id: i32,
    pub product_id: Uuid,
    pub quantity: i32,
    pub last_updated: Option<DateTime<Utc>>,
    pub warehouse_location: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrder {
    pub user_id: Uuid,
    pub items: Vec<CreateOrderItem>,
}

#[derive(Deserialize, Debug)]
pub struct CreateStock {
    pub product_id: Uuid,
    pub quantity: i32,
    pub warehouse_location: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateStock {
    pub quantity: Option<i32>,
    pub warehouse_location: Option<String>,
}

