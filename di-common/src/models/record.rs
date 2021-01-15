use sqlx::{Postgres, FromRow, postgres::*};
use chrono::{DateTime, Utc};
use crate::{Visibility, Status};
use serde::{Serialize, Deserialize};
use super::{
    user::User, item::Item,
};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Record {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    pub uid: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default = "Visibility::private")]
    pub visibility: crate::types::Visibility,
    #[serde(default = "Status::active")]
    pub status: crate::types::Status,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl super::Model for Record {
    fn table() -> String { String::from("Records") }
}

impl Record {

    pub fn new(uid: uuid::Uuid, name: String) -> Record {
        Self {
            id: uuid::Uuid::new_v4(), uid, name,
            description: None,
            created_at: Utc::now(),
            visibility: Visibility::private(),
            status: Status::active(),
            attributes: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub async fn insert(&self, pool: &PgPool)
    -> sqlx::Result<()> {
        sqlx::query("INSERT INTO Record
        (uid, name, description, status, visibility, notes, attributes, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);")
            .bind(self.uid)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.notes)
            .bind(&self.attributes)
            .bind(self.created_at)
            .execute(pool).await?;
        Ok(())
    }

    pub async fn get_all(&self, pool: &PgPool) -> sqlx::Result<Vec<Record>> {
        let records: Vec<Record> = sqlx::query_as::<Postgres, Record>("
            SELECT * FROM Records")
            .fetch_all(pool).await?;
        Ok(records)
    }

    pub async fn from_id(&self, pool: &PgPool, id: uuid::Uuid) -> sqlx::Result<Record> {
        let record: Record = sqlx::query_as::<Postgres, Record>("
            SELECT * FROM Records WHERE id = ?")
            .bind(id)
            .fetch_one(pool).await?;
        Ok(record)
    }

    pub async fn from_user(&self, pool: &PgPool, uid: uuid::Uuid) -> sqlx::Result<Vec<Self>> {
        let records: Vec<Self> = sqlx::query_as::<Postgres, Record>("
            SELECT * FROM Records where uid = ?")
            .bind(&uid)
            .fetch_all(pool).await?;
        Ok(records)
    }
}
