#![allow(non_snake_case, dead_code, unused_variables)]
use bdk::prelude::*;
use models::{
    deliberation_response::{DeliberationResponse, DeliberationType},
    deliberation_survey::DeliberationSurvey,
    response::Answer,
    Question, SurveyV2,
};

use crate::{
    pages::projects::_id::components::{
        my_sample_survey::MySampleSurvey, sample_survey_info::SampleSurveyInfo,
        sample_survey_question::SampleSurveyQuestion,
    },
    service::user_service::UserService,
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
                    survey_completed: ctrl.survey_completed(),
                    onchange: move |step| {
                        survey_step.set(step);
                    },
                }
            } else if survey_step() == SurveyStep::WriteSurvey {
                SampleSurveyQuestion {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        survey_step.set(SurveyStep::Display);
                    },
                    onsend: move |_| async move {
                        ctrl.send_sample_response().await;
                        survey_step.set(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else if survey_step() == SurveyStep::MySurvey {
                MySampleSurvey {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        survey_step.set(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onupdate: move |_| async move {
                        ctrl.update_sample_response().await;
                        survey_step.set(SurveyStep::Display);
                    },
                    onremove: move |_| async move {
                        ctrl.remove_sample_response().await;
                        survey_step.set(SurveyStep::Display);
                    },
                }
            } else {

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

    survey_completed: Signal<bool>,
    response_id: Signal<i64>,

    pub user: UserService,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

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
            survey_completed: use_signal(|| false),
            response_id: use_signal(|| 0),

            user,
        };

        use_effect(move || {
            let surveys = (ctrl.survey)().unwrap_or_default().surveys;
            let survey = if surveys.len() == 0 {
                SurveyV2::default()
            } else {
                surveys[0].clone()
            };

            let mut answers = vec![];
            let mut completed = false;
            let mut response_id = 0;

            let user_id = (ctrl.user.user_id)();

            for response in (ctrl.survey)().unwrap_or_default().responses {
                if response.deliberation_type == DeliberationType::Sample
                    && response.user_id == user_id
                {
                    answers = response.answers;
                    completed = true;
                    response_id = response.id;
                }
            }

            if answers.len() == 0 {
                answers = survey
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
            }

            ctrl.answers.set(answers);
            ctrl.survey_completed.set(completed);
            ctrl.response_id.set(response_id);
        });

        Ok(ctrl)
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }

    pub async fn remove_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.project_id)();
        let response_id = (self.response_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .remove_respond_answer(deliberation_id, response_id)
            .await
        {
            Ok(_) => {
                self.survey.restart();
            }
            Err(e) => {
                btracing::error!("update response failed with error: {:?}", e);
            }
        }
    }

    pub async fn update_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.project_id)();
        let response_id = (self.response_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers = self.answers();

        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .update_respond_answer(deliberation_id, response_id, answers)
            .await
        {
            Ok(_) => {
                self.survey.restart();
            }
            Err(e) => {
                btracing::error!("update response failed with error: {:?}", e);
            }
        };
    }

    pub async fn send_sample_response(&mut self) {
        let user_id = (self.user.user_id)();
        let deliberation_id = (self.project_id)();

        if user_id == 0 {
            btracing::error!("login is required");
            return;
        }

        let answers = self.answers();

        match DeliberationResponse::get_client(&crate::config::get().api_url)
            .respond_answer(
                deliberation_id,
                answers,
                models::deliberation_response::DeliberationType::Sample,
            )
            .await
        {
            Ok(_) => {
                self.survey.restart();
            }
            Err(e) => {
                btracing::error!("send response failed with error: {:?}", e);
            }
        };
    }
}

translate! {
    SampleSurveyTranslate;

    title: {
        ko: "표본 조사 주제",
        en: "Sample Survey Title",
    }
    see_detail: {
        ko: "자세히 보기",
        en: "See Detail"
    }
    my_answer: {
        ko: "내가 작성한 답변",
        en: "My Answer"
    }
    response_per_question: {
        ko: "질문별 응답",
        en: "Responses to each question"
    }
    update: {
        ko: "수정하기",
        en: "Update"
    }
    remove: {
        ko: "삭제하기",
        en: "Remove"
    }
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
