use reqwest::Method;
use serde_json::json;

use crate::hevy::client::{HevyClient, Result, paginated_endpoint};
use crate::hevy::types::{
    PaginatedResponse, Workout, WorkoutCountResponse, WorkoutEventsResponse, WorkoutInput,
    WorkoutResponse, WorkoutsResponse,
};

impl HevyClient {
    pub async fn get_workouts(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<WorkoutsResponse>> {
        let endpoint = paginated_endpoint("/workouts", page, page_size);
        let response = self.request(Method::GET, &endpoint).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_workouts_count(&self) -> Result<WorkoutCountResponse> {
        let response = self.request(Method::GET, "/workouts/count").send().await?;
        self.handle_response(response).await
    }

    pub async fn get_workout_events(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
        since: Option<String>,
    ) -> Result<WorkoutEventsResponse> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("pageSize", page_size.to_string()));
        }
        if let Some(since) = since {
            params.push(("since", since));
        }

        let response = self
            .request(Method::GET, "/workouts/events")
            .query(&params)
            .send()
            .await?;
        self.handle_response(response).await
    }

    pub async fn get_workout(&self, id: &str) -> Result<Workout> {
        let response = self
            .request(Method::GET, &format!("/workouts/{id}"))
            .send()
            .await?;
        let result: WorkoutResponse = self.handle_response(response).await?;
        Ok(result.into_workout())
    }

    pub async fn create_workout(&self, workout: WorkoutInput) -> Result<Workout> {
        let response = self
            .request(Method::POST, "/workouts")
            .json(&json!({"workout": workout}))
            .send()
            .await?;
        let result: WorkoutResponse = self.handle_response(response).await?;
        Ok(result.into_workout())
    }

    pub async fn update_workout(&self, id: &str, workout: WorkoutInput) -> Result<Workout> {
        let response = self
            .request(Method::PUT, &format!("/workouts/{id}"))
            .json(&json!({"workout": workout}))
            .send()
            .await?;
        let result: WorkoutResponse = self.handle_response(response).await?;
        Ok(result.into_workout())
    }
}
