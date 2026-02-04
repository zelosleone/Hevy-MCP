use std::env;

use thiserror::Error;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use hevy_mcp_server::{HevyRouter, http};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    let api_key = match env::var("HEVY_API_KEY") {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => return Err(AppError::MissingApiKey),
        Err(env::VarError::NotUnicode(_)) => {
            return Err(AppError::EnvVar(
                "HEVY_API_KEY must be valid UTF-8".to_string(),
            ));
        }
    };

    let router = HevyRouter::new(api_key);
    let addr = match env::var("HEVY_HTTP_ADDR") {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".to_string(),
        Err(env::VarError::NotUnicode(_)) => {
            return Err(AppError::EnvVar(
                "HEVY_HTTP_ADDR must be valid UTF-8".to_string(),
            ));
        }
    };
    let addr = addr.parse().map_err(AppError::InvalidAddr)?;
    http::serve(router, addr)
        .await
        .map_err(|err| AppError::HttpServe(err.to_string()))?;

    Ok(())
}

#[derive(Error, Debug)]
enum AppError {
    #[error(
        "HEVY_API_KEY environment variable must be set. Get your API key from https://hevy.com/settings?developer"
    )]
    MissingApiKey,
    #[error("{0}")]
    EnvVar(String),
    #[error("HEVY_HTTP_ADDR must be a valid socket address: {0}")]
    InvalidAddr(std::net::AddrParseError),
    #[error("HTTP server error: {0}")]
    HttpServe(String),
}
