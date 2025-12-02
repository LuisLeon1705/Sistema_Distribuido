use sqlx::{PgPool, Result};
use uuid::Uuid;

pub async fn seed_stock(pool: &PgPool) -> Result<()> {
    let product_ids = vec![
        "f47ac10b-58cc-4372-a567-0e02b2c3d479",
        "a47ac10b-58cc-4372-a567-0e02b2c3d479",
    ];

    for id_str in product_ids {
        let product_id = Uuid::parse_str(id_str).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO stock (product_id, quantity, warehouse_location)
            VALUES ($1, 100, 'default')
            ON CONFLICT (product_id) DO NOTHING
            "#,
            product_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
