#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use indexmap::IndexMap;
use models::{
    deliberation::Deliberation,
    deliberation_project::DeliberationProject,
    deliberation_response::{DeliberationResponse, DeliberationType},
    deliberation_user::DeliberationUser,
    deliberation_vote::DeliberationVote,
    response::Answer,
    step::Step,
    step_type::StepType,
    ChoiceQuestion, PanelCountsV2, PanelV2, ParsedQuestion, Question, ResourceFile, ResourceType,
    SubjectiveQuestion, SurveyV2,
};

use crate::{
    pages::projects::_id::components::sample::{
        not_complete_survey_modal::NotCompleteSurveyModal, remove_survey_modal::RemoveSurveyModal,
    },
    service::popup_service::{self, PopupService},
    utils::time::formatted_timestamp_to_sec,
};

use super::i18n::ProjectTranslate;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SurveyResponses {
    pub answers: IndexMap<i64, (String, ParsedQuestion)>, // question_id, (title, response_count, <panel_id, answer>)
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    id: ReadOnlySignal<i64>,

    #[allow(dead_code)]
    summary: Resource<DeliberationProject>,

    answers: Signal<Vec<Answer>>,
    // NOTE: Whether I have ever filled out a survey
    // NOTE: In the future, it will be linked to the API and the relevant part should be checked.
    check_edit: Signal<bool>,
    pub survey_responses: Signal<SurveyResponses>,
}

impl Controller {
    pub fn init(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let summary = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().api_url;
                DeliberationProject::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let mut ctrl = Self {
            answers: use_signal(|| vec![]),
            check_edit: use_signal(|| false),
            survey_responses: use_signal(|| SurveyResponses::default()),

            lang,
            id,
            summary,
        };

        use_context_provider(|| ctrl);

        let questions = ctrl.clone().get_deliberation().surveys[0].questions.clone();
        let responses = ctrl.clone().get_deliberation_responses();

        //FIXME: After connecting the API, you need to check whether the relevant part is working properly.
        let memoized_answers = use_memo({
            let questions = questions.clone();
            move || {
                questions
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
                    .collect::<Vec<_>>()
            }
        });

        let memoized_survey_responses = use_memo({
            let questions = questions.clone();
            let responses = responses.clone();
            move || SurveyResponses {
                answers: ctrl.parsing_answers(questions.clone(), responses.clone()),
            }
        });

        let mut prev_questions = use_signal(|| vec![]);
        let mut prev_responses = use_signal(|| vec![]);

        use_effect(move || {
            if *prev_questions() != questions || *prev_responses() != responses {
                ctrl.answers.set(memoized_answers());
                ctrl.survey_responses.set(memoized_survey_responses());
                ctrl.check_edit.set(true);

                prev_questions.set(questions.clone());
                prev_responses.set(responses.clone());
            }
        });

        Ok(ctrl)
    }

    pub fn parsing_answers(
        &self,
        questions: Vec<Question>,
        responses: Vec<DeliberationResponse>,
    ) -> IndexMap<i64, (String, ParsedQuestion)> {
        let mut survey_maps: IndexMap<i64, (String, ParsedQuestion)> = IndexMap::new();

        for response in responses {
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

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }

    pub fn check_edit(&self) -> bool {
        (self.check_edit)()
    }

    pub fn answers(&self) -> Vec<Answer> {
        (self.answers)()
    }

    pub fn remove_sample_survey(&self, lang: Language) {
        let tr: ProjectTranslate = translate(&lang);
        let mut popup_service: PopupService = use_context();
        let survey_id = self.get_deliberation().id;

        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang,
                    onclose: move |_| {
                        popup_service.close();
                    },
                    onremove: move |_| {
                        tracing::debug!("remove survey answer");
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_survey")
            .with_title(tr.remove_modal_title);

        tracing::debug!("remove survey answer: {}", survey_id);
    }

    pub fn update_sample_survey(&self, lang: Language) {
        //TODO: update survey answer value to use API
        let answers = self.answers();
        let survey_id = self.get_deliberation().id;
        tracing::debug!("update survey answer: {} {:?}", survey_id, answers);
    }

    pub fn send_sample_survey(&self, lang: Language) {
        let tr: ProjectTranslate = translate(&lang);
        let mut popup_service: PopupService = use_context();
        let mut is_empty = false;
        let answers = self.answers();
        let ended_at = self.get_deliberation().ended_at;
        let formatted_ended_at = formatted_timestamp_to_sec(ended_at);
        let description = self.sample_survey_modal_description(lang, formatted_ended_at);

        for answer in answers {
            match answer {
                Answer::SingleChoice { answer } => {
                    if answer == 0 {
                        is_empty = true;
                        break;
                    }
                }
                Answer::MultipleChoice { answer } => {
                    if answer.len() == 0 {
                        is_empty = true;
                        break;
                    }
                }
                Answer::ShortAnswer { answer } => {
                    if answer == "" {
                        is_empty = true;
                        break;
                    }
                }
                Answer::Subjective { answer } => {
                    if answer == "" {
                        is_empty = true;
                        break;
                    }
                }
            }
        }

        if is_empty {
            popup_service
                .open(rsx! {
                    NotCompleteSurveyModal {
                        lang,
                        description,
                        onclose: move |_| {
                            popup_service.close();
                        },
                        onsave: move |_| {
                            tracing::debug!("send survey answer");
                            popup_service.close();
                        },
                    }
                })
                .with_id("empty_survey")
                .with_title(tr.not_complete_survey_modal_title);
        } else {
            //TODO: send survey answer value to use API
            tracing::debug!("send survey answer");
        }
    }

    pub fn sample_survey_modal_description(&self, lang: Language, ended_at: String) -> String {
        match lang {
            Language::Ko => format!("모든 질문 항목에 응답하지 않으면, 보상 대상에서 제외됩니다.\n이번 조사는 [{ended_at} (UTC 기준)]까지 다시 참여할 수 있습니다.\n조사를 계속하시겠습니까?"),
            Language::En => format!("If you do not answer all the questions, you will not be eligible for rewards.\nYou can re-take this survey until [{ended_at} (UTC)].\nDo you want to continue taking the survey?"),
        }
    }

    pub fn get_deliberation_responses(&self) -> Vec<DeliberationResponse> {
        vec![
            DeliberationResponse {
                id: 1,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 1,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1, 2] },
                    Answer::Subjective {
                        answer: "subjective answer 1".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 1".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
            DeliberationResponse {
                id: 2,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 2,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1] },
                    Answer::Subjective {
                        answer: "subjective answer 2".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 2".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
            DeliberationResponse {
                id: 3,
                created_at: 1741103145,
                updated_at: 1741103145,
                deliberation_id: 1,
                user_id: 3,
                answers: vec![
                    Answer::SingleChoice { answer: 1 },
                    Answer::SingleChoice { answer: 1 },
                    Answer::MultipleChoice { answer: vec![1, 3] },
                    Answer::Subjective {
                        answer: "subjective answer 3".to_string(),
                    },
                    Answer::ShortAnswer {
                        answer: "short answer 3".to_string(),
                    },
                ],
                deliberation_type: DeliberationType::Sample,
            },
        ]
    }

    pub fn get_deliberation(&self) -> Deliberation {
        Deliberation {
            id: 1,
            created_at: 1741103145,
            updated_at: 1741103145,
            org_id: 1,
            started_at: 1741103145,
            ended_at: 1742399145,
            votes: vec![],
            steps: vec![
                Step {
                    id: 1,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    deliberation_id: 1,
                    step_type: StepType::GeneralPost,
                    name: "정보 제공".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step {
                    id: 2,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    deliberation_id: 1,
                    step_type: StepType::GeneralPost,
                    name: "토론 및 숙의".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step {
                    id: 3,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    deliberation_id: 1,
                    step_type: StepType::GeneralPost,
                    name: "의견 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step {
                    id: 4,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    deliberation_id: 1,
                    step_type: StepType::GeneralPost,
                    name: "합의 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step {
                    id: 5,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    deliberation_id: 1,
                    step_type: StepType::GeneralPost,
                    name: "결과 분석".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
            ],
            project_area: models::ProjectArea::Education,
            title: "지역사회 교통 개선 프로젝트".to_string(),
            description: "1. 공론조사의 목적 및 배경\n지역 주민들의 blah blah".to_string(),
            resources: vec![
                ResourceFile {
                    id: 1,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    title: "지역 사회 교통 개선 프로젝트".to_string(),
                    resource_type: Some(ResourceType::Presentation),
                    project_area: Some(models::ProjectArea::Education),
                    usage_purpose: Some(models::UsagePurpose::AcademicResearch),
                    source: Some(models::Source::Internal),
                    access_level: Some(models::AccessLevel::Public),
                    org_id: 1,
                    files: vec![],
                },
                ResourceFile {
                    id: 2,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    title: "지역 사회 교통 개선 프로젝트".to_string(),
                    resource_type: Some(ResourceType::Presentation),
                    project_area: Some(models::ProjectArea::Education),
                    usage_purpose: Some(models::UsagePurpose::AcademicResearch),
                    source: Some(models::Source::Internal),
                    access_level: Some(models::AccessLevel::Public),
                    org_id: 1,
                    files: vec![],
                },
            ],
            surveys: vec![SurveyV2 {
                id: 1,
                created_at: 1741103145,
                updated_at: 1741103145,
                name: "지역사회 교통 개선 시민 조사".to_string(),
                project_type: models::ProjectType::Survey,
                project_area: models::ProjectArea::Health,
                status: models::ProjectStatus::InProgress,
                started_at: 1741103145,
                ended_at: 1742014251,
                description: "지역사회 교통 개선 시민 조사".to_string(),
                quotes: 100,
                org_id: 1,
                questions: vec![
                    Question::SingleChoice(ChoiceQuestion {
                        title: "체크박스 선택지입니다?".to_string(),
                        description: Some("체크박스 선택지".to_string()),
                        options: vec![
                            "5시간 이상".to_string(),
                            "4시간 이상".to_string(),
                            "3시간 이상".to_string(),
                        ],
                    }),
                    Question::SingleChoice(ChoiceQuestion {
                        title: "체크박스 선택지입니다?".to_string(),
                        description: Some("체크박스 선택지".to_string()),
                        options: vec![
                            "5시간 이상".to_string(),
                            "4시간 이상".to_string(),
                            "3시간 이상".to_string(),
                        ],
                    }),
                    Question::MultipleChoice(ChoiceQuestion {
                        title: "체크박스 선택지입니다?".to_string(),
                        description: Some("체크박스 선택지".to_string()),
                        options: vec![
                            "5시간 이상".to_string(),
                            "4시간 이상".to_string(),
                            "3시간 이상".to_string(),
                        ],
                    }),
                    Question::Subjective(SubjectiveQuestion {
                        title: "주관식 선택지입니다?".to_string(),
                        description: "주관식 선택지".to_string(),
                    }),
                    Question::ShortAnswer(SubjectiveQuestion {
                        title: "주관식 선택지입니다?".to_string(),
                        description: "주관식 선택지".to_string(),
                    }),
                ],
                panels: vec![PanelV2 {
                    id: 1,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    name: "패널".to_string(),
                    user_count: 100,
                    attributes: vec![],
                    org_id: 1,
                }],
                panel_counts: vec![PanelCountsV2 {
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    panel_id: 1,
                    panel_survey_id: 1,
                    user_count: 100,
                }],
                noncelab_id: Some(1),
                response_count: 50,
            }],
            members: vec![
                DeliberationUser {
                    id: 1,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 1,
                    deliberation_id: 1,
                    organization_id: 1,
                    role: models::Role::Analyst,
                },
                DeliberationUser {
                    id: 2,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 2,
                    deliberation_id: 1,
                    organization_id: 1,
                    role: models::Role::Admin,
                },
                DeliberationUser {
                    id: 3,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 3,
                    deliberation_id: 1,
                    organization_id: 1,
                    role: models::Role::DeliberationAdmin,
                },
            ],
            panels: vec![PanelV2 {
                id: 1,
                created_at: 1741103145,
                updated_at: 1741103145,
                name: "패널".to_string(),
                user_count: 100,
                attributes: vec![],
                org_id: 1,
            }],
            comments: vec![],
            response_count: 10,
            discussions: vec![],
        }
    }
}
