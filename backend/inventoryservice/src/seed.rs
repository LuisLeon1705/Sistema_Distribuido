use serde::Deserialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Product {
    id: Uuid,
    codigo: String,
}

// Map product codes to their desired stock levels
fn get_stock_map() -> HashMap<&'static str, i32> {
    let mut stock_map = HashMap::new();
    stock_map.insert("BEB-000001", 150);
    stock_map.insert("BEB-000002", 120);
    stock_map.insert("BEB-000003", 80);
    stock_map.insert("BEB-000004", 200);
    stock_map.insert("VIB-000001", 180);
    stock_map.insert("VIB-000002", 160);
    stock_map.insert("VIB-000003", 100);
    stock_map.insert("VIB-000004", 250);
    stock_map.insert("DUL-000001", 300);
    stock_map.insert("DUL-000002", 90);
    stock_map.insert("DUL-000003", 110);
    stock_map.insert("DUL-000004", 140);
    stock_map
}

async fn fetch_products_with_retry(
    client: &reqwest::Client,
    url: &str,
    retries: u32,
) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response.json::<Vec<Product>>().await?);
                } else {
                    eprintln!(
                        "Attempt {} to fetch products failed with status: {}",
                        attempts,
                        response.status()
                    );
                }
            }
            Err(e) => {
                eprintln!("Attempt {} to fetch products failed with error: {}", attempts, e);
            }
        }

        if attempts >= retries {
            return Err("Failed to fetch products after multiple retries".into());
        }
        sleep(Duration::from_secs(5)).await; // Wait 5 seconds before retrying
    }
}

pub async fn seed_stock(db: &PgPool) -> Result<(), Box<dyn Error>> {
    println!("Seeding database...");

    let client = reqwest::Client::new();
    let product_service_url = "http://productservice/api/productos";

    let products = fetch_products_with_retry(&client, product_service_url, 5).await?;
    
    if products.is_empty() {
        println!("No products found from productservice, skipping stock seeding.");
        return Ok(());
    }

    println!("Fetched {} products from productservice.", products.len());

    let stock_map = get_stock_map();

    // Clear existing stock to make seeding idempotent
    sqlx::query!("TRUNCATE TABLE stock RESTART IDENTITY")
        .execute(db)
        .await?;
    println!("Stock table cleared.");

    for product in products {
        if let Some(&quantity) = stock_map.get(product.codigo.as_str()) {
            sqlx::query!(
                r#"
                INSERT INTO stock (product_id, quantity, warehouse_location)
                VALUES ($1, $2, $3)
                "#,
                product.id,
                quantity,
                "Default Location",
            )
            .execute(db)
            .await?;
            println!("Seeded stock for product ID: {}", product.id);
        }
    }

    println!("Database seeding completed successfully.");
    Ok(())
}
