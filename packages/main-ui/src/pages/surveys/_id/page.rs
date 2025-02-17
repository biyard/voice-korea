#![allow(non_snake_case)]
use by_components::charts::{
    horizontal_bar::HorizontalBar, pie_chart::*, StackBarChart, StackBarData,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::SurveyV2;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::_id::{
        controller::{Controller, ParsedQuestion},
        i18n::{
            ObjectiveBoxTranslate, PanelSelectionBoxTranslate, SubjectiveBoxTranslate,
            SurveyPanelReportTranslate, SurveyResultTranslate,
        },
    },
    routes::Route,
    utils::time::{convert_timestamp_to_date, format_remaining_time},
};

use super::controller::{Panel, PanelResponses, SurveyResponses};

#[component]
pub fn SurveyResultPage(lang: Language, survey_id: i64) -> Element {
    let ctrl = Controller::new(lang, survey_id);
    let tr: SurveyResultTranslate = translate(&lang);

    let survey = ctrl.get_survey();
    let panel_report = ctrl.get_panel_responses();
    let survey_report = ctrl.get_survey_responses();
    let total_panels = ctrl.get_total_panels();
    if survey.is_none() {
        return rsx! {};
    }

    let survey = survey.unwrap();

    rsx! {
        document::Script { src: "https://cdn.jsdelivr.net/npm/d3@7" }
        div { class: "w-full flex flex-col gap-[40px] items-start justify-start",
            Nav {
                lang,
                name: "{survey.name}",
                menu: "{tr.survey_management} / {tr.update_survey}",
            }

            div { class: "w-full flex flex-col gap-[20px]",

                div { class: "w-full flex flex-row items-center justify-end gap-[20px]",
                    PrimaryButton {
                        onclick: move |_| async move {
                            ctrl.simulate_response().await;
                        },

                        "{tr.simulate_response}"
                    }
                    PrimaryButton {
                        onclick: move |_| async move {
                            ctrl.download_excel().await;
                        },
                        "{tr.download_excel}"
                    }
                }

                div { class: "flex flex-col gap-[20px] items-start justify-center",
                    SurveySummaryReport { lang, survey }
                    SurveyPanelReport { lang, panel_report: panel_report.clone() }
                    SurveyAnswerReport {
                        lang,
                        total_panels,
                        panel_report,
                        survey_report,
                    }
                }
            }
        }
    }
}

#[component]
pub fn PrimaryButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "flex items-center justify-center px-[20px] py-[14px] text-[16px] text-white bg-[#2A60D3] rounded-[4px]",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

// FIXME: breadcrumb should be placed in layout.
#[component]
pub fn Nav(lang: Language, menu: String, name: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-[10px]",
            div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]", "{menu}" }
            div { class: "flex flex-row w-full justify-start items-center",
                Link { class: "mr-[6px]", to: Route::SurveyPage { lang: lang },
                    ArrowLeft { width: "24", height: "24", color: "#555462" }
                }
                div { class: "text-[#222222] font-semibold text-[28px]", "{name}" }
            }
        }
    }
}

#[component]
pub fn ObjectiveBox(
    index: usize,
    lang: Language,
    title: String,
    responses: i64,
    answers: Vec<String>,
    answer_count: Vec<i64>,

    total_panels: Vec<Panel>,
    selected_panel: i64,
    onchange: EventHandler<i64>,
    #[props(default = false)] is_single: bool,
) -> Element {
    let tr: ObjectiveBoxTranslate = translate(&lang);
    let mut pie_charts: Signal<Vec<PieChartData>> = use_signal(|| vec![]);
    let mut total_panel: Signal<i64> = use_signal(|| 0);

    use_effect(use_reactive(&answer_count, {
        let answers = answers.clone();
        move |answer_count| {
            let mut pies = vec![];
            let mut panel = 0;

            for (i, answer) in answers.iter().enumerate() {
                pies.push(PieChartData::new(answer.clone(), answer_count[i] as i32));
                panel += answer_count[i];
            }

            pie_charts.set(pies);
            total_panel.set(panel);
        }
    }));

    let total_panel = total_panel();

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-[40px] py-[20px] rounded-[8px] gap-[20px]",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-[20px]",
                        div { class: "flex flex-row justify-start items-center gap-[5px]",
                            if is_single {
                                div { class: "font-semibold text-[16px] text-[#eb5757]",
                                    "{tr.necessary}"
                                }
                            } else {
                                div { class: "font-semibold text-[16px] text-[#2a60d3]",
                                    "{tr.plural}"
                                }
                            }
                            div { class: "font-semibold text-[#222222] text-[16px] leading-[22.5px]",
                                "{title}"
                            }
                        }
                        PanelSelectionBox {
                            lang,
                            total_panels,
                            selected_panel,
                            onchange,
                        }
                    }
                    div { class: "font-medium text-[#2d2d2d] text-[16px] leading-[22.5px]",
                        "{total_panel}{tr.people_participated}"
                    }
                }
                div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#ebeff5] my-[7px]" }
            }

            div { class: "flex flex-row w-full justify-between items-start",
                div { class: "flex flex-col flex-1 justify-start items-start gap-[20px]",
                    for (i , answer) in answers.clone().iter().enumerate() {
                        div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
                            div { class: "font-medium text-[#2d2d2d] text-[15px] leading-[22.5px]",
                                "{answer}"
                            }

                            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                                if total_panel != 0 {
                                    HorizontalBar {
                                        id: format!("horizontal_bar_{}{}", index, i),
                                        value: answer_count[i],
                                        height: "23px",
                                        max_value: total_panel,
                                        class: "flex flex-row flex-1 bg-[#EEEEEE] rounded-[6px] overflow-hidden",
                                    }
                                }

                                div { class: "w-[200px] font-medium text-[#2d2d2d] text-[15px] leading-[22.5px]",
                                    {
                                        format!(
                                            "{:?}{} ({:.2}%)",
                                            answer_count[i],
                                            tr.people,
                                            if total_panel != 0 {
                                                answer_count[i] as f64 * 100.0 / total_panel as f64
                                            } else {
                                                0.0
                                            },
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                PieChart {
                    id: format!("pie_chart_{index}"),
                    width: "500px",
                    height: "500px",
                    class: "w-[500px]",
                    data: pie_charts(),
                }
            }
        }
    }
}

#[component]
pub fn SubjectiveBox(
    lang: Language,
    title: String,
    responses: i64,
    answers: Vec<String>,
    total_panels: Vec<Panel>,
    selected_panel: i64,
    onchange: EventHandler<i64>,
) -> Element {
    let tr: SubjectiveBoxTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full  bg-white px-[40px] py-[20px] rounded-[8px] gap-[20px]",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-[20px]",
                        div { class: "font-semibold text-[#222222] text-[16px] leading-[22.5px]",
                            "{title}"
                        }
                        PanelSelectionBox {
                            lang,
                            total_panels,
                            selected_panel,
                            onchange,
                        }
                    }
                    div { class: "font-medium text-[#2d2d2d] text-[16px] leading-[22.5px]",
                        "{responses}{tr.people_participated}"
                    }
                }
                div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#ebeff5] my-[7px]" }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-[5px]",
                div { class: "font-medium text-[#2d2d2d] text-[15px]", "{tr.subjective_answer}" }

                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    for answer in answers.clone() {
                        div { class: "flex flex-row w-full justify-start items-center px-[15px] py-[10px] rounded-[4px] bg-[#f7f7f7]",
                            div { class: "font-medium text-[#222222] text-[15px] leading-[22.5px]",
                                "{answer}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SurveyAnswerReport(
    lang: Language,
    total_panels: Signal<Vec<Panel>>,
    panel_report: PanelResponses,
    survey_report: SurveyResponses,
) -> Element {
    let answer_divs: Signal<Vec<Element>> = use_signal(|| vec![]);
    let answers = survey_report.clone().answers;

    let mut selected_panels = use_signal(|| vec![0; total_panels().len()]);
    let total_panels = total_panels();

    use_effect(use_reactive(&total_panels, move |total_panels| {
        if total_panels.len() != 0 {
            //FIXME: fix to answers length
            selected_panels.set(vec![total_panels[0].id; 1000]);
        }
    }));

    use_effect(use_reactive((&answers, &selected_panels), {
        let mut answer_divs = answer_divs.clone();
        move |(answers, mut selected_panels)| {
            let keys: Vec<i64> = answers.keys().cloned().collect();
            let mut new_divs = vec![];

            for (i, key) in keys.iter().enumerate() {
                if selected_panels.len() != 0 {
                    if let Some(answer) = answers.get(&key) {
                        let title = answer.0.clone();
                        let responses = answer.1;
                        let panel_map: std::collections::HashMap<i64, ParsedQuestion> =
                            answer.2.clone();

                        if let ParsedQuestion::ShortAnswer { answers } =
                            panel_map.get(&selected_panels()[i]).unwrap()
                        {
                            new_divs.push(rsx! {
                                SubjectiveBox {
                                    lang,
                                    title,
                                    responses,
                                    answers: answers.clone(),

                                    total_panels: total_panels.clone(),
                                    selected_panel: selected_panels()[i],
                                    onchange: move |id: i64| {
                                        let mut panels = selected_panels();
                                        panels[i] = id;
                                        selected_panels.set(panels);
                                    },
                                }
                            });
                        } else if let ParsedQuestion::Subjective { answers } =
                            panel_map.get(&selected_panels()[i]).unwrap()
                        {
                            new_divs.push(rsx! {
                                SubjectiveBox {
                                    lang,
                                    title,
                                    responses,
                                    answers: answers.clone(),

                                    total_panels: total_panels.clone(),
                                    selected_panel: selected_panels()[i],
                                    onchange: move |id: i64| {
                                        let mut panels = selected_panels();
                                        panels[i] = id;
                                        selected_panels.set(panels);
                                    },
                                }
                            });
                        } else if let ParsedQuestion::SingleChoice {
                            answers,
                            response_count,
                        } = panel_map.get(&selected_panels()[i]).unwrap()
                        {
                            new_divs.push(rsx! {
                                ObjectiveBox {
                                    index: i,
                                    lang,
                                    title,
                                    responses,
                                    answers: answers.clone(),
                                    answer_count: response_count.clone(),
                                    is_single: true,

                                    total_panels: total_panels.clone(),
                                    selected_panel: selected_panels()[i],
                                    onchange: move |id: i64| {
                                        let mut panels = selected_panels();
                                        panels[i] = id;
                                        selected_panels.set(panels);
                                    },
                                }
                            });
                        } else if let ParsedQuestion::MultipleChoice {
                            answers,
                            response_count,
                        } = panel_map.get(&selected_panels()[i]).unwrap()
                        {
                            new_divs.push(rsx! {
                                ObjectiveBox {
                                    index: i,
                                    lang,
                                    title,
                                    responses,
                                    answers: answers.clone(),
                                    answer_count: response_count.clone(),
                                    is_single: false,

                                    total_panels: total_panels.clone(),
                                    selected_panel: selected_panels()[i],
                                    onchange: move |id: i64| {
                                        let mut panels = selected_panels();
                                        panels[i] = id;
                                        selected_panels.set(panels);
                                    },
                                }
                            });
                        }
                    }
                }
            }

            answer_divs.set(new_divs);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
            for div in answer_divs() {
                {div}
            }
        }
    }
}

#[component]
pub fn SurveyPanelReport(lang: Language, panel_report: PanelResponses) -> Element {
    let tr: SurveyPanelReportTranslate = translate(&lang);
    let panels = panel_report.clone().panels;
    let mut stack_bars: Signal<Vec<StackBarData>> = use_signal(|| vec![]);

    use_effect(use_reactive(&panels, move |panels| {
        let mut stacks = vec![];
        let keys: Vec<i64> = panels.keys().cloned().collect();

        for key in keys.clone() {
            let panel = panels.get(&key).unwrap();
            stacks.push(StackBarData::new(panel.name.clone(), panel.count));
        }

        stack_bars.set(stacks);
    }));

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-[40px] py-[20px] rounded-[8px] gap-[20px]",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "font-semibold text-[#222222] text-[16px] leading-[22.5px]",
                    "{tr.survey_participation_rate}"
                }
                div { class: "font-medium text-[#2d2d2d] text-[16px] leading-[22.5px]",
                    "{panel_report.response_count} / {panel_report.quotes}{tr.people}"
                }
            }

            StackBarChart {
                id: "stack bar".to_string(),
                class: "w-full flex flex-col gap-[10px] rounded-[8px] overflow-hidden",
                height: "54px",
                data: stack_bars(),
            }
        }
    }
}

#[component]
pub fn SurveySummaryReport(lang: Language, survey: SurveyV2) -> Element {
    let tr: SurveyResultTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-[10px]",
            SurveyResponseBox {
                title: "{tr.total_survey_target}",
                value: survey.quotes.to_formatted_string(&Locale::en),
            }
            SurveyResponseBox {
                title: "{tr.number_of_responses}",
                value: survey.response_count.to_formatted_string(&Locale::en),
            }
            SurveyResponseBox {
                title: "{tr.rate_of_responses}",
                value: if survey.quotes == 0 { "0%" } else { "{survey.response_count * 100 / survey.quotes}%" },
            }
            SurveyResponseBox {
                title: "{tr.remaining_period}",
                value: "{format_remaining_time(survey.ended_at)}",
            }
            SurveyResponseBox {
                title: "{tr.survey_period}",
                value: "{convert_timestamp_to_date(survey.started_at)} - {convert_timestamp_to_date(survey.ended_at)}",
            }
        }
    }
}

#[component]
pub fn SurveyResponseBox(title: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col justify-center items-center py-[18px] px-[24px] gap-[20px] rounded-[8px] border border-[#ebeff5] bg-[#ffffff]",
            div { class: "font-semibold text-[#35343f] text-[15px]", "{title}" }
            div { class: "font-bold text-[#435393] text-[24px]", "{value}" }
        }
    }
}

#[component]
pub fn PanelSelectionBox(
    lang: Language,
    total_panels: Vec<Panel>,
    selected_panel: i64,
    onchange: EventHandler<i64>,
) -> Element {
    let tr: PanelSelectionBoxTranslate = translate(&lang);
    rsx! {
        select {
            class: "flex flex-row w-[215px] h-[40px] justify-center items-center bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] text-[#222222]",
            value: selected_panel,
            onchange: move |evt: Event<FormData>| {
                onchange.call(evt.value().parse::<i64>().unwrap());
            },
            option { value: 0, disabled: true, selected: selected_panel == 0,
                "{tr.choose_participate}"
            }
            for panel in total_panels.clone() {
                option { value: panel.id, "{panel.name}" }
            }
        }
    }
}
