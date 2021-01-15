use sqlx::{Postgres, FromRow, postgres::*};
use crate::{Visibility, Status};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct Item {
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

impl super::Model for Item {
    fn table() -> String { String::from("Items") }
}

impl Item {

    pub fn new(uid: uuid::Uuid, name: String) -> Item {
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
        sqlx::query("INSERT INTO Items
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

    pub async fn get_all(&self, pool: &PgPool) -> sqlx::Result<Vec<Item>> {
        let items: Vec<Item> = sqlx::query_as::<Postgres, Item>("
            SELECT * FROM Items")
            .fetch_all(pool).await?;
        Ok(items)
    }

    pub async fn from_user(self, pool: &PgPool, uid: uuid::Uuid) -> sqlx::Result<Vec<Item>> {
        let users: Vec<Item> = sqlx::query_as::<Postgres, Item>("
            SELECT * FROM Items where uid = ?")
            .bind(&uid)
            .fetch_all(pool).await?;
        Ok(users)
    }

}
