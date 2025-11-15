// src/main.rs
#![allow(unused)]

use std::net::SocketAddr;
use axum::Router;
pub use self::error::{Result, Error};

mod error;
mod web;
mod model;
mod configuration;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, routes_all()).await;

    Ok(())
}


pub fn routes_all() -> Router {
    Router::new()
        .merge(web::routes_health::routes())
        .fallback_service(web::routes_static::routes())
}

