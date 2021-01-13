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
    pub async fn new() -> sqlx::Result<Self> {
        let options: PgConnectOptions = dotenv::var("DATABASE_URL")
            .expect("DATABASE_URL unset")
            .parse()?;
        let pool = PgPool::connect_with(options).await?;
        Ok ( Db { pool } )
    }

    pub async fn listen(self, channel: &str) -> sqlx::Result<()> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(channel).await?;
        loop  {
            let notif = listener.recv().await?;
            println!("Listener received: {:?}", notif.payload());
            if notif.payload() == "break" { break }
        }
        Ok(())
    }

    pub async fn clear(self) -> sqlx::Result<Self> {
        Ok(self)
    }

    pub async fn up(self) -> sqlx::Result<Self> {
        Ok(self)
    }
}


impl Drop for Db {
    fn drop(&mut self) {
        println!("Closing db...");
    }
}




