use actix_web::web;
use crate::{api::Router, db::Db, gql, models::Item};

impl Router for Item {

    fn scope() -> &'static str { "/user" }

    fn routes() -> actix_web::Scope {
        use web::scope;
        scope(Self::scope())
    }

}
