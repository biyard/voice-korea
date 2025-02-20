#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::{v2::PublicOpinionProjectSummary, ProjectArea};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    public_opinions: Signal<Vec<PublicOpinionProjectSummary>>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl =
            Self {
                lang,
                public_opinions: use_signal(|| {
                    vec![PublicOpinionProjectSummary {
                    id: 0,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Economy),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionProjectSummary {
                    id: 1,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Economy),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionProjectSummary {
                    id: 2,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Economy),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionProjectSummary {
                    id: 3,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Health),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionProjectSummary {
                    id: 4,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Health),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionProjectSummary {
                    id: 5,
                    updated_at: 10000000000,
                    title: "Title".to_string(),
                    description:
                        "The official server for the them. Welcome to our channel, Traveler! "
                            .to_string(),
                    policy_making_institution: "정책 결정 기관".to_string(),
                    project_area: Some(ProjectArea::Health),
                    org_id: 5,
                    num_of_participation: 9121,
                    num_of_vote: 1102,
                }]
                }),
            };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_public_opinions(&self) -> Vec<PublicOpinionProjectSummary> {
        (self.public_opinions)()
    }
}
