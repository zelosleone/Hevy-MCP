use mcp_spec::handler::ToolError::{self, ExecutionError};

use crate::hevy::HevyClient;

pub(crate) async fn handle_get_user_info(client: &HevyClient) -> Result<String, ToolError> {
    let user = client
        .get_user_info()
        .await
        .map_err(|e| ExecutionError(format!("Error fetching user info: {}", e)))?;

    let mut output = format!("User: {}\nID: {}\n", user.name, user.id);
    if let Some(url) = &user.url {
        output.push_str(&format!("Profile: {}\n", url));
    }
    Ok(output)
}
