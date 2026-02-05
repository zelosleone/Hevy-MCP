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
                    "exercise_type": {"type": "string", "description": "Exercise type, for example weight_reps"},
                    "equipment_category": {"type": "string", "description": "Equipment category, for example barbell"},
                    "muscle_group": {"type": "string", "description": "Primary muscle group, for example chest"},
                    "other_muscles": {
                        "type": "array",
                        "description": "Optional additional muscle groups",
                        "items": {
                            "type": "string",
                            "description": "Muscle group name (e.g., Abdominals, Biceps, Chest, Triceps, Lats, Quadriceps, etc.)"
                        }
                    }
                },
                "required": ["title", "exercise_type", "equipment_category", "muscle_group"]
            }),
        ),
    ]
}
