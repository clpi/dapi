use super::Model;
use juniper::graphql_object;
use sqlx::{
    Type, FromRow, prelude::*,
    types::{ Uuid, chrono::{Utc, DateTime} }
};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<Uuid>,
    pub name: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ..Default::default()
        }
    }
}

impl Model for Item {

}

#[graphql_object]
impl Item {
    pub fn name(&self) -> String { String::from(&self.name) }

}
