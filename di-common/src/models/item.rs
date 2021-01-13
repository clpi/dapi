use sqlx::{Postgres, FromRow, postgres::*};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{
    Status, Permission, Priority, Model,
    record::Record,
    //link::{ItemFieldLink, RecordItemLink},
};

#[derive(FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct Item {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    pub uid: uuid::Uuid,
    pub name: String,
    //#[serde(default = "Status::active")]
    pub status: String,
    //#[serde(default = "Permission::private")]
    pub permission: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Model for Item {
    fn table() -> String { String::from("Items") }
}
