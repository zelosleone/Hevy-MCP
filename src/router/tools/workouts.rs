use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "get_workouts",
            "List workouts from Hevy with pagination. Returns workout summaries including title, date, and exercises.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 10)"}
                }
            }),
        ),
        Tool::new(
            "get_workouts_count",
            "Get the total number of workouts on the account.",
            json!({"type": "object", "properties": {}}),
        ),
        Tool::new(
            "get_workout_events",
            "List workout update and delete events since a given date, with pagination.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 10)"},
                    "since": {"type": "string", "description": "ISO 8601 timestamp to start from"}
                }
            }),
        ),
        Tool::new(
            "get_workout",
            "Get a single workout by its ID. Returns full workout details including all exercises and sets.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The workout ID"}
                },
                "required": ["id"]
            }),
        ),
        Tool::new(
            "create_workout",
            "Create a new workout in Hevy. Requires title, start_time (ISO 8601), and exercises with sets.",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string", "description": "Workout title"},
                    "is_private": {"type": "boolean", "description": "Whether the workout is private"},
                    "start_time": {"type": "string", "description": "Start time in ISO 8601 format"},
                    "description": {"type": "string", "description": "Optional description"},
                    "end_time": {"type": "string", "description": "End time in ISO 8601 format"},
                    "exercises": {
                        "type": "array",
                        "description": "Exercises in the workout",
                        "items": {
                            "type": "object",
                            "properties": {
                                "exercise_template_id": {"type": "string", "description": "Exercise template ID"},
                                "superset_id": {"type": "number", "description": "Superset group ID"},
                                "notes": {"type": "string", "description": "Exercise notes"},
                                "sets": {
                                    "type": "array",
                                    "description": "Sets in the exercise",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "set_type": {"type": "string", "enum": ["normal", "warmup", "dropset", "failure"], "description": "Type of set"},
                                            "weight_kg": {"type": "number", "description": "Weight in kg"},
                                            "reps": {"type": "number", "description": "Number of reps"},
                                            "duration_seconds": {"type": "number", "description": "Duration in seconds"},
                                            "distance_meters": {"type": "number", "description": "Distance in meters"},
                                            "rpe": {"type": "number", "description": "Rate of perceived exertion (1-10)"},
                                            "custom_metric": {"type": "number", "description": "Custom metric value"}
                                        }
                                    }
                                }
                            },
                            "required": ["exercise_template_id"]
                        }
                    }
                },
                "required": ["title", "start_time"]
            }),
        ),
        Tool::new(
            "update_workout",
            "Update an existing workout by ID. Replaces all workout data with the provided values.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The workout ID to update"},
                    "title": {"type": "string", "description": "Workout title"},
                    "is_private": {"type": "boolean", "description": "Whether the workout is private"},
                    "start_time": {"type": "string", "description": "Start time in ISO 8601 format"},
                    "description": {"type": "string", "description": "Optional description"},
                    "end_time": {"type": "string", "description": "End time in ISO 8601 format"},
                    "exercises": {
                        "type": "array",
                        "description": "Exercises in the workout",
                        "items": {
                            "type": "object",
                            "properties": {
                                "exercise_template_id": {"type": "string", "description": "Exercise template ID"},
                                "superset_id": {"type": "number", "description": "Superset group ID"},
                                "notes": {"type": "string", "description": "Exercise notes"},
                                "sets": {
                                    "type": "array",
                                    "description": "Sets in the exercise",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "set_type": {"type": "string", "enum": ["normal", "warmup", "dropset", "failure"], "description": "Type of set"},
                                            "weight_kg": {"type": "number", "description": "Weight in kg"},
                                            "reps": {"type": "number", "description": "Number of reps"},
                                            "duration_seconds": {"type": "number", "description": "Duration in seconds"},
                                            "distance_meters": {"type": "number", "description": "Distance in meters"},
                                            "rpe": {"type": "number", "description": "Rate of perceived exertion (1-10)"},
                                            "custom_metric": {"type": "number", "description": "Custom metric value"}
                                        }
                                    }
                                }
                            },
                            "required": ["exercise_template_id"]
                        }
                    }
                },
                "required": ["id", "title", "start_time"]
            }),
        ),
    ]
}
