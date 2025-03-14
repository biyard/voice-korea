use crate::{
    components::{
        calendar::Calendar,
        icons::{ArrowRight, BottomDropdownArrow, CalendarIcon, MenuDial, TopDropdownArrow, Trash},
    },
    pages::deliberations::new::i18n::DeliberationNewTranslate,
    routes::Route,
};

use super::controller::CurrentStep;
use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{prelude::step_type::StepType, step::StepCreateRequest};
use std::str::FromStr;

#[derive(Props, Clone, PartialEq)]
pub struct OrganizationDeliberationProcedureTranslate {
    organization_of_procedures: String,
    organization_of_procedures_description: String,
    no_contents: String,
    no_selection: String,
    remove: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct PeriodDeliberationProcedureTranslate {
    duration_per_procedure: String,
    duration_per_procedure_description: String,
    no_contents: String,
    required_period_selection: String,
    starting_day: String,
    last_day: String,
}

#[component]
pub fn CompositionDeliberation(
    lang: Language,
    deliberation_sequences: Vec<StepCreateRequest>,
    check_sequence: bool,
    onadd: EventHandler<MouseEvent>,
    onupdate: EventHandler<(usize, StepCreateRequest)>,
    ondelete: EventHandler<usize>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translates: DeliberationNewTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#000000] mb-[10px]",
                "{translates.organization_and_period_title}"
            }
            OrganizationDeliberationProcedure {
                deliberation_sequences: deliberation_sequences.clone(),
                lang,
                onadd,
                ondelete,
                onupdate,
                i18n: OrganizationDeliberationProcedureTranslate {
                    organization_of_procedures: translates.organization_of_procedures.to_string(),
                    organization_of_procedures_description: translates
                        .organization_of_procedures_description
                        .to_string(),
                    no_contents: translates.no_contents.to_string(),
                    no_selection: translates.no_selection.to_string(),
                    remove: translates.remove.to_string(),
                },
            }
            PeriodDeliberationProcedure {
                deliberation_sequences: deliberation_sequences.clone(),
                onupdate,
                i18n: PeriodDeliberationProcedureTranslate {
                    duration_per_procedure: translates.duration_per_procedure.to_string(),
                    duration_per_procedure_description: translates
                        .duration_per_procedure_description
                        .to_string(),
                    no_contents: translates.no_contents.to_string(),
                    required_period_selection: translates.required_period_selection.to_string(),
                    starting_day: translates.starting_day.to_string(),
                    last_day: translates.last_day.to_string(),
                },
            }
            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                Link { to: Route::OpinionCreatePage { lang },
                    div { class: "flex flex-row w-[170px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                        {translates.to_public_opinion_management_list}
                    }
                }
                div {
                    class: "flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    //TODO: implement temporary save
                    onclick: move |_| {},
                    {translates.temporary_save}
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: {
                        move |_| {
                            if check_sequence {
                                onstep.call(CurrentStep::InputInformation);
                            } else {
                                tracing::error!("opinion info is empty");
                            }
                        }
                    },
                    {translates.next}
                }
            }
        }
    }
}

#[component]
pub fn PeriodDeliberationProcedure(
    deliberation_sequences: Vec<StepCreateRequest>,
    i18n: PeriodDeliberationProcedureTranslate,
    onupdate: EventHandler<(usize, StepCreateRequest)>,
) -> Element {
    let mut clicked_index = use_signal(|| 0);

    let mut updated_sequence = use_signal(|| StepCreateRequest::default());
    updated_sequence.set(deliberation_sequences[clicked_index()].clone());

    let start_datetime = Utc.timestamp_opt(updated_sequence().started_at, 0).unwrap();
    let start_date = Some(start_datetime.format("%Y/%m/%d").to_string());

    let end_datetime = Utc
        .timestamp_opt(updated_sequence().ended_at as i64, 0)
        .unwrap();
    let end_date = Some(end_datetime.format("%Y/%m/%d").to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white mt-[20px]",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-col w-full justify-start items-start px-[40px] mt-[24px]",
                    div { class: "flex flex-row h-full items-center justify-center",
                        div { class: "text-[16px] font-bold text-[#eb5757] mt-[5px] mr-[2px]",
                            "*"
                        }
                        div { class: "text-[18px] font-bold text-[#222222]",
                            "{i18n.duration_per_procedure}"
                        }
                    }
                    div { class: "text-[14px] font-normal text-[#6d6d6d]",
                        "{i18n.duration_per_procedure_description}"
                    }
                }
            }
            div { class: "flex flex-row w-full px-[40px]",
                div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] mt-[20px] mb-[20px]" }
            }
            div { class: "flex flex-row w-full justify-end items-start",
                div { class: "flex flex-col w-[415px] justify-end items-end h-full",
                    for (index , sequence) in deliberation_sequences.iter().enumerate() {
                        div {
                            class: format!(
                                "cursor-pointer flex flex-col w-[415px] h-[100px] justify-start items-start px-[40px] py-[20px] {}",
                                if index == clicked_index() { "bg-white" } else { "bg-[#f7f7f7]" },
                            ),
                            onclick: {
                                move |_| {
                                    spawn(async move {
                                        clicked_index.set(index);
                                    });
                                }
                            },
                            div { class: "font-semibold text-[16px] text-[#222222] mb-[10px]",
                                if sequence.name != "" {
                                    "{index + 1}. {sequence.name}"
                                } else {
                                    "{index + 1}. {i18n.no_contents}"
                                }
                            }

                            if sequence.started_at == 0 || sequence.ended_at == 0 {
                                div { class: "font-normal text-[#6d6d6d] text-[15px]",
                                    "{i18n.required_period_selection}"
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-row w-full justify-center items-center px-[60px] pb-[45px] gap-x-[10px]",

                    div { class: "flex flex-col justify-center items-start",
                        div { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px] mb-[10px]",
                            div { class: "font-normal text-[16px] text-[#b4b4b4]",
                                if let Some(v) = start_date {
                                    "{v}"
                                } else {
                                    "{i18n.starting_day}"
                                }
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        Calendar {
                            timestamp: if updated_sequence().started_at != 0 { Some(updated_sequence().started_at as u64) } else { None },
                            update_date: {
                                let sequence = deliberation_sequences[clicked_index()].clone();
                                move |timestamp: i64| {
                                    let mut value = sequence.clone();
                                    value.started_at = timestamp as i64;
                                    tracing::debug!("{:?}", value);
                                    spawn(async move {
                                        tracing::debug!("{:?}", value);
                                        onupdate.call((clicked_index(), value));
                                    });
                                }
                            },
                        }
                    }

                    div { class: "flex justify-center items-center mx-[10px]",
                        ArrowRight { width: "24", height: "24" }
                    }

                    div { class: "flex flex-col justify-start items-start",
                        div { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px] mb-[10px]",
                            div { class: "font-normal text-[16px] text-[#b4b4b4]",
                                if let Some(v) = end_date {
                                    "{v}"
                                } else {
                                    "{i18n.last_day}"
                                }
                            }
                            CalendarIcon { width: "28", height: "28" }
                        }
                        Calendar {
                            timestamp: if updated_sequence().ended_at != 0 { Some(updated_sequence().ended_at as u64) } else { None },
                            update_date: {
                                let sequence = deliberation_sequences[clicked_index()].clone();
                                move |timestamp: i64| {
                                    let mut value = sequence.clone();
                                    value.ended_at = timestamp as i64;
                                    spawn(async move {
                                        onupdate.call((clicked_index(), value));
                                    });
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn OrganizationDeliberationProcedure(
    lang: Language,
    deliberation_sequences: Vec<StepCreateRequest>,
    i18n: OrganizationDeliberationProcedureTranslate,

    onadd: EventHandler<MouseEvent>,
    onupdate: EventHandler<(usize, StepCreateRequest)>,
    ondelete: EventHandler<usize>,
) -> Element {
    let mut composition_clicked = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row h-full items-center justify-center",
                        div { class: "text-[16px] font-bold text-[#eb5757] mt-[5px] mr-[2px]",
                            "*"
                        }
                        div { class: "text-[18px] font-bold text-[#222222]",
                            "{i18n.organization_of_procedures}"
                        }
                    }
                    div { class: "text-[14px] font-normal text-[#6d6d6d]",
                        "{i18n.organization_of_procedures_description}"
                    }
                }
                div {
                    onclick: move |_| {
                        let clicked = composition_clicked();
                        composition_clicked.set(!clicked);
                    },
                    div { class: "cursor-pointer",
                        if composition_clicked() {
                            TopDropdownArrow { width: "24", height: "24" }
                        } else {
                            BottomDropdownArrow { width: "24", height: "24" }
                        }
                    }
                }
            }

            //sequence
            div { class: "flex flex-wrap w-full justify-start items-center mt-[20px]",
                for (index , sequence) in deliberation_sequences.iter().enumerate() {
                    div { class: "flex flex-row w-[184px] h-[55px] justify-start items-center p-[15px] border border-[#bfc8d9] rounded-[4px]",
                        if sequence.name != "" {
                            "{index + 1}. {sequence.name}"
                        } else {
                            "{index + 1}. {i18n.no_contents}"
                        }
                    }
                    if index < deliberation_sequences.len() - 1 {
                        div { class: "mx-[15px]",
                            ArrowRight { width: "18", height: "24" }
                        }
                    }
                }
            }

            //sequence detail
            if composition_clicked() {
                div { class: "flex flex-col w-full",
                    div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] mt-[10px] mb-[20px]" }
                    div { class: "flex flex-col w-full justify-start items-start ",
                        for (index , sequence) in deliberation_sequences.iter().enumerate() {
                            div { class: "flex flex-row w-full justify-start items-center mb-[20px]",
                                MenuDial { width: "24", height: "24" }
                                div { class: "ml-[10px] mr-[35px] w-[260px] text-[16px] font-medium text-black",
                                    if sequence.name != "" {
                                        "{sequence.name}"
                                    } else {
                                        "{i18n.no_contents}"
                                    }
                                }
                                select {
                                    class: "focus:outline-none w-[200px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[10px]",
                                    value: sequence.step_type.to_string(),
                                    onchange: {
                                        let sequence = sequence.clone();
                                        move |e: Event<FormData>| {
                                            let mut value = sequence.clone();
                                            let opinion_type = StepType::from_str(e.value().as_str()).ok();
                                            value.step_type = opinion_type.unwrap_or_default();
                                            onupdate.call((index, value));
                                        }
                                    },
                                    // option {
                                    //     value: "",
                                    //     disabled: true,
                                    //     selected: sequence.step_type.is_none(),
                                    //     "{i18n.no_selection}"
                                    // }
                                    for option_type in StepType::VARIANTS.iter() {
                                        option {
                                            value: option_type.translate(&lang),
                                            selected: sequence.step_type == option_type.clone(),
                                            "{option_type.translate(&lang)}"
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-full focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[40px]",
                                    input {
                                        class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                        r#type: "text",
                                        placeholder: "{i18n.no_contents}",
                                        value: sequence.name.clone(),
                                        oninput: {
                                            let sequence = sequence.clone();
                                            move |e: FormEvent| {
                                                let mut value = sequence.clone();
                                                value.name = e.value();
                                                onupdate.call((index, value));
                                            }
                                        },
                                    }
                                }
                                div {
                                    class: "flex flex-row w-[108px] h-[55px] justify-start items-center bg-white border border-[#bfc8d9] rounded-lg px-[15px] cursor-pointer",
                                    onclick: move |_| {
                                        ondelete.call(index);
                                    },
                                    div { class: "font-medium text-[#222222] text-[15px] mr-[2px]",
                                        "{i18n.remove}"
                                    }
                                    Trash { width: "24", height: "24" }
                                }
                            }
                        }
                    }
                    div { class: "relative w-full flex items-center justify-center mt-[20px] mb-[24px]",
                        div { class: "border-t border-dashed border-gray-300 w-full" }
                        button {
                            class: "absolute bg-[#f7f7f7] border border-[#bfc8d9] rounded-[100px] w-[43px] h-[43px] flex items-center justify-center shadow",
                            onclick: move |e: Event<MouseData>| {
                                onadd.call(e);
                            },
                            "+"
                        }
                    }
                }
            }
        }
    }
}
