pub mod user;
pub mod record;
pub mod item;
pub mod fact;

use std::sync::{Arc, Mutex};
use crate::{db::Db, gql};
use actix_web::{web, App, middleware, http};

pub struct Api {}


impl Api {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> tokio::io::Result<()> {
        std::env::set_var("RUST_LOG", "info");
        let db = Db::new().await.expect("Could not init DB");
        let app = App::new()
            .data(Arc::new(Mutex::new(db)))
            .wrap(middleware::Logger::default());
        actix_web::HttpServer::new(app)
            .bind("0.0.0.0:8080")?
            .run().await?;
        Ok(())
    }

}

pub trait Router {

    fn scope() -> &'static str;

    fn routes() -> actix_web::Scope;

    fn register(c: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
        c.service(Self::routes())
    }

}

pub struct Root;

impl Router for Root {

    fn scope() -> &'static str { "/" }

    fn routes() -> actix_web::Scope {
        use web::{scope, resource, get, post};
        scope("/graphql")
            .service(resource("")
                .route(get().to(gql::gql_get))
                .route(post().to(gql::gql_post))
            )
            .service(resource("/sub")
                .route(post().to(gql::get_sub))
                .route(get().to(gql::post_sub))
            )
            .service(resource("/playground").route(get().to(gql::playground))
            )
    }
}
