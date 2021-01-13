use tide::log::LogMiddleware;
use tide::sessions::SessionMiddleware;
use tide::security::{Origin, CorsMiddleware};
use opentelemetry::sdk::propagation::{
    BaggagePropagator, TextMapCompositePropagator, TraceContextPropagator,
};
use opentelemetry::{
    global,
    trace::{FutureExt, TraceContextExt, Tracer},
    Context as OTContext, KeyValue,
};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_tide::OpenTelemetryTracingMiddleware;
use crate::context::Context;

pub async fn set(mut app: tide::Server<Context>) -> tide::Result<tide::Server<Context>> {
    app.with(CorsMiddleware::new()
        .allow_credentials(true)
        .allow_origin("http://localhost:5000")
        .allow_origin("http://localhost:5001"));

    app.with(LogMiddleware::new());

    app.with(SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        std::env::var("SESSION_SECRET").expect("Must be 32 byte key").as_bytes(),
    ));

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name("di-api")
        .install()
        .expect("Could not install pipeline");
    app.with(OpenTelemetryTracingMiddleware::new(tracer));

    app.with(tide::utils::Before(
        |mut request: tide::Request<Context>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));
    Ok(app)
}
