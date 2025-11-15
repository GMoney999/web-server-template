// src/web/routes_static.rs

use axum::routing::{get_service, MethodRouter};
use tower_http::services::ServeDir;

pub fn routes() -> MethodRouter {
    get_service(ServeDir::new("./"))
}