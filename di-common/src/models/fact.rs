use dynomite::{
    Attribute, DynamoDbExt, FromAttributes, AttributeValue,
    dynamodb::{DynamoDb, DynamoDbClient}
};
use crate::{Visibility, Status};
use sqlx::{Postgres, FromRow, postgres::*};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(FromRow, dynomite::Item, Serialize, Deserialize, Clone)]
pub struct FactEntry {
    #[serde(default = "uuid::Uuid::new_v4")]
    #[dynomite(sort_key)]
    pub id: uuid::Uuid,
    #[dynomite(partition_key)]
    pub uid: uuid::Uuid,
    pub name: String,
    pub units: Option<String>,
    #[serde(default = "Visibility::private")]
    pub visibility: crate::types::Visibility,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct FactType {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    pub uid: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub value_type: String,
    pub units: Vec<String>,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default = "Visibility::private")]
    pub visibility: crate::types::Visibility,
    #[serde(default = "Status::active")]
    pub status: crate::types::Status,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

// impl super::Model for FactType {

// }

// impl super::Model for FactEntry {

// }
