use sqlx::PgPool;
use crate::db;
use crate::models::TemporalOrder;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub temporal_orders: Arc<DashMap<String, TemporalOrder>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = db::create_pool().await?;
        Ok(Self { 
            db_pool: pool,
            temporal_orders: Arc::new(DashMap::new()),
        })
    }
}
