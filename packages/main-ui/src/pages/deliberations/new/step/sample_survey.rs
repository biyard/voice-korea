#![allow(dead_code, unused)]
use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::{
    deliberation::DeliberationCreateRequest,
    deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest,
    deliberation_user::DeliberationUserCreateRequest, OrganizationMember, OrganizationMemberQuery,
    OrganizationMemberSummary,
};

use crate::{
    components::{expandable_card::ExpandableCard, icons::ArrowLeft},
    pages::deliberations::new::components::{
        calendar_dropdown::CalendarDropdown, committee_dropdown::CommitteeDropdown,
    },
    service::login_service::LoginService,
    utils::time::current_timestamp,
};

use super::composition_deliberation::DeliberationStep;

// TODO: implement sample survey
#[component]
pub fn SampleSurvey(
    lang: Language,
    visibility: bool,

    req: DeliberationCreateRequest,

    onprev: EventHandler<(DeliberationCreateRequest, DeliberationStep)>,
    onnext: EventHandler<(DeliberationCreateRequest, DeliberationStep)>,
) -> Element {
    let mut ctrl = Controller::new(lang, req.clone())?;
    let tr: SampleSurveyTranslate = translate(&lang);
    let sample_survey = ctrl.get_sample_survey();

    use_effect({
        let mut sample = req
            .sample_surveys
            .get(0)
            .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
            .clone();

        let started_at = if sample.started_at == 0 {
            current_timestamp()
        } else {
            sample.started_at
        };

        let ended_at = if sample.ended_at == 0 {
            current_timestamp()
        } else {
            sample.ended_at
        };

        move || {
            sample.started_at = started_at;
            sample.ended_at = ended_at;
            ctrl.sample_survey.set(sample.clone());
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
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div {
                    onclick: {
                        let new_req = {
                            let mut r = req.clone();
                            r.sample_surveys = vec![ctrl.get_sample_survey()];
                            r
                        };
                        move |_| {
                            onprev.call((new_req.clone(), DeliberationStep::None));
                        }
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", "{tr.sample_survey}" }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", "{tr.input_introduction}" }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Introduction {
                        lang,
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
                    }

                    SampleSurveyReward {
                        lang,
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
                    }

                    SampleSurveyMember {
                        lang,
                        total_committees: ctrl.get_committees(),
                        selected_committees: ctrl.get_selected_committee(),
                        sample_survey: sample_survey.clone(),
                        set_sample_survey: move |survey| {
                            ctrl.set_sample_survey(survey);
                        },
                    }
                }
                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: {
                            let new_req = {
                                let mut r = req.clone();
                                r.sample_surveys = vec![ctrl.get_sample_survey()];
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
                                r.sample_surveys = vec![ctrl.get_sample_survey()];
                                r
                            };
                            move |_| {
                                onprev.call((new_req.clone(), DeliberationStep::None));
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
pub fn SampleSurveyReward(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,
) -> Element {
    let tr: SampleSurveyRewardTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            div { class: "flex flex-row w-full justify-start items-center gap-100",
                ResponseForm {
                    label: tr.expected_time,
                    hint: tr.expected_time_hint,
                    value: sample_survey.estimate_time,
                    oninput: {
                        let mut sample = sample_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                sample.estimate_time = v;
                                set_sample_survey.call(sample.clone());
                            }
                        }
                    },
                }

                ResponseForm {
                    label: tr.expected_point,
                    hint: tr.expected_point_hint,
                    value: sample_survey.point,
                    oninput: {
                        let mut sample = sample_survey.clone();
                        move |e: Event<FormData>| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                sample.point = v;
                                set_sample_survey.call(sample.clone());
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn ResponseForm(
    label: String,
    hint: String,
    value: i64,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            div { class: "flex flex-row max-w-180 w-full justify-start items-center font-normal text-[15px] text-black",
                "{label}"
            }
            input {
                class: "flex flex-row w-full justify-start items-center rounded-sm px-15 py-10 placeholder-hint-gray bg-background-gray font-medium text-text-black text-[15px]",
                r#type: "text",
                placeholder: hint,
                value,
                oninput,
            }
        }
    }
}

#[component]
pub fn SampleSurveyMember(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: SampleSurveyMemberTranslate = translate(&lang);

    let select_ids: Vec<i64> = selected_committees.clone().iter().map(|v| v.id).collect();
    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "sample-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut sample = sample_survey.clone();
                    move |member: OrganizationMemberSummary| {
                        select_ids.push(member.id);
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
                remove_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut sample = sample_survey.clone();
                    move |id: i64| {
                        select_ids.retain(|committee_id| !(committee_id.clone() == id));
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
                clear_committee: {
                    let mut sample = sample_survey.clone();
                    move |_| {
                        let select_ids = vec![];
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
            }
        }
    }
}

#[component]
pub fn Introduction(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,
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
                            value: sample_survey.clone().title,
                            oninput: {
                                let mut survey = sample_survey.clone();
                                move |e: Event<FormData>| {
                                    survey.title = e.value();
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        CalendarDropdown {
                            id: "sample_survey_start_date",
                            date: sample_survey.started_at,
                            onchange: {
                                let mut survey = sample_survey.clone();
                                move |e| {
                                    survey.started_at = e;
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }

                        div { class: "flex flex-row w-16 h-2 bg-label-border-gray" }

                        CalendarDropdown {
                            id: "sample_survey_end_date",
                            date: sample_survey.ended_at,
                            onchange: {
                                let mut survey = sample_survey.clone();
                                move |e| {
                                    survey.ended_at = e;
                                    set_sample_survey.call(survey.clone());
                                }
                            },
                        }
                    }
                }

                div { class: "flex flex-row w-full h-1 bg-period-border-gray" }

                RichText {
                    id: "sample-survey-rich-text",
                    content: sample_survey.clone().description,
                    onchange: {
                        let mut survey = sample_survey.clone();
                        move |e| {
                            survey.description = e;
                            set_sample_survey.call(survey.clone());
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
    sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,
}

impl Controller {
    pub fn new(
        lang: Language,
        req: DeliberationCreateRequest,
    ) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let sample_survey = use_signal(|| DeliberationSampleSurveyCreateRequest::default());

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

        let mut ctrl = Self {
            lang,
            sample_survey,

            members,
            committee_members: use_signal(|| vec![]),
        };

        ctrl.committee_members.set(req.roles.clone());
        Ok(ctrl)
    }

    pub fn set_sample_survey(&mut self, info: DeliberationSampleSurveyCreateRequest) {
        self.sample_survey.set(info);
    }

    pub fn get_sample_survey(&self) -> DeliberationSampleSurveyCreateRequest {
        (self.sample_survey)()
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

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let sample_survey = self.get_sample_survey();
        let roles = sample_survey.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.id))
            .collect()
    }
}

translate! {
    SampleSurveyMemberTranslate;

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
    SampleSurveyTranslate;

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
    input_introduction: {
        ko: "소개글 입력",
        en: "Input Introduction"
    }

    sample_survey: {
        ko: "표본 조사",
        en: "Sample Survey"
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
    SampleSurveyRewardTranslate;

    title: {
        ko: "예상 소요 시간 및 리워드",
        en: "Expected Time and Rewards"
    }

    description: {
        ko: "설문 응답에 걸리는 예상 소요 시간과 리워드를 입력해주세요. 입력된 시간은 리워드 지급과는 무관합니다.",
        en: "Please enter the estimated time it will take to complete the survey and the reward you wish to receive. The time you enter has no bearing on reward payment."
    }

    expected_time: {
        ko: "예상 소요 시간",
        en: "Estimated time"
    }

    expected_time_hint: {
        ko: "소요 시간 입력 (단위: 초)",
        en: "Enter the time required (in seconds)"
    }

    expected_point: {
        ko: "응답 시 지급 포인트 입력",
        en: "Enter payment points when responding"
    }

    expected_point_hint: {
        ko: "포인트 입력",
        en: "Input point"
    }
}
