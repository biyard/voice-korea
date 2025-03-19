use bdk::prelude::*;
use by_components::icons::validations::Extra;
use models::{response::Answer, Question, SurveyV2};

use crate::{
    components::icons::left_arrow::LeftArrow,
    pages::projects::_id::components::{
        multiple_objective::MultipleObjective, sample_survey::SampleSurveyTranslate,
        single_objective::SingleObjective, subjective::Subjective,
    },
};

#[component]
pub fn MySampleSurvey(
    lang: Language,
    survey: SurveyV2,
    answers: Vec<Answer>,
    onprev: EventHandler<MouseEvent>,
    onchange: EventHandler<(usize, Answer)>,
    onupdate: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let tr: SampleSurveyTranslate = translate(&lang);
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

                div { class: "group relative",
                    div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                        button { class: "cursor-pointer", Extra {} }
                        nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                            ul { class: "py-1",
                                li {
                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                    onclick: move |e: Event<MouseData>| {
                                        onupdate.call(e);
                                    },
                                    "수정하기"
                                }
                                li {
                                    class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                    onclick: move |e: Event<MouseData>| {
                                        onremove.call(e);
                                    },
                                    "삭제하기"
                                }
                            }
                        }
                    }
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
        }
    }
}
