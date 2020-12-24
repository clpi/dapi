pub mod user;
pub mod record;
pub mod item;
pub mod fact;

use std::sync::{Arc, Mutex};
use crate::{db::Db, gql, middleware};
use actix_web::{web, App, HttpServer, HttpResponse};

pub struct Api {}


impl Api {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> tokio::io::Result<()> {
        std::env::set_var("RUST_LOG", "info");
        let db = Db::new().await.expect("Could not init DB");
        HttpServer::new(move ||
            App::new()
                .data(Arc::new(Mutex::new(db.clone())))
                .wrap(middleware::cors())
                .wrap(middleware::compress())
                .wrap(middleware::logger())
                .configure(Root::register))
            .bind(format!("{}:{}", "127.0.0.1", 8010))?
            .run().await?;
        Ok(())
    }

}

pub trait Router {

    fn scope() -> &'static str;

    fn routes() -> actix_web::Scope;

    fn default_route() -> actix_web::Route {
        web::route().to(|| HttpResponse::Found()
            .body(Self::scope())
        )
    }

    fn register(c: &mut web::ServiceConfig) {
        c.service(Self::routes());
    }

}

pub struct Root;

impl Router for Root {

    fn scope() -> &'static str { "/" }

    fn routes() -> actix_web::Scope {
        use web::{scope, resource, get, post};
        scope("")
            .service(resource("/graphql")
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
