use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "get_routines",
            "List workout routines from Hevy with pagination. Routines are workout templates you can start workouts from.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 10)"}
                }
            }),
        ),
        Tool::new(
            "get_routine",
            "Get a single routine by its ID. Returns full routine details including all exercises and set templates.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The routine ID"}
                },
                "required": ["id"]
            }),
        ),
        Tool::new(
            "create_routine",
            "Create a new workout routine in Hevy. Routines serve as templates for workouts.",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string", "description": "Routine title"},
                    "folder_id": {"type": "string", "description": "Folder ID to place the routine in"},
                    "notes": {"type": "string", "description": "Optional notes"},
                    "exercises": {"type": "array", "description": "Exercises in the routine"}
                },
                "required": ["title"]
            }),
        ),
        Tool::new(
            "update_routine",
            "Update an existing routine by ID. Replaces all routine data with the provided values.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The routine ID to update"},
                    "title": {"type": "string", "description": "Routine title"},
                    "notes": {"type": "string", "description": "Optional notes"},
                    "exercises": {"type": "array", "description": "Exercises in the routine"}
                },
                "required": ["id", "title"]
            }),
        ),
    ]
}
