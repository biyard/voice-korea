use dioxus::prelude::*;

use crate::prelude::Language;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemberSummary {
    pub role_counts: Vec<u64>, // [전체 팀원, 관리자 수, 공론 관리자 수, 분석가 수, 중개자 수, 강연자 수],
    pub members: Vec<Member>,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Member {
    pub member_id: String,
    pub profile: Option<String>,
    pub profile_name: Option<String>,
    pub email: String,
    pub group: String,
    pub role: String,
    pub projects: Vec<String>, //유저가 속해있는 프로젝트
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub members: Signal<MemberSummary>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
}

impl Controller {
    pub fn init(_lang: Language) -> Self {
        let mut ctrl = Self {
            members: use_signal(|| MemberSummary::default()),
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

        ctrl.members.set(MemberSummary {
            role_counts: vec![5, 0, 0, 0, 1, 4],
            members: vec![
                Member {
                    member_id: "1".to_string(),
                    profile: None,
                    profile_name: Some("보이스".to_string()),
                    email: "email@email.com".to_string(),
                    group: "보이스코리아".to_string(),
                    role: "중계자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                Member {
                    member_id: "2".to_string(),
                    profile: None,
                    profile_name: Some("보이스".to_string()),
                    email: "email@email.com".to_string(),
                    group: "보이스코리아".to_string(),
                    role: "강연자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                Member {
                    member_id: "3".to_string(),
                    profile: None,
                    profile_name: Some("보이스".to_string()),
                    email: "email@email.com".to_string(),
                    group: "보이스코리아".to_string(),
                    role: "강연자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                Member {
                    member_id: "4".to_string(),
                    profile: None,
                    profile_name: Some("보이스".to_string()),
                    email: "email@email.com".to_string(),
                    group: "보이스코리아".to_string(),
                    role: "강연자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                Member {
                    member_id: "5".to_string(),
                    profile: None,
                    profile_name: Some("보이스".to_string()),
                    email: "email@email.com".to_string(),
                    group: "보이스코리아".to_string(),
                    role: "강연자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
            ],
        });

        ctrl
    }

    pub fn get_members(&self) -> MemberSummary {
        (self.members)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<String> {
        (self.roles)()
    }
}
