use crate::models::{CreateOrderItem, Order, OrderItem, OrderStatus};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::{Transaction, Postgres};
use sqlx::PgPool;
use uuid::Uuid;
// Importar async_trait si OrderManager implementara un trait, 
// pero en este caso, se usa como estructura estática, no es necesario.
// use async_trait::async_trait; 

// 🆕 Estructura OrderManager (Vacía, solo para agrupar métodos)
pub struct OrderManager;

// 🆕 Implementación de los métodos del OrderManager
impl OrderManager {
    // La función original ahora es un método asociado público
    pub async fn create_order_in_db(
        // Usamos &mut Transaction<'_, Postgres> para ser más explícitos
        tx: &mut Transaction<'_, Postgres>, 
        user_id: Uuid,
        items: &[CreateOrderItem],
    ) -> Result<Order, sqlx::Error> {
        // Cálculo del precio total
        let total_price = items.iter().fold(dec!(0.0), |acc, item| {
            // Asegúrate de que las operaciones de Decimal y i32/i16/etc. sean correctas
            acc + item.price * Decimal::from(item.quantity) 
        });

        // 1. Insertar la orden principal
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (user_id, total_price, status) 
            VALUES ($1, $2, $3::order_status) 
            RETURNING id as "id!", user_id as "user_id!", total_price as "total_price!", status as "status!: _", created_at as "created_at!"
            "#,
            user_id,
            total_price,
            // Asegúrate de castear correctamente el enum en PostgreSQL
            OrderStatus::Pending as OrderStatus 
        )
        .fetch_one(&mut **tx)
        .await?;

        // 2. Insertar los ítems de la orden y actualizar el stock
        for item in items {
            // Insertar OrderItem
            sqlx::query_as!(
                OrderItem,
                r#"
                INSERT INTO order_items (order_id, product_id, quantity, price_at_time_of_purchase)
                VALUES ($1, $2, $3, $4)
                RETURNING id as "id!", order_id as "order_id!", product_id as "product_id!", quantity as "quantity!", price_at_time_of_purchase as "price_at_time_of_purchase!"
                "#,
                order.id,
                item.product_id,
                item.quantity,
                item.price
            )
            .fetch_one(&mut **tx)
            .await?;

            // Actualizar Stock (Restar cantidad)
            sqlx::query!(
                "UPDATE stock SET quantity = quantity - $1 WHERE product_id = $2",
                item.quantity,
                item.product_id
            )
            .execute(&mut **tx)
            .await?;
        }

        Ok(order)
    }

    pub async fn get_order_by_id(pool: &PgPool, order_id: i32) -> Result<Option<Order>, sqlx::Error> {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id as "id!",
                   user_id as "user_id!",
                   total_price as "total_price!",
                   status as "status!: _",
                   created_at as "created_at!"
            FROM orders
            WHERE id = $1
            "#,
            order_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(order)
    }

    pub async fn get_orders_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Order>, sqlx::Error> {
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT id as "id!",
                   user_id as "user_id!",
                   total_price as "total_price!",
                   status as "status!: _",
                   created_at as "created_at!"
            FROM orders
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }
}