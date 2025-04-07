use bdk::prelude::*;

use models::{
    deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest,
    deliberation_user::DeliberationUserCreateRequest, *,
};

use crate::service::login_service::LoginService;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committee_members: Signal<Vec<DeliberationUserCreateRequest>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let sample_survey = use_signal(|| DeliberationSampleSurveyCreateRequest::default());

        let members = use_server_future(move || {
            let page = 1;
            let size = 100;
            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = OrganizationMember::get_client(endpoint)
                    .query(
                        org_id.unwrap().id,
                        OrganizationMemberQuery::new(size).with_page(page),
                    )
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        // use_effect({
        //     let mut sample = req
        //         .sample_surveys
        //         .get(0)
        //         .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
        //         .clone();

        //     let started_at = if sample.started_at == 0 {
        //         current_timestamp()
        //     } else {
        //         sample.started_at
        //     };

        //     let ended_at = if sample.ended_at == 0 {
        //         current_timestamp()
        //     } else {
        //         sample.ended_at
        //     };

        //     move || {
        //         sample.started_at = started_at;
        //         sample.ended_at = ended_at;
        //         ctrl.sample_survey.set(sample.clone());
        //     }
        // });

        let ctrl = Self {
            lang,
            sample_survey,

            members,
            committee_members: use_signal(|| vec![]),
        };

        // FIXME: anti-pattern
        // ctrl.committee_members.set(req.roles.clone());
        Ok(ctrl)
    }

    pub fn set_sample_survey(&mut self, info: DeliberationSampleSurveyCreateRequest) {
        self.sample_survey.set(info);
    }

    pub fn get_sample_survey(&self) -> DeliberationSampleSurveyCreateRequest {
        (self.sample_survey)()
    }

    pub fn get_committees(&self) -> Vec<OrganizationMemberSummary> {
        let committees = self.committee_members();
        let members = self.members().unwrap_or_default();

        let d = members
            .clone()
            .into_iter()
            .filter(|member| {
                committees
                    .iter()
                    .any(|committee| committee.user_id == member.user_id)
            })
            .collect();

        d
    }

    pub fn get_selected_committee(&self) -> Vec<OrganizationMemberSummary> {
        let total_committees = self.members().unwrap_or_default();
        let sample_survey = self.get_sample_survey();
        let roles = sample_survey.clone().users;
        total_committees
            .clone()
            .into_iter()
            .filter(|member| roles.iter().any(|id| id.clone() == member.id))
            .collect()
    }
}
