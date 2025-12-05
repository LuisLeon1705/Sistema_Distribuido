use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::str::FromStr;

pub async fn seed_stock(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Definimos los mismos UUIDs que usamos en ProductService
    // (Tupla: ID del Producto, Cantidad Inicial)
    let items = vec![
        // Bebidas
        ("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", 50), // Coca Cola
        ("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12", 50), // Jugo Naranja
        ("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13", 50), // Te Manzana
        ("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14", 50), // Arizona
        // Víveres
        ("b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b21", 100), // Harina
        ("b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b22", 100), // Pasta
        ("b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b23", 100), // Aceite
        ("b0eebc99-9c0b-4ef8-bb6d-6bb9bd380b24", 100), // Arroz
        // Dulces
        ("c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c31", 200), // Oreo
        ("c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c32", 200), // Savoy
        ("c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c33", 200), // Pringles
        ("c0eebc99-9c0b-4ef8-bb6d-6bb9bd380c34", 200), // Flips
    ];

    for (uuid_str, quantity) in items {
        let product_id = Uuid::from_str(uuid_str).unwrap();

        let exists = sqlx::query("SELECT id FROM stock WHERE product_id = $1")
            .bind(product_id) // Usamos .bind() para pasar variables
            .fetch_optional(pool)
            .await?;

        if exists.is_none() {
            // 2. CAMBIO: Igual aquí, sqlx::query(...) sin exclamación
            sqlx::query(
                r#"
                INSERT INTO stock (product_id, quantity, last_updated, warehouse_location)
                VALUES ($1, $2, NOW(), 'Principal')
                "#
            )
            .bind(product_id) // $1
            .bind(quantity)   // $2
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}