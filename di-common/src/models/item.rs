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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
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
