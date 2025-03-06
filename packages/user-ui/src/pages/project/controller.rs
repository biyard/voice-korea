#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::{
    ChoiceQuestion, Deliberation, DeliberationUser, PanelCountsV2, PanelV2, Question, Resource,
    ResourceType, Step, SurveyV2,
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {};

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
                ended_at: 1741103145,
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
        }
    }
}
