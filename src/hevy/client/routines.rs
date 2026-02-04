use reqwest::Method;
use serde_json::json;

use crate::hevy::client::{HevyClient, Result, paginated_endpoint};
use crate::hevy::types::{
    PaginatedResponse, Routine, RoutineInput, RoutineResponse, RoutinesResponse,
};

impl HevyClient {
    pub async fn get_routines(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<RoutinesResponse>> {
        let endpoint = paginated_endpoint("/routines", page, page_size);
        let response = self.request(Method::GET, &endpoint).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_routine(&self, id: &str) -> Result<Routine> {
        let response = self
            .request(Method::GET, &format!("/routines/{id}"))
            .send()
            .await?;
        let result: RoutineResponse = self.handle_response(response).await?;
        Ok(result.routine)
    }

    pub async fn create_routine(&self, routine: RoutineInput) -> Result<Routine> {
        let response = self
            .request(Method::POST, "/routines")
            .json(&json!({"routine": routine}))
            .send()
            .await?;
        let result: RoutineResponse = self.handle_response(response).await?;
        Ok(result.routine)
    }

    pub async fn update_routine(&self, id: &str, routine: RoutineInput) -> Result<Routine> {
        let response = self
            .request(Method::PUT, &format!("/routines/{id}"))
            .json(&json!({"routine": routine}))
            .send()
            .await?;
        let result: RoutineResponse = self.handle_response(response).await?;
        Ok(result.routine)
    }
}
