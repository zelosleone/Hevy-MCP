use std::future::Future;
use std::pin::Pin;

use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::hevy::HevyClient;
use crate::router::args::parse_args;

pub(crate) trait CrudHandler {
    type CreateArgs: DeserializeOwned;
    type UpdateArgs: DeserializeOwned;
    type Input;
    type Output;

    fn entity_name() -> &'static str;
    fn format(output: &Self::Output) -> String;
    fn extract_create(args: Self::CreateArgs) -> Self::Input;
    fn extract_update(args: Self::UpdateArgs) -> (String, Self::Input);
    fn create<'a>(
        client: &'a HevyClient,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>>;
    fn update<'a>(
        client: &'a HevyClient,
        id: &'a str,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = crate::hevy::Result<Self::Output>> + Send + 'a>>;
    fn handle_create<'a>(
        client: &'a HevyClient,
        args: &'a Value,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send + 'a>>
    where
        Self: Sized,
    {
        Box::pin(async move {
            let parsed_args: Self::CreateArgs = parse_args(args.clone())?;
            let input = Self::extract_create(parsed_args);

            let created = Self::create(client, input).await.map_err(|e| {
                ExecutionError(format!("Error creating {}: {}", Self::entity_name(), e))
            })?;
            Ok(format!(
                "{} created successfully!\n\n{}",
                Self::entity_name(),
                Self::format(&created)
            ))
        })
    }

    fn handle_update<'a>(
        client: &'a HevyClient,
        args: &'a Value,
    ) -> Pin<Box<dyn Future<Output = Result<String, ToolError>> + Send + 'a>>
    where
        Self: Sized,
    {
        Box::pin(async move {
            let parsed_args: Self::UpdateArgs = parse_args(args.clone())?;
            let (id, input) = Self::extract_update(parsed_args);

            let updated = Self::update(client, &id, input).await.map_err(|e| {
                ExecutionError(format!("Error updating {}: {}", Self::entity_name(), e))
            })?;
            Ok(format!(
                "{} updated successfully!\n\n{}",
                Self::entity_name(),
                Self::format(&updated)
            ))
        })
    }
}
