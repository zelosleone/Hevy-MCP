use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::HevyClient;
use crate::router::args::{ExerciseHistoryArgs, parse_args};

pub(crate) async fn handle_get_exercise_history(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: ExerciseHistoryArgs = parse_args(args.clone())?;
    let response = client
        .get_exercise_history(&args.exercise_template_id, args.start_date, args.end_date)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching exercise history: {}", e)))?;

    Ok(format_exercise_history(&response))
}

fn format_exercise_history(response: &crate::hevy::ExerciseHistoryResponse) -> String {
    if response.exercise_history.is_empty() {
        return "No exercise history found.".to_string();
    }

    let mut output = String::new();
    for entry in &response.exercise_history {
        output.push_str(&format_exercise_history_entry(entry));
        output.push_str("---\n");
    }
    output
}

fn format_exercise_history_entry(entry: &crate::hevy::ExerciseHistoryEntry) -> String {
    let mut output = format!(
        "Workout: {} (ID: {})\n",
        entry.workout_title, entry.workout_id
    );
    output.push_str(&format!("Start: {}\n", entry.workout_start_time));
    if let Some(end) = entry.workout_end_time {
        output.push_str(&format!("End: {}\n", end));
    }
    output.push_str(&format!(
        "Exercise Template ID: {}\n",
        entry.exercise_template_id
    ));
    if let Some(weight) = entry.weight_kg {
        output.push_str(&format!("Weight: {:.1}kg\n", weight));
    }
    if let Some(reps) = entry.reps {
        output.push_str(&format!("Reps: {}\n", reps));
    }
    if let Some(rpe) = entry.rpe {
        output.push_str(&format!("RPE: {}\n", rpe));
    }
    output
}
