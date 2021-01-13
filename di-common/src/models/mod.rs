pub mod user;
pub mod record;
pub mod item;
pub mod group;
pub mod action;

pub use self::{
    user::User,
    record::Record,
    item::Item,
};

pub use sqlx::FromRow;
use chrono::Utc;
pub use serde::{Deserialize, Serialize};

pub trait Model {
    fn table() -> String;
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
