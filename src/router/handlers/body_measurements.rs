use mcp_spec::handler::ToolError::{self, ExecutionError};
use serde_json::Value;

use crate::hevy::types::BodyMeasurementsResponse;
use crate::hevy::{BodyMeasurement, HevyClient};
use crate::router::args::{BodyMeasurementArgs, DateArgs, PaginationArgs, parse_args};
use crate::router::formatters::format_body_measurement;

pub(crate) async fn handle_get_body_measurements(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: PaginationArgs = parse_args(args.clone())?;

    let response = client
        .get_body_measurements(args.page, args.page_size)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching body measurements: {}", e)))?;
    Ok(format_body_measurements_page(&response))
}

pub(crate) async fn handle_get_body_measurement(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: DateArgs = parse_args(args.clone())?;
    let measurement = client
        .get_body_measurement(&args.date)
        .await
        .map_err(|e| ExecutionError(format!("Error fetching body measurement: {}", e)))?;
    Ok(format_body_measurement(&measurement))
}

pub(crate) async fn handle_create_body_measurement(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: BodyMeasurementArgs = parse_args(args.clone())?;
    let date = args.date.clone();
    client
        .create_body_measurement(BodyMeasurement {
            date: args.date,
            values: args.values,
        })
        .await
        .map_err(|e| ExecutionError(format!("Error creating body measurement: {}", e)))?;
    Ok(format!("Body measurement created for {}.", date))
}

pub(crate) async fn handle_update_body_measurement(
    client: &HevyClient,
    args: &Value,
) -> Result<String, ToolError> {
    let args: BodyMeasurementArgs = parse_args(args.clone())?;
    client
        .update_body_measurement(&args.date, args.values)
        .await
        .map_err(|e| ExecutionError(format!("Error updating body measurement: {}", e)))?;
    Ok(format!("Body measurement updated for {}.", args.date))
}

fn format_body_measurements_page(
    response: &crate::hevy::PaginatedResponse<BodyMeasurementsResponse>,
) -> String {
    let mut output = format!("Page {} of {}\n\n", response.page, response.page_count);
    if response.data.body_measurements.is_empty() {
        output.push_str("No body measurements found.");
        return output;
    }

    for measurement in &response.data.body_measurements {
        output.push_str(&format_body_measurement(measurement));
        output.push_str("\n---\n\n");
    }
    output
}
