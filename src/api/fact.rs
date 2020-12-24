use actix_web::web;
use crate::{api::Router, db::Db, gql, models::Fact};

impl Router for Fact {

    fn scope() -> &'static str { "/user" }

    fn routes() -> actix_web::Scope {
        use web::scope;
        scope(Self::scope())
    }

}
