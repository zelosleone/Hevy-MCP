use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::hevy::client::error::{HevyError, Result};

const HEVY_API_BASE_URL: &str = "https://api.hevyapp.com/v1";

pub(crate) fn paginated_endpoint(path: &str, page: Option<u32>, page_size: Option<u32>) -> String {
    match (page, page_size) {
        (Some(p), Some(ps)) => format!("{}?page={}&pageSize={}", path, p, ps),
        _ => path.to_string(),
    }
}

#[derive(Clone)]
pub struct HevyClient {
    client: Client,
    api_key: String,
}

impl HevyClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }

    pub(crate) fn request(&self, method: Method, endpoint: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{HEVY_API_BASE_URL}{endpoint}"))
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
    }

    pub(crate) async fn check_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response> {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(response),
            StatusCode::UNAUTHORIZED => Err(HevyError::Unauthorized),
            StatusCode::NOT_FOUND => {
                let body = response.text().await.unwrap_or_default();
                Err(HevyError::NotFound(body))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(HevyError::RateLimited),
            status => {
                let body = response.text().await.unwrap_or_default();
                Err(HevyError::Api {
                    status: status.as_u16(),
                    message: body,
                })
            }
        }
    }

    pub(crate) async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let response = self.check_response(response).await?;
        let body = response.text().await?;
        from_str(&body).map_err(|err| HevyError::SerializationWithBody {
            message: err.to_string(),
            body,
        })
    }
}
