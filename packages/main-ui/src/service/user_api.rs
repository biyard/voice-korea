pub type Result<T> = std::result::Result<T, ServerFnError>;

use crate::utils::api::ReqwestClient;
use dioxus::prelude::*;
use models::prelude::{AuthActionRequest, ResetParams, SignUpParams};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct UserApi {
    pub endpoint: Signal<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignupRequest {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetRequest {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

impl UserApi {
    pub fn init() {
        let srv = Self {
            endpoint: use_signal(|| crate::config::get().api_url.to_string()),
        };
        use_context_provider(|| srv);
    }

    pub async fn login_user(&self, req: LoginRequest) -> Result<String> {
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/v1/users/login"))
            .json(&req)
            .send()
            .await?;

        let res = res.error_for_status()?;

        Ok(res.text().await?)
    }

    pub async fn signup_user(&self, req: SignupRequest) -> Result<()> {
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/v1/users"))
            .json(&AuthActionRequest::SignUp(SignUpParams {
                auth_id: req.auth_id,
                auth_value: req.auth_value,
                email: req.email,
                password: req.password,
            }))
            .send()
            .await?;

        let _ = res.error_for_status()?;

        Ok(())
    }

    pub async fn reset_password(&self, req: ResetRequest) -> Result<()> {
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/v1/users"))
            .json(&AuthActionRequest::Reset(ResetParams {
                auth_id: req.auth_id,
                auth_value: req.auth_value,
                email: req.email,
                password: req.password,
            }))
            .send()
            .await?;

        let _ = res.error_for_status()?;

        Ok(())
    }
}
