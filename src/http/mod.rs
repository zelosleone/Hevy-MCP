use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::{delete, post}};
use thiserror::Error;

use crate::HevyRouter;

mod handler;
mod session;

pub async fn serve(router: HevyRouter, addr: SocketAddr) -> Result<(), HttpError> {
    let session_timeout_secs = std::env::var("HEVY_SESSION_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600);

    let session_manager = Arc::new(session::SessionManager::new(session_timeout_secs));
    session_manager.clone().start_cleanup_task();

    let state = handler::AppState {
        router: Arc::new(router),
        session_manager,
    };

    let path = match std::env::var("HEVY_MCP_PATH") {
        Ok(p) if p.starts_with('/') => p,
        Ok(p) => format!("/{p}"),
        Err(_) => "/".to_string(),
    };

    let app = Router::new()
        .route(&path, post(handler::mcp_handler))
        .route(&path, delete(handler::delete_session))
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
