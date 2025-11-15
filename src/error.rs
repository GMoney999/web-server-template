// src/error.rs
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = color_eyre::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Config Error: Missing Env Var: {0}")]
    MissingEnvVar(String),

    #[error("API Error: Login Failed")]
    LoginFail
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} – INTO_RES", "HANDLER");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
