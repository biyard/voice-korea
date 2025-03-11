use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{deliberation::Deliberation, step::StepCreateRequest, step_type::StepType, *};

use crate::{
    config,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{
    composition_panel::{AddAttributeModal, CreateNewPanelModal},
    i18n::{CompositionPanelTranslate, OpinionNewTranslate, PreviewTranslate},
    preview::SendAlertModal,
};

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

    //step 4
    total_attributes: Signal<Vec<AttributeResponse>>,
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
        let user: LoginService = use_context();
        let popup_service: PopupService = use_context();
        let translates: OpinionNewTranslate = translate(&lang.clone());

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
            //FIXME: fix to connect api
            total_attributes: use_signal(|| {
                vec![
                    AttributeResponse {
                        id: "1".to_string(),
                        name: Some("직업".to_string()),
                        attribute: vec![AttributeItemInfo {
                            id: "1".to_string(),
                            name: "개발자".to_string(),
                        }],
                    },
                    AttributeResponse {
                        id: "2".to_string(),
                        name: Some("성별".to_string()),
                        attribute: vec![AttributeItemInfo {
                            id: "1".to_string(),
                            name: "여성".to_string(),
                        }],
                    },
                    AttributeResponse {
                        id: "3".to_string(),
                        name: Some("나이".to_string()),
                        attribute: vec![
                            AttributeItemInfo {
                                id: "1".to_string(),
                                name: "20대".to_string(),
                            },
                            AttributeItemInfo {
                                id: "2".to_string(),
                                name: "30대".to_string(),
                            },
                            AttributeItemInfo {
                                id: "3".to_string(),
                                name: "40대".to_string(),
                            },
                            AttributeItemInfo {
                                id: "4".to_string(),
                                name: "50대".to_string(),
                            },
                            AttributeItemInfo {
                                id: "5".to_string(),
                                name: "60대".to_string(),
                            },
                        ],
                    },
                    AttributeResponse {
                        id: "4".to_string(),
                        name: Some("학력".to_string()),
                        attribute: vec![AttributeItemInfo {
                            id: "1".to_string(),
                            name: "대학원".to_string(),
                        }],
                    },
                    AttributeResponse {
                        id: "5".to_string(),
                        name: Some("거주지".to_string()),
                        attribute: vec![AttributeItemInfo {
                            id: "1".to_string(),
                            name: "서울".to_string(),
                        }],
                    },
                    AttributeResponse {
                        id: "6".to_string(),
                        name: Some("국적".to_string()),
                        attribute: vec![AttributeItemInfo {
                            id: "1".to_string(),
                            name: "국내".to_string(),
                        }],
                    },
                ]
            }),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_total_attributes(&self) -> Vec<AttributeResponse> {
        (self.total_attributes)()
    }

    pub fn update_opinion_info(&mut self, index: usize, opinion: StepCreateRequest) {
        let mut sequences = self.get_deliberation_sequences();
        sequences[index] = opinion;
        self.deliberation_sequences.set(sequences);
    }

    pub fn delete_opinion_info(&mut self, index: usize) {
        let mut sequences = self.get_deliberation_sequences();
        sequences.remove(index);
        self.deliberation_sequences.set(sequences);
    }

    pub fn add_opinion_info(&mut self) {
        let mut sequences = self.get_deliberation_sequences();
        sequences.push(StepCreateRequest {
            step_type: StepType::GeneralPost,
            name: "".to_string(),
            started_at: 0,
            ended_at: 0,
        });
        self.deliberation_sequences.set(sequences);
    }

    pub fn check_opinion_info(&self) -> bool {
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

    pub fn update_opinion_field_type_from_str(
        &self,
        opinion_field_type: String,
    ) -> Option<ProjectArea> {
        let field = opinion_field_type.parse::<ProjectArea>();

        match field {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn update_deliberation_information(&mut self, information: DeliberationInformation) {
        self.deliberation_informations.set(information);
    }

    pub fn open_create_panel_modal(&self, lang: Language, translates: CompositionPanelTranslate) {
        let mut popup_service = (self.popup_service)().clone();
        let attributes = self.total_attributes;
        popup_service
            .open(rsx! {
                CreateNewPanelModal {
                    attributes: attributes.clone(),
                    lang: lang.clone(),
                    onsave: move |panel_name: String| {
                        tracing::debug!("panel name: {panel_name}");
                    },
                    onclick: {
                        move |panel_name: String| {
                            tracing::debug!("panel name: {panel_name}");
                            popup_service
                                .open(rsx! {
                                    AddAttributeModal {
                                        lang,
                                        onclose: move |_e: MouseEvent| {
                                            popup_service.close();
                                        },
                                    }
                                })
                                .with_id("add_attribute")
                                .with_title(translates.add_attribute);
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create_panel")
            .with_title(translates.create_panel);
    }

    pub fn open_add_attribute_modal(&self, lang: Language) {
        let translates: CompositionPanelTranslate = translate(&lang);
        let mut popup_service = (self.popup_service)().clone();
        popup_service
            .open(rsx! {
                AddAttributeModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("add_attribute")
            .with_title(translates.add_attribute);
    }

    pub fn open_send_alerm_modal(&self, lang: Language) {
        let translates: PreviewTranslate = translate(&lang);
        let mut popup_service = (self.popup_service)().clone();
        let ctrl = self.clone();
        popup_service
            .open(rsx! {
                SendAlertModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onclick: move |_| {
                        async move {
                            match ctrl.create_deliberation().await {
                                Ok(_) => {
                                    popup_service.close();
                                }
                                Err(e) => {
                                    tracing::error!("Create Deliberation Failed Reason: {:?}", e);
                                }
                            }
                        }
                    },
                }
            })
            .with_id("send_alert")
            .with_title(translates.send_alerm);
    }

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

    pub async fn create_deliberation(&self) -> Result<()> {
        let user: LoginService = use_context();
        let org = user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let opinion_informations = self.get_deliberation_informations();
        let deliberation_sequences = self.get_deliberation_sequences();
        let total_attributes = self.get_total_attributes();
        let total_fields = self.get_total_fields();

        tracing::debug!("opinion_informations: {:?}", opinion_informations);
        tracing::debug!("deliberation_sequences: {:?}", deliberation_sequences);
        tracing::debug!("total_attributes: {:?}", total_attributes);
        tracing::debug!("total_fields: {:?}", total_fields);

        let client = Deliberation::get_client(&crate::config::get().api_url);

        let (started_at, ended_at) = self.get_period();

        match client
            .create(
                org_id,
                started_at as i64,
                ended_at as i64,
                opinion_informations.deliberation_type.unwrap_or_default(),
                opinion_informations.title.unwrap_or_default(),
                opinion_informations.description.unwrap_or_default(),
                vec![], // TODO: panels
                vec![], // TODO: resources
                vec![], // TODO: surveys
                vec![], // roles
                deliberation_sequences,
                vec![], // elearning
                vec![], // discussions
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    //step 2
    pub async fn create_resource(&self, file: File) -> Result<()> {
        let org = self.user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::ResourceFile::get_client(&config::get().api_url);
        let mut ctrl = self.clone();

        match client
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
        {
            Ok(v) => {
                let mut info = (ctrl.deliberation_informations)();
                let mut documents = info.documents;
                documents.push(v);
                info.documents = documents;
                ctrl.deliberation_informations.set(info);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub fn delete_resource(&mut self, id: i64) {
        let mut info = (self.deliberation_informations)();
        let mut documents = info.documents;
        documents.retain(|doc| doc.id != id);
        info.documents = documents;
        self.deliberation_informations.set(info);
    }

    pub fn resources(&self) -> Vec<ResourceFile> {
        (self.deliberation_informations)().documents
    }
}
