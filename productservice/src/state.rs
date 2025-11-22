use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use crate::models::{Product, Stock, User};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuthDb {
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct ProductsDb {
    products: Vec<Product>,
}

#[derive(Debug, Deserialize)]
struct InventoryDb {
    stock: Vec<Stock>,
}

#[derive(Debug, Deserialize)]
struct FullDb {
    auth_db: AuthDb,
    products_db: ProductsDb,
    inventory_db: InventoryDb,
}

#[derive(Clone)]
pub struct AppState {
    pub products: Arc<Mutex<HashMap<i32, Product>>>,
    pub inventory: Arc<Mutex<HashMap<i32, Stock>>>, // Key is product_id
    // Even if we don't use users directly in handlers, 
    // it's good practice to have them in the state.
    pub users: Arc<Mutex<HashMap<i32, User>>>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let json_str = fs::read_to_string("../db/temp.json")?;
        let db: FullDb = serde_json::from_str(&json_str)?;

        let products_map = db.products_db.products.into_iter().map(|p| (p.id, p)).collect();
        
        // The inventory needs to be mapped by product_id for easy lookup
        let inventory_map = db.inventory_db.stock.into_iter().map(|s| (s.product_id, s)).collect();
        
        let users_map = db.auth_db.users.into_iter().map(|u| (u.id, u)).collect();

        Ok(Self {
            products: Arc::new(Mutex::new(products_map)),
            inventory: Arc::new(Mutex::new(inventory_map)),
            users: Arc::new(Mutex::new(users_map)),
        })
    }
}
