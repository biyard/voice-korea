use dioxus::prelude::*;

use crate::prelude::Language;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectType {
    #[default]
    Investigation, //조사
    PublicOpinion, //공론
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectStatus {
    #[default]
    Ready, //준비
    InProgress, //진행
    Finished,   //마감
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemberDetail {
    pub email: String,
    pub profile_image: Option<String>,
    pub profile_name: Option<String>,
    pub group: String,
    pub role: String,
    pub register_date: String,
    pub project_history: Vec<ProjectHistory>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectHistory {
    pub history_id: String,
    pub project_type: ProjectType,
    pub project_subject: String,
    pub role: String,
    pub panel: Vec<String>,
    pub period: String, //FIXME: fix start date, end date form
    pub project_status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub member: Signal<MemberDetail>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
}

impl Controller {
    pub fn init(_lang: Language, _member_id: String) -> Self {
        let mut ctrl = Self {
            member: use_signal(|| MemberDetail::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    "관리자".to_string(),
                    "공론 관리자".to_string(),
                    "분석가".to_string(),
                    "중계자".to_string(),
                    "강연자".to_string(),
                ]
            }),
        };

        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        ctrl.member.set(MemberDetail {
            email: "email@email.com".to_string(),
            profile_image: None,
            profile_name: Some("보이스".to_string()),
            group: "보이스코리아".to_string(),
            role: "관리자".to_string(),
            register_date: "2024년 8월 12일".to_string(),
            project_history: vec![
                ProjectHistory {
                    history_id: "1".to_string(),
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                ProjectHistory {
                    history_id: "2".to_string(),
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::InProgress,
                },
                ProjectHistory {
                    history_id: "3".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::InProgress,
                },
                ProjectHistory {
                    history_id: "4".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::InProgress,
                },
                ProjectHistory {
                    history_id: "5".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Ready,
                },
                ProjectHistory {
                    history_id: "6".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Ready,
                },
                ProjectHistory {
                    history_id: "7".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Ready,
                },
                ProjectHistory {
                    history_id: "8".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Ready,
                },
                ProjectHistory {
                    history_id: "9".to_string(),
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    role: "관리자".to_string(),
                    panel: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    period: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Ready,
                },
            ],
        });

        ctrl
    }

    pub fn get_member(&self) -> MemberDetail {
        (self.member)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<String> {
        (self.roles)()
    }
}
