use mcp_spec::handler::ToolError;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) fn parse_args<T: DeserializeOwned>(arguments: Value) -> Result<T, ToolError> {
    serde_path_to_error::deserialize(arguments)
        .map_err(|err| ToolError::InvalidParameters(format!("invalid parameters: {}", err)))
}

#[derive(Deserialize)]
pub(crate) struct PaginationArgs {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Deserialize)]
pub(crate) struct IdArgs {
    pub id: String,
}
