use sqlx::{Postgres, FromRow, postgres::*};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::{
    Status, Permission, Model,
    user::User, item::Item,
    //link::{UserRecordLink, RecordItemLink},
};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Record {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    pub uid: uuid::Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    //#[serde(default = "Status::active")]
    pub status: String,
    //#[serde(default = "Permission::Private")]
    pub permission: String,
    //#[serde(default = "Permission::private")]
    //pub permission: Permission,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Model for Record {
    fn table() -> String { String::from("Records") }
}

impl Record {

    pub fn new(uid: uuid::Uuid, name: String) -> Record {
        Self {
            id: uuid::Uuid::new_v4(), uid, name,
            description: None,
            status: "active".to_string(),
            permission: "private".to_string(),
            created_at: Utc::now(),
        }
    }

    pub async fn from_id(pool: PgPool, id: uuid::Uuid) -> sqlx::Result<Self> {
        let record = sqlx::query_as::<_, Self>("SELECT * FROM Records WHERE id=?;")
            .bind(id)
            .fetch_one(&pool).await?;
        Ok(record)
    }

    pub async fn from_uid(pool: PgPool, uid: uuid::Uuid) -> sqlx::Result<Vec<Self>> {
        let records = sqlx::query_as::<_, Record>(
            "SELECT * FROM Records WHERE uid= ?;")
            .bind(uid)
            .fetch_all(&pool).await?;
        for record in &records {
            println!("record: {}", &record.id);
            println!("{}", serde_json::to_string(&record).unwrap());
        }
        Ok(records)
    }

    pub async fn insert(self, pool: PgPool)
    -> sqlx::Result<Self> {
        sqlx::query("INSERT INTO Records
        (uid, name, status, private, created_at)
        VALUES ($1, $2, $3, $4, $5);")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.permission)
            .bind(Utc::now())
            .execute(&pool).await?;
        Ok(self)
    }

    pub async fn get_items(self, pool: PgPool) -> sqlx::Result<Vec<Item>> {
        let items: Vec<Item> = sqlx::query_as::<Postgres, Item>("
            SELECT * FROM Items INNER JOIN RecordItemLinks
            ON RecordItemLinks.iid=Items.id WHERE RecordItemLinks.iid=?;")
            .bind(&self.id)
            .fetch_all(&pool).await?;
        Ok(items)
    }

    pub async fn get_users(self, pool: PgPool) -> sqlx::Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as::<Postgres, User>("
            SELECT * FROM Users INNER JOIN UserRecordLinks
            ON UserRecordLinks.uid=Users.id WHERE UserRecordLinks.rid=?;")
            .bind(&self.id)
            .fetch_all(&pool).await?;
        Ok(users)
    }

    pub async fn associated_with_user(pool: PgPool, user: &User) -> sqlx::Result<Vec<Self>> {
        let records: Vec<Self> = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM Records INNER JOIN UserRecordLinks
            ON UserRecordLinks.uid=Users.id
            WHERE UserRecordLinks.uid=?
              AND Records.uid!=?;")
            .bind(user.id)
            .fetch_all(&pool).await?;
        Ok(records)
    }
}
