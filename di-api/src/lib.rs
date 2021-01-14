pub mod handlers;
pub mod middleware;
pub mod context;
pub mod routes;

pub use divc::models::{User, UserLogin};
pub use divd;
pub use tide::{
    http::Cookie,
    Response, StatusCode, Request
};


pub async fn run(host: &str, port: &str) -> tide::Result<()> {

    tide::log::start();

    let cx = context::create().await?;
    let mut app = tide::with_state(cx);
    configure(&mut app);
    app.listen(format!("{}:{}", host, port)).await?;

    Ok(())
}

pub fn configure(app: &mut tide::Server<context::Context>) {
    middleware::set(app);
    routes::set(app);
}

pub trait RequestExt {
    fn resp(&self) -> String;
}

impl<Context> RequestExt for tide::Request<Context> {
    fn resp(&self) -> String {
        "response".to_string()
    }
}



