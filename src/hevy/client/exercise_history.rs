use reqwest::Method;

use crate::hevy::client::{HevyClient, Result};
use crate::hevy::types::ExerciseHistoryResponse;

impl HevyClient {
    pub async fn get_exercise_history(
        &self,
        exercise_template_id: &str,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<ExerciseHistoryResponse> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(start_date) = start_date {
            params.push(("start_date", start_date));
        }
        if let Some(end_date) = end_date {
            params.push(("end_date", end_date));
        }

        let response = self
            .request(
                Method::GET,
                &format!("/exercise_history/{exercise_template_id}"),
            )
            .query(&params)
            .send()
            .await?;
        self.handle_response(response).await
    }
}
