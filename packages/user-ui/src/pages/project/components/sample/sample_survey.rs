use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{response::Answer, Question, SurveyV2};

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::project::{
        components::{
            multiple_objective::MultipleObjective, single_objective::SingleObjective,
            subjective::Subjective,
        },
        i18n::SurveyTranslate,
    },
};

#[component]
pub fn Survey(
    lang: Language,
    survey: SurveyV2,
    answers: Vec<Answer>,
    onprev: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
    onsend: EventHandler<MouseEvent>,
) -> Element {
    let tr: SurveyTranslate = translate(&lang);
    let survey_title = survey.name;

    rsx! {
        div { class: "flex flex-col gap-[10px]",
            div { class: "flex flex-row w-full justify-start items-center gap-[8px] mb-[10px]",
                div {
                    class: "cursor-pointer w-[24px] h-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        onprev.call(e);
                    },
                    LeftArrow { stroke: "black" }
                }
                div { class: "font-semibold text-[#222222] text-[20px]", "{survey_title}" }
            }
            for (i , question) in survey.questions.iter().enumerate() {
                match question {
                    Question::SingleChoice(v) => {
                        let answer = if let Answer::SingleChoice { answer } = &answers[i] {
                            answer.clone()
                        } else {
                            0
                        };
                        rsx! {
                            SingleObjective {
                                id: None,
                                question: v.clone(),
                                answer,
                                onchange: move |e| { onchange.call((i, Answer::SingleChoice { answer: e })) },
                            }
                        }
                    }
                    Question::MultipleChoice(v) => {
                        let answer = if let Answer::MultipleChoice { answer } = &answers[i] {
                            answer.clone()
                        } else {
                            vec![]
                        };
                        rsx! {
                            MultipleObjective {
                                id: None,
                                question: v.clone(),
                                answer,
                                onchange: move |e| {
                                    onchange
                                        .call((
                                            i,
                                            Answer::MultipleChoice {
                                                answer: e,
                                            },
                                        ))
                                },
                            }
                        }
                    }
                    Question::ShortAnswer(v) => {
                        let answer = if let Answer::ShortAnswer { answer } = &answers[i] {
                            answer.clone()
                        } else {
                            String::new()
                        };
                        rsx! {
                            Subjective {
                                lang,
                                id: None,
                                question: v.clone(),
                                answer,
                                onchange: move |e| {
                                    onchange.call((i, Answer::ShortAnswer { answer: e }));
                                },
                            }
                        }
                    }
                    Question::Subjective(v) => {
                        let answer = if let Answer::Subjective { answer } = &answers[i] {
                            answer.clone()
                        } else {
                            String::new()
                        };
                        rsx! {
                            Subjective {
                                lang,
                                id: None,
                                question: v.clone(),
                                answer,
                                onchange: move |e| {
                                    onchange.call((i, Answer::Subjective { answer: e }));
                                },
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-center items-center",
                div {
                    class: "cursor-pointer flex flex-row justify-center items-center w-[200px] py-[13px] font-bold text-white text-[16px] bg-[#8095EA] rounded-[8px]",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    "{tr.submit}"
                }
            }
        }
    }
}
