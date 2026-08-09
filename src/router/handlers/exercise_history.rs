use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::HevyClient;
use crate::router::args::{ExerciseHistoryArgs, parse_args};
use crate::router::formatters::{SetMetrics, format_set_metrics};

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
    let metrics = format_set_metrics(&SetMetrics {
        set_type: entry.set_type.as_ref(),
        weight_kg: entry.weight_kg,
        reps: entry.reps,
        rep_range: None,
        duration_seconds: entry.duration_seconds,
        distance_meters: entry.distance_meters,
        rpe: entry.rpe,
        custom_metric: entry.custom_metric,
    });
    output.push_str(&format!("Set: {}\n", metrics));
    output
}
