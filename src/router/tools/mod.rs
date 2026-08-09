use mcp_spec::Tool;
use serde_json::{Value, json};

mod body_measurements;
mod exercise_history;
mod exercise_templates;
mod routine_folders;
mod routines;
mod user;
mod workouts;

pub(crate) fn list_tools() -> Vec<Tool> {
    let mut tools = Vec::new();
    tools.extend(workouts::tools());
    tools.extend(routines::tools());
    tools.extend(exercise_templates::tools());
    tools.extend(exercise_history::tools());
    tools.extend(routine_folders::tools());
    tools.extend(body_measurements::tools());
    tools.extend(user::tools());
    tools
}

/// `mcp-spec` 0.1 has no field for tool annotations, so they are injected into
/// the serialised `tools/list` result instead. Clients use these hints to group
/// tools by whether they read or modify data.
pub(crate) fn annotate_tools(result: &mut Value) {
    let Some(tools) = result.get_mut("tools").and_then(Value::as_array_mut) else {
        return;
    };

    for tool in tools {
        let Some(name) = tool.get("name").and_then(Value::as_str) else {
            continue;
        };
        let Some(annotations) = annotations_for(name) else {
            continue;
        };
        if let Some(tool) = tool.as_object_mut() {
            tool.insert("annotations".to_string(), annotations);
        }
    }
}

fn annotations_for(name: &str) -> Option<Value> {
    // Every tool either reads (get_*), adds (create_*), or replaces (update_*).
    // Updates are destructive because the Hevy API overwrites the whole record.
    let (read_only, destructive, idempotent) = match name.split_once('_')?.0 {
        "get" => (true, false, true),
        "create" => (false, false, false),
        "update" => (false, true, true),
        _ => return None,
    };

    Some(json!({
        "readOnlyHint": read_only,
        "destructiveHint": destructive,
        "idempotentHint": idempotent,
        "openWorldHint": true,
    }))
}
