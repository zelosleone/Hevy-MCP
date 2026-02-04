use reqwest::Method;
use serde_json::json;

use crate::hevy::client::error::HevyError;
use crate::hevy::client::{HevyClient, Result, paginated_endpoint};
use crate::hevy::types::{
    CreateExerciseTemplateInput, CreateExerciseTemplateResponse, ExerciseTemplate,
    ExerciseTemplateResponse, ExerciseTemplatesResponse, PaginatedResponse,
};

impl HevyClient {
    pub async fn get_exercise_templates(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<ExerciseTemplatesResponse>> {
        let endpoint = paginated_endpoint("/exercise_templates", page, page_size);
        let response = self.request(Method::GET, &endpoint).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_exercise_template(&self, id: &str) -> Result<ExerciseTemplate> {
        let response = self
            .request(Method::GET, &format!("/exercise_templates/{id}"))
            .send()
            .await?;
        let result: ExerciseTemplateResponse = self.handle_response(response).await?;
        Ok(result.into_exercise_template())
    }

    pub async fn create_exercise_template(
        &self,
        input: CreateExerciseTemplateInput,
    ) -> Result<CreateExerciseTemplateResponse> {
        let response = self
            .request(Method::POST, "/exercise_templates")
            .json(&json!({ "exercise": input }))
            .send()
            .await?;
        let response = self.check_response(response).await?;
        let body = response.text().await?;

        if let Ok(parsed) = serde_json::from_str::<CreateExerciseTemplateResponse>(&body) {
            return Ok(parsed);
        }
        if let Ok(id) = serde_json::from_str::<String>(&body) {
            return Ok(CreateExerciseTemplateResponse { id });
        }
        if let Ok(id) = serde_json::from_str::<u64>(&body) {
            return Ok(CreateExerciseTemplateResponse { id: id.to_string() });
        }

        let trimmed = body.trim();
        if !trimmed.is_empty() {
            return Ok(CreateExerciseTemplateResponse {
                id: trimmed.to_string(),
            });
        }

        Err(HevyError::SerializationWithBody {
            message: "Unexpected create exercise template response".to_string(),
            body,
        })
    }
}
