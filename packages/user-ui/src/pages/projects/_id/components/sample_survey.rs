#![allow(non_snake_case, dead_code, unused_variables)]
use bdk::prelude::*;
use indexmap::IndexMap;
use models::{
    deliberation_response::{DeliberationResponse, DeliberationType},
    deliberation_survey::DeliberationSurvey,
    response::Answer,
    ParsedQuestion, Question, SurveyV2,
};

use crate::{
    pages::projects::_id::components::{
        my_sample_survey::MySampleSurvey, remove_survey_modal::RemoveSurveyModal,
        sample_statistics::SampleStatistics, sample_survey_info::SampleSurveyInfo,
        sample_survey_question::SampleSurveyQuestion,
    },
    service::{popup_service::PopupService, user_service::UserService},
    utils::time::current_timestamp,
};

use super::remove_survey_modal::RemoveSurveyModalTranslate;

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    let steps = survey.clone().steps;

    let mut start_date = 0;
    let mut end_date = 0;

    if steps.len() == 5 {
        start_date = steps[1].started_at;
        end_date = steps[1].ended_at;
    }

    let step = ctrl.survey_step();

    rsx! {
        div {
            id: "sample-survey",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit justify-center items-center",
            ..attributes,

            if step == SurveyStep::Display {
                SampleSurveyInfo {
                    lang,
                    survey,
                    start_date,
                    end_date,
                    survey_completed: ctrl.survey_completed(),
                    onchange: move |step| {
                        ctrl.set_step(step);
                    },
                }
            } else if step == SurveyStep::WriteSurvey {
                SampleSurveyQuestion {
                    lang,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onsend: move |_| async move {
                        ctrl.send_sample_response().await;
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                }
            } else if step == SurveyStep::MySurvey {
                MySampleSurvey {
                    lang,
                    start_date,
                    end_date,
                    survey: if survey.surveys.len() != 0 { survey.surveys[0].clone() } else { SurveyV2::default() },
                    answers: ctrl.answers(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onchange: move |(index, answer)| {
                        ctrl.change_answer(index, answer);
                    },
                    onupdate: move |_| async move {
                        ctrl.update_sample_response().await;
                        ctrl.set_step(SurveyStep::Display);
                    },
                    onremove: move |_| async move {
                        ctrl.open_remove_sample_modal();
                    },
                }
            } else {
                SampleStatistics {
                    lang,
                    responses: ctrl.survey_responses(),
                    onprev: move |_| {
                        ctrl.set_step(SurveyStep::Display);
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

    survey_completed: Signal<bool>,
    response_id: Signal<i64>,

    pub user: UserService,
    pub survey_responses: Signal<SampleSurveyResponses>,
    popup_service: PopupService,
    survey_step: Signal<SurveyStep>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SampleSurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
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
            survey_responses: use_signal(|| SampleSurveyResponses::default()),
            popup_service: use_context(),
            survey_step: use_signal(|| SurveyStep::Display),
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

            let questions = if (ctrl.survey)().unwrap_or_default().surveys.is_empty() {
                vec![]
            } else {
                (ctrl.survey)().unwrap_or_default().surveys[0]
                    .clone()
                    .questions
            };
            let responses = (ctrl.survey)().unwrap_or_default().responses;

            let survey_responses = SampleSurveyResponses {
                answers: ctrl
                    .clone()
                    .parsing_sample_answers(questions.clone(), responses.clone()),
            };

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

            ctrl.survey_responses.set(survey_responses);
            ctrl.answers.set(answers);
            ctrl.survey_completed.set(completed);
            ctrl.response_id.set(response_id);
        });

        Ok(ctrl)
    }

    pub fn parsing_sample_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
            if response.deliberation_type == DeliberationType::Survey {
                continue;
            }

            for (i, answer) in response.answers.iter().enumerate() {
                let questions = questions.clone();
                let question = &questions[i];
                let title = question.title();

                let parsed_question: ParsedQuestion = (question, answer).into();

                survey_maps
                    .entry(i as i64)
                    .and_modify(|survey_data| match &mut survey_data.1 {
                        ParsedQuestion::SingleChoice { response_count, .. } => {
                            if let Answer::SingleChoice { answer } = answer {
                                response_count[(answer - 1) as usize] += 1;
                            }
                        }
                        ParsedQuestion::MultipleChoice { response_count, .. } => {
                            if let Answer::MultipleChoice { answer } = answer {
                                for ans in answer {
                                    response_count[(ans - 1) as usize] += 1;
                                }
                            }
                        }
                        ParsedQuestion::ShortAnswer { answers } => {
                            if let Answer::ShortAnswer { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                        ParsedQuestion::Subjective { answers } => {
                            if let Answer::Subjective { answer } = answer {
                                answers.push(answer.clone());
                            }
                        }
                    })
                    .or_insert_with(|| (title, parsed_question.clone()));
            }
        }

        survey_maps
    }

    pub fn set_step(&mut self, step: SurveyStep) {
        self.survey_step.set(step);
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
                self.set_step(SurveyStep::Display);
            }
            Err(e) => {
                btracing::error!("update response failed with error: {:?}", e);
            }
        }
    }

    pub fn open_remove_sample_modal(&mut self) {
        let mut popup_service = self.popup_service;
        let mut ctrl = self.clone();
        let lang = self.lang;
        let tr: RemoveSurveyModalTranslate = translate(&lang);

        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang,
                    onclose: move |_| {
                        popup_service.close();
                    },
                    onremove: move |_| async move {
                        ctrl.remove_sample_response().await;
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_sample")
            .with_title(tr.title);
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
    necessary: {
        ko: "[필수]",
        en: "[Necessary]"
    }
    plural: {
        ko: "[복수]",
        en: "[Plural]"
    }
    unit: {
        ko: "명",
        en: "Unit"
    }
    subjective_answer: {
        ko: "주관식 답변",
        en: "Subjective Answer"
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
