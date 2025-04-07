use bdk::prelude::*;
use models::{deliberation_user::DeliberationUserCreateRequest, *};

use super::*;
use crate::{config, routes::Route, service::login_service::LoginService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub surveys: Resource<Vec<SurveyV2Summary>>,
    basic_info: Signal<DeliberationBasicInfoCreateRequest>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,

    #[allow(dead_code)]
    pub search_keyword: Signal<String>,
    #[allow(dead_code)]
    pub documents: Signal<Vec<ResourceFile>>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let basic_info = use_signal(|| DeliberationBasicInfoCreateRequest::default());
        let search_keyword = use_signal(|| "".to_string());

        let members = use_server_future(move || {
            let page = 1;
            let size = 100;
            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = OrganizationMember::get_client(endpoint)
                    .query(
                        org_id.unwrap().id,
                        OrganizationMemberQuery::new(size).with_page(page),
                    )
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 100;
            let keyword = search_keyword().clone();
            async move {
                let client = ResourceFile::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }

                if keyword.is_empty() {
                    let query = ResourceFileQuery::new(size).with_page(page);
                    client
                        .query(org_id.unwrap().id, query)
                        .await
                        .unwrap_or_default()
                        .items
                } else {
                    client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                        .unwrap_or_default()
                        .items
                }
            }
        })?;

        let surveys = use_server_future(move || {
            let page = 1;
            let size = 100;

            async move {
                let client = SurveyV2::get_client(&crate::config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }

                match client
                    .query(org_id.unwrap().id, SurveyV2Query::new(size).with_page(page))
                    .await
                {
                    Ok(res) => res.items,
                    Err(e) => {
                        tracing::error!("Failed to list surveys: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        //     use_effect({
        //     let mut basic_info = req
        //         .basic_infos
        //         .get(0)
        //         .unwrap_or(&DeliberationBasicInfoCreateRequest::default())
        //         .clone();

        //     let started_at = if basic_info.started_at == 0 {
        //         current_timestamp()
        //     } else {
        //         basic_info.started_at
        //     };

        //     let ended_at = if basic_info.ended_at == 0 {
        //         current_timestamp()
        //     } else {
        //         basic_info.ended_at
        //     };

        //     let v: Vec<OrganizationMemberSummary> = total_committees
        //         .clone()
        //         .into_iter()
        //         .filter(|member| {
        //             basic_info
        //                 .clone()
        //                 .users
        //                 .iter()
        //                 .any(|id| id.clone() == member.id)
        //         })
        //         .collect();

        //     let r: Vec<ResourceFile> = metadatas
        //         .clone()
        //         .into_iter()
        //         .filter(|resource| {
        //             basic_info
        //                 .clone()
        //                 .resources
        //                 .iter()
        //                 .any(|id| id.clone() == resource.id)
        //         })
        //         .map(|v| v.into())
        //         .collect();

        //     move || {
        //         basic_info.started_at = started_at;
        //         basic_info.ended_at = ended_at;
        //         ctrl.basic_info.set(basic_info.clone());
        //     }
        // });

        let ctrl = Self {
            lang,
            basic_info,
            members,
            metadatas,
            surveys,
            parent: use_context(),
            nav: use_navigator(),

            search_keyword,

            documents: use_signal(|| vec![]),
            committee_members: use_signal(|| vec![]),
        };

        // FIXME: anti-pattern
        // ctrl.committee_members.set(req.roles.clone());

        Ok(ctrl)
    }

    pub fn set_basic_info(&mut self, info: DeliberationBasicInfoCreateRequest) {
        self.basic_info.set(info);
    }

    pub fn get_basic_info(&self) -> DeliberationBasicInfoCreateRequest {
        (self.basic_info)()
    }

    pub async fn create_resource(&mut self, file: File) -> Result<()> {
        let metadata = self.create_metadata(file).await;

        match metadata {
            Ok(v) => {
                let mut basic_info = self.basic_info();

                basic_info.resources.push(v.id);
                self.basic_info.set(basic_info);
                self.metadatas.restart();
                Ok(())
            }
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub fn get_committees(&self) -> Vec<OrganizationMemberSummary> {
        let committees = self.committee_members();
        let members = self.members().unwrap_or_default();

        let d = members
            .clone()
            .into_iter()
            .filter(|member| {
                committees
                    .iter()
                    .any(|committee| committee.user_id == member.user_id)
            })
            .collect();

        d
    }

    pub fn get_selected_surveys(&self) -> Vec<SurveyV2Summary> {
        let total_surveys = self.surveys().unwrap_or_default();
        let basic_info = self.get_basic_info();
        let surveys = basic_info.clone().surveys;

        total_surveys
            .clone()
            .into_iter()
            .filter(|survey| surveys.iter().any(|id| id.clone() == survey.id))
            .collect()
    }

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let basic_info = self.get_basic_info();
        let roles = basic_info.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.id))
            .collect()
    }

    pub fn get_selected_resources(&self) -> Vec<ResourceFile> {
        let metadatas = self.metadatas().unwrap_or_default();
        let resources = self.get_basic_info().resources;

        metadatas
            .clone()
            .into_iter()
            .filter(|resource| resources.iter().any(|id| id.clone() == resource.id))
            .map(|v| v.into())
            .collect()
    }

    pub fn add_resource(&mut self, resource: ResourceFile) {
        let mut basic_info = self.basic_info();
        basic_info.resources.push(resource.id);
        self.basic_info.set(basic_info);
    }

    pub fn delete_resource(&mut self, id: i64) {
        let mut basic_info = self.basic_info();
        basic_info.resources.retain(|doc| doc.clone() != id);
        self.basic_info.set(basic_info);
    }

    pub async fn create_metadata(&self, file: File) -> Result<ResourceFile> {
        let user: LoginService = use_context();
        let org = user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::ResourceFile::get_client(&config::get().api_url);

        client
            .create(
                org_id,
                file.name.clone(),
                None,
                None,
                None,
                None,
                None,
                vec![file],
            )
            .await
    }

    pub fn back(&mut self) {
        self.parent.save_basic_info(self.basic_info());
        self.nav
            .replace(Route::CompositionPanel { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_basic_info(self.basic_info());
        self.parent.temporary_save().await;
    }

    pub fn next(&mut self) {
        self.parent.save_basic_info(self.basic_info());
        self.nav
            .push(Route::DeliberationSampleSurveySettingPage { lang: self.lang });
    }
}
