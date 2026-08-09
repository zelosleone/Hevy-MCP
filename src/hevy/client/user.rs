use reqwest::Method;

use crate::hevy::client::{HevyClient, Result};
use crate::hevy::types::{UserInfo, UserInfoResponse};

impl HevyClient {
    pub async fn get_user_info(&self) -> Result<UserInfo> {
        let response = self.request(Method::GET, "/user/info").send().await?;
        let result: UserInfoResponse = self.handle_response(response).await?;
        Ok(result.data)
    }
}
