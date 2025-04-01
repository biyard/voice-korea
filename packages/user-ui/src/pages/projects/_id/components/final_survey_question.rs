#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{response::Answer, Question, SurveyV2};

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::projects::_id::components::{
        final_survey::FinalSurveyTranslate, multiple_objective::MultipleObjective,
        single_objective::SingleObjective, subjective::Subjective,
    },
};

#[component]
pub fn FinalSurveyQuestion(
    lang: Language,
    survey: SurveyV2,
    answers: Vec<Answer>,
    onprev: EventHandler<MouseEvent>,
    onsend: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
) -> Element {
    let tr: FinalSurveyTranslate = translate(&lang);
    let survey_title = survey.name;

    rsx! {
        div { class: "max-[1000px]:px-30 flex flex-col w-full justify-start items-start gap-10 mt-28",
            div { class: "flex flex-row w-full justify-start items-center gap-8 mb-10",
                div {
                    class: "cursor-pointer w-24 h-24",
                    onclick: move |e: Event<MouseData>| {
                        onprev.call(e);
                    },
                    LeftArrow { stroke: "black" }
                }
                div { class: "font-semibold text-text-black text-20", "{survey_title}" }
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

            div { class: "flex flex-row w-full justify-center items-center mb-40",
                div {
                    class: "cursor-pointer flex flex-row justify-center items-center w-200 py-13 font-bold text-white text-base bg-button-primary rounded-lg",
                    onclick: move |e: Event<MouseData>| {
                        onsend.call(e);
                    },
                    "{tr.submit}"
                }
            }
        }
    }
}
