use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use mcp_server::router::{CapabilitiesBuilder, Router};
use mcp_spec::handler::{PromptError, ResourceError, ToolError};
use mcp_spec::prompt::Prompt;
use mcp_spec::protocol::ServerCapabilities;
use mcp_spec::{Content, Resource, Tool};
use serde_json::Value;

use crate::hevy::HevyClient;

mod args;
mod formatters;
mod handlers;
mod tools;

use crate::router::handlers::call_tool;
use crate::router::tools::list_tools;

#[derive(Clone)]
pub struct HevyRouter {
    client: Arc<HevyClient>,
}

impl HevyRouter {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Arc::new(HevyClient::new(api_key)),
        }
    }
}

impl Router for HevyRouter {
    fn name(&self) -> String {
        "hevy-mcp-server".to_string()
    }

    fn instructions(&self) -> String {
        "Hevy MCP Server - Interact with the Hevy fitness API. \
        Available operations include managing workouts, routines, \
        exercise templates, and folders. Requires a valid Hevy API key."
            .to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new().with_tools(false).build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        list_tools()
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        call_tool(self.client.clone(), tool_name, arguments)
    }

    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async { Err(ResourceError::NotFound("No resources available".into())) })
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        vec![]
    }

    fn get_prompt(
        &self,
        _prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        Box::pin(async { Err(PromptError::NotFound("No prompts available".into())) })
    }
}
