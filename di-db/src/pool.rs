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
        sqlx::query!("DROP TABLE IF EXISTS RecordItemLinks CASCADE;")
            .execute(&self.pool).await?;
        sqlx::query!("DROP TABLE IF EXISTS Items CASCADE;")
            .execute(&self.pool).await?;
        sqlx::query!("DROP TABLE IF EXISTS Records CASCADE;")
            .execute(&self.pool).await?;
        sqlx::query!("DROP TABLE IF EXISTS Users CASCADE;")
            .execute(&self.pool).await?;
        Ok(self)
    }

    pub async fn up(self) -> sqlx::Result<Self> {

        sqlx::query_file_unchecked!("sql/tables/users.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/records.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/fields.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/groups.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/items.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/entrytypes.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/userrecordlinks.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/recorditemlinks.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/itemfieldlinks.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/fieldentrylinks.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/usergrouplinks.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/userinfo.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/entryentries.sql")
            .execute(&self.pool).await.unwrap();

        sqlx::query_file_unchecked!("sql/tables/fieldentries.sql")
            .execute(&self.pool).await.unwrap();

        Ok(self)
    }
}


impl Drop for Db {
    fn drop(&mut self) {
        println!("Closing db...");
    }
}




