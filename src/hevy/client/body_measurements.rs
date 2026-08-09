use reqwest::Method;

use crate::hevy::client::{HevyClient, Result, paginated_endpoint};
use crate::hevy::types::{
    BodyMeasurement, BodyMeasurementValues, BodyMeasurementsResponse, PaginatedResponse,
};

impl HevyClient {
    pub async fn get_body_measurements(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<BodyMeasurementsResponse>> {
        let endpoint = paginated_endpoint("/body_measurements", page, page_size);
        let response = self.request(Method::GET, &endpoint).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_body_measurement(&self, date: &str) -> Result<BodyMeasurement> {
        let response = self
            .request(Method::GET, &format!("/body_measurements/{date}"))
            .send()
            .await?;
        self.handle_response(response).await
    }

    pub async fn create_body_measurement(&self, measurement: BodyMeasurement) -> Result<()> {
        let response = self
            .request(Method::POST, "/body_measurements")
            .json(&measurement)
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }

    pub async fn update_body_measurement(
        &self,
        date: &str,
        values: BodyMeasurementValues,
    ) -> Result<()> {
        let response = self
            .request(Method::PUT, &format!("/body_measurements/{date}"))
            .json(&values)
            .send()
            .await?;
        self.check_response(response).await?;
        Ok(())
    }
}
