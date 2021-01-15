use divc::models::Model;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use sqlx::postgres::*;
use sqlx::FromRow;
use sqlx::prelude::*;
pub use sqlx::postgres::{PgPool, PgConnection};
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {

    pub fn url() -> dotenv::Result<String> {
        dotenv::var("DATABASE_URL")
    }

    pub fn pool_options() -> sqlx::pool::PoolOptions<Postgres> {
        PgPoolOptions::new()
            .max_connections(5)
    }

    pub async fn new() -> divc::DResult<Self> {
        let pool = Self::pool_options()
            .connect(&Self::url()?)
            .await?;
        Ok ( Db { pool } )
    }

    pub fn new_blocking() -> divc::DResult<Self> {
        let url = Self::url()?;
        let pool = Self::pool_options()
            .connect(&url);
        let pool = async_std::task::block_on(pool)?;
        Ok( Self { pool  } )

    }

    pub async fn listen(self, channel: &str) -> divc::DResult<()> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(channel).await?;
        loop  {
            let notif = listener.recv().await?;
            println!("Listener received: {:?}", notif.payload());
            if notif.payload() == "break" { break }
        }
        Ok(())
    }

    pub async fn execute(&self, query: &str) -> sqlx::Result<()> {
        (self.pool).execute(query).await?;
        Ok(())
    }

    pub async fn up(&self) -> sqlx::Result<()> {
        (self.pool).execute(include_str!("../sql/up.sql")).await?;
        Ok(())
    }

    pub async fn down(&self) -> sqlx::Result<()> {
        (self.pool).execute(include_str!("../sql/down.sql")).await?;
        Ok(())
    }

    pub async fn clear(&self) -> sqlx::Result<()> {
        (self.pool).execute(include_str!("../sql/clear.sql")).await?;
        Ok(())
    }

    pub async fn clear_table(&self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM ?")
            .bind(table)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn drop_table(&self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DROP TABLE IF EXISTS ?")
            .bind(table)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn conn(&self) -> sqlx::Result<sqlx::pool::PoolConnection<Postgres>> {
        Ok(self.pool.acquire().await?)

    }

    pub async fn from_id(&self, id: uuid::Uuid) -> sqlx::Result<()> {
        Ok(())
    }
}

pub struct TableQuery {
    table: String,
    id: Option<uuid::Uuid>,
}


impl Drop for Db {
    fn drop(&mut self) {
        println!("Closing db...");
    }
}

