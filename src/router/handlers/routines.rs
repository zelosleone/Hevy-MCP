use std::future::Future;
use std::pin::Pin;

use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::types::RoutinesResponse;
use crate::hevy::{HevyClient, Routine, RoutineInput};
use crate::router::args::{
    CreateRoutineArgs, IdArgs, PaginationArgs, UpdateRoutineArgs, parse_args,
};
use crate::router::formatters::format_routine;

use crate::router::handlers::crud::CrudHandler;

struct RoutineHandler;

impl CrudHandler for RoutineHandler {
    type CreateArgs = CreateRoutineArgs;
    type UpdateArgs = UpdateRoutineArgs;
    type Input = RoutineInput;
    type Output = Routine;

    fn entity_name() -> &'static str {
        "Routine"
    }

    fn format(output: &Self::Output) -> String {
        format_routine(output)
    }

    fn extract_create(args: Self::CreateArgs) -> Self::Input {
        args.routine.into_input()
    }

    fn extract_update(args: Self::UpdateArgs) -> (String, Self::Input) {
        (args.id, args.routine.into_input())
    }

    fn create<'a>(
        client: &'a HevyClient,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>> {
        Box::pin(client.create_routine(input))
    }

    fn update<'a>(
        client: &'a HevyClient,
        id: &'a str,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>> {
        Box::pin(client.update_routine(id, input))
    }
}

pub(crate) async fn handle_get_routines(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: PaginationArgs = parse_args(args.clone())?;

    let response = client
        .get_routines(args.page, args.page_size)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching routines: {}", e)))?;
    Ok(format_routines_page(&response))
}

pub(crate) async fn handle_get_routine(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: IdArgs = parse_args(args.clone())?;

    let routine = client
        .get_routine(&args.id)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching routine: {}", e)))?;
    Ok(format_routine(&routine))
}

pub(crate) async fn handle_create_routine(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    RoutineHandler::handle_create(client, args).await
}

pub(crate) async fn handle_update_routine(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    RoutineHandler::handle_update(client, args).await
}

fn format_routines_page(response: &crate::hevy::PaginatedResponse<RoutinesResponse>) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.data.routines.is_empty() {
        output.push_str("No routines found.");
        return output;
    }

    for routine in &response.data.routines {
        output.push_str(&format_routine(routine));
        output.push_str("\n---\n\n");
    }
    output
}
