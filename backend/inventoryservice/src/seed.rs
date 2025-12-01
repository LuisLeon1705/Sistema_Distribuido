use crate::{
    db::Db,
    models::{CreateOrderItem, OrderStatus},
    order_logic::OrderManager,
};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use rust_decimal_macros::dec;
use rust_decimal::Decimal; // Corrected import
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct SeedOrderItem {
    product_id: i32,
    quantity: i32,
}

#[derive(Deserialize, Debug)]
struct SeedOrder {
    user_id: i32,
    #[serde(skip)] // total_price will be calculated
    total_price: Option<f64>,
    status: OrderStatus,
    items: Vec<SeedOrderItem>,
}

pub async fn seed_db(db: &Db) {
    println!("Checking if database needs seeding...");

    let orders_count_option: Option<i64> = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM orders"#)
        .fetch_one(db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to count orders: {}", e);
            None
        });

    let orders_count = orders_count_option.unwrap_or(0);

    if orders_count > 0 {
        println!("Database already contains orders. Skipping seeding.");
        return;
    }

    println!("Database is empty. Seeding from orders.json...");

    // Read orders.json
    let mut file = match File::open("orders.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open orders.json: {}", e);
            return;
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read orders.json");


    let seed_orders: Vec<SeedOrder> = match serde_json::from_str(&contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse orders.json: {}", e);
            return;
        }
    };
    
    // Namespace for generating UUIDs from integer IDs
    const NAMESPACE: Uuid = Uuid::from_bytes([
        0x6e, 0x62, 0x72, 0x69, 0x73, 0x74, 0x6f, 0x77, // "enma" in hex
        0x2d, 0x73, 0x65, 0x65, 0x64, 0x2d, 0x6e, 0x73, // "-seed-ns"
    ]);


    for seed_order in seed_orders {
        let mut tx = db.begin().await.expect("Failed to begin transaction for seed order");

        let create_order_items: Vec<CreateOrderItem> = seed_order
            .items
            .into_iter()
            .map(|item| CreateOrderItem {
                product_id: Uuid::new_v5(&NAMESPACE, item.product_id.to_string().as_bytes()),
                quantity: item.quantity,
                price: dec!(100.0),
            })
            .collect();
        let user_id_uuid = Uuid::new_v5(&NAMESPACE, seed_order.user_id.to_string().as_bytes());

        match OrderManager::create_order_in_db(&mut tx, user_id_uuid, &create_order_items).await {
            Ok(_) => {
                tx.commit().await.expect("Failed to commit seed order transaction");
                println!("Seeded order for user_id: {}", seed_order.user_id);
            }
            Err(e) => {
                eprintln!("Failed to seed order for user_id {}: {}", seed_order.user_id, e);
                let _ = tx.rollback().await;
            }
        }
    }

    println!("Database seeding complete.");
}