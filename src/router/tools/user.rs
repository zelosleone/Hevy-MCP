use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![Tool::new(
        "get_user_info",
        "Get the account info of the user the API key belongs to.",
        json!({
            "type": "object",
            "properties": {}
        }),
    )]
}
