pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreateOpinionRequest, OpinionActionRequest, OpinionByIdActionRequest, OpinionResponse,
    PanelInfo, ProjectField, ProjectStatus, UpdateOpinionRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct OpinionApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl OpinionApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let organization_service: OrganizationApi = use_context();
        let srv = Self {
            endpoint: use_signal(|| crate::config::get().api_url.to_string()),
            login_service,
            organization_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn update_opinion(
        &self,
        project_id: String,
        req: UpdateOpinionRequest,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionByIdActionRequest::Update(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn create_opinion(&self, req: CreateOpinionRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/public-opinions/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionActionRequest::Create(req))
            .send()
            .await?;

        let _res = res.error_for_status()?;

        Ok(())
    }

    pub async fn remove_opinion(&self, project_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionByIdActionRequest::Delete)
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_project_status(
        &self,
        project_id: String,
        status: ProjectStatus,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionByIdActionRequest::UpdateStatus(status))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_panels(&self, project_id: String, panels: Vec<PanelInfo>) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionByIdActionRequest::UpdatePanels(panels))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_project_type(
        &self,
        project_id: String,
        project_type: ProjectField,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&OpinionByIdActionRequest::UpdateProjectType(project_type))
            .send()
            .await?;
        Ok(())
    }

    pub async fn search_opinion(
        &self,
        keyword: String,
    ) -> Result<CommonQueryResponse<OpinionResponse>> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;
        let mut params = HashMap::new();
        params.insert("keyword", keyword);

        let res = client
            .get(&format!("/public-opinions/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .query(&params)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn get_opinion(&self, project_id: String) -> Result<OpinionResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/public-opinions/v1/{project_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn list_opinions(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<OpinionResponse>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/public-opinions/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let opinions = res.json().await?;
        Ok(opinions)
    }

    pub fn get_organization_id(&self) -> String {
        let id = self.organization_service.get_selected_organization_id();
        id
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
