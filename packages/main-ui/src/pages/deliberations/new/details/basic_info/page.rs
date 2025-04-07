use bdk::prelude::*;
use by_components::rich_texts::RichText;
use models::{
    deliberation_basic_infos::deliberation_basic_info::DeliberationBasicInfoCreateRequest, File,
    OrganizationMemberSummary, ResourceFile, ResourceFileSummary, SurveyV2Summary,
};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::{
            calendar_dropdown::CalendarDropdown, committee_dropdown::CommitteeDropdown,
            survey_dropdown::SurveyDropdown,
        },
        step::material_upload::MaterialUpload,
    },
};

use super::*;
use controller::*;
use i18n::*;

#[component]
pub fn DeliberationBasicInfoSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: BasicInfoTranslate = translate(&lang);
    let basic_info = ctrl.get_basic_info();

    let metadatas = ctrl.metadatas()?;

    let surveys = ctrl.surveys()?;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", "{tr.post_setting}" }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    BasicInfoIntroduction {
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
                        set_basic_info: move |info: DeliberationBasicInfoCreateRequest| {
                            tracing::debug!("info:{:?}", info);
                            ctrl.set_basic_info(info.clone());
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
                        oncreate: move |file: File| {
                            async move {
                                let _ = ctrl.create_resource(file).await;
                            }
                        },
                        onremove: move |id: i64| {
                            let _ = ctrl.delete_resource(id);
                        },
                        onadd: move |resource: ResourceFileSummary| {
                            let _ = ctrl.add_resource(resource.into());
                        },
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| { ctrl.back() },
                        {tr.backward}
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| async move {
                            ctrl.temp_save().await;
                        },
                        {tr.temporary_save}
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| { ctrl.next() },
                        {tr.next}
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
pub fn BasicInfoIntroduction(
    lang: Language,

    basic_info: DeliberationBasicInfoCreateRequest,
    set_basic_info: EventHandler<DeliberationBasicInfoCreateRequest>,
) -> Element {
    let tr: BasicInfoIntroductionTranslate = translate(&lang);

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
