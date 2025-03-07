use crate::{
    pages::project::{
        components::sample::{
            my_sample_survey::MySurvey, sample_survey::Survey, survey_info::SurveyInfo,
        },
        controller,
    },
    utils::time::current_timestamp,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language, Translate};

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SurveyStep {
    Display,
    WriteSurvey,
    MySurvey,
    Statistics,
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
    let mut survey_clicked: Signal<SurveyStep> = use_signal(|| SurveyStep::Display);

    let deliberation = ctrl.get_deliberation();
    let survey = deliberation.surveys.get(0).unwrap().clone();
    let members = deliberation.members;
    let answers = ctrl.answers();

    let check_edit = ctrl.check_edit();

    rsx! {
        div { class: "flex flex-col w-full",
            //FIXME: fix to use div display attribute
            if survey_clicked() == SurveyStep::WriteSurvey {
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
                        survey_clicked.set(SurveyStep::Display);
                    },
                }
            } else if survey_clicked() == SurveyStep::Display {
                SurveyInfo {
                    lang,
                    survey: survey.clone(),
                    members,
                    check_edit,
                    onchange: move |step: SurveyStep| {
                        survey_clicked.set(step);
                    },
                }
            } else if survey_clicked() == SurveyStep::MySurvey {
                MySurvey {
                    lang,
                    survey: survey.clone(),
                    answers,
                    onprev: move |_| {
                        survey_clicked.set(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onupdate: move |_| {
                        ctrl.update_sample_survey(lang);
                    },
                    onremove: move |_| {
                        ctrl.remove_sample_survey(lang);
                    },
                }
            }
        }
    }
}
