use crate::db::{Db, query::Query};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use juniper::{
    graphql_object, graphql_subscription,
    RootNode, EmptyMutation,
};
use juniper_actix::{
    graphql_handler, playground_handler,
};
use juniper_graphql_ws::{ConnectionConfig, DataPayload,};

type Schema = RootNode<'static, Query, EmptyMutation<Db>, Subscription>;

pub struct Subscription {

}

pub async fn schema() -> () {

}

pub async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(""))
}

pub async fn gql_get() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn gql_post() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn get_sub() -> impl Responder {
    HttpResponse::Ok().body("")
}

pub async fn post_sub() -> impl Responder {
    HttpResponse::Ok().body("")
}


pub async fn playground() -> impl Responder {
    HttpResponse::Ok().body("")
}
