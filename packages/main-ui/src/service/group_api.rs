pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreateGroupRequest, GroupActionRequest, GroupByIdActionRequest, GroupResponse,
    TeamMemberRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct GroupApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl GroupApi {
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

    pub async fn create_group(&self, req: CreateGroupRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/groups/v1"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&GroupActionRequest::Create(req))
            .send()
            .await?;

        let _res = res.error_for_status();

        Ok(())
    }

    pub async fn add_team_member(&self, group_id: i64, req: TeamMemberRequest) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/groups/v1/{group_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&GroupByIdActionRequest::AddTeamMember(req))
            .send()
            .await?;

        let _res = res.error_for_status();

        Ok(())
    }

    pub async fn remove_team_member(&self, group_id: String, member_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/groups/v1/{group_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&GroupByIdActionRequest::RemoveTeamMember(member_id))
            .send()
            .await?;

        let _res = res.error_for_status();

        Ok(())
    }

    pub async fn update_group_name(&self, group_id: String, group_name: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/groups/v1/{}", group_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&GroupByIdActionRequest::UpdateName(group_name))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove_group(&self, group_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/groups/v1/{}", group_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&GroupByIdActionRequest::Delete)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_groups(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<GroupResponse>> {
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
            .get(&format!("/groups/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let groups = res.json().await?;
        Ok(groups)
    }

    pub async fn get_group(&self, group_id: String) -> Result<GroupResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/groups/v1/{group_id}"))
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
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
