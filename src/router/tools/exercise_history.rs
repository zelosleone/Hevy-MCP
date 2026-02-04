use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![Tool::new(
        "get_exercise_history",
        "Get exercise history for a specific exercise template.",
        json!({
            "type": "object",
            "properties": {
                "exercise_template_id": {"type": "string", "description": "Exercise template ID"},
                "start_date": {"type": "string", "description": "Optional ISO 8601 start date"},
                "end_date": {"type": "string", "description": "Optional ISO 8601 end date"}
            },
            "required": ["exercise_template_id"]
        }),
    )]
}
