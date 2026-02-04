use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use mcp_spec::Content;
use mcp_spec::handler::ToolError::{self, NotFound};
use serde_json::Value;

use crate::hevy::HevyClient;

mod crud;
mod exercise_history;
mod routine_folders;
mod routines;
mod templates;
mod workouts;

use exercise_history::handle_get_exercise_history;
use routine_folders::{
    handle_create_routine_folder, handle_get_routine_folder, handle_get_routine_folders,
};
use routines::{
    handle_create_routine, handle_get_routine, handle_get_routines, handle_update_routine,
};
use templates::{
    handle_create_exercise_template, handle_get_exercise_template, handle_get_exercise_templates,
};
use workouts::{
    handle_create_workout, handle_get_workout, handle_get_workout_events, handle_get_workouts,
    handle_get_workouts_count, handle_update_workout,
};

pub(crate) fn call_tool(
    client: Arc<HevyClient>,
    tool_name: &str,
    arguments: Value,
) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
    let tool_name = tool_name.to_string();

    Box::pin(async move {
        let result = match tool_name.as_str() {
            "get_workouts" => handle_get_workouts(&client, &arguments).await,
            "get_workouts_count" => handle_get_workouts_count(&client).await,
            "get_workout_events" => handle_get_workout_events(&client, &arguments).await,
            "get_workout" => handle_get_workout(&client, &arguments).await,
            "create_workout" => handle_create_workout(&client, &arguments).await,
            "update_workout" => handle_update_workout(&client, &arguments).await,
            "get_routines" => handle_get_routines(&client, &arguments).await,
            "get_routine" => handle_get_routine(&client, &arguments).await,
            "create_routine" => handle_create_routine(&client, &arguments).await,
            "update_routine" => handle_update_routine(&client, &arguments).await,
            "get_exercise_templates" => handle_get_exercise_templates(&client, &arguments).await,
            "get_exercise_template" => handle_get_exercise_template(&client, &arguments).await,
            "create_exercise_template" => {
                handle_create_exercise_template(&client, &arguments).await
            }
            "get_exercise_history" => handle_get_exercise_history(&client, &arguments).await,
            "get_routine_folders" => handle_get_routine_folders(&client, &arguments).await,
            "get_routine_folder" => handle_get_routine_folder(&client, &arguments).await,
            "create_routine_folder" => handle_create_routine_folder(&client, &arguments).await,
            _ => Err(NotFound(format!("Unknown tool: {}", tool_name))),
        };

        result.map(|text| vec![Content::text(text)])
    })
}
