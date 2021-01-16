use divd::PgPool;
use divc::auth::{get_secret_key, get_jwt_secret};

pub async fn create() -> tide::Result<Context> {
    let db = divd::Db::new().await?;
    db.down().await?;
    db.up().await?;
    let secret_key = get_secret_key().await?;
    let jwt_key = get_jwt_secret().await?;
    let mut tera = tera::Tera::new("assets/static/templates/**/*").expect("Could not load tera");
    tera.autoescape_on(vec!["html"]);
    let state = Context {
        data: "Data".to_string(),
        pool: db.pool.clone(),
        secret_key, jwt_key, tera,
    };
    Ok(state)
}

#[derive(Clone)]
pub struct Context {
    pub data: String,
    pub pool: PgPool,
    pub secret_key: String,
    pub jwt_key: String,
    pub tera: tera::Tera,
}
