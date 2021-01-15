use super::Model;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use super::Time;
use uuid::Uuid;
use sqlx::{
    prelude::*, Postgres,
    postgres::{PgPool, PgConnection, PgDone}
};
use crate::auth::{hash_pwd, get_secret_key};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Action {
    pub id: Uuid,
}
