pub mod user;
pub mod public;
pub mod item;
pub mod record;
pub mod auth;
pub mod admin;

use async_std::prelude::*;
pub use divc::models::{User, UserLogin};
use tide_websockets::{Message, WebSocket};
use crate::context::Context;
use crate::*;
use tera::Tera;
use tide_tera::prelude::*;

pub use tide::{
    http::Cookie,
    Response, StatusCode, Request
};


pub fn set(app: &mut tide::Server<Context>) {
    app.at("/auth/login").post(|req: tide::Request<Context>| async move {
        Ok(self::auth::login(req).await?)
    });

    app.at("/auth/signup").post(|req: Request<Context>| async move {
        Ok(self::auth::signup(req).await?)
    });

    app.at("/user").post(|_: Request<Context>| async move {
        Ok("hello")
    }).get(|req: Request<Context>| async move {
        Ok(self::user::get_all(req).await?)
    });


    app.at("/wsm")
        .with(WebSocket::new(|_request, mut stream| async move {
            while let Some(Ok(Message::Text(input))) = stream.next().await {
                let output: String = input.chars().rev().collect();
                stream.send_string(format!("{} | {}", &input, &output)).await?;
            }
            Ok(())
        }))
        .get(|_| async move { Ok("this was not a websocket request") });

    app.at("/wse")
        .get(WebSocket::new(|_request, mut stream| async move {
            while let Some(Ok(Message::Text(input))) = stream.next().await {
                let output: String = input.chars().rev().collect();
                stream.send_string(format!("{} | {}", &input, &output)).await?;
            }
        Ok(())
    }));

    app.at("/user/:username").get(|mut req: Request<Context>| async move {
        let res = Response::new(200);
        let username: String = req.param("username").unwrap().to_string();
        Ok(Response::new(200))
    });

    app.at("/index").get(|req: tide::Request<Context>| async move {
        Ok (req.resp())
    });

    app.at("/usertest").get(|mut req: tide::Request<Context>| async move {
       let user: User = req.body_json().await?;
       println!("user is {}", user.username);
       let mut res = Response::new(200);
       res.set_body(tide::Body::from_json(&user)?);
       Ok(res)
    });

    public::set_routes(app);
    admin::set_routes(app);
}
