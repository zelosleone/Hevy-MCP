use mcp_spec::Tool;

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
