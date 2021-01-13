use super::Model;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use super::Time;
use sqlx::Postgres;
use sqlx::postgres::{PgPool, PgConnection, PgDone};
use sqlx::prelude::*;
use crate::auth::{hash_pwd, get_secret_key};

#[derive(FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> User {
        User {
            id: None, email, username, password, created_at: Utc::now(),
        }
    }

    //pub async fn query() -> QueryBuilder<User> {}

    //TODO commit transaction
    pub async fn create(pool: PgPool, user: User) -> sqlx::Result<i32> {
        let query = sqlx::query("INSERT INTO Users (email, username, password)
            VALUES ($1, $2, $3)")
            .bind(user.email)
            .bind(user.username)
            .bind(user.password);
        let res = pool.execute(query).await?;
        Ok(res.rows_affected() as i32)
    }

    //TODO commit transaction
    pub async fn insert(self, pool: PgPool) -> sqlx::Result<u32> {
        let mut conn = pool.acquire().await?;
        let query = sqlx::query("INSERT INTO Users (email, username, password)
            VALUES ($1, $2, $3)")
            .bind(self.email)
            .bind(self.username)
            .bind(self.password)
            .execute(&pool).await?;
        //let res = pool.execute(query).await?;
        Ok(query.rows_affected() as u32)
    }

    pub async fn get_all(pool: PgPool) -> sqlx::Result<Vec<User>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>("SELECT * FROM Users;")
            .fetch_all(&pool).await?;
        Ok(res)
    }

    pub async fn from_id(pool: &PgPool, uid: i32) -> sqlx::Result<User> {
        let res: User = sqlx::query_as::<Postgres, User>("SELECT * FROM Users WHERE id=$1;")
            .bind(uid)
            .fetch_one(pool).await?;
        Ok(res)
    }

    pub async fn from_username(pool: PgPool, username: String) -> sqlx::Result<User> {
        let res: User = sqlx::query_as::<Postgres, User>("SELECT * FROM Users WHERE username=$1;")
            .bind(username)
            .fetch_one(&pool).await?;
        Ok(res)
    }

    pub async fn delete_by_id(pool: PgPool, uid: i32) -> sqlx::Result<i32> {
        let res: i32 = sqlx::query_scalar("DELETE FROM Users WHERE id=$1 RETURNING id")
            .bind(uid)
            .fetch_one(&pool).await?;
        Ok(res)
    }

    pub async fn get_count(pool: PgPool) -> sqlx::Result<i32> {
        let (count,): (i32,) = sqlx::query_as("SELECT COUNT(*) FROM Users")
            .fetch_one(&pool).await?;
        Ok(count)
    }

}

#[test]
pub fn create_retrieve_user() -> () {}

impl Model for User {
    fn table() -> String { String::from("Users") }
}
