use crate::http::session::SessionManager;
use crate::router::RequestRouter;
use crate::HevyRouter;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use mcp_server::router::RouterService;
use mcp_spec::protocol::{
    ErrorData, INTERNAL_ERROR, INVALID_REQUEST, JsonRpcError, JsonRpcMessage, JsonRpcNotification,
    JsonRpcRequest, JsonRpcResponse, PARSE_ERROR,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_service::Service;

const MCP_SESSION_HEADER: &str = "Mcp-Session-Id";

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) router: Arc<HevyRouter>,
    pub(crate) session_manager: Arc<SessionManager>,
}

#[derive(Deserialize)]
pub(crate) struct McpQuery {
    apikey: Option<String>,
}

pub(crate) async fn mcp_handler(
    State(state): State<AppState>,
    Query(query): Query<McpQuery>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return error_response(INTERNAL_ERROR, format!("Failed to read request body: {}", err))
        }
    };

    let message: JsonRpcMessage = match serde_json::from_slice(&body_bytes) {
        Ok(msg) => msg,
        Err(err) => return error_response(PARSE_ERROR, format!("Invalid JSON: {}", err)),
    };

    let session_id = headers
        .get(MCP_SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match message {
        JsonRpcMessage::Request(request) => {
            handle_request(state, query, session_id, request).await
        }
        JsonRpcMessage::Notification(notification) => {
            handle_notification(state, session_id, notification).await
        }
        _ => error_response(INVALID_REQUEST, "Expected request or notification".to_string()),
    }
}

async fn handle_request(
    state: AppState,
    query: McpQuery,
    session_id: Option<String>,
    request: JsonRpcRequest,
) -> Response {
    if request.method == "initialize" {
        return handle_initialize(state, query, request).await;
    }

    let session_id = match session_id {
        Some(id) => id,
        None => {
            return error_response(
                INVALID_REQUEST,
                "Missing Mcp-Session-Id header. Call initialize first.".to_string(),
            )
        }
    };

    let session = match state.session_manager.get_session(&session_id) {
        Some(session) => session,
        None => {
            return (
                StatusCode::NOT_FOUND,
                json_body(error_json(
                    request.id,
                    INVALID_REQUEST,
                    "Session not found or expired".to_string(),
                )),
            )
                .into_response()
        }
    };

    state.session_manager.update_activity(&session_id);

    let request_router = RequestRouter::new(state.router.clone(), session.api_key.clone());
    let mut service = RouterService(request_router);

    let id = request.id;
    let response = match service.call(request).await {
        Ok(response) => response,
        Err(err) => {
            let error_message = format!("{:?}", err);
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(ErrorData {
                    code: INTERNAL_ERROR,
                    message: error_message,
                    data: None,
                }),
            }
        }
    };

    json_response(response)
}

async fn handle_initialize(
    state: AppState,
    query: McpQuery,
    request: JsonRpcRequest,
) -> Response {
    let api_key = match query
        .apikey
        .or_else(|| state.router.default_api_key.clone())
    {
        Some(key) => key,
        None => {
            return error_response(
                INVALID_REQUEST,
                "API key required. Provide via ?apikey=xxx query parameter or HEVY_API_KEY environment variable".to_string(),
            )
        }
    };

    let client_capabilities = request
        .params
        .as_ref()
        .and_then(|p| p.get("capabilities"))
        .cloned()
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    let session = state
        .session_manager
        .new_session(api_key, client_capabilities);

    let request_router = RequestRouter::new(state.router.clone(), session.api_key.clone());
    let mut service = RouterService(request_router);

    let id = request.id;
    let response = match service.call(request).await {
        Ok(response) => response,
        Err(err) => {
            let error_message = format!("{:?}", err);
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(ErrorData {
                    code: INTERNAL_ERROR,
                    message: error_message,
                    data: None,
                }),
            }
        }
    };

    let mut headers = HeaderMap::new();
    if let Ok(header_value) = session.session_id.parse() {
        headers.insert(MCP_SESSION_HEADER, header_value);
    }
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    (StatusCode::OK, headers, json_body(response)).into_response()
}

async fn handle_notification(
    state: AppState,
    session_id: Option<String>,
    notification: JsonRpcNotification,
) -> Response {
    if notification.method == "notifications/initialized" {
        return (StatusCode::ACCEPTED, "").into_response();
    }

    if notification.method == "notifications/cancelled" {
        return (StatusCode::ACCEPTED, "").into_response();
    }

    if let Some(session_id) = session_id {
        if state.session_manager.get_session(&session_id).is_some() {
            state.session_manager.update_activity(&session_id);
        }
    }

    (StatusCode::ACCEPTED, "").into_response()
}

pub(crate) async fn delete_session(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    let session_id = match headers
        .get(MCP_SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
    {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing Mcp-Session-Id header".to_string(),
            )
                .into_response()
        }
    };

    if state.session_manager.remove_session(session_id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Session not found".to_string()).into_response()
    }
}

fn json_response(response: JsonRpcResponse) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        json_body(response),
    )
        .into_response()
}

fn error_response(code: i32, message: String) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        json_body(error_json(None, code, message)),
    )
        .into_response()
}

fn error_json(
    id: Option<u64>,
    code: i32,
    message: String,
) -> JsonRpcMessage {
    JsonRpcMessage::Error(JsonRpcError {
        jsonrpc: "2.0".to_string(),
        id,
        error: ErrorData {
            code,
            message,
            data: None,
        },
    })
}

fn json_body<T: serde::Serialize>(value: T) -> String {
    serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
}
