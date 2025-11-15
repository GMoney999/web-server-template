// src/web/routes_health.rs
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    Html("This is a <strong>health check</strong>")
}