use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::types::ExerciseTemplatesResponse;
use crate::hevy::{CreateExerciseTemplateInput, HevyClient};
use crate::router::args::{CreateExerciseTemplateArgs, IdArgs, PaginationArgs, parse_args};
use crate::router::formatters::format_exercise_template;

pub(crate) async fn handle_get_exercise_templates(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: PaginationArgs = parse_args(args.clone())?;

    let response = client
        .get_exercise_templates(args.page, args.page_size)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching exercise templates: {}", e)))?;
    Ok(format_exercise_templates_page(&response))
}

pub(crate) async fn handle_get_exercise_template(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: IdArgs = parse_args(args.clone())?;

    let template = client
        .get_exercise_template(&args.id)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching exercise template: {}", e)))?;
    Ok(format_exercise_template(&template))
}

pub(crate) async fn handle_create_exercise_template(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: CreateExerciseTemplateArgs = parse_args(args.clone())?;
    let created = client
        .create_exercise_template(CreateExerciseTemplateInput {
            title: args.title,
            exercise_type: args.exercise_type,
            equipment_category: args.equipment_category,
            muscle_group: args.muscle_group,
            other_muscles: args.other_muscles,
        })
        .await
        .map_err(|e| ExecutionError(format!("Error creating exercise template: {}", e)))?;

    Ok(format_created_exercise_template(&created.id))
}

fn format_exercise_templates_page(
    response: &crate::hevy::PaginatedResponse<ExerciseTemplatesResponse>,
) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.data.exercise_templates.is_empty() {
        output.push_str("No exercise templates found.");
        return output;
    }

    for template in &response.data.exercise_templates {
        output.push_str(&format_exercise_template(template));
        output.push_str("\n---\n\n");
    }
    output
}

fn format_created_exercise_template(id: &str) -> String {
    format!("Exercise template created successfully!\nID: {}", id)
}
