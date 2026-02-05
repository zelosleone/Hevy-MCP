use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "get_routine_folders",
            "List routine folders from Hevy. Folders help organize your workout routines.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 10)"}
                }
            }),
        ),
        Tool::new(
            "get_routine_folder",
            "Get a single routine folder by its ID.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The routine folder ID"}
                },
                "required": ["id"]
            }),
        ),
        Tool::new(
            "create_routine_folder",
            "Create a new folder to organize workout routines in Hevy.",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string", "description": "Folder title"}
                },
                "required": ["title"]
            }),
        ),
    ]
}
