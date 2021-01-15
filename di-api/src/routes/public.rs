use divc::models::user::{User, UserLogin};
use async_std::prelude::*;
use tide_websockets::{Message, WebSocket};
use crate::context::Context;
use crate::*;
use tera::Tera;
use tide_tera::prelude::*;

pub mod dashboard;

pub fn set_routes(app: &mut tide::Server<Context>) {
    app.at("/").get(|req: tide::Request<Context>| index(req));
    app.at("/users").get(|req: Request<Context>| async move {
        let tera = &req.state().tera;
        let users = User::get_all(&req.state().pool).await?;
        tera.render_response("users.html", &context! { "users" => users })
    });

    app.at("/:user/page").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        let user: &str = req.param("user")?;
        let user = User::from_username(&req.state().pool, user.to_string()).await?;
        tera.render_response("user.html", &context! { "user" => user })
    });

    app.at("/contact").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("contact.html", &context! {})
    });
    app.at("/cover").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("cover.html", &context! {})
    });
    app.at("/about").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("about.html", &context! {})
    });
    app.at("/dashboard").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("dashboard.html", &context! {})
    });
    app.at("/login").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("login.html", &context! {})
    });
    app.at("/signup").get(|req: tide::Request<Context>| async move {
        let tera = &req.state().tera;
        tera.render_response("signup.html", &context! {})
    });
}

/// GET "/" -> HTML
pub async fn index(req: tide::Request<Context>) -> tide::Result {
        let tera = &req.state().tera;
        let _auth_cookie = req.cookie("auth");
        let host = req.host();
        let session = req.session().id();
        let is_expired = req.session().is_expired();
        let expiry_in = req.session().expires_in().map(|d| d.as_secs());
        let remote = req.remote();
        let _auth_header = req.header("auth");
        let headers = req.header_names().zip(req.header_values())
            .map(|(h, v)| (h.to_string(), v.to_string()))
            .collect::<Vec<(String, String)>>();
        tera.render_response("index.html",
            &context! {
                "headers" => headers,
                "host" => host,
                "session" => session,
                "remote" => remote,
                "expiry_in" => expiry_in,
                "is_expired" => is_expired,
            })
    // });
}
