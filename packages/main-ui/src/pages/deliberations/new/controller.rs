use bdk::prelude::*;
use by_macros::DioxusController;
use chrono::Utc;
use models::{
    deliberation::Deliberation, deliberation_user::DeliberationUserCreateRequest,
    discussions::DiscussionCreateRequest, step::StepCreateRequest, step_type::StepType, *,
};

use crate::{
    config,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MeetingInfo {
    pub meeting_type: MeetingType,
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub users: i64,
}

// TODO: refactor this @henry
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliberationSequence {
    pub name: String,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
    pub step_type: Option<StepType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliberationInformation {
    pub deliberation_type: Option<ProjectArea>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub documents: Vec<ResourceFile>,
    pub projects: Vec<SurveyV2Summary>,
}

use super::i18n::DeliberationNewTranslate;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    popup_service: Signal<PopupService>,
    current_step: Signal<CurrentStep>,
    user: LoginService,

    //step 1
    deliberation_sequences: Signal<Vec<StepCreateRequest>>,

    //step 2
    total_fields: Signal<Vec<String>>,
    deliberation_informations: Signal<DeliberationInformation>,
    pub surveys: Resource<Vec<SurveyV2Summary>>,
    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub search_keyword: Signal<String>,

    //step 3
    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committees: Signal<Vec<DeliberationUserCreateRequest>>,

    //step 4
    pub panels: Resource<Vec<PanelV2Summary>>,
    pub selected_panels: Signal<Vec<PanelV2Summary>>,
    // total_attributes: Signal<Vec<AttributeResponse>>,

    //step 5
    pub discussions: Signal<Vec<MeetingInfo>>,
    pub discussion_resources: Signal<Vec<ResourceFileSummary>>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentStep {
    PublicOpinionComposition, // 공론 구성 및 기간
    InputInformation,         //필수정보 입력
    CommitteeComposition,     //공론 위원회 구성
    PanelComposition,         //참여자 패널 구성
    DiscussionSetting,        //토론 설정
    Preview,                  //전체 미리보기
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let timestamp = Utc::now().timestamp();
        let user: LoginService = use_context();
        let popup_service: PopupService = use_context();
        let translates: DeliberationNewTranslate = translate(&lang.clone());
        let search_keyword = use_signal(|| "".to_string());

        let client = SurveyV2::get_client(&crate::config::get().api_url);
        let surveys = use_server_future(move || {
            let page = 1;
            let size = 20;
            let client = client.clone();

            async move {
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

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 20;
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

        let members = use_server_future(move || {
            let page = 1;
            let size = 20;
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

        let panels = use_server_future(move || {
            let page = 1;
            let size = 20;
            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = PanelV2::get_client(endpoint)
                    .query(org_id.unwrap().id, PanelV2Query::new(size).with_page(page))
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let ctrl = Self {
            user,
            popup_service: use_signal(|| popup_service),
            current_step: use_signal(|| CurrentStep::PublicOpinionComposition),
            deliberation_sequences: use_signal(|| {
                // TODO: refactor this @henry
                vec![
                    StepCreateRequest {
                        step_type: StepType::GeneralPost,
                        name: translates.information_provided.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::VideoConference,
                        name: translates.discussion_and_deliberation.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Post,
                        name: translates.derive_opinions.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Vote,
                        name: translates.reach_consensus.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Report,
                        name: translates.analysis_result.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                ]
            }),

            // step 2
            // TODO: refactor this @henry
            total_fields: use_signal(|| {
                vec![
                    "경제".to_string(),
                    "사회".to_string(),
                    "환경".to_string(),
                    "교육".to_string(),
                    "문화".to_string(),
                    "노동".to_string(),
                    "도시".to_string(),
                    "기술".to_string(),
                    "보건".to_string(),
                    "정치".to_string(),
                ]
            }),
            deliberation_informations: use_signal(|| DeliberationInformation {
                deliberation_type: None,
                title: None,
                description: None,
                documents: vec![],
                projects: vec![],
            }),
            surveys,
            search_keyword,
            metadatas,
            members,
            panels,
            selected_panels: use_signal(|| vec![]),
            committees: use_signal(|| vec![]),

            discussions: use_signal(|| {
                vec![MeetingInfo {
                    meeting_type: models::prelude::MeetingType::Offline,
                    title: "".to_string(),
                    start_date: timestamp,
                    end_date: timestamp,
                    description: "".to_string(),
                    users: 20,
                }]
            }),
            discussion_resources: use_signal(|| vec![]),
            //FIXME: fix to connect api
            // total_attributes: use_signal(|| {
            //     vec![
            //         AttributeResponse {
            //             id: "1".to_string(),
            //             name: Some("직업".to_string()),
            //             attribute: vec![AttributeItemInfo {
            //                 id: "1".to_string(),
            //                 name: "개발자".to_string(),
            //             }],
            //         },
            //         AttributeResponse {
            //             id: "2".to_string(),
            //             name: Some("성별".to_string()),
            //             attribute: vec![AttributeItemInfo {
            //                 id: "1".to_string(),
            //                 name: "여성".to_string(),
            //             }],
            //         },
            //         AttributeResponse {
            //             id: "3".to_string(),
            //             name: Some("나이".to_string()),
            //             attribute: vec![
            //                 AttributeItemInfo {
            //                     id: "1".to_string(),
            //                     name: "20대".to_string(),
            //                 },
            //                 AttributeItemInfo {
            //                     id: "2".to_string(),
            //                     name: "30대".to_string(),
            //                 },
            //                 AttributeItemInfo {
            //                     id: "3".to_string(),
            //                     name: "40대".to_string(),
            //                 },
            //                 AttributeItemInfo {
            //                     id: "4".to_string(),
            //                     name: "50대".to_string(),
            //                 },
            //                 AttributeItemInfo {
            //                     id: "5".to_string(),
            //                     name: "60대".to_string(),
            //                 },
            //             ],
            //         },
            //         AttributeResponse {
            //             id: "4".to_string(),
            //             name: Some("학력".to_string()),
            //             attribute: vec![AttributeItemInfo {
            //                 id: "1".to_string(),
            //                 name: "대학원".to_string(),
            //             }],
            //         },
            //         AttributeResponse {
            //             id: "5".to_string(),
            //             name: Some("거주지".to_string()),
            //             attribute: vec![AttributeItemInfo {
            //                 id: "1".to_string(),
            //                 name: "서울".to_string(),
            //             }],
            //         },
            //         AttributeResponse {
            //             id: "6".to_string(),
            //             name: Some("국적".to_string()),
            //             attribute: vec![AttributeItemInfo {
            //                 id: "1".to_string(),
            //                 name: "국내".to_string(),
            //             }],
            //         },
            //     ]
            // }),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    // pub fn get_total_attributes(&self) -> Vec<AttributeResponse> {
    //     (self.total_attributes)()
    // }

    pub fn update_deliberation_info(&mut self, index: usize, opinion: StepCreateRequest) {
        let mut sequences = self.get_deliberation_sequences();
        sequences[index] = opinion;
        self.deliberation_sequences.set(sequences);
    }

    pub fn delete_deliberation_info(&mut self, index: usize) {
        let mut sequences = self.get_deliberation_sequences();
        sequences.remove(index);
        self.deliberation_sequences.set(sequences);
    }

    pub fn add_deliberation_info(&mut self) {
        let mut sequences = self.get_deliberation_sequences();
        sequences.push(StepCreateRequest {
            step_type: StepType::GeneralPost,
            name: "".to_string(),
            started_at: 0,
            ended_at: 0,
        });
        self.deliberation_sequences.set(sequences);
    }

    pub fn check_deliberation_info(&self) -> bool {
        let sequences = &self.get_deliberation_sequences();

        for sequence in sequences {
            if sequence.started_at == 0 || sequence.ended_at == 0 {
                return false;
            }

            if sequence.started_at > sequence.ended_at {
                return false;
            }
        }

        true
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        tracing::debug!("informations: {:?}", self.deliberation_informations());
        self.current_step.set(step);
    }

    pub fn get_deliberation_sequences(&self) -> Vec<StepCreateRequest> {
        (self.deliberation_sequences)()
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn use_service() -> Self {
        use_context()
    }

    // step 2
    pub fn get_total_fields(&self) -> Vec<String> {
        (self.total_fields)()
    }

    pub fn get_deliberation_informations(&self) -> DeliberationInformation {
        (self.deliberation_informations)()
    }

    pub fn update_deliberation_information(&mut self, information: DeliberationInformation) {
        self.deliberation_informations.set(information);
    }

    pub async fn create_resource(&mut self, file: File) -> Result<()> {
        let metadata = self.create_metadata(file).await;

        match metadata {
            Ok(v) => {
                let mut info = (self.deliberation_informations)();
                let mut documents = info.documents;
                documents.push(v);
                info.documents = documents;
                self.deliberation_informations.set(info);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub fn add_resource(&mut self, resource: ResourceFile) {
        let mut info = (self.deliberation_informations)();
        let mut documents = info.documents;
        documents.push(resource);
        info.documents = documents;
        self.deliberation_informations.set(info);
    }

    pub fn delete_resource(&mut self, id: i64) {
        let mut info = (self.deliberation_informations)();
        let mut documents = info.documents;
        documents.retain(|doc| doc.id != id);
        info.documents = documents;
        self.deliberation_informations.set(info);
    }

    pub fn set_projects(&mut self, surveys: Vec<SurveyV2Summary>) {
        let mut info = (self.deliberation_informations)();
        info.projects = surveys;
        self.deliberation_informations.set(info);
    }

    pub fn resources(&self) -> Vec<ResourceFile> {
        (self.deliberation_informations)().documents
    }

    pub fn selected_surveys(&self) -> Vec<SurveyV2Summary> {
        (self.deliberation_informations)().projects
    }

    //step 3
    pub fn get_committees(&self) -> Vec<DeliberationUserCreateRequest> {
        (self.committees)()
    }

    pub fn add_committee(&mut self, committee: DeliberationUserCreateRequest) {
        self.committees.push(committee);
    }

    pub fn remove_committee(&mut self, user_id: i64, role: Role) {
        self.committees
            .retain(|committee| !(committee.user_id == user_id && committee.role == role));
    }

    pub fn clear_committee(&mut self, role: Role) {
        self.committees
            .retain(|committee| !(committee.role == role));
    }

    //step 4
    pub fn get_selected_panels(&self) -> Vec<PanelV2Summary> {
        (self.selected_panels)()
    }

    pub fn add_selected_panel(&mut self, panel: PanelV2Summary) {
        self.selected_panels.push(panel);
    }

    pub fn remove_selected_panel(&mut self, panel_id: i64) {
        self.selected_panels.retain(|panel| !(panel.id == panel_id));
    }

    pub fn clear_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
    }

    pub fn change_selected_panel_by_index(&mut self, index: usize, value: u64) {
        self.selected_panels.with_mut(|panels| {
            panels[index].user_count = value;
        });
    }

    //step 5
    pub fn get_discussions(&self) -> Vec<MeetingInfo> {
        (self.discussions)()
    }

    pub fn add_discussion(&mut self) {
        let timestamp = Utc::now().timestamp();

        self.discussions.push(MeetingInfo {
            meeting_type: models::prelude::MeetingType::Offline,
            title: "".to_string(),
            start_date: timestamp,
            end_date: timestamp,
            description: "".to_string(),
            users: 20,
        });
    }

    pub fn remove_discussion(&mut self, index: usize) {
        self.discussions.remove(index);
    }

    pub fn update_discussion(&mut self, index: usize, discussion: MeetingInfo) {
        let mut discussions = (self.discussions)();
        discussions[index] = discussion;
        self.discussions.set(discussions);
    }

    pub fn get_discussion_resources(&self) -> Vec<ResourceFileSummary> {
        (self.discussion_resources)()
    }

    pub fn remove_discussion_resource(&mut self, id: i64) {
        self.discussion_resources
            .retain(|resource| !(resource.id == id));
    }

    pub fn clear_discussion_resource(&mut self) {
        self.discussion_resources.set(vec![]);
    }

    pub async fn create_metadata(&self, file: File) -> Result<ResourceFile> {
        let org = self.user.get_selected_org();
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

    pub async fn create_discussion_resource(&mut self, file: File) -> Result<()> {
        let metadata = self.create_metadata(file).await;

        match metadata {
            Ok(v) => {
                let mut info = (self.discussion_resources)();
                info.push(v.into());
                self.discussion_resources.set(info);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    // pub fn open_create_panel_modal(&self, lang: Language, translates: CompositionPanelTranslate) {
    //     let mut popup_service = (self.popup_service)().clone();
    //     let attributes = self.total_attributes;
    //     popup_service
    //         .open(rsx! {
    //             CreateNewPanelModal {
    //                 attributes: attributes.clone(),
    //                 lang: lang.clone(),
    //                 onsave: move |panel_name: String| {
    //                     tracing::debug!("panel name: {panel_name}");
    //                 },
    //                 onclick: {
    //                     move |panel_name: String| {
    //                         tracing::debug!("panel name: {panel_name}");
    //                         popup_service
    //                             .open(rsx! {
    //                                 AddAttributeModal {
    //                                     lang,
    //                                     onclose: move |_e: MouseEvent| {
    //                                         popup_service.close();
    //                                     },
    //                                 }
    //                             })
    //                             .with_id("add_attribute")
    //                             .with_title(translates.add_attribute);
    //                     }
    //                 },
    //                 onclose: move |_e: MouseEvent| {
    //                     popup_service.close();
    //                 },
    //             }
    //         })
    //         .with_id("create_panel")
    //         .with_title(translates.create_panel);
    // }

    // pub fn open_send_alerm_modal(&self, lang: Language) {
    //     let translates: PreviewTranslate = translate(&lang);
    //     let mut popup_service = (self.popup_service)().clone();
    //     let ctrl = self.clone();
    //     popup_service
    //         .open(rsx! {
    //             SendAlertModal {
    //                 lang,
    //                 onclose: move |_e: MouseEvent| {
    //                     popup_service.close();
    //                 },
    //                 onclick: move |_| {
    //                     async move {
    //                         match ctrl.create_deliberation().await {
    //                             Ok(_) => {
    //                                 popup_service.close();
    //                             }
    //                             Err(e) => {
    //                                 tracing::error!("Create Deliberation Failed Reason: {:?}", e);
    //                             }
    //                         }
    //                     }
    //                 },
    //             }
    //         })
    //         .with_id("send_alert")
    //         .with_title(translates.send_alerm);
    // }

    pub fn get_period(&self) -> (u64, u64) {
        let sequences = self.get_deliberation_sequences();
        if sequences.is_empty() {
            return (0, 0);
        }
        let mut start = sequences[0].started_at;
        let mut end = sequences[sequences.len() - 1].ended_at;
        for sequence in sequences.iter() {
            if sequence.started_at < start {
                start = sequence.started_at;
            }

            if sequence.ended_at > end {
                end = sequence.ended_at;
            }
        }

        (start as u64, end as u64)
    }

    pub async fn create_deliberation(&self, lang: Language) -> Result<()> {
        let navigator = use_navigator();

        let sequences = self.get_deliberation_sequences();
        let informations = self.get_deliberation_informations();
        let selected_panels = self.get_selected_panels();
        let committees = self.get_committees();
        let meetings = self.get_discussions();
        let discussion_resources = self.get_discussion_resources();

        let endpoint = crate::config::get().api_url;
        let client = Deliberation::get_client(endpoint);

        let org_id = self.user.get_selected_org();
        if org_id.is_none() {
            tracing::error!("Organization ID is missing");
            return Err(ApiError::OrganizationNotFound);
        }

        let mut discussions: Vec<DiscussionCreateRequest> = vec![];
        let deliberation_time = self.get_deliberation_time(sequences.clone());

        for meeting in meetings.clone() {
            discussions.push(DiscussionCreateRequest {
                started_at: meeting.start_date,
                ended_at: meeting.end_date,
                name: meeting.title.clone(),
                description: meeting.description.clone(),
                resources: discussion_resources
                    .iter()
                    .map(|resource| resource.id)
                    .collect(),
            });
        }

        match client
            .create(
                org_id.unwrap().id,
                deliberation_time.0,
                deliberation_time.1,
                informations.deliberation_type.unwrap_or_default(),
                informations.title.unwrap_or_default(),
                informations.description.unwrap_or_default(),
                informations
                    .documents
                    .iter()
                    .map(|document| document.id)
                    .collect(),
                informations
                    .projects
                    .iter()
                    .map(|project| project.id)
                    .collect(),
                committees,
                selected_panels.iter().map(|panel| panel.id).collect(),
                sequences,
                vec![],
                discussions,
            )
            .await
        {
            Ok(_) => {
                btracing::debug!("success to create deliberation");
                navigator.push(Route::DeliberationPage { lang });
                Ok(())
            }
            Err(e) => {
                btracing::error!("failed to create deliberation: {}", e.translate(&lang));
                return Err(e);
            }
        }
    }

    pub fn get_deliberation_time(&self, steps: Vec<StepCreateRequest>) -> (i64, i64) {
        let started_at = steps.iter().map(|s| s.started_at).min().unwrap_or(0);
        let ended_at = steps.iter().map(|s| s.ended_at).max().unwrap_or(0);

        (started_at, ended_at)
    }
}
