// src/web/routes_health.rs
use axum::extract::{Path, Query};
// src/web/routes_health.rs
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, post};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/hello", post(hello_query))
        .route("/hello/{name}", get(hello_path))
}

pub async fn health_check() -> impl IntoResponse {
    Html("This is a <strong>health check</strong>")
}

#[derive(Deserialize)]
pub struct QueryParams {
    name: Option<String>
}

pub async fn hello_query(Query(params): Query<QueryParams>) -> impl IntoResponse {
    println!("->> {:<12} — health_check_query", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{}!!!</strong>", name))
}

pub async fn hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} — health_check_path", "HANDLER"); 
    Html(format!("Hello <strong>{}</strong>", name))
}

