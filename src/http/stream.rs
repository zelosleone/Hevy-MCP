use crate::HevyRouter;
use crate::router::RequestRouter;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use futures_util::StreamExt;
use mcp_server::router::RouterService;
use mcp_spec::protocol::{
    ErrorData, INTERNAL_ERROR, INVALID_REQUEST, JsonRpcError, JsonRpcMessage, JsonRpcRequest,
    JsonRpcResponse, PARSE_ERROR,
};
use serde::Deserialize;
use std::{io, sync::Arc};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::{
    codec::{FramedRead, LinesCodec, LinesCodecError},
    io::StreamReader,
};
use tower_service::Service;
#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) router: Arc<HevyRouter>,
}

#[derive(Deserialize)]
pub(crate) struct McpQuery {
    apikey: Option<String>,
}

pub(crate) async fn mcp_stream(
    State(state): State<AppState>,
    Query(query): Query<McpQuery>,
    body: Body,
) -> Response {
    let (tx, rx) = mpsc::channel::<Result<Bytes, io::Error>>(32);
    let router = state.router.clone();

    let api_key = match query.apikey.or_else(|| state.router.default_api_key.clone()) {
        Some(key) => key,
        None => {
            let error = error_message(ErrorData {
                code: INVALID_REQUEST,
                message: "API key required. Provide via ?apikey=xxx query parameter or HEVY_API_KEY environment variable".to_string(),
                data: None,
            });
            let json = serde_json::to_string(&error).unwrap_or_default();
            let body = Body::from(format!("{json}\n"));
            return ([(header::CONTENT_TYPE, "application/jsonl")], body).into_response();
        }
    };

    tokio::spawn(async move {
        let stream = body
            .into_data_stream()
            .map(|result| result.map_err(io::Error::other));
        let reader = StreamReader::new(stream);
        let lines = FramedRead::new(reader, LinesCodec::new());
        const MAX_INFLIGHT: usize = 32;

        let _ = lines
            .filter_map(move |line: Result<String, LinesCodecError>| {
                let router = router.clone();
                let api_key = api_key.clone();
                async move { handle_line_result(router, api_key, line).await }
            })
            .for_each_concurrent(Some(MAX_INFLIGHT), |message| async {
                let _ = send_message(&tx, message).await;
            })
            .await;
    });

    let body = Body::from_stream(ReceiverStream::new(rx));
    ([(header::CONTENT_TYPE, "application/jsonl")], body).into_response()
}
async fn send_message(
    tx: &mpsc::Sender<Result<Bytes, io::Error>>,
    message: JsonRpcMessage,
) -> Result<(), StreamError> {
    let json = serde_json::to_string(&message).map_err(StreamError::Serialize)?;
    tx.send(Ok(Bytes::from(format!("{json}\n"))))
        .await
        .map_err(StreamError::Send)
}

async fn handle_message(
    router: Arc<HevyRouter>,
    api_key: String,
    message: JsonRpcMessage,
) -> Option<JsonRpcMessage> {
    match message {
        JsonRpcMessage::Request(request) => {
            let response = call_service(router, api_key, request).await;
            Some(JsonRpcMessage::Response(response))
        }
        JsonRpcMessage::Response(_)
        | JsonRpcMessage::Notification(_)
        | JsonRpcMessage::Nil
        | JsonRpcMessage::Error(_) => None,
    }
}

async fn call_service(
    router: Arc<HevyRouter>,
    api_key: String,
    request: JsonRpcRequest,
) -> JsonRpcResponse {
    let id = request.id;
    let request_router = RequestRouter::new(router, api_key);
    let mut service = RouterService(request_router);

    match service.call(request).await {
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
    }
}

async fn process_line(
    router: Arc<HevyRouter>,
    api_key: String,
    line: String,
) -> Option<JsonRpcMessage> {
    let message = match parse_message(&line) {
        Ok(Some(message)) => message,
        Ok(None) => return None,
        Err(error) => return Some(error_message(error)),
    };

    handle_message(router, api_key, message).await
}

async fn handle_line_result(
    router: Arc<HevyRouter>,
    api_key: String,
    line: Result<String, LinesCodecError>,
) -> Option<JsonRpcMessage> {
    let line = match line {
        Ok(line) => line,
        Err(err) => {
            return Some(error_message(ErrorData {
                code: PARSE_ERROR,
                message: err.to_string(),
                data: None,
            }));
        }
    };

    process_line(router, api_key, line).await
}

fn parse_message(line: &str) -> Result<Option<JsonRpcMessage>, ErrorData> {
    if line.trim().is_empty() {
        return Ok(None);
    }

    let message: JsonRpcMessage = serde_json::from_str(line).map_err(|err| ErrorData {
        code: PARSE_ERROR,
        message: err.to_string(),
        data: None,
    })?;

    let version = match &message {
        JsonRpcMessage::Request(request) => request.jsonrpc.as_str(),
        JsonRpcMessage::Response(response) => response.jsonrpc.as_str(),
        JsonRpcMessage::Notification(notification) => notification.jsonrpc.as_str(),
        JsonRpcMessage::Error(error) => error.jsonrpc.as_str(),
        JsonRpcMessage::Nil => "2.0",
    };

    if version != "2.0" {
        return Err(ErrorData {
            code: INVALID_REQUEST,
            message: "Missing or invalid jsonrpc version".to_string(),
            data: None,
        });
    }

    Ok(Some(message))
}

fn error_message(error: ErrorData) -> JsonRpcMessage {
    JsonRpcMessage::Error(JsonRpcError {
        jsonrpc: "2.0".to_string(),
        id: None,
        error,
    })
}

#[derive(Error, Debug)]
enum StreamError {
    #[error("failed to serialize JSON-RPC message: {0}")]
    Serialize(serde_json::Error),
    #[error("failed to send response: {0}")]
    Send(mpsc::error::SendError<Result<Bytes, io::Error>>),
}
