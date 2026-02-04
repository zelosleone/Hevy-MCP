use reqwest::Method;
use serde_json::json;

use crate::hevy::client::{HevyClient, Result, paginated_endpoint};
use crate::hevy::types::{
    PaginatedResponse, RoutineFolder, RoutineFolderInput, RoutineFolderResponse,
    RoutineFoldersResponse,
};

impl HevyClient {
    pub async fn get_routine_folders(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<RoutineFoldersResponse>> {
        let endpoint = paginated_endpoint("/routine_folders", page, page_size);
        let response = self.request(Method::GET, &endpoint).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_routine_folder(&self, id: &str) -> Result<RoutineFolder> {
        let response = self
            .request(Method::GET, &format!("/routine_folders/{id}"))
            .send()
            .await?;
        let result: RoutineFolderResponse = self.handle_response(response).await?;
        Ok(result.into_routine_folder())
    }

    pub async fn create_routine_folder(&self, folder: RoutineFolderInput) -> Result<RoutineFolder> {
        let response = self
            .request(Method::POST, "/routine_folders")
            .json(&json!({"routine_folder": folder}))
            .send()
            .await?;
        let result: RoutineFolderResponse = self.handle_response(response).await?;
        Ok(result.into_routine_folder())
    }
}
