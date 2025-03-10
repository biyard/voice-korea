use crate::{
    components::icons::{
        left_arrow::LeftArrow,
        person::Person,
        triangle::{TriangleDown, TriangleUp},
    },
    pages::projects::_id::{
        components::{
            multiple_objective::MultipleObjective, single_objective::SingleObjective,
            subjective::Subjective,
        },
        controller,
        i18n::{SampleTranslate, SurveyTranslate},
    },
    utils::time::{current_timestamp, formatted_timestamp},
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language, Translate};
use models::{deliberation_user::DeliberationUser, response::Answer, Question, SurveyV2};

#[derive(Translate, PartialEq)]
pub enum SurveyStatus {
    #[translate(ko = "조사가 준비중입니다.", en = "The investigation is underway.")]
    Ready,
    #[translate(ko = "조사 참여하기", en = "Take part in the survey")]
    InProgress,
    #[translate(
        ko = "조사가 마감되었습니다.",
        en = "The investigation has been closed."
    )]
    Finish,
}

pub fn get_survey_status(started_at: i64, ended_at: i64) -> SurveyStatus {
    let current = current_timestamp();

    if started_at > current {
        SurveyStatus::Ready
    } else if ended_at < current {
        SurveyStatus::Finish
    } else {
        SurveyStatus::InProgress
    }
}

#[component]
pub fn Sample(lang: Language) -> Element {
    let mut ctrl = controller::SampleController::init(lang)?;
    let mut survey_clicked = use_signal(|| false);

    let deliberation = ctrl.get_deliberation();
    let survey = deliberation.surveys.get(0).unwrap().clone();
    let members = deliberation.members;
    let answers = ctrl.answers();

    let check_edit = ctrl.check_edit();

    rsx! {
        div { class: "flex flex-col w-full",
            //FIXME: fix to use div display attribute
            if survey_clicked() {
                Survey {
                    lang,
                    survey,
                    answers,
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onsend: move |_| {
                        ctrl.send_sample_survey(lang);
                    },
                    onprev: move |_| {
                        survey_clicked.set(false);
                    },
                }
            } else {
                SurveyInfo {
                    lang,
                    survey: survey.clone(),
                    members,
                    check_edit,
                    survey_clicked: survey_clicked(),
                    onchange: move |v: bool| {
                        survey_clicked.set(v);
                    },
                }
            }
        }
    }
}

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

#[component]
pub fn SurveyInfo(
    lang: Language,
    survey: SurveyV2,
    members: Vec<DeliberationUser>,

    check_edit: bool,
    survey_clicked: bool,

    onchange: EventHandler<bool>,
) -> Element {
    let editor = 4; //FIXME: fix to connect model data

    let mut clicked = use_signal(|| false);
    let status = get_survey_status(survey.started_at, survey.ended_at);
    let tr: SampleTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-[20px]",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "font-semibold text-[#222222] text-[20px]", "{tr.sample_survey}" }
                div { class: "flex flex-row justify-start items-center gap-[80px]",
                    div { class: "font-medium text-black text-[15px]",
                        {
                            format!(
                                "{} ~ {}",
                                formatted_timestamp(survey.started_at),
                                formatted_timestamp(survey.ended_at),
                            )
                        }
                    }
                    div { class: "flex flex-row justify-center items-center gap-[20px]",
                        div { class: "relative flex items-center",
                            img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full z-10" }
                            img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-20" }
                            img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full -ml-2 z-30" }
                            img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-40" }
                        }
                        div { class: "flex flex-row items-center gap-[4px]",
                            //count
                            span { "{editor}" }
                            Person {}
                        }
                    }
                }
            }

            div {
                class: "flex flex-col w-full gap-[20px]",
                display: if check_edit { "none" } else { "flex" },
                div { class: "flex flex-col w-full rounded-[8px] bg-[#ffffff] justify-start items-start py-[14px] px-[20px] gap-[10px]",
                    div {
                        class: "flex flex-col w-full  justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| clicked.set(!clicked()),
                        div { class: "w-full flex flex-row justify-between items-center",
                            div { class: "font-bold text-[#222222] text-[16px]", "{survey.name}" }
                            if clicked() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }

                        div {
                            class: "flex flex-col w-full",
                            display: if clicked() { "flex" } else { "none" },
                            div { class: "w-full h-[1px] bg-[#eeeeee] my-[12px]" }
                            div { class: "flex flex-col w-full gap-[20px]",
                                div { class: "font-bold text-[18px] text-black", "{survey.description}" }
                                div { class: "flex flex-col w-full",
                                    for (i , question) in survey.questions.iter().enumerate() {
                                        div { class: "font-normal text-[15px] text-black",
                                            "{i + 1}. {question.title()}"
                                        }
                                    }
                                }
                                div { class: "flex flex-wrap w-full gap-[40px]",
                                    for member in members {
                                        div { class: "flex flex-row gap-[8px] justify-start items-center",
                                            div { class: "w-[40px] h-[40px] rounded-[100px] bg-[#d9d9d9]" }
                                            div { class: "font-semibold text-[12px] text-[#222222]",
                                                {member.role.translate(&lang)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex flex-row w-full justify-center",
                    div {
                        class: format!(
                            "flex flex-row px-[15px] py-[13px] {} rounded-[8px] font-bold text-white text-[16px]",
                            if status == SurveyStatus::InProgress {
                                "bg-[#8095EA] cursor-pointer"
                            } else {
                                "bg-[#B4B4B4]"
                            },
                        ),
                        onclick: move |_| {
                            if status == SurveyStatus::InProgress {
                                onchange.call(true);
                            }
                        },
                        {status.translate(&lang)}
                    }
                }
            }
        }
    }
}
