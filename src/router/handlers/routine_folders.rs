use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::types::RoutineFoldersResponse;
use crate::hevy::{HevyClient, RoutineFolderInput};
use crate::router::args::{CreateRoutineFolderArgs, IdArgs, PaginationArgs, parse_args};
use crate::router::formatters::format_folder;

pub(crate) async fn handle_get_routine_folders(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: PaginationArgs = parse_args(args.clone())?;

    let response = client
        .get_routine_folders(args.page, args.page_size)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching routine folders: {}", e)))?;
    Ok(format_routine_folders_page(&response))
}

pub(crate) async fn handle_get_routine_folder(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: IdArgs = parse_args(args.clone())?;
    let folder = client
        .get_routine_folder(&args.id)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching routine folder: {}", e)))?;
    Ok(format_folder(&folder))
}

pub(crate) async fn handle_create_routine_folder(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: CreateRoutineFolderArgs = parse_args(args.clone())?;
    let folder = client
        .create_routine_folder(RoutineFolderInput {
            title: args.title,
            index: args.index,
        })
        .await
        .map_err(|e| ExecutionError(format!("Error creating Folder: {}", e)))?;
    Ok(format_created_folder(&folder))
}

fn format_routine_folders_page(
    response: &crate::hevy::PaginatedResponse<RoutineFoldersResponse>,
) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.data.routine_folders.is_empty() {
        output.push_str("No routine folders found.");
        return output;
    }

    for folder in &response.data.routine_folders {
        output.push_str(&format_folder(folder));
        output.push_str("\n---\n\n");
    }
    output
}

fn format_created_folder(folder: &crate::hevy::RoutineFolder) -> String {
    format!("Folder created successfully!\n\n{}", format_folder(folder))
}
