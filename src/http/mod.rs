use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::post};
use thiserror::Error;

use crate::HevyRouter;

mod stream;

pub async fn serve(router: HevyRouter, addr: SocketAddr) -> Result<(), HttpError> {
    let state = stream::AppState {
        router: Arc::new(router),
    };

    let app = Router::new()
        .route("/mcp", post(stream::mcp_stream))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| HttpError::Bind(err.to_string()))?;
    tracing::info!("HTTP MCP server listening on {}", addr);
    axum::serve(listener, app)
        .await
        .map_err(|err| HttpError::Serve(err.to_string()))?;
    Ok(())
}

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("failed to bind HTTP listener: {0}")]
    Bind(String),
    #[error("HTTP server error: {0}")]
    Serve(String),
}
