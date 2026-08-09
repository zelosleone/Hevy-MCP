use mcp_spec::Tool;
use serde_json::json;

pub(crate) fn tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "get_exercise_templates",
            "List exercise templates from Hevy's exercise library. These are the exercises you can add to workouts and routines.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 100)"}
                }
            }),
        ),
        Tool::new(
            "get_exercise_template",
            "Get a single exercise template by its ID. Returns exercise details including muscle groups and equipment.",
            json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The exercise template ID"}
                },
                "required": ["id"]
            }),
        ),
        Tool::new(
            "create_exercise_template",
            "Create a new custom exercise template.",
            json!({
                "type": "object",
                "properties": {
                    "title": {"type": "string", "description": "Exercise title"},
                    "exercise_type": {
                        "type": "string",
                        "enum": ["weight_reps", "reps_only", "bodyweight_weighted", "bodyweight_assisted", "duration", "distance_duration", "short_distance_weight", "floors_duration", "steps_duration"],
                        "description": "Exercise type"
                    },
                    "equipment_category": {
                        "type": "string",
                        "enum": ["none", "barbell", "dumbbell", "kettlebell", "machine", "plate", "resistance_band", "suspension", "other"],
                        "description": "Equipment category"
                    },
                    "muscle_group": {
                        "type": "string",
                        "enum": ["abdominals", "abductors", "adductors", "biceps", "calves", "cardio", "chest", "forearms", "full_body", "glutes", "hamstrings", "lats", "lower_back", "neck", "quadriceps", "shoulders", "traps", "triceps", "upper_back", "other"],
                        "description": "Primary muscle group"
                    },
                    "other_muscles": {
                        "type": "array",
                        "description": "Optional additional muscle groups",
                        "items": {
                            "type": "string",
                            "enum": ["abdominals", "abductors", "adductors", "biceps", "calves", "cardio", "chest", "forearms", "full_body", "glutes", "hamstrings", "lats", "lower_back", "neck", "quadriceps", "shoulders", "traps", "triceps", "upper_back", "other"]
                        }
                    }
                },
                "required": ["title", "exercise_type", "equipment_category", "muscle_group"]
            }),
        ),
    ]
}
