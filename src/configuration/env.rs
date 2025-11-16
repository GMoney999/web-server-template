// src/configuration/env.rs
use crate::Error;

pub async fn get_env(env_var: impl Into<String>) -> crate::Result<String> {
    let var: String = env_var.into();
    let value = std::env::var(var.clone()).map_err(|_| Error::MissingEnvVar(var.clone()))?;
    Ok(value)
}