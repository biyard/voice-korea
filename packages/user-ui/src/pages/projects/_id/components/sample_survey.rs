#![allow(non_snake_case, dead_code, unused_variables)]
use by_macros::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;
use models::{deliberation_survey::DeliberationSurvey, response::Answer, Question, SurveyV2};

use crate::{
    pages::projects::_id::components::{
        sample_survey_info::SampleSurveyInfo, sample_survey_question::SampleSurveyQuestion,
    },
    utils::time::current_timestamp,
};

#[derive(Translate, PartialEq, Default, Debug)]
pub enum SurveyStatus {
    #[default]
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SurveyStep {
    Display,
    WriteSurvey,
    MySurvey,
    Statistics,
}

#[component]
pub fn SampleSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctrl = Controller::new(lang, project_id)?;
    let survey = ctrl.survey()?;
    let mut survey_step: Signal<SurveyStep> = use_signal(|| SurveyStep::Display);

    rsx! {
        div {
            id: "sample-survey",
            class: "flex flex-col w-full h-fit justify-center items-center",
            ..attributes,

            if survey_step() == SurveyStep::Display {
                SampleSurveyInfo {
                    lang,
                    survey,
                    onchange: move |step| {
                        survey_step.set(step);
                    },
                }
            } else {
                SampleSurveyQuestion {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        survey_step.set(SurveyStep::Display);
                    },
                    onsend: move |_| {
                        tracing::debug!("answers: {:?}", ctrl.answers());
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            }
        
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    survey: Resource<DeliberationSurvey>,
    answers: Signal<Vec<Answer>>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let survey = use_server_future(move || async move {
            DeliberationSurvey::get_client(&crate::config::get().api_url)
                .read(project_id())
                .await
                .unwrap_or_default()
        })?;

        let mut ctrl = Self {
            lang,
            project_id,
            survey,

            answers: use_signal(|| vec![]),
        };

        use_effect(move || {
            let surveys = (ctrl.survey)().unwrap_or_default().surveys;
            let survey = if surveys.len() == 0 {
                SurveyV2::default()
            } else {
                surveys[0].clone()
            };

            let answers = survey
                .questions
                .iter()
                .map(|question| match question {
                    Question::SingleChoice(_) => Answer::SingleChoice { answer: 0 },
                    Question::MultipleChoice(_) => Answer::MultipleChoice { answer: vec![] },
                    Question::ShortAnswer(_) => Answer::ShortAnswer {
                        answer: "".to_string(),
                    },
                    Question::Subjective(_) => Answer::Subjective {
                        answer: "".to_string(),
                    },
                })
                .collect::<Vec<_>>();

            ctrl.answers.set(answers);
        });

        Ok(ctrl)
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }
}

translate! {
    SampleSurveyTranslate;

    title: {
        ko: "표본 조사 주제",
        en: "Sample Survey Title",
    },
    submit: {
        ko: "제출하기",
        en: "Submit"
    }
}

pub fn get_survey_status(started_at: i64, ended_at: i64) -> SurveyStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return SurveyStatus::default();
    }

    if started_at > current {
        SurveyStatus::Ready
    } else if ended_at < current {
        SurveyStatus::Finish
    } else {
        SurveyStatus::InProgress
    }
}
