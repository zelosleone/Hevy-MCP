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
                    "folder_id": {"type": "string", "description": "Folder ID to place the routine in (use get_routine_folders to list available folders)"},
                    "notes": {"type": "string", "description": "Optional notes"},
                    "exercises": {
                        "type": "array",
                        "description": "Exercises in the routine",
                        "items": {
                            "type": "object",
                            "properties": {
                                "exercise_template_id": {"type": "string", "description": "Exercise template ID"},
                                "superset_id": {"type": "number", "description": "Superset group ID"},
                                "rest_seconds": {"type": "number", "description": "Rest time between sets in seconds"},
                                "notes": {"type": "string", "description": "Exercise notes"},
                                "sets": {
                                    "type": "array",
                                    "description": "Set templates in the exercise",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "set_type": {"type": "string", "enum": ["normal", "warmup", "dropset", "failure"], "description": "Type of set"},
                                            "weight_kg": {"type": "number", "description": "Weight in kg"},
                                            "reps": {"type": "number", "description": "Number of reps"},
                                            "rep_range": {
                                                "type": "object",
                                                "description": "Rep range for the set",
                                                "properties": {
                                                    "start": {"type": "number", "description": "Min reps"},
                                                    "end": {"type": "number", "description": "Max reps"}
                                                }
                                            },
                                            "duration_seconds": {"type": "number", "description": "Duration in seconds"},
                                            "distance_meters": {"type": "number", "description": "Distance in meters"},
                                            "custom_metric": {"type": "number", "description": "Custom metric value"}
                                        }
                                    }
                                }
                            },
                            "required": ["exercise_template_id"]
                        }
                    }
                },
                "required": ["title", "folder_id"]
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
                    "exercises": {
                        "type": "array",
                        "description": "Exercises in the routine",
                        "items": {
                            "type": "object",
                            "properties": {
                                "exercise_template_id": {"type": "string", "description": "Exercise template ID"},
                                "superset_id": {"type": "number", "description": "Superset group ID"},
                                "rest_seconds": {"type": "number", "description": "Rest time between sets in seconds"},
                                "notes": {"type": "string", "description": "Exercise notes"},
                                "sets": {
                                    "type": "array",
                                    "description": "Set templates in the exercise",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "set_type": {"type": "string", "enum": ["normal", "warmup", "dropset", "failure"], "description": "Type of set"},
                                            "weight_kg": {"type": "number", "description": "Weight in kg"},
                                            "reps": {"type": "number", "description": "Number of reps"},
                                            "rep_range": {
                                                "type": "object",
                                                "description": "Rep range for the set",
                                                "properties": {
                                                    "start": {"type": "number", "description": "Min reps"},
                                                    "end": {"type": "number", "description": "Max reps"}
                                                }
                                            },
                                            "duration_seconds": {"type": "number", "description": "Duration in seconds"},
                                            "distance_meters": {"type": "number", "description": "Distance in meters"},
                                            "custom_metric": {"type": "number", "description": "Custom metric value"}
                                        }
                                    }
                                }
                            },
                            "required": ["exercise_template_id"]
                        }
                    }
                },
                "required": ["id", "title"]
            }),
        ),
    ]
}
