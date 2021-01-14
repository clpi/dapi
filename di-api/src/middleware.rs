use tide::log::LogMiddleware;
use tide::sessions::SessionMiddleware;
use tide::security::{Origin, CorsMiddleware};
use crate::context::Context;

pub fn set(app: &mut tide::Server<Context>) {
    app.with(cors_mw());
    app.with(LogMiddleware::new());
    //app.with(trace_middleware());
    app.with(session_middleware());
    session_validate(app);
}

pub fn session_validate(app: &mut tide::Server<Context>) {
    app.with(tide::utils::Before(
        |mut request: tide::Request<Context>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));
}

pub fn cors_mw() -> tide::security::CorsMiddleware {
    CorsMiddleware::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:5000")
        .allow_origin("http://localhost:5001")
}

pub fn session_middleware() -> SessionMiddleware<tide::sessions::CookieStore> {
    SessionMiddleware::new(
        tide::sessions::CookieStore::new(),
        std::env::var("SESSION_SECRET").expect("Must be 32 byte key").as_bytes(),
    )
}

pub fn trace_middleware() -> tide_tracing::TraceMiddleware {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("no global subscriber has been set");
    tide_tracing::TraceMiddleware::new()
}
