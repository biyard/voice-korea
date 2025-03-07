use by_components::charts::{
    horizontal_bar::HorizontalBar,
    pie_chart::{PieChart, PieChartData},
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::ParsedQuestion;

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::project::{controller::SurveyResponses, i18n::StatisticsTranslate},
};

#[component]
pub fn SampleStatistics(
    lang: Language,
    responses: SurveyResponses,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    let tr: StatisticsTranslate = translate(&lang);
    let answers = responses.answers;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
            div { class: "flex flex-row justify-start items-center gap-[8px]",
                div {
                    class: "cursor-pointer w-[24px] h-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        onprev.call(e);
                    },
                    LeftArrow { stroke: "black" }
                }
                div { class: "font-semibold text-[#222222] text-[20px]", "{tr.response}" }
            }
            for (i , (_key , (title , parsed_question))) in answers.iter().enumerate() {
                match parsed_question {
                    ParsedQuestion::SingleChoice { answers, response_count } => {
                        rsx! {
                            div { class: "flex flex-col w-full",
                                ObjectiveBox {
                                    lang,
                                    title,
                                    answers: answers.clone(),
                                    answer_count: response_count.clone(),
                                    index: i,
                                }
                            }
                        }
                    }
                    ParsedQuestion::MultipleChoice { answers, response_count } => {
                        rsx! {
                            div { class: "flex flex-col w-full",
                                ObjectiveBox {
                                    lang,
                                    title,
                                    answers: answers.clone(),
                                    answer_count: response_count.clone(),
                                    index: i,
                                }
                            }
                        }
                    }
                    ParsedQuestion::ShortAnswer { answers } => {
                        rsx! {
                            div { class: "flex flex-col w-full",
                                SubjectiveBox { lang, title, answers: answers.clone() }
                            }
                        }
                    }
                    ParsedQuestion::Subjective { answers } => {
                        rsx! {
                            div { class: "flex flex-col w-full",
                                SubjectiveBox { lang, title, answers: answers.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ObjectiveBox(
    lang: Language,
    index: usize,
    title: String,
    answers: Vec<String>,
    answer_count: Vec<i64>,
    #[props(default = false)] is_single: bool,
) -> Element {
    let tr: StatisticsTranslate = translate(&lang);
    let mut pie_charts: Signal<Vec<PieChartData>> = use_signal(|| vec![]);
    let mut total_answers: Signal<i32> = use_signal(|| 0);

    use_effect(use_reactive(&answer_count, {
        let answers = answers.clone();
        move |answer_count| {
            let mut pies = vec![];
            let mut totals = 0;

            for (i, answer) in answers.iter().enumerate() {
                pies.push(PieChartData::new(answer.clone(), answer_count[i] as i32));
                totals += answer_count[i] as i32;
            }

            pie_charts.set(pies);
            total_answers.set(totals);
        }
    }));

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
                                    "{tr.necessary}"
                                }
                            }
                            div { class: "font-semibold text-[#222222] text-[16px] leading-[22.5px]",
                                "{title}"
                            }
                        }
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
                                if total_answers() != 0 {
                                    HorizontalBar {
                                        id: format!("horizontal_bar_{}{}", index, i),
                                        value: answer_count[i],
                                        height: "23px",
                                        max_value: total_answers() as i64,
                                        class: "flex flex-row flex-1 bg-[#EEEEEE] rounded-[6px] overflow-hidden",
                                    }
                                }

                                div { class: "w-[200px] font-medium text-[#2d2d2d] text-[15px] leading-[22.5px]",
                                    {
                                        format!(
                                            "{:?}{} ({:.2}%)",
                                            answer_count[i],
                                            tr.unit,
                                            if total_answers() != 0 {
                                                answer_count[i] as f64 * 100.0 / total_answers() as f64
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
                    class: "w-[500px] max-[1300px]:w-[300px] max-[800px]:hidden sm:block",
                    data: pie_charts(),
                }
            }
        }
    }
}

#[component]
pub fn SubjectiveBox(lang: Language, title: String, answers: Vec<String>) -> Element {
    let tr: StatisticsTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full  bg-white px-[40px] py-[20px] rounded-[8px] gap-[20px]",
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row justify-start items-center gap-[20px]",
                        div { class: "font-semibold text-[#222222] text-[16px] leading-[22.5px]",
                            "{title}"
                        }
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
