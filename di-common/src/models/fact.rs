use dynomite::{
    Attribute, DynamoDbExt, FromAttributes, AttributeValue,
    dynamodb::{DynamoDb, DynamoDbClient}
};
use sqlx::{Postgres, FromRow, postgres::*};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{
    Status, Permission, Priority, Model,
    record::Record,
    //link::{ItemFieldLink, RecordItemLink},
};

#[derive(FromRow, dynomite::Item, Serialize, Deserialize, Clone)]
pub struct Fact {
    #[serde(default = "uuid::Uuid::new_v4")]
    #[dynomite(sort_key)]
    pub id: uuid::Uuid,
    #[dynomite(partition_key)]
    pub uid: uuid::Uuid,
    pub name: String,
    //#[serde(default = "Status::active")]
    pub status: String,
    //#[serde(default = "Permission::private")]
    pub permission: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

