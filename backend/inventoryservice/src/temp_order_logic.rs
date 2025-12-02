use crate::models::{CreateOrder, TempOrder, TempOrderItem};
use crate::stock_logic::{StockManager, StockManagement};
use sqlx::PgPool;
use std::fs;
use std::io;
use std::path::Path;
use chrono::Utc;
use uuid::Uuid;

const TEMP_ORDER_FILE: &str = "data/order_list.json";

pub async fn add_temp_order(pool: &PgPool, order: CreateOrder) -> Result<TempOrder, io::Error> {
    let mut temp_items = Vec::new();
    for item in order.items {
        let stock_vec = StockManager::get_stock(pool, Some(item.product_id)).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let current_stock = stock_vec.first().map_or(0, |s| s.quantity);

        let mut temp_item = TempOrderItem {
            product_id: item.product_id,
            quantity: item.quantity,
            price: item.price,
        };

        if item.quantity > current_stock {
            temp_item.quantity = current_stock;
        }

        if temp_item.quantity > 0 {
            temp_items.push(temp_item);
        }
    }

    let temp_order = TempOrder {
        user_id: order.user_id,
        items: temp_items,
        created_at: Utc::now(),
    };

    let path = Path::new(TEMP_ORDER_FILE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut orders: Vec<TempOrder> = if path.exists() {
        let file_content = fs::read_to_string(path)?;
        if file_content.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&file_content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        }
    } else {
        Vec::new()
    };

    // Remove any existing order for the same user_id
    orders.retain(|o| o.user_id != temp_order.user_id);

    orders.push(temp_order.clone());

    let new_content = serde_json::to_string_pretty(&orders).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    fs::write(path, new_content)?;

    Ok(temp_order)
}

pub async fn get_temp_orders_by_user_id(user_id: Uuid) -> Result<Vec<TempOrder>, io::Error> {
    let path = Path::new(TEMP_ORDER_FILE);
    if !path.exists() || fs::read_to_string(path)?.trim().is_empty() {
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(path)?;
    let orders: Vec<TempOrder> = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let filtered_orders = orders.into_iter()
        .filter(|order| order.user_id == user_id)
        .collect();

    Ok(filtered_orders)
}

pub fn delete_temp_order_by_user_id(user_id: Uuid) -> io::Result<()> {
    let path = Path::new(TEMP_ORDER_FILE);
    if !path.exists() {
        return Ok(());
    }

    let file_content = fs::read_to_string(path)?;
    if file_content.trim().is_empty() {
        return Ok(());
    }

    let orders: Vec<TempOrder> = serde_json::from_str(&file_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let filtered_orders: Vec<TempOrder> = orders
        .into_iter()
        .filter(|order| order.user_id != user_id)
        .collect();

    let new_content = serde_json::to_string_pretty(&filtered_orders)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    fs::write(path, new_content)?;

    Ok(())
}

pub fn cleanup_expired_orders() -> io::Result<()> {
    let path = Path::new(TEMP_ORDER_FILE);
    if !path.exists() {
        return Ok(());
    }

    let file_content = fs::read_to_string(path)?;
    if file_content.trim().is_empty() {
        return Ok(());
    }
    
    let orders: Vec<TempOrder> = serde_json::from_str(&file_content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    
    let now = Utc::now();
    let thirty_minutes_ago = now - chrono::Duration::minutes(30);

    let valid_orders: Vec<TempOrder> = orders
        .into_iter()
        .filter(|order| order.created_at > thirty_minutes_ago)
        .collect();

    let new_content = serde_json::to_string_pretty(&valid_orders).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    fs::write(path, new_content)?;

    Ok(())
}
