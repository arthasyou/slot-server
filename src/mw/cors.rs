use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any)
}
