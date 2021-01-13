use async_std::prelude::*;
pub use divc::models::{User, UserLogin};
use tide_websockets::{Message, WebSocket};
use tide::log::LogMiddleware;
use tide::sessions::SessionMiddleware;
use tide::security::{Origin, CorsMiddleware};
use crate::handlers::*;
use divd::PgPool;
use crate::context::Context;
use crate::*;
use tera::Tera;
use tide_tera::prelude::*;

pub use divd;
pub use tide::{
    http::Cookie,
    Response, StatusCode, Request
};


pub async fn set(mut app: tide::Server<Context>) -> tide::Result<tide::Server<Context>> {
    app.at("/").get(|mut req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        let _auth_cookie = req.cookie("auth");
        let host = req.host();
        let session = req.session().id();
        let is_expired = req.session().is_expired();
        let expiry_in = req.session().expires_in();
        let remote = req.remote();
        let _auth_header = req.header("auth");
        let headers = req.header_names().map(|h| h.to_string()).collect::<Vec<String>>();
        tera.render_response("index.html",
            &context! {
                "headers" => headers,
                "host" => host,
                "session" => session,
                "remote" => remote,
                "expiry_in" => expiry_in,
                "is_expired" => is_expired,

            })
    });

    app.at("/auth/login").post(|req: tide::Request<Context>| async move {
        Ok(auth::login(req).await?)
    });

    app.at("/auth/signup").post(|req: Request<Context>| async move {
        Ok(auth::signup(req).await?)
    });

    app.at("/user").post(|req: Request<Context>| async move {
        Ok("hello")
    }).get(|req: Request<Context>| async move {
        Ok(user::get_all(req).await?)
    });

    app.at("/:user/page").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        let user: &str = req.param("user")?;
        let user = user::User::from_username(&req.state().pool, user.to_string()).await?;
        tera.render_response("user.html", &context! { "user" => user })
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

    Ok(app)
}
