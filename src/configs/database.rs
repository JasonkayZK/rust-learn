use crate::CONFIG;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
}

pub struct DbHandle {
    pub pool: Pool<Postgres>,
}

impl DbHandle {
    pub async fn new() -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(CONFIG.database.max_connections)
            .connect(&CONFIG.database.url)
            .await?;

        Ok(Self { pool })
    }
}
