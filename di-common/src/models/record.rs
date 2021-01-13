use sqlx::{Postgres, FromRow, postgres::*};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::{
    Status, Permission, Model,
    user::User, item::Item,
    //link::{UserRecordLink, RecordItemLink},
};

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
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

    pub fn new(uid: i32, name: String) -> Record {
        Self { 
            id: None, uid, name, 
            description: None,
            status: "active".to_string(),
            permission: "private".to_string(),
            created_at: Utc::now(),
        }
    }

    pub async fn from_id(pool: PgPool, id: i32) -> sqlx::Result<Self> {
        let record = sqlx::query_as::<_, Self>("SELECT * FROM Records WHERE id=?;")  
            .bind(id)
            .fetch_one(&pool).await?;
        Ok(record)
    }

    pub async fn from_uid(pool: PgPool, uid: i32) -> sqlx::Result<Vec<Self>> {
        let records = sqlx::query_as::<_, Record>( 
            "SELECT * FROM Records WHERE uid= ?;")
            .bind(uid)
            .fetch_all(&pool).await?;
        for record in &records {
            println!("record: {}", &record.id.unwrap());
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

pub struct RecordBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub items: Option<Vec<std::rc::Rc<Item>>>,
}

impl RecordBuilder {
    pub fn with_name(mut self, name: String) -> RecordBuilder {
        self.name = Some(name); self
    }
}

#[derive(Serialize, Deserialize)]
pub enum Relation {
    Parent(i32, i32), //not sure if i want "following?"
    Child(i32, i32),
    Flat(i32, i32),
    NoRelation,
}

impl Relation {
    fn none() -> Relation { Relation::NoRelation }
}

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RecordRelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub rid1: i32,
    pub rid2: i32,
    #[serde(default = "Relation::none")]
    pub rel: Relation,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl RecordRelation {

}
