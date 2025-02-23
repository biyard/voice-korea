use dioxus::prelude::*;

use dioxus_translate::{translate, Language};
use models::prelude::Question;

use crate::{
    components::icons::{Minus, Plus, RowMenuDial, Trash},
    pages::surveys::{
        components::type_selection::QuestionTypeSelector,
        i18n::{ObjectiveTranslate, QuestionListViewTranslate, SubjectiveTranslate},
        new::i18n::AddQuestionTranslate,
    },
};
// onchange: move |(index, survey): (usize, Question)| {
//     questions.with_mut(move |q| q[index] = survey);
// },
// onremove: move |index: usize| {
//     questions.remove(index);
// },

#[component]
pub fn QuestionListView(
    lang: Language,
    questions: Signal<Vec<Question>>,
    onchange: EventHandler<Vec<Question>>,
    // onchange: EventHandler<(usize, Question)>,
    // onremove: EventHandler<usize>,
) -> Element {
    let tr: QuestionListViewTranslate = translate(&lang);

    rsx! {
        for index in 0..questions.len() {
            div {
                class: "flex flex-col w-full justify-start items-start pt-[5px] px-[40px] pb-[25px] bg-white rounded-[8px] mt-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",

                div { class: "flex flex-row w-full justify-center items-center mb-[10px]",
                    RowMenuDial { width: "24", height: "24" }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-start items-center",
                        QuestionTypeSelector {
                            selected: questions()[index].to_type(&lang),
                            lang,
                            onchange: {
                                move |qtype: String| {
                                    questions()[index].to_type(&lang);
                                    questions
                                        .with_mut(move |q| {
                                            q[index] = Question::new(&qtype);
                                        });
                                }
                            },
                        }

                        input {
                            class: format!(
                                "flex flex-row flex-1 h-[55px] justify-start items-center bg-[#f7f7f7] focus:outline-none px-[15px] py-[10px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] rounded-[4px]",
                            ),
                            r#type: "text",
                            placeholder: "{tr.input_title}",
                            value: questions()[index].title(),
                            oninput: move |e: Event<FormData>| {
                                questions
                                    .with_mut(move |q| {
                                        q[index].set_title(&e.value());
                                    });
                            },
                        }
                    }

                    if matches!(questions()[index], Question::ShortAnswer(_) | Question::Subjective(_)) {
                        Subjective {
                            lang,
                            onchange: move |v: Question| {
                                questions.with_mut(move |q| q[index] = v);
                            },
                            onremove: move |_| {
                                questions.remove(index);
                            },
                            question: questions()[index].clone(),
                        }
                    } else {
                        Objective {
                            lang,
                            onchange: move |v: Question| {
                                questions.with_mut(move |q| q[index] = v);
                            },
                            onremove: move |_| {
                                questions.remove(index);
                            },
                            question: questions()[index].clone(),
                        }
                    }
                }
            
            }
        }

        button {
            class: "flex flex-row w-full",
            onclick: move |_| {
                questions.with_mut(|q| q.push(Question::default()));
            },
            AddQuestion { lang }
        }
    }
}

#[component]
pub fn AddQuestion(lang: Language) -> Element {
    let translates: AddQuestionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full h-[200px] rounded-[8px] justify-center items-center border border-dashed border-[#b4b4b4] mt-[20px]",
            div { class: "flex flex-row w-[45px] h-[45px] justify-center items-center rounded-[100px] border border-[#b4b4b4]",
                Plus { width: "12", height: "12", color: "#b4b4b4" }
            }
            div { class: "mt-[10px] font-medium text-[15px] text-[#b4b4b4] leading-[22px]",
                "{translates.add_description}"
            }
        }
    }
}

#[component]
pub fn Objective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let options = question.options();
    let tr: ObjectiveTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            for (index , option) in options.iter().enumerate() {
                div { class: "flex flex-row w-full justify-start items-center mt-[10px]",
                    div { class: "flex flex-row w-[20px] h-[20px] border-[3px] bg-white border-[#9f9f9f] mr-[10px] rounded-[100px]" }
                    input {
                        class: "flex flex-row w-[888px] h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#9f9f9f] px-[15px] py-[15px] font-medium text-[#9f9f9f] text-[15px] leading-[22px] mr-[10px]",
                        r#type: "text",
                        placeholder: format!("{} {}", tr.option, index + 1),
                        value: option.clone(),
                        oninput: {
                            let mut question = question.clone();
                            move |e: Event<FormData>| {
                                question.change_option(index, &e.value());
                                onchange.call(question.clone());
                            }
                        },
                    }
                    button {
                        onclick: {
                            let mut question = question.clone();
                            move |_| {
                                question.remove_option(index);
                                onchange.call(question.clone());
                            }
                        },
                        Minus { width: "20", height: "20" }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-start items-center mt-[30px]",
                div { class: "flex flex-row w-[20px] h-[20px] border-[3px] bg-white border-[#9f9f9f] mr-[10px] rounded-[100px]" }
                button {
                    class: "font-medium text-[16px] text-[#3a94ff]",
                    onclick: {
                        let mut question = question.clone();
                        move |_| {
                            question.add_option("");
                            onchange.call(question.clone());
                        }
                    },
                    "{tr.add_option}"
                }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[5px] mt-[10px]",
                button {
                    class: "flex flex-row w-[80px] items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-[#222222] text-[15px]", "{tr.remove}" }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}

#[component]
pub fn Subjective(
    lang: Language,
    onchange: EventHandler<Question>,
    onremove: EventHandler<MouseEvent>,
    question: Question,
) -> Element {
    let tr: SubjectiveTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }

            input {
                class: "flex flex-row w-full h-[55px] justify-start items-center bg-white focus:outline-none border-b-[1px] border-[#bfc8d9] px-[15px] py-[15px] font-medium text-[#b4b4b4] text-[15px] leading-[22px] mb-[20px]",
                r#type: "text",
                placeholder: tr.input_description_hint,
                value: question.description(),
                oninput: move |e: Event<FormData>| {
                    question.set_description(&e.value());
                    onchange.call(question.clone());
                },
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[5px]",
                button {
                    class: "flex flex-row w-[80px] items-center justify-end",
                    onclick: move |e: Event<MouseData>| {
                        onremove.call(e);
                    },
                    div { class: "font-medium text-[#222222] text-[15px]", "{tr.remove}" }
                    Trash { width: "18", height: "18" }
                }
            }
        }
    }
}
