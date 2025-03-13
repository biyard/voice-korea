#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::{edit::Edit, user::User};
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::Tab;

use crate::components::icons::triangle::{TriangleDown, TriangleUp};

#[derive(Debug, Translate, Clone, PartialEq, Default)]
pub enum SurveyStatus {
    #[default]
    #[translate(ko = "조사가 준비중입니다", en = "The survey is being prepared")]
    Draft,
    #[translate(ko = "조사 참여하기 ", en = "Participate in a survey")]
    InProgress,
    #[translate(ko = "조사가 마감되었습니다", en = "The survey has closed")]
    Finished,
}

impl SurveyStatus {
    pub fn button_class(&self) -> &'static str {
        match self {
            SurveyStatus::Draft | SurveyStatus::Finished => "bg-[#B4B4B4] text-white ",
            SurveyStatus::InProgress => "bg-[#8095EA] text-white cursor-pointer",
        }
    }

    pub fn button_text(&self) -> &'static str {
        match self {
            SurveyStatus::Draft => "조사가 준비중입니다",
            SurveyStatus::InProgress => "조사 참여하기",
            SurveyStatus::Finished => "조사가 마감되었습니다",
        }
    }

    pub fn is_clickable(&self) -> bool {
        matches!(self, SurveyStatus::InProgress)
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    survey_status: Signal<SurveyStatus>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let survey_status = Signal::new(SurveyStatus::InProgress);
        Ok(Self {
            lang,
            project_id,
            survey_status,
        })
    }

    pub fn set_survey_status(&mut self, status: SurveyStatus) {
        self.survey_status.set(status);
    }

    pub fn get_survey_status(&self) -> SurveyStatus {
        self.survey_status.read().clone()
    }
}

#[component]
pub fn SampleSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id).unwrap();
    let survey_status = use_signal(|| ctrl.get_survey_status());
    let tr: SampleSurveyTranslate = translate(&lang);
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);
    let tab_title: &str = Tab::SampleSurvey.translate(&lang);

    rsx! {
        div { class: "w-full h-auto bg-[#F7F7F7] flex flex-col gap-[20px] pb-[40px]",
            // Header
            div { class: "w-full h-[32px] flex flex-row justify-between items-center",
                p { class: "w-[150px] h-[32px] mt-[28px] font-semibold text-[20px]",
                    "{tab_title}"
                }
                div { class: "w-full h-[32px] flex justify-end gap-[80px]",
                    div { class: "flex flex-row justify-start items-center gap-[20px]",
                        div { class: "text-[15px]", "2월 14일 2025년" }
                        div { class: "flex flex-row items-center gap-[4px]",
                            span { "6" }
                            Edit { class: "&>path]:stroke-[#555462]" }
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
                            span { "4" }
                            User { class: "&>path]:stroke-[#555462]" }
                        }
                    }
                }
            }

            // Introduction Section
            div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                div {
                    class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                    onclick: move |_| {
                        clicked1.set(!(*clicked1)());
                        clicked2.set(false);
                    },
                    div { class: "w-full flex flex-row justify-between items-center",
                        span { "{tr.main_title}" }
                        if (*clicked1)() {
                            TriangleUp {}
                        } else {
                            TriangleDown {}
                        }
                    }
                }
                if (*clicked1)() {
                    hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                    div { class: "w-full justify-start mt-[15px] mb-[20px] font-bold text-[18px]",
                        "제목 구간입니다(Title)."
                    }
                    div { class: "w-full flex justify-start text-[15px]",
                        "내용 구간입니다(details)."
                    }
                }
            }

            // Survey Button
            div { class: "w-full flex flex-row justify-center items-center",
                button {
                    class: format!(
                        "w-[200px] h-[50px] flex justify-center items-center rounded-[8px] px-[14px] py-[16px] {}",
                        survey_status().button_class(),
                    ),
                    disabled: !survey_status().is_clickable(),
                    //TODO(web): connect to survey page.
                    onclick: move |_| {
                        if survey_status().is_clickable() {
                            println!("조사 참여 페이지로 이동!");
                        }
                    },
                    "{survey_status().button_text()}"
                }
            }
        }
    }
}

translate! {
    SampleSurveyTranslate;

    main_title: {
        ko: "표본 조사 주제",
        en: "Sample survey topic"
    }
}
