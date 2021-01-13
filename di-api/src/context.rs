use divd::PgPool;
use divc::auth::{get_secret_key, get_jwt_secret};

pub async fn create() -> tide::Result<Context> {
    let db = divd::Db::new().await.unwrap()
        .clear().await.unwrap()
        .up().await.unwrap();

    let secret_key = get_secret_key().await.unwrap();
    let jwt_key = get_jwt_secret().await.unwrap();

    let state = Context {
        data: "Data".to_string(),
        pool: db.pool.clone(),
        secret_key, jwt_key
    };
    Ok(state)
}

#[derive(Clone)]
pub struct Context {
    pub data: String,
    pub pool: PgPool,
    pub secret_key: String,
    pub jwt_key: String,
}
