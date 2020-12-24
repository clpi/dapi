use actix_web::web;
use crate::{api::Router, db::Db, gql, models::Record};

impl Router for Record {

    fn scope() -> &'static str { "/record" }

    fn routes() -> actix_web::Scope {
        use web::scope;
        scope(Self::scope())
    }

}
