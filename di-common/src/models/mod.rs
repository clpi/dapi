pub mod user;
pub mod record;
pub mod item;
pub mod group;
pub mod action;
pub mod fact;

pub use self::{
    user::User,
    record::Record,
    item::Item,
};

pub use sqlx::{FromRow, prelude::*,};
use chrono::Utc;
pub use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Model {
    fn table() -> String;

    async fn insert(&self, pool: sqlx::PgPool) -> sqlx::Result<()> {
        // pool.execute("INSERT INTO ? ")
        Ok(())
    }

}


pub use user::*;

pub enum Time {
    Now,
    Tomorrow,
    Yesterday,
}

#[derive(sqlx::Type)]
#[sqlx(rename = "permission", rename_all = "lowercase")]
pub enum Permission {
    Private,
    Public,
    MutualOnly,
    InviteOnly,
}

#[derive(sqlx::Type)]
#[sqlx(rename = "priority", rename_all = "lowercase")]
pub enum Priority {
    Lowest,
    Low,
    High,
    Highest,
}

#[derive(sqlx::Type)]
#[sqlx(rename = "priority", rename_all = "lowercase")]
pub enum Status {
    Deleted,
    Archived,
    Active
}

impl Time {
    pub fn now() -> i32 { Utc::now().timestamp() as i32 }
}
