#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    v2::{InstitutionSummary, PublicOpinionProjectSummary, ReviewSummary},
    ProjectField,
};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    public_opinions: Signal<Vec<PublicOpinionProjectSummary>>,
    public_opinion_institutions: Signal<Vec<InstitutionSummary>>,
    public_opinion_reviews: Signal<Vec<ReviewSummary>>
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
                        project_area: Some(ProjectField::Economy),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                    PublicOpinionProjectSummary {
                        id: 1,
                        updated_at: 10000000000,
                        title: "Title".to_string(),
                        description:
                            "The official server for the them. Welcome to our channel, Traveler! "
                                .to_string(),
                        policy_making_institution: "정책 결정 기관".to_string(),
                        project_area: Some(ProjectField::Economy),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                    PublicOpinionProjectSummary {
                        id: 2,
                        updated_at: 10000000000,
                        title: "Title".to_string(),
                        description:
                            "The official server for the them. Welcome to our channel, Traveler! "
                                .to_string(),
                        policy_making_institution: "정책 결정 기관".to_string(),
                        project_area: Some(ProjectField::Economy),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                    PublicOpinionProjectSummary {
                        id: 3,
                        updated_at: 10000000000,
                        title: "Title".to_string(),
                        description:
                            "The official server for the them. Welcome to our channel, Traveler! "
                                .to_string(),
                        policy_making_institution: "정책 결정 기관".to_string(),
                        project_area: Some(ProjectField::Health),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                    PublicOpinionProjectSummary {
                        id: 4,
                        updated_at: 10000000000,
                        title: "Title".to_string(),
                        description:
                            "The official server for the them. Welcome to our channel, Traveler! "
                                .to_string(),
                        policy_making_institution: "정책 결정 기관".to_string(),
                        project_area: Some(ProjectField::Health),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                    PublicOpinionProjectSummary {
                        id: 5,
                        updated_at: 10000000000,
                        title: "Title".to_string(),
                        description:
                            "The official server for the them. Welcome to our channel, Traveler! "
                                .to_string(),
                        policy_making_institution: "정책 결정 기관".to_string(),
                        project_area: Some(ProjectField::Health),
                        org_id: 5,
                        num_of_participation: 9121,
                        num_of_vote: 1102,
                        institution_id: 1,
                        accepters: 1102,
                        rejecters: 1102,
                    },
                ]
            }),
            public_opinion_institutions: use_signal(|| {
                vec![
                    InstitutionSummary {
                        id: 1,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 2,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 3,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 4,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 5,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 6,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 7,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 8,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 9,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }, 
                    InstitutionSummary {
                        id: 10,
                        updated_at: 10000000000,
                        name: "부산광역시".to_string(),
                        description: "거버넌스를 소개하는 상세내용이 들어갑니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다. 4줄이 넘어가는 경우 자동 생략 처리가 됩니다.....".to_string(),
                        projects: vec![
                        ],
                        num_of_projects: 9121,
                        num_of_vote: 1102,
                        num_of_participation: 560000,
                    }
                ]
            }),
            public_opinion_reviews: use_signal(|| vec![
                ReviewSummary { 
                    id: 0, 
                    created_at: 1740063600, 
                    updated_at: 1740063600, 
                    name: "BGIR19$1".to_string(), 
                    image: "".to_string(), 
                    review: "고급 분석 리포트가 제공되어 제 의견이 어떻게 사회에 영향을 미치는지 더 깊이 이해할 수 있었어요. 또한, 전문가의 개인화된 상담을 통해 많은 도움을 받았습니다. 확실히 유료 서비스가 가치를 더하는 것 같아요.".to_string() 
                },
                ReviewSummary { 
                    id: 1, 
                    created_at: 1740063600, 
                    updated_at: 1740063600, 
                    name: "6fkEWI".to_string(), 
                    image: "".to_string(), 
                    review: "공론조사에 참여하면서 내 의견이 중요한 사회적 결정을 만드는 데 기여하고 있다는 느낌을 받았어요. 특히, 이 플랫폼은 모더레이션 기능 덕분에 의견 교환이 정말 건전하고 유익하게 이루어집니다. 기술 지원도 빠르고, 사용하기 정말 편리한 시스템이었습니다.".to_string() 
                },
                ReviewSummary { 
                    id: 2, 
                    created_at: 1740063600, 
                    updated_at: 1740063600, 
                    name: "FGR129".to_string(), 
                    image: "".to_string(), 
                    review: "내 의견을 쉽게 표현할 수 있었습니다. 유료 서비스에서 제공하는 심층 분석 리포트는 정말 유익했어요. 참여하면서 제가 내놓은 의견이 실제로 어떻게 반영되는지 확인할 수 있는 점이 큰 매력입니다.".to_string() 
                }
            ])
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn send_inquiry(&self, name: String, email: String, message: String) {
        tracing::debug!("send inquiry button clicked: {} {} {}", name, email, message);
    }

    pub fn get_public_opinion_reviews(&self) -> Vec<ReviewSummary> {
        (self.public_opinion_reviews)()
    }

    pub fn get_public_opinions(&self) -> Vec<PublicOpinionProjectSummary> {
        (self.public_opinions)()
    }

    pub fn get_public_opinion_institutions(&self) -> Vec<InstitutionSummary> {
        (self.public_opinion_institutions)()
    }
}
