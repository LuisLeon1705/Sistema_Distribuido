use crate::models::{Stock, CreateStock, UpdateStock};
use sqlx::{PgPool, Error, QueryBuilder, Postgres};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StockManagement {
    async fn add_stock(pool: &PgPool, new_stock: CreateStock) -> Result<Stock, Error>;
    async fn update_stock(pool: &PgPool, product_id: Uuid, updated_stock: UpdateStock) -> Result<Option<Stock>, Error>;
    async fn delete_stock(pool: &PgPool, product_id: Uuid) -> Result<bool, Error>;
    async fn get_stock(pool: &PgPool, product_id: Option<Uuid>) -> Result<Vec<Stock>, Error>;
}

pub struct StockManager;

#[async_trait]
impl StockManagement for StockManager {
    async fn add_stock(pool: &PgPool, new_stock: CreateStock) -> Result<Stock, Error> {
        let stock = sqlx::query_as!(
            Stock,
            r#"
            INSERT INTO stock (product_id, quantity, warehouse_location)
            VALUES ($1, $2, $3)
            RETURNING id, product_id, quantity, last_updated, warehouse_location
            "#,
            new_stock.product_id,
            new_stock.quantity,
            new_stock.warehouse_location
        )
        .fetch_one(pool)
        .await?;

        Ok(stock)
    }

    async fn update_stock(pool: &PgPool, product_id: Uuid, updated_stock: UpdateStock) -> Result<Option<Stock>, Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE stock SET ");
        let mut updates_made = false;

        if let Some(quantity) = updated_stock.quantity {
            query_builder.push("quantity = ").push_bind(quantity);
            updates_made = true;
        }

        if let Some(warehouse_location) = updated_stock.warehouse_location {
            if updates_made {
                query_builder.push(", ");
            }
            query_builder.push("warehouse_location = ").push_bind(warehouse_location);
            updates_made = true;
        }
        
        if !updates_made {
            return Ok(None); // No fields to update
        }

        query_builder.push(" WHERE product_id = ").push_bind(product_id);
        query_builder.push(" RETURNING id, product_id, quantity, last_updated, warehouse_location");

        let stock = query_builder
            .build_query_as::<Stock>()
            .fetch_optional(pool)
            .await?;

        Ok(stock)
    }

    async fn delete_stock(pool: &PgPool, product_id: Uuid) -> Result<bool, Error> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM stock
            WHERE product_id = $1
            "#,
            product_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn get_stock(pool: &PgPool, product_id: Option<Uuid>) -> Result<Vec<Stock>, Error> {
        let stocks = match product_id {
            Some(id) => {
                sqlx::query_as!(
                    Stock,
                    r#"
                    SELECT id, product_id, quantity, last_updated, warehouse_location
                    FROM stock
                    WHERE product_id = $1
                    "#,
                    id
                )
                .fetch_all(pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    Stock,
                    r#"
                    SELECT id, product_id, quantity, last_updated, warehouse_location
                    FROM stock
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
        };
        Ok(stocks)
    }
}