use bdk::prelude::*;
use models::{response::Answer, Question, SurveyV2};

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::projects::_id::components::{
        final_survey::FinalSurveyTranslate, multiple_objective::MultipleObjective,
        single_objective::SingleObjective, subjective::Subjective,
    },
};

#[component]
pub fn MyFinalSurvey(
    lang: Language,
    survey: SurveyV2,
    answers: Vec<Answer>,
    onprev: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
) -> Element {
    let tr: FinalSurveyTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full gap-[10px] mb-[40px] mt-[28px]",
            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "flex flex-row justify-start items-center gap-[8px]",
                    div {
                        class: "cursor-pointer w-[24px] h-[24px]",
                        onclick: move |e: Event<MouseData>| {
                            onprev.call(e);
                        },
                        LeftArrow { stroke: "black" }
                    }
                    div { class: "font-semibold text-[#222222] text-[20px]", "{tr.title}" }
                }
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
                                blocked: true,
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
                                blocked: true,
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
                                blocked: true,
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
                                blocked: true,
                            }
                        }
                    }
                }
            }
        }
    }
}
