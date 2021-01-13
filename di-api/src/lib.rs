pub mod api;
pub mod db;
pub mod models;
pub mod middleware;
pub mod state;
pub mod routes;
pub mod util;

use tide::Request;
use tide::prelude::*;
use serde::{Serialize, Deserialize};

pub async fn run() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/hello/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

pub async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}
