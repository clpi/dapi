use actix_web::web;
use crate::{api::Router, db::Db, gql, models::User};

impl Router for User {

    fn scope() -> &'static str { "/user" }

    fn routes() -> actix_web::Scope {
        use web::scope;
        scope(Self::scope())
    }

}
