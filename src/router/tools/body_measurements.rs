use mcp_spec::Tool;
use serde_json::{Map, Value, json};

const MEASUREMENT_FIELDS: &[(&str, &str)] = &[
    ("weight_kg", "Body weight in kilograms"),
    ("lean_mass_kg", "Lean mass in kilograms"),
    ("fat_percent", "Body fat percentage"),
    ("neck_cm", "Neck circumference in centimetres"),
    ("shoulder_cm", "Shoulder circumference in centimetres"),
    ("chest_cm", "Chest circumference in centimetres"),
    ("left_bicep_cm", "Left bicep circumference in centimetres"),
    ("right_bicep_cm", "Right bicep circumference in centimetres"),
    (
        "left_forearm_cm",
        "Left forearm circumference in centimetres",
    ),
    (
        "right_forearm_cm",
        "Right forearm circumference in centimetres",
    ),
    ("abdomen", "Abdomen circumference in centimetres"),
    ("waist", "Waist circumference in centimetres"),
    ("hips", "Hip circumference in centimetres"),
    ("left_thigh", "Left thigh circumference in centimetres"),
    ("right_thigh", "Right thigh circumference in centimetres"),
    ("left_calf", "Left calf circumference in centimetres"),
    ("right_calf", "Right calf circumference in centimetres"),
];

fn measurement_properties(date_description: &str) -> Value {
    let mut properties = Map::new();
    properties.insert(
        "date".to_string(),
        json!({"type": "string", "description": date_description}),
    );
    for (name, description) in MEASUREMENT_FIELDS {
        properties.insert(
            name.to_string(),
            json!({"type": "number", "description": description}),
        );
    }

    json!({
        "type": "object",
        "properties": Value::Object(properties),
        "required": ["date"]
    })
}

pub(crate) fn tools() -> Vec<Tool> {
    vec![
        Tool::new(
            "get_body_measurements",
            "List body measurement entries from Hevy, newest first.",
            json!({
                "type": "object",
                "properties": {
                    "page": {"type": "number", "description": "Page number (1-indexed)"},
                    "page_size": {"type": "number", "description": "Number of items per page (max 10)"}
                }
            }),
        ),
        Tool::new(
            "get_body_measurement",
            "Get the body measurement entry recorded on a single date.",
            json!({
                "type": "object",
                "properties": {
                    "date": {"type": "string", "description": "The measurement date (YYYY-MM-DD)"}
                },
                "required": ["date"]
            }),
        ),
        Tool::new(
            "create_body_measurement",
            "Create a body measurement entry for a date. Fails if an entry already exists for that date; use update_body_measurement instead.",
            measurement_properties("The measurement date (YYYY-MM-DD)"),
        ),
        Tool::new(
            "update_body_measurement",
            "Replace the body measurement entry for a date. All fields are overwritten: any field you omit is cleared, so send every value you want to keep.",
            measurement_properties("The date of the entry to update (YYYY-MM-DD)"),
        ),
    ]
}
