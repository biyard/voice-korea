#![allow(dead_code, unused)]
use crate::{
    components::{
        calendar::Calendar,
        icons::{ArrowRight, BottomDropdownArrow, CalendarIcon, MenuDial, TopDropdownArrow, Trash},
    },
    pages::deliberations::new::{
        controller::CurrentStep,
        step::{
            basic_info::BasicInfo, deliberation::Deliberation, discussion::Discussion,
            recommendation::Recommendation, sample_survey::SampleSurvey, vote::Vote,
        },
    },
};

use by_components::icons::edit::Edit1;
use by_macros::DioxusController;
use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    deliberation::DeliberationCreateRequest, prelude::step_type::StepType, step::StepCreateRequest,
};
use std::str::FromStr;

#[derive(Props, Clone, PartialEq)]
pub struct OrganizationDeliberationProcedureTranslate {
    organization_of_procedures: String,
    organization_of_procedures_description: String,
    no_contents: String,
    no_selection: String,
    remove: String,
    edit: String,
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DeliberationStep {
    None,
    BasicInfo,
    SampleSurvey,
    Deliberation,
    Discussion,
    Vote,
    Recommendation,
}

#[component]
pub fn CompositionDeliberation(
    lang: Language,
    visibility: bool,

    req: DeliberationCreateRequest,

    onprev: EventHandler<(DeliberationCreateRequest, CurrentStep)>,
    onnext: EventHandler<(DeliberationCreateRequest, CurrentStep)>,
) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let mut deliberation_step = use_signal(|| DeliberationStep::None);
    let deliberation_sequences = ctrl.get_deliberation_sequences();
    let check_sequence = ctrl.check_deliberation_info();

    use_effect({
        let req = req.clone();
        move || {
            let steps = req.steps.clone();

            if !steps.is_empty() {
                ctrl.set_deliberation_sequences(steps);
            }
        }
    });

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full justify-start items-start {}",
                if !visibility { "hidden" } else { "" },
            ),

            CompositionSchedule {
                lang,
                deliberation_sequences: ctrl.get_deliberation_sequences(),
                check_sequence,
                deliberation_step: deliberation_step(),

                onadd: move |_| {
                    let _ = ctrl.add_deliberation_info();
                },
                onupdate: move |(index, opinion): (usize, StepCreateRequest)| {
                    let _ = ctrl.update_deliberation_info(index, opinion);
                },
                ondelete: move |index: usize| {
                    let _ = ctrl.delete_deliberation_info(index);
                },
                onprev: {
                    let new_req = {
                        let mut r = req.clone();
                        r = ctrl.change_deliberation_sequences(req.clone());
                        r
                    };
                    move |step: CurrentStep| {
                        onprev.call((new_req.clone(), step));
                    }
                },
                onnext: {
                    let new_req = {
                        let mut r = req.clone();
                        r = ctrl.change_deliberation_sequences(req.clone());
                        r
                    };
                    move |step: CurrentStep| {
                        onnext.call((new_req.clone(), step));
                    }
                },
                change_deliberation_step: move |step| {
                    deliberation_step.set(step);
                },
            }

            BasicInfo {
                lang,
                visibility: deliberation_step() == DeliberationStep::BasicInfo,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }

            SampleSurvey {
                lang,
                visibility: deliberation_step() == DeliberationStep::SampleSurvey,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }

            Deliberation {
                lang,
                visibility: deliberation_step() == DeliberationStep::Deliberation,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }

            Discussion {
                lang,
                visibility: deliberation_step() == DeliberationStep::Discussion,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }

            Vote {
                lang,
                visibility: deliberation_step() == DeliberationStep::Vote,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }

            Recommendation {
                lang,
                visibility: deliberation_step() == DeliberationStep::Recommendation,
                onprev: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onprev.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
                onnext: {
                    let req = req.clone();
                    move |step: DeliberationStep| {
                        deliberation_step.set(step);
                        onnext.call((req.clone(), CurrentStep::DeliberationSchedule));
                    }
                },
            }
        }
    }
}

#[component]
pub fn CompositionSchedule(
    lang: Language,
    deliberation_sequences: Vec<StepCreateRequest>,
    check_sequence: bool,
    deliberation_step: DeliberationStep,

    onadd: EventHandler<MouseEvent>,
    onupdate: EventHandler<(usize, StepCreateRequest)>,
    ondelete: EventHandler<usize>,
    onprev: EventHandler<CurrentStep>,
    onnext: EventHandler<CurrentStep>,
    change_deliberation_step: EventHandler<DeliberationStep>,
) -> Element {
    let tr: CompositionDeliberationTranslate = translate(&lang);

    rsx! {
        div {
            class: format!(
                "flex flex-col w-full justify-start items-start {}",
                if deliberation_step != DeliberationStep::None { "hidden" } else { "" },
            ),
            div { class: "font-medium text-base text-black mb-10", "{tr.organization_and_period_title}" }
            OrganizationDeliberationProcedure {
                deliberation_sequences: deliberation_sequences.clone(),
                lang,
                change_deliberation_step: move |step: DeliberationStep| {
                    change_deliberation_step.call(step);
                    onnext.call(CurrentStep::EditContent);
                },
                onadd,
                ondelete,
                onupdate,
                i18n: OrganizationDeliberationProcedureTranslate {
                    organization_of_procedures: tr.organization_of_procedures.to_string(),
                    organization_of_procedures_description: tr
                        .organization_of_procedures_description
                        .to_string(),
                    no_contents: tr.no_contents.to_string(),
                    no_selection: tr.no_selection.to_string(),
                    edit: tr.edit.to_string(),
                    remove: tr.remove.to_string(),
                },
            }
            PeriodDeliberationProcedure {
                deliberation_sequences: deliberation_sequences.clone(),
                onupdate,
                i18n: PeriodDeliberationProcedureTranslate {
                    duration_per_procedure: tr.duration_per_procedure.to_string(),
                    duration_per_procedure_description: tr
                        .duration_per_procedure_description
                        .to_string(),
                    no_contents: tr.no_contents.to_string(),
                    required_period_selection: tr.required_period_selection.to_string(),
                    starting_day: tr.starting_day.to_string(),
                    last_day: tr.last_day.to_string(),
                },
            }
            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        onprev.call(CurrentStep::CompositionPanel);
                    },
                    "{tr.backward}"
                }
                div {
                    class: "flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    //TODO: implement temporary save
                    onclick: move |_| {},
                    {tr.temporary_save}
                }
                div {
                    class: "cursor-pointer flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: {
                        move |_| {
                            if check_sequence {
                                onnext.call(CurrentStep::Preview);
                            } else {
                                tracing::error!("opinion info is empty");
                            }
                        }
                    },
                    {tr.next}
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
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white mt-20",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-col w-full justify-start items-start px-40 mt-24",
                    div { class: "flex flex-row h-full items-center justify-center",
                        div { class: "text-base font-bold text-necessary mt-5 mr-2",
                            "*"
                        }
                        div { class: "text-lg font-bold text-text-black",
                            "{i18n.duration_per_procedure}"
                        }
                    }
                    div { class: "text-sm font-normal text-text-gray",
                        "{i18n.duration_per_procedure_description}"
                    }
                }
            }
            div { class: "flex flex-row w-full px-40",
                div { class: "flex flex-row w-full h-1 bg-period-border-gray mt-20 mb-20" }
            }
            div { class: "flex flex-row w-full justify-end items-start",
                div { class: "flex flex-col w-415 justify-end items-end h-full",
                    for (index , sequence) in deliberation_sequences.iter().enumerate() {
                        div {
                            class: format!(
                                "cursor-pointer flex flex-col w-415 h-100 justify-start items-start px-40 py-20 {}",
                                if index == clicked_index() { "bg-white" } else { "bg-background-gray" },
                            ),
                            onclick: {
                                move |_| {
                                    spawn(async move {
                                        clicked_index.set(index);
                                    });
                                }
                            },
                            div { class: "font-semibold text-[16px] text-text-black mb-10",
                                if sequence.name != "" {
                                    "{index + 1}. {sequence.name}"
                                } else {
                                    "{index + 1}. {i18n.no_contents}"
                                }
                            }

                            if sequence.started_at == 0 || sequence.ended_at == 0 {
                                div { class: "font-normal text-text-gray text-15",
                                    "{i18n.required_period_selection}"
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-row w-full justify-center items-center px-60 pb-45 gap-x-10",

                    div { class: "flex flex-col justify-center items-start",
                        div { class: "flex flex-row w-190 focus:outline-none h-55 justify-between items-center bg-white border border-label-border-gray rounded-lg px-20 mb-10",
                            div { class: "font-normal text-base text-hint-gray",
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

                    div { class: "flex justify-center items-center mx-10",
                        ArrowRight { width: "24", height: "24" }
                    }

                    div { class: "flex flex-col justify-start items-start",
                        div { class: "flex flex-row w-190 focus:outline-none h-55 justify-between items-center bg-white border border-label-border-gray rounded-lg px-20 mb-10",
                            div { class: "font-normal text-base text-hint-gray",
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
    change_deliberation_step: EventHandler<DeliberationStep>,
) -> Element {
    let mut composition_clicked = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row h-full items-center justify-center",
                        div { class: "text-base font-bold text-necessary mt-5 mr-2",
                            "*"
                        }
                        div { class: "text-lg font-bold text-text-black",
                            "{i18n.organization_of_procedures}"
                        }
                    }
                    div { class: "text-sm font-normal text-text-gray",
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
            div { class: "flex flex-wrap w-full justify-start items-center mt-20",
                for (index , sequence) in deliberation_sequences.iter().enumerate() {
                    div {
                        class: "cursor-pointer flex flex-row w-184 h-55 justify-start items-center p-15 border border-label-border-gray rounded-sm",
                        onclick: {
                            let step_type = sequence.step_type.clone();
                            move |_| {
                                if step_type == StepType::GeneralPost {
                                    change_deliberation_step.call(DeliberationStep::BasicInfo);
                                } else if step_type == StepType::SampleSurvey {
                                    change_deliberation_step.call(DeliberationStep::SampleSurvey);
                                } else if step_type == StepType::Post {
                                    change_deliberation_step.call(DeliberationStep::Deliberation);
                                } else if step_type == StepType::VideoConference {
                                    change_deliberation_step.call(DeliberationStep::Discussion);
                                } else if step_type == StepType::Survey {
                                    change_deliberation_step.call(DeliberationStep::Vote);
                                } else {
                                    change_deliberation_step.call(DeliberationStep::Recommendation);
                                }
                            }
                        },
                        if sequence.name != "" {
                            "{index + 1}. {sequence.name}"
                        } else {
                            "{index + 1}. {i18n.no_contents}"
                        }
                    }
                    if index < deliberation_sequences.len() - 1 {
                        div { class: "mx-15",
                            ArrowRight { width: "18", height: "24" }
                        }
                    }
                }
            }

            //sequence detail
            if composition_clicked() {
                div { class: "flex flex-col w-full",
                    div { class: "flex flex-row w-full h-1 bg-period-border-gray mt-10 mb-20" }
                    div { class: "flex flex-col w-full justify-start items-start ",
                        for (index , sequence) in deliberation_sequences.iter().enumerate() {
                            div { class: "flex flex-row w-full justify-start items-center mb-20 gap-10",
                                MenuDial { width: "24", height: "24" }
                                div { class: "flex flex-row w-full focus:outline-none h-55 justify-start items-center bg-background-gray rounded-sm px-15",
                                    div { class: "font-medium text-base text-text-black ",
                                        {sequence.name.clone()}
                                    }
                                }
                                div { class: "focus:outline-none min-w-200 h-55 justify-start items-start p-15 bg-background-gray rounded-sm",
                                    div { class: "font-medium text-base text-text-black ",
                                        {sequence.step_type.translate(&lang)}
                                    }
                                }
                                div {
                                    class: "cursor-pointer flex flex-row w-fit min-w-85 p-15 justify-center items-center gap-5 border border-label-border-gray rounded-lg",
                                    onclick: {
                                        let step_type = sequence.step_type.clone();
                                        move |_| {
                                            if step_type == StepType::GeneralPost {
                                                change_deliberation_step.call(DeliberationStep::BasicInfo);
                                            } else if step_type == StepType::SampleSurvey {
                                                change_deliberation_step.call(DeliberationStep::SampleSurvey);
                                            } else if step_type == StepType::Post {
                                                change_deliberation_step.call(DeliberationStep::Deliberation);
                                            } else if step_type == StepType::VideoConference {
                                                change_deliberation_step.call(DeliberationStep::Discussion);
                                            } else if step_type == StepType::Survey {
                                                change_deliberation_step.call(DeliberationStep::Vote);
                                            } else {
                                                change_deliberation_step.call(DeliberationStep::Recommendation);
                                            }
                                        }
                                    },
                                    div { class: "font-normal text-base text-text-black w-fit whitespace-nowrap",
                                        "{i18n.edit}"
                                    }
                                    Edit1 {
                                        class: "[&>path]:stroke-third",
                                        width: "24",
                                        height: "24",
                                        fill: "none",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    deliberation_sequences: Signal<Vec<StepCreateRequest>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let tr: CompositionDeliberationTranslate = translate(&lang);
        let ctrl = Self {
            lang,
            deliberation_sequences: use_signal(|| {
                vec![
                    StepCreateRequest {
                        step_type: StepType::GeneralPost,
                        name: tr.basic_information.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::SampleSurvey,
                        name: tr.sample_survey.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Post,
                        name: tr.deliberation.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::VideoConference,
                        name: tr.discussion.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Survey,
                        name: tr.vote.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                    StepCreateRequest {
                        step_type: StepType::Report,
                        name: tr.final_recommendation.to_string(),
                        started_at: 0,
                        ended_at: 0,
                    },
                ]
            }),
        };

        Ok(ctrl)
    }

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

    pub fn set_deliberation_sequences(&mut self, steps: Vec<StepCreateRequest>) {
        self.deliberation_sequences.set(steps);
    }

    pub fn get_deliberation_sequences(&self) -> Vec<StepCreateRequest> {
        (self.deliberation_sequences)()
    }

    pub fn change_deliberation_sequences(
        &self,
        req: DeliberationCreateRequest,
    ) -> DeliberationCreateRequest {
        let mut req = req;
        req.steps = self.get_deliberation_sequences();

        let deliberation_time = self.get_deliberation_time(req.clone().steps);
        req.started_at = deliberation_time.0;
        req.ended_at = deliberation_time.1;

        req
    }

    pub fn get_deliberation_time(&self, steps: Vec<StepCreateRequest>) -> (i64, i64) {
        let started_at = steps.iter().map(|s| s.started_at).min().unwrap_or(0);
        let ended_at = steps.iter().map(|s| s.ended_at).max().unwrap_or(0);

        (started_at, ended_at)
    }
}

translate! {
    CompositionDeliberationTranslate;

    organization_and_period_title: {
        ko: "공론 절차 구성 및 기간",
        en: "Organization and period of public deliberation procedures"
    }

    duration_per_procedure: {
        ko: "절차별 기간",
        en: "Duration per procedure"
    }
    duration_per_procedure_description: {
        ko: "공론의 방식과 순서를 정해주세요.",
        en: "Please decide the method and order of public discussion."
    }
    no_contents: {
        ko: "내용 없음",
        en: "No Contents"
    }
    required_period_selection: {
        ko: "기간 선택 필요",
        en: "Period selection required"
    }
    starting_day: {
        ko: "시작하는 날",
        en: "Starting day"
    }
    last_day: {
        ko: "마지막 날",
        en: "Last day"
    }

    basic_information: {
        ko: "기본 정보",
        en: "Basic Information"
    }
    sample_survey: {
        ko: "표본 조사",
        en: "Sample Survey"
    }
    deliberation: {
        ko: "숙의",
        en: "Deliberation"
    }
    discussion: {
        ko: "토론",
        en: "Discussion"
    }
    vote: {
        ko: "투표",
        en: "Vote"
    }
    final_recommendation: {
        ko: "최종 권고안",
        en: "Final Recommendation"
    }

    organization_of_procedures: {
        ko: "공론 절차 구성",
        en: "Organization of public opinion procedures"
    }
    organization_of_procedures_description: {
        ko: "공론의 방식과 순서를 정해주세요.",
        en: "Please decide the method and order of public discussion."
    }
    no_selection: {
        ko: "선택 없음",
        en: "No Selection"
    }
    edit: {
        ko: "편집",
        en: "Edit"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
    to_public_opinion_management_list: {
        ko: "공론관리 목록으로",
        en: "To public opinion management list"
    }
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
}
