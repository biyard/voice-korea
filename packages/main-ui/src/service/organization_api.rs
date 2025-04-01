pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::OrganizationMemberResponse;

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::login_service::LoginService;

#[derive(Debug, Clone, Copy)]
pub struct OrganizationApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organizations: Signal<Vec<OrganizationMemberResponse>>,
    pub selected_organization_id: Signal<String>,
}

impl OrganizationApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let srv = Self {
            endpoint: use_signal(|| {
                format!(
                    "{}",
                    option_env!("API_URL").unwrap_or("https://voice-korea-api.dev.voice-korea.com")
                )
            }),
            login_service,
            organizations: use_signal(|| vec![]),
            selected_organization_id: use_signal(|| "".to_string()),
        };
        use_context_provider(|| srv);
    }

    pub async fn list_organizations(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<OrganizationMemberResponse>> {
        let token = self.get_token();

        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let client = ReqwestClient::new()?;

        let res = client
            .get("/organizations/v1")
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let organizations = res.json().await?;
        Ok(organizations)
    }

    pub fn set_organization(&mut self, organizations: Vec<OrganizationMemberResponse>) {
        self.organizations.set(organizations.clone());

        if organizations.len() != 0 {
            self.selected_organization_id
                .set(organizations.get(0).unwrap().organization_id.clone());
        }
    }

    pub fn get_organizations(&self) -> Vec<OrganizationMemberResponse> {
        (self.organizations)()
    }

    pub fn set_selected_organization_id(&mut self, id: String) {
        self.selected_organization_id.set(id);
    }

    pub fn get_selected_organization_id(&self) -> String {
        (self.selected_organization_id)()
    }

    pub fn get_token(&self) -> String {
        let cookie = if cfg!(feature = "web") {
            self.login_service
                .get_cookie_value()
                .unwrap_or_else(|| "".to_string())
        } else {
            "".to_string()
        };

        let token = cookie.replace('"', "");
        let format_cookie = format!("token={token}");
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        token
    }
}
