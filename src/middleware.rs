use actix_cors::Cors;
use actix_web::middleware::{Compress, Logger};

pub fn cors() -> actix_cors::Cors {
    Cors::default()
        .allowed_methods(vec!["POST", "GET"])
        .supports_credentials()
        .max_age(3600)
}

pub fn compress() -> Compress {
    Compress::default()
}

pub fn logger() -> Logger {
    Logger::default()
}
