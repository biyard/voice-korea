use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::{
    deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest,
    OrganizationMemberSummary,
};

use crate::{
    components::{expandable_card::ExpandableCard, icons::ArrowLeft},
    pages::deliberations::new::components::{
        calendar_dropdown::CalendarDropdown, committee_dropdown::CommitteeDropdown,
    },
};

use super::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationSampleSurveySettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: SampleSurveyTranslate = translate(&lang);
    let sample_survey = ctrl.get_sample_survey();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div { onclick: move |_| {},
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
                        onclick: move |_| {},
                        "{tr.backward}"
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        "{tr.temporary_save}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {},
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
