use std::future::Future;
use std::pin::Pin;

use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::types::WorkoutsResponse;
use crate::hevy::{HevyClient, Workout, WorkoutInput};
use crate::router::args::{
    CreateWorkoutArgs, IdArgs, PaginationArgs, UpdateWorkoutArgs, WorkoutEventsArgs, parse_args,
};
use crate::router::formatters::format_workout;

use crate::router::handlers::crud::CrudHandler;

struct WorkoutHandler;

impl CrudHandler for WorkoutHandler {
    type CreateArgs = CreateWorkoutArgs;
    type UpdateArgs = UpdateWorkoutArgs;
    type Input = WorkoutInput;
    type Output = Workout;

    fn entity_name() -> &'static str {
        "Workout"
    }

    fn format(output: &Self::Output) -> String {
        format_workout(output)
    }

    fn extract_create(args: Self::CreateArgs) -> Self::Input {
        args.workout.into_input()
    }

    fn extract_update(args: Self::UpdateArgs) -> (String, Self::Input) {
        (args.id, args.workout.into_input())
    }

    fn create<'a>(
        client: &'a HevyClient,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>> {
        Box::pin(client.create_workout(input))
    }

    fn update<'a>(
        client: &'a HevyClient,
        id: &'a str,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>> {
        Box::pin(client.update_workout(id, input))
    }
}

pub(crate) async fn handle_get_workouts(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: PaginationArgs = parse_args(args.clone())?;

    let response = client
        .get_workouts(args.page, args.page_size)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching workouts: {}", e)))?;
    Ok(format_workouts_page(&response))
}

pub(crate) async fn handle_get_workouts_count(client: &HevyClient) -> Result<String, ToolError> {
    let response = client
        .get_workouts_count()
        .await
        .map_err(|e| ExecutionError(format!("Error fetching workout count: {}", e)))?;
    Ok(format!("Workout count: {}", response.workout_count))
}

pub(crate) async fn handle_get_workout_events(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: WorkoutEventsArgs = parse_args(args.clone())?;
    let response = client
        .get_workout_events(args.page, args.page_size, args.since)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching workout events: {}", e)))?;
    Ok(format_workout_events(&response))
}

pub(crate) async fn handle_get_workout(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: IdArgs = parse_args(args.clone())?;

    let workout = client
        .get_workout(&args.id)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching workout: {}", e)))?;
    Ok(format_workout(&workout))
}

pub(crate) async fn handle_create_workout(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    WorkoutHandler::handle_create(client, args).await
}

pub(crate) async fn handle_update_workout(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    WorkoutHandler::handle_update(client, args).await
}

fn format_workouts_page(response: &crate::hevy::PaginatedResponse<WorkoutsResponse>) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.data.workouts.is_empty() {
        output.push_str("No workouts found.");
        return output;
    }

    for workout in &response.data.workouts {
        output.push_str(&format_workout(workout));
        output.push_str("\n---\n\n");
    }
    output
}

fn format_workout_events(response: &crate::hevy::WorkoutEventsResponse) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.events.is_empty() {
        output.push_str("No workout events found.");
        return output;
    }

    for event in &response.events {
        output.push_str(&format_workout_event(event));
        output.push_str("---\n");
    }

    output
}

fn format_workout_event(event: &crate::hevy::WorkoutEvent) -> String {
    match event.event_type.as_str() {
        "updated" => format_updated_workout_event(event),
        "deleted" => format_deleted_workout_event(event),
        other => format!("Event: {}\n", other),
    }
}

fn format_updated_workout_event(event: &crate::hevy::WorkoutEvent) -> String {
    match &event.workout {
        Some(workout) => format!("Updated workout: {} (ID: {})\n", workout.title, workout.id),
        None => "Updated workout: missing details\n".to_string(),
    }
}

fn format_deleted_workout_event(event: &crate::hevy::WorkoutEvent) -> String {
    match (&event.id, &event.deleted_at) {
        (Some(id), Some(deleted_at)) => format!("Deleted workout: {} at {}\n", id, deleted_at),
        (Some(id), None) => format!("Deleted workout: {}\n", id),
        (None, _) => "Deleted workout: missing ID\n".to_string(),
    }
}
