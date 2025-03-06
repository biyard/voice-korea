pub type Result<T> = std::result::Result<T, ServerFnError>;
use std::collections::HashMap;

use dioxus::prelude::*;
use models::{
    prelude::{
        CreateMemberRequest, InviteMemberRequest, ListMemberResponse, MemberActionRequest,
        MemberByIdActionRequest, MemberSummary, UpdateMemberRequest,
    },
    OrganizationMember,
};

use super::{login_service::LoginService, organization_api::OrganizationApi};
use crate::utils::api::ReqwestClient;

#[derive(Debug, Clone, Copy)]
pub struct MemberApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl MemberApi {
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

    pub async fn create_member(&self, req: CreateMemberRequest) -> Result<()> {
        // let token = self.get_token();
        // let id = self.get_organization_id();

        // let client = ReqwestClient::new()?;

        // let res = client
        //     .post(&format!("/members/v1"))
        //     .header("Authorization", token)
        //     .header("x-organization", id)
        //     .json(&MemberActionRequest::Create(req))
        //     .send()
        //     .await?;

        // let _res = res.error_for_status()?;
        Ok(())
    }

    pub async fn update_member(&self, user_id: String, req: UpdateMemberRequest) -> Result<()> {
        // let token = self.get_token();
        // let id = self.get_organization_id();

        // let client = ReqwestClient::new()?;

        // let _res = client
        //     .post(format!("/members/v1/{}", user_id).as_str())
        //     .header("Authorization", token)
        //     .header("x-organization", id)
        //     .json(&MemberByIdActionRequest::Update(req))
        //     .send()
        //     .await?;

        Ok(())
    }

    pub async fn remove_member(&self, user_id: String) -> Result<()> {
        // let token = self.get_token();
        // let id = self.get_organization_id();

        // let client = ReqwestClient::new()?;

        // let _res = client
        //     .post(format!("/members/v1/{}", user_id).as_str())
        //     .header("Authorization", token)
        //     .header("x-organization", id)
        //     .json(&MemberByIdActionRequest::Delete)
        //     .send()
        //     .await?;

        Ok(())
    }

    pub async fn list_members(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<ListMemberResponse> {
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
            .get(&format!("/members/v1"))
            .query(&params)
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub async fn get_member(&self, user_id: String) -> Result<MemberSummary> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(format!("/members/v1/{}", user_id).as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub async fn invite_member(
        &self,
        InviteMemberRequest {
            email,
            name,
            group,
            role,
            ..
        }: InviteMemberRequest,
    ) -> Result<()> {
        let endpoint = crate::config::get().api_url;
        OrganizationMember::get_client(endpoint)
            .create(org_id, name, role, None, email)
            .await?;

        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/members/v1/invite").as_str())
            .header("Authorization", token)
            .header("x-organization", id)
            .json(&req)
            .send()
            .await?;

        Ok(())
    }

    pub fn get_organization_id(&self) -> i64 {
        self.login_service.get_selected_org().unwrap_or_default().id
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
