#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::{
    v2::{PublicOpinionInstitutionSummary, PublicOpinionProjectSummary},
    ProjectArea,
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    public_opinions: Signal<Vec<PublicOpinionProjectSummary>>,
    public_opinion_institutions: Signal<Vec<PublicOpinionInstitutionSummary>>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            public_opinions: use_signal(|| {
                vec![
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                    PublicOpinionProjectSummary {
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
                        institution_id: 1,
                    },
                ]
            }),
            public_opinion_institutions: use_signal(|| {
                vec![PublicOpinionInstitutionSummary {
                    id: 1,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 2,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 3,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 4,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 5,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 6,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 7,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 8,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 9,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
                    num_of_vote: 1102,
                }, PublicOpinionInstitutionSummary {
                    id: 10,
                    updated_at: 10000000000,
                    name: "부산광역시".to_string(),
                    description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                    projects: vec![
                    ],
                    num_of_projects: 9121,
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

    pub fn get_public_opinion_institutions(&self) -> Vec<PublicOpinionInstitutionSummary> {
        (self.public_opinion_institutions)()
    }
}
