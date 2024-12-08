#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::{
    SurveyDraftStatus, SurveyQuestion, SurveyQuestionType, UpsertSurveyDraftRequest,
};

use crate::{
    api::v2::survey::{get_survey, upsert_survey_draft},
    models::survey::StatusType,
};

use super::Language;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QuestionStep {
    List,
    Input,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    step: Signal<QuestionStep>,
    pub survey_response: Resource<models::prelude::Survey>,
    question_types: Signal<Vec<QuestionOption>>,
    question_title: Signal<String>,
    selected_question_types: Signal<u64>,
    objective_questions: Signal<Vec<ObjectiveQuestionOption>>,
    deleted_key_list: Signal<Vec<usize>>,
    update_key: Signal<usize>,
    update_button_clicked: Signal<bool>,

    survey_id: Signal<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct QuestionOption {
    pub value: i64,
    pub label: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObjectiveQuestionOption {
    pub hint: String,
    pub text_value: String,
}

impl Controller {
    pub fn init(_lang: Language, id: String) -> Self {
        let id_copy = id.clone();
        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            async move {
                let survey = get_survey(id_value).await;
                survey.unwrap_or_default()
            }
        });

        let mut ctrl = Self {
            step: use_signal(|| QuestionStep::List),
            survey_response,
            question_title: use_signal(|| "".to_string()),
            question_types: use_signal(|| {
                vec![
                    QuestionOption {
                        value: 0,
                        label: "객관식 질문".to_string(),
                    },
                    QuestionOption {
                        value: 1,
                        label: "주관식 질문(단답형)".to_string(),
                    },
                    QuestionOption {
                        value: 2,
                        label: "주관식 질문(서술형)".to_string(),
                    },
                ]
            }),
            selected_question_types: use_signal(|| 0),
            objective_questions: use_signal(|| {
                vec![
                    ObjectiveQuestionOption {
                        hint: "옵션 1".to_string(),
                        text_value: "".to_string(),
                    },
                    ObjectiveQuestionOption {
                        hint: "옵션 2".to_string(),
                        text_value: "".to_string(),
                    },
                ]
            }),
            deleted_key_list: use_signal(|| vec![]),
            update_key: use_signal(|| 0),
            update_button_clicked: use_signal(|| false),
            survey_id: use_signal(|| "".to_string()),
        };

        ctrl.survey_id.set(id_copy);

        ctrl
    }

    pub fn get_survey_id(&self) -> String {
        (self.survey_id)()
    }

    pub fn clear_data(&mut self) {
        self.question_title.set("".to_string());
        self.selected_question_types.set(0);
        self.objective_questions.set(vec![
            ObjectiveQuestionOption {
                hint: "옵션 1".to_string(),
                text_value: "".to_string(),
            },
            ObjectiveQuestionOption {
                hint: "옵션 2".to_string(),
                text_value: "".to_string(),
            },
        ]);
        self.deleted_key_list.set(vec![]);
        self.update_button_clicked.set(false);
    }

    pub fn refresh_survey_response(&mut self) {
        (self.survey_response).restart();
    }

    pub fn get_question_title(&mut self) -> String {
        (self.question_title)()
    }

    pub fn get_update_key(&mut self) -> usize {
        (self.update_key)()
    }

    pub fn get_question_types(&mut self) -> Vec<QuestionOption> {
        (self.question_types)()
    }

    pub fn get_selected_question(&mut self) -> u64 {
        (self.selected_question_types)()
    }

    pub fn get_survey(&mut self) -> models::prelude::Survey {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }

    pub fn get_step(&mut self) -> QuestionStep {
        (self.step)()
    }

    pub fn get_objective_questions(&mut self) -> Vec<ObjectiveQuestionOption> {
        (self.objective_questions)()
    }

    pub fn delete_key_lists(&mut self) -> Vec<usize> {
        (self.deleted_key_list)()
    }

    pub fn change_question_title(&mut self, title: String) {
        self.question_title.set(title);
    }

    pub fn change_step(&mut self, step: QuestionStep) {
        self.step.set(step);
    }

    pub fn change_selected_question(&mut self, selected: u64) {
        self.selected_question_types.set(selected);
    }

    pub fn change_objective_question(&mut self, index: usize, text_value: String) {
        let mut objectives = self.get_objective_questions();
        objectives[index] = ObjectiveQuestionOption {
            hint: objectives[index].clone().hint,
            text_value,
        };
        self.objective_questions.set(objectives);
    }

    pub fn add_objective_question(&mut self) {
        let mut objectives = self.get_objective_questions();
        let objective_len = objectives.len();
        objectives.push(ObjectiveQuestionOption {
            hint: format!("옵션 {:?}", objective_len + 1),
            text_value: "".to_string(),
        });
        self.objective_questions.set(objectives);
    }

    pub fn remove_objective_question(&mut self, index: usize) {
        let mut objectives = self.get_objective_questions();
        objectives.remove(index);
        self.objective_questions.set(objectives);
    }

    pub async fn write_question(&mut self, _status: StatusType) {
        let selected_question_type = self.get_selected_question();
        let question_title = self.get_question_title();

        let mut questions = self.get_survey().questions;

        if (self.update_button_clicked)() {
            let question = questions.get((self.update_key)()).unwrap();
            let survey_question: SurveyQuestion = if selected_question_type == 0 {
                //객관식
                let object_options = self.get_objective_questions();
                let mut options: Vec<String> = vec![];

                for d in object_options {
                    options.push(d.text_value);
                }

                SurveyQuestion {
                    question_id: question.question_id,
                    title: question_title,
                    description: question.description.clone(),
                    answer_type: SurveyQuestionType::SingleChoice,
                    options: Some(options),
                }
            } else if selected_question_type == 1 {
                //단답형
                SurveyQuestion {
                    question_id: question.question_id,
                    title: question_title,
                    description: question.description.clone(),
                    answer_type: SurveyQuestionType::ShortAnswer,
                    options: None,
                }
            } else {
                //서술형
                SurveyQuestion {
                    question_id: question.question_id,
                    title: question_title,
                    description: question.description.clone(),
                    answer_type: SurveyQuestionType::LongAnswer,
                    options: None,
                }
            };

            questions[(self.update_key)()] = survey_question;

            tracing::debug!("questions title: {}", questions[(self.update_key)()].title);
        } else {
            let survey_question: SurveyQuestion = if selected_question_type == 0 {
                //객관식
                let object_options = self.get_objective_questions();
                let mut options: Vec<String> = vec![];

                for d in object_options {
                    options.push(d.text_value);
                }
                SurveyQuestion {
                    question_id: None,
                    title: question_title,
                    description: None,
                    answer_type: SurveyQuestionType::SingleChoice,
                    options: Some(options),
                }
            } else if selected_question_type == 1 {
                //단답형
                SurveyQuestion {
                    question_id: None,
                    title: question_title,
                    description: None,
                    answer_type: SurveyQuestionType::ShortAnswer,
                    options: None,
                }
            } else {
                //서술형
                SurveyQuestion {
                    question_id: None,
                    title: question_title,
                    description: None,
                    answer_type: SurveyQuestionType::LongAnswer,
                    options: None,
                }
            };

            questions.push(survey_question);
        };

        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Question),
            title: None,
            quotas: None,
            questions: Some(questions),
            started_at: None,
            ended_at: None,
        })
        .await;
    }

    pub async fn remove_question(&mut self, index: usize) {
        tracing::info!("remove question button clicked");

        let mut keys = (self.deleted_key_list)();
        keys.push(index);

        self.deleted_key_list.set(keys);
    }

    pub fn clicked_update_button(&mut self, index: usize) {
        self.update_key.set(index);
        self.update_button_clicked.set(true);
        let questions = self.get_survey().questions;
        let question = questions.get(index);

        let question_title = question.unwrap().title.clone();
        let options = question.unwrap().options.clone();
        self.question_title.set(question_title.clone());

        if question.unwrap().answer_type == SurveyQuestionType::SingleChoice {
            let mut os: Vec<ObjectiveQuestionOption> = vec![];

            for (i, d) in options.unwrap().iter().enumerate() {
                os.push(ObjectiveQuestionOption {
                    hint: format!("옵션 {}", i + 1),
                    text_value: d.to_string(),
                });
            }

            self.objective_questions.set(os);
            self.selected_question_types.set(0);
        } else if question.unwrap().answer_type == SurveyQuestionType::ShortAnswer {
            self.selected_question_types.set(1);
        } else {
            self.selected_question_types.set(2);
        }
    }

    pub async fn clicked_temporary_save(&mut self) {
        let keys = (self.deleted_key_list)();
        let questions = self.get_survey().questions;

        let mut unremoved_questions = vec![];

        for (i, d) in questions.iter().enumerate() {
            if keys.contains(&i) {
                continue;
            }

            unremoved_questions.push(d.clone());
        }

        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Question),
            title: None,
            quotas: None,
            questions: Some(unremoved_questions),
            started_at: None,
            ended_at: None,
        })
        .await;

        self.deleted_key_list.set(vec![]);
    }

    pub async fn clicked_save(&mut self) {
        let keys = (self.deleted_key_list)();
        let questions = self.get_survey().questions;

        let mut unremoved_questions = vec![];

        for (i, d) in questions.iter().enumerate() {
            if keys.contains(&i) {
                continue;
            }

            unremoved_questions.push(d.clone());
        }

        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Quotas),
            title: None,
            quotas: None,
            questions: Some(unremoved_questions),
            started_at: None,
            ended_at: None,
        })
        .await;
    }

    pub async fn clicked_back(&mut self) {
        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Title),
            title: None,
            quotas: None,
            questions: None,
            started_at: None,
            ended_at: None,
        })
        .await;
    }
}
