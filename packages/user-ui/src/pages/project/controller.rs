#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    response::Answer, ChoiceQuestion, Deliberation, DeliberationUser, PanelCountsV2, PanelV2,
    Question, Resource, ResourceType, Step, SubjectiveQuestion, SurveyV2,
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {};

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn get_deliberation(&self) -> Deliberation {
        Deliberation {
            id: 1,
            created_at: 1741103145,
            updated_at: 1741103145,
            org_id: 1,
            started_at: 1741103145,
            ended_at: 1742399145,
            steps: vec![
                Step::GeneralBoard {
                    name: "정보 제공".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::VideoConference {
                    name: "토론 및 숙의".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "의견 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "합의 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "결과 분석".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
            ],
            project_area: models::ProjectArea::Education,
            title: "지역사회 교통 개선 프로젝트".to_string(),
            description: "1. 공론조사의 목적 및 배경\n지역 주민들의 blah blah".to_string(),
            resources: vec![
                Resource {
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
                Resource {
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
                    deliveration_id: 1,
                    role: models::Role::Analyst,
                },
                DeliberationUser {
                    id: 2,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 2,
                    deliveration_id: 1,
                    role: models::Role::Admin,
                },
                DeliberationUser {
                    id: 3,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 3,
                    deliveration_id: 1,
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleController {
    answers: Signal<Vec<Answer>>,
}

impl SampleController {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let mut ctrl = Self {
            answers: use_signal(|| vec![]),
        };

        use_context_provider(|| ctrl);

        let questions = ctrl.get_deliberation().surveys[0].questions.clone();

        use_effect({
            let questions = questions.clone();
            move || {
                let mut answers = vec![];

                for question in questions.clone() {
                    match question {
                        Question::SingleChoice(choice_question) => {
                            answers.push(Answer::SingleChoice { answer: 0 })
                        }
                        Question::MultipleChoice(choice_question) => {
                            answers.push(Answer::MultipleChoice { answer: vec![] })
                        }
                        Question::ShortAnswer(subjective_question) => {
                            answers.push(Answer::ShortAnswer {
                                answer: "".to_string(),
                            })
                        }
                        Question::Subjective(subjective_question) => {
                            answers.push(Answer::Subjective {
                                answer: "".to_string(),
                            })
                        }
                    }
                }

                tracing::debug!("this line come");

                ctrl.answers.set(answers);
            }
        });

        Ok(ctrl)
    }

    pub fn change_answer(&mut self, index: usize, answer: Answer) {
        let mut answers = self.answers();
        answers[index] = answer;
        self.answers.set(answers.clone());
    }

    pub fn answers(&self) -> Vec<Answer> {
        (self.answers)()
    }

    pub fn get_deliberation(&self) -> Deliberation {
        Deliberation {
            id: 1,
            created_at: 1741103145,
            updated_at: 1741103145,
            org_id: 1,
            started_at: 1741103145,
            ended_at: 1742399145,
            steps: vec![
                Step::GeneralBoard {
                    name: "정보 제공".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::VideoConference {
                    name: "토론 및 숙의".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "의견 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "합의 도출".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
                Step::GeneralBoard {
                    name: "결과 분석".to_string(),
                    started_at: 1741103145,
                    ended_at: 1742399145,
                },
            ],
            project_area: models::ProjectArea::Education,
            title: "지역사회 교통 개선 프로젝트".to_string(),
            description: "1. 공론조사의 목적 및 배경\n지역 주민들의 blah blah".to_string(),
            resources: vec![
                Resource {
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
                Resource {
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
                    deliveration_id: 1,
                    role: models::Role::Analyst,
                },
                DeliberationUser {
                    id: 2,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 2,
                    deliveration_id: 1,
                    role: models::Role::Admin,
                },
                DeliberationUser {
                    id: 3,
                    created_at: 1741103145,
                    updated_at: 1741103145,
                    user_id: 3,
                    deliveration_id: 1,
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
        }
    }
}
