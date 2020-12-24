use super::Model;
use sqlx::{
    Type, FromRow, prelude::*,
    types::{ Uuid, chrono::{Utc, DateTime} }
};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            created_at: Utc::now(),
            ..Default::default()
        }
    }
}

impl Model for User {

}
