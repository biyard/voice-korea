#![allow(dead_code, unused)]
use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::{
    deliberation::DeliberationCreateRequest,
    deliberation_basic_infos::deliberation_basic_info::DeliberationBasicInfoCreateRequest,
    deliberation_user::DeliberationUserCreateRequest, File, OrganizationMember,
    OrganizationMemberQuery, OrganizationMemberSummary, ResourceFile, ResourceFileQuery,
    ResourceFileSummary, SurveyV2, SurveyV2Query, SurveyV2Summary,
};

use crate::{
    components::{expandable_card::ExpandableCard, icons::ArrowLeft},
    config,
    pages::deliberations::new::{
        components::{
            calendar_dropdown::CalendarDropdown, committee_dropdown::CommitteeDropdown,
            survey_dropdown::SurveyDropdown,
        },
        step::material_upload::MaterialUpload,
    },
    service::login_service::LoginService,
    utils::time::current_timestamp,
};

use super::composition_deliberation::DeliberationStep;

#[derive(Clone, PartialEq)]
pub enum DocumentTabType {
    DirectUpload,
    Import,
}

// TODO: implement basic info
#[component]
pub fn BasicInfo(
    lang: Language,
    visibility: bool,

    req: DeliberationCreateRequest,

    onprev: EventHandler<(DeliberationCreateRequest, DeliberationStep)>,
    onnext: EventHandler<(DeliberationCreateRequest, DeliberationStep)>,
) -> Element {
    let mut ctrl = Controller::new(lang, req.clone())?;
    let tr: BasicInfoTranslate = translate(&lang);
    let basic_info = ctrl.get_basic_info();

    let total_committees = ctrl.members()?;
    let roles = basic_info.clone().users;

    let documents = ctrl.documents();
    let metadatas = ctrl.metadatas()?;
    let resources = basic_info.clone().resources;

    let surveys = ctrl.surveys()?;

    use_effect({
        let mut basic_info = req
            .basic_infos
            .get(0)
            .unwrap_or(&DeliberationBasicInfoCreateRequest::default())
            .clone();

        let started_at = if basic_info.started_at == 0 {
            current_timestamp()
        } else {
            basic_info.started_at
        };

        let ended_at = if basic_info.ended_at == 0 {
            current_timestamp()
        } else {
            basic_info.ended_at
        };

        let v: Vec<OrganizationMemberSummary> = total_committees
            .clone()
            .into_iter()
            .filter(|member| {
                basic_info
                    .clone()
                    .users
                    .iter()
                    .any(|id| id.clone() == member.id)
            })
            .collect();

        let r: Vec<ResourceFile> = metadatas
            .clone()
            .into_iter()
            .filter(|resource| {
                basic_info
                    .clone()
                    .resources
                    .iter()
                    .any(|id| id.clone() == resource.id)
            })
            .map(|v| v.into())
            .collect();

        move || {
            basic_info.started_at = started_at;
            basic_info.ended_at = ended_at;
            ctrl.basic_info.set(basic_info.clone());
        }
    });

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),

            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-30 gap-10",
                div {
                    onclick: {
                        let new_req = {
                            let mut r = req.clone();
                            r.basic_infos = vec![ctrl.get_basic_info()];
                            r
                        };
                        move |_| {
                            onprev.call((new_req.clone(), DeliberationStep::None));
                        }
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", "{tr.basic_info}" }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", "{tr.post_setting}" }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        basic_info: basic_info.clone(),
                        set_basic_info: move |info| {
                            ctrl.set_basic_info(info);
                        },
                    }
                    BasicMember {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        basic_info: basic_info.clone(),
                        set_basic_info: {
                            let total_committees = total_committees.clone();
                            move |info: DeliberationBasicInfoCreateRequest| {
                                tracing::debug!("info:{:?}", info);
                                ctrl.set_basic_info(info.clone());
                            }
                        },
                    }
                    BasicMaterial {
                        lang,
                        total_surveys: surveys.clone(),
                        selected_surveys: ctrl.get_selected_surveys(),
                        basic_info: basic_info.clone(),
                        set_basic_info: move |info| {
                            ctrl.set_basic_info(info);
                        },

                        metadatas: metadatas.clone(),
                        resources: ctrl.get_selected_resources(),
                        oncreate: {
                            move |file: File| {
                                async move {
                                    let _ = ctrl.create_resource(file).await;
                                }
                            }
                        },
                        onremove: {
                            let metadatas = metadatas.clone();
                            move |id: i64| {
                                let _ = ctrl.delete_resource(id);
                            }
                        },
                        onadd: move |resource: ResourceFileSummary| {
                            let _ = ctrl.add_resource(resource.into());
                        },
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: {
                            let new_req = {
                                let mut r = req.clone();
                                r.basic_infos = vec![ctrl.get_basic_info()];
                                r
                            };
                            move |_| {
                                onprev.call((new_req.clone(), DeliberationStep::None));
                            }
                        },
                        "{tr.backward}"
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        "{tr.temporary_save}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: {
                            let new_req = {
                                let mut r = req.clone();
                                r.basic_infos = vec![ctrl.get_basic_info()];
                                r
                            };
                            move |_| {
                                onnext.call((new_req.clone(), DeliberationStep::None));
                            }
                        },
                        "{tr.next}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn BasicMaterial(
    lang: Language,
    total_surveys: Vec<SurveyV2Summary>,
    selected_surveys: Vec<SurveyV2Summary>,
    basic_info: DeliberationBasicInfoCreateRequest,
    set_basic_info: EventHandler<DeliberationBasicInfoCreateRequest>,

    metadatas: Vec<ResourceFileSummary>,
    resources: Vec<ResourceFile>,
    oncreate: EventHandler<File>,
    onadd: EventHandler<ResourceFileSummary>,
    onremove: EventHandler<i64>,
) -> Element {
    let tr: BasicMaterialTranslate = translate(&lang);
    let mut files = use_signal(|| vec![]);

    use_effect(use_reactive(&resources, move |resources| {
        let all_files: Vec<File> = resources.iter().flat_map(|r| &r.files).cloned().collect();

        files.set(all_files);
    }));

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                MaterialUpload {
                    lang,
                    resources,
                    metadatas,
                    oncreate,
                    onremove,
                    onadd,
                }
                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }
                ConnectProject {
                    lang,

                    basic_info,
                    set_basic_info,

                    total_surveys,
                    selected_surveys,
                }
            }
        }
    }
}

#[component]
pub fn ConnectProject(
    lang: Language,
    basic_info: DeliberationBasicInfoCreateRequest,
    set_basic_info: EventHandler<DeliberationBasicInfoCreateRequest>,

    total_surveys: Vec<SurveyV2Summary>,
    selected_surveys: Vec<SurveyV2Summary>,
) -> Element {
    let tr: ConnectProjectTranslate = translate(&lang);
    let select_ids: Vec<i64> = selected_surveys.clone().iter().map(|v| v.id).collect();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
            //TODO: implement preview
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "text-lg font-bold text-text-black", "{tr.title}" }
                div { class: "text-sm font-normal text-text-gray", "{tr.description}" }
            }

            SurveyDropdown {
                id: "basic-survey",
                hint: tr.survey_hint,

                selected_surveys,
                surveys: total_surveys,

                add_survey: {
                    let mut select_ids = select_ids.clone();
                    let mut basic = basic_info.clone();
                    move |survey: SurveyV2Summary| {
                        select_ids.push(survey.id);
                        basic.surveys = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
                remove_survey: {
                    let mut select_ids = select_ids.clone();
                    let mut basic = basic_info.clone();
                    move |id: i64| {
                        select_ids.retain(|survey_id| !(survey_id.clone() == id));
                        basic.surveys = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
                clear_survey: {
                    let mut basic = basic_info.clone();
                    move |_| {
                        let select_ids = vec![];
                        basic.surveys = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
            }
        }
    }
}

#[component]
pub fn BasicMember(
    lang: Language,

    basic_info: DeliberationBasicInfoCreateRequest,
    set_basic_info: EventHandler<DeliberationBasicInfoCreateRequest>,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: BasicMemberTranslate = translate(&lang);

    let select_ids: Vec<i64> = selected_committees.clone().iter().map(|v| v.id).collect();
    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "basic-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut basic = basic_info.clone();
                    move |member: OrganizationMemberSummary| {
                        select_ids.push(member.id);
                        basic.users = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
                remove_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut basic = basic_info.clone();
                    move |id: i64| {
                        select_ids.retain(|committee_id| !(committee_id.clone() == id));
                        basic.users = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
                clear_committee: {
                    let mut basic = basic_info.clone();
                    move |_| {
                        let select_ids = vec![];
                        basic.users = select_ids.clone();
                        set_basic_info.call(basic.clone());
                    }
                },
            }
        }
    }
}

#[component]
pub fn Introduction(
    lang: Language,

    basic_info: DeliberationBasicInfoCreateRequest,
    set_basic_info: EventHandler<DeliberationBasicInfoCreateRequest>,
) -> Element {
    let tr: IntroductionTranslate = translate(&lang);

    rsx! {
        ExpandableCard {
            required: true,
            header: tr.input_introduction_title,
            description: tr.input_introduction_description,
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "flex flex-row w-full gap-20",
                    div { class: "flex flex-row gap-20 px-15 w-full h-54 bg-background-gray rounded-sm justify-center items-center",
                        input {
                            class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: tr.input_title_hint,
                            value: basic_info.clone().title,
                            oninput: {
                                let mut info = basic_info.clone();
                                move |e: Event<FormData>| {
                                    info.title = e.value();
                                    set_basic_info.call(info.clone());
                                }
                            },
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        CalendarDropdown {
                            id: "basic_start_date",
                            date: basic_info.started_at,
                            onchange: {
                                let mut info = basic_info.clone();
                                move |e| {
                                    info.started_at = e;
                                    set_basic_info.call(info.clone());
                                }
                            },
                        }

                        div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }

                        CalendarDropdown {
                            id: "basic_end_date",
                            date: basic_info.ended_at,
                            onchange: {
                                let mut info = basic_info.clone();
                                move |e| {
                                    info.ended_at = e;
                                    set_basic_info.call(info.clone());
                                }
                            },
                        }
                    }
                }

                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }

                RichText {
                    id: "introduction-rich-text",
                    content: basic_info.clone().description,
                    onchange: {
                        let mut info = basic_info.clone();
                        move |e| {
                            info.description = e;
                            set_basic_info.call(info.clone());
                        }
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub surveys: Resource<Vec<SurveyV2Summary>>,
    basic_info: Signal<DeliberationBasicInfoCreateRequest>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,

    pub search_keyword: Signal<String>,
    pub documents: Signal<Vec<ResourceFile>>,
}

impl Controller {
    pub fn new(
        lang: Language,
        req: DeliberationCreateRequest,
    ) -> std::result::Result<Self, RenderError> {
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

        let mut ctrl = Self {
            lang,
            basic_info,
            members,
            metadatas,
            surveys,

            search_keyword,

            documents: use_signal(|| vec![]),
            committee_members: use_signal(|| vec![]),
        };

        ctrl.committee_members.set(req.roles.clone());

        Ok(ctrl)
    }

    pub fn set_basic_info(&mut self, info: DeliberationBasicInfoCreateRequest) {
        self.basic_info.set(info);
    }

    pub fn get_basic_info(&self) -> DeliberationBasicInfoCreateRequest {
        (self.basic_info)()
    }

    pub async fn create_resource(&mut self, file: File) -> Result<(), models::ApiError> {
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

    pub async fn create_metadata(&self, file: File) -> Result<ResourceFile, models::ApiError> {
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
}

translate! {
    IntroductionTranslate;

    input_introduction_title: {
        ko: "소개글 입력",
        en: "Input Introduction"
    }

    input_introduction_description: {
        ko: "공론의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다.",
        en: "Please explain the topic and purpose of the public discussion. This will make it easier for participants to understand and participate actively."
    }

    input_title_hint: {
        ko: "제목을 입력해주세요.",
        en: "Please enter a title."
    }
}

translate! {
    BasicMemberTranslate;

    title: {
        ko: "담당자 지정",
        en: "Designate a person in charge"
    }

    description: {
        ko: "각 단계별 역할을 수행할 담당자를 선택하여 공론 과정에서의 책임과 역할을 명확하게 할 수 있도록 설정합니다.",
        en: "Select a person to perform each step of the process to ensure that responsibilities and roles are clear during the public hearing."
    }

    search_committee: {
        ko: "공론 위원회에서 검색",
        en: "Search in the Deliberation Committee"
    }
}

translate! {
    ConnectProjectTranslate;

    title: {
        ko: "연관 프로젝트 연동",
        en: "Link to related projects"
    }

    description: {
        ko: "해당 관련 조사 자료를 입력하여 검색하세요 (예: 여론조사, 설문조사, 기타 조사 등)",
        en: "Enter relevant research data to search (e.g. opinion polls, surveys, other surveys, etc.)"
    }

    survey_hint: {
        ko: "공론 및 여론 조사에서 검색",
        en: "Search in deliberation and polls"
    }
}

translate! {
    BasicMaterialTranslate;

    title: {
        ko: "연관 자료 업로드",
        en: "Upload related data"
    }

    description: {
        ko: "해당 공론과 관련된 자료를 업로드해주세요. (예, 공론 소개, 참여 방법, 가이드라인)",
        en: "Please upload materials related to the public discussion (e.g. public discussion introduction, participation method, guidelines)"
    }
}

translate! {
    BasicInfoTranslate;

    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }

    organization_management: {
        ko: "조직 관리",
        en: "Organization Management"
    }
    deliberation_management: {
        ko: "공론 관리",
        en: "Deliberation Management"
    }
    start_deliberation: {
        ko: "공론 시작하기",
        en: "Start Deliberation"
    }
    post_setting: {
        ko: "게시글 설정",
        en: "Post Setting"
    }

    basic_info: {
        ko: "기본 정보",
        en: "Basic Information"
    }
}
