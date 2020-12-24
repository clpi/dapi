pub mod query;

use crate::error::DbResult;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgListener},
    prelude::*,
    types::{Uuid, Json,
        chrono::{Utc, DateTime},
    }
};

pub struct Db{
    pool: PgPool,
}

impl Db {
    pub async fn new() -> sqlx::Result<Self> {
        let db_url = dotenv::var("DB_URL")
            .unwrap_or_default();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str()).await?;
        Ok (Self { pool })
    }

    pub fn pool(self) -> PgPool { self.pool }
}

