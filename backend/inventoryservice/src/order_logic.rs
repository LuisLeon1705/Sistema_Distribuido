use crate::models::{CreateOrderItem, Order, OrderItem, OrderStatus};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::{Transaction, Postgres};
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::{Result, anyhow}; // For easier error propagation
use crate::temp_order_logic; // Import temp_order_logic


// 🆕 Estructura OrderManager (Vacía, solo para agrupar métodos)
pub struct OrderManager;

// 🆕 Implementación de los métodos del OrderManager
impl OrderManager {
    // La función original ahora es un método asociado público
    pub async fn create_order_in_db(
        tx: &mut Transaction<'_, Postgres>, 
        user_id: Uuid,
    ) -> Result<Order, anyhow::Error> { // Change error type to anyhow::Error
        // Fetch temporary orders for the given user_id
        let temp_orders = temp_order_logic::get_temp_orders_by_user_id(user_id).await
            .map_err(|e| anyhow!("Failed to get temporary orders: {}", e))?;

        let temp_order = temp_orders.into_iter().next()
            .ok_or_else(|| anyhow!("No temporary order found for user_id: {}", user_id))?;

        let items: Vec<CreateOrderItem> = temp_order.items.into_iter().map(|temp_item| {
            CreateOrderItem {
                product_id: temp_item.product_id,
                quantity: temp_item.quantity,
                price: temp_item.price,
            }
        }).collect();

        if items.is_empty() {
            return Err(anyhow!("Temporary order for user_id {} has no items.", user_id));
        }

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
        for item in &items { // Iterate over the `items` vector
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

        // Delete the temporary order after successful creation
        temp_order_logic::delete_temp_order_by_user_id(user_id)
            .map_err(|e| anyhow!("Failed to delete temporary order: {}", e))?;

        Ok(order)
    }

    pub async fn update_order_status(
        pool: &PgPool,
        order_id: i32,
        new_status: OrderStatus,
    ) -> Result<Order, anyhow::Error> {
        let mut tx = pool.begin().await?;

        // 1. Get the current order status
        let current_order = sqlx::query_as!(
            Order,
            r#"
            SELECT id as "id!",
                   user_id as "user_id!",
                   total_price as "total_price!",
                   status as "status!: _",
                   created_at as "created_at!"
            FROM orders
            WHERE id = $1
            FOR UPDATE -- Lock the row to prevent race conditions
            "#,
            order_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow!("Order with ID {} not found", order_id))?;

        // 2. Check if the status transition is valid
        if current_order.status != OrderStatus::Pending {
            return Err(anyhow!(
                "Cannot change status from {:?} to {:?}. Only 'pending' orders can be updated.",
                current_order.status,
                new_status
            ));
        }

        // 3. If new status is "cancelled", return items to stock
        if new_status == OrderStatus::Cancelled {
            let order_items = sqlx::query_as!(
                OrderItem,
                r#"
                SELECT id as "id!", order_id as "order_id!", product_id as "product_id!", quantity as "quantity!", price_at_time_of_purchase as "price_at_time_of_purchase!"
                FROM order_items
                WHERE order_id = $1
                "#,
                order_id
            )
            .fetch_all(&mut *tx)
            .await?;

            for item in order_items {
                sqlx::query!(
                    "UPDATE stock SET quantity = quantity + $1 WHERE product_id = $2",
                    item.quantity,
                    item.product_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        // 4. Update the order status
        let updated_order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET status = $1::order_status
            WHERE id = $2
            RETURNING id as "id!", user_id as "user_id!", total_price as "total_price!", status as "status!: _", created_at as "created_at!"
            "#,
            new_status as OrderStatus,
            order_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(updated_order)
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

    pub async fn get_order_items_by_order_id(pool: &PgPool, order_id: i32) -> Result<Vec<OrderItem>, sqlx::Error> {
        let order_items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT id as "id!", order_id as "order_id!", product_id as "product_id!", quantity as "quantity!", price_at_time_of_purchase as "price_at_time_of_purchase!"
            FROM order_items
            WHERE order_id = $1
            "#,
            order_id
        )
        .fetch_all(pool)
        .await?;

        Ok(order_items)
    }
}