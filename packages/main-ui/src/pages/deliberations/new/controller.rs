use bdk::prelude::*;
use by_macros::DioxusController;
use models::{deliberation_user::DeliberationUserCreateRequest, step::StepCreateRequest, *};

use crate::{
    config,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

#[derive(
    Debug, Clone, PartialEq, Copy, serde::Serialize, serde::Deserialize, EnumProp, Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum DeliberationNewStep {
    #[default]
    SettingInfo, // Setting Deliberation Overview
    CompositionCommittee, // Composition Deliberation Committee
    CompositionPanel,     // Composition Participant Panel
    DeliberationSchedule, // Deliberation Procedure and Schedule
    Preview,              // Final Recommendation

    EditContent,
}

impl From<Route> for DeliberationNewStep {
    fn from(route: Route) -> Self {
        match route {
            Route::DeliberationNewPage { .. } => Self::SettingInfo,
            Route::CompositionCommitee { .. } => Self::CompositionCommittee,
            Route::CompositionPanel { .. } => Self::CompositionPanel,
            Route::Preview { .. } => Self::Preview,

            _ => DeliberationNewStep::DeliberationSchedule,
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    popup_service: Signal<PopupService>,
    current_step: DeliberationNewStep,
    user: LoginService,

    deliberation_requests: Signal<DeliberationCreateRequest>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let popup_service: PopupService = use_context();
        let route = use_route::<Route>();
        let current_step = route.clone().into();

        let ctrl = Self {
            lang,
            user,
            current_step,
            popup_service: use_signal(|| popup_service),
            deliberation_requests: use_signal(|| DeliberationCreateRequest::default()),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn change_request(&mut self, req: DeliberationCreateRequest) {
        tracing::debug!("req: {:?}", req);
        self.deliberation_requests.set(req);
    }

    pub fn get_current_step(&self) -> DeliberationNewStep {
        self.current_step
    }

    pub fn use_service() -> Self {
        use_context()
    }

    pub async fn create_metadata(&self, file: File) -> Result<ResourceFile> {
        let org = self.user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::ResourceFile::get_client(&config::get().api_url);

        client
            .create(
                org_id,
                file.name.clone(),
                None,
                None,
                None,
                None,
                None,
                vec![file],
            )
            .await
    }

    pub async fn create_deliberation(&self, lang: Language) -> Result<()> {
        let navigator = use_navigator();

        let endpoint = crate::config::get().api_url;
        let client = Deliberation::get_client(endpoint);

        let org_id = self.user.get_selected_org();
        if org_id.is_none() {
            tracing::error!("Organization ID is missing");
            return Err(ApiError::OrganizationNotFound);
        }

        let req = self.deliberation_requests();

        match client
            .create(
                org_id.unwrap().id,
                req.started_at,
                req.ended_at,
                req.thumbnail_image,
                req.title.clone(),
                req.description.clone(),
                req.project_area,
                req.resource_ids,
                req.survey_ids,
                req.roles,
                req.panel_ids,
                req.steps,
                req.elearning,
                req.basic_infos,
                req.sample_surveys,
                req.contents,
                req.deliberation_discussions,
                req.final_surveys,
                req.drafts,
            )
            .await
        {
            Ok(_) => {
                btracing::debug!("success to create deliberation");
                navigator.push(Route::DeliberationPage { lang });
                Ok(())
            }
            Err(e) => {
                btracing::error!("failed to create deliberation: {}", e.translate(&lang));
                return Err(e);
            }
        }
    }

    pub fn get_deliberation_time(&self, steps: Vec<StepCreateRequest>) -> (i64, i64) {
        let started_at = steps.iter().map(|s| s.started_at).min().unwrap_or(0);
        let ended_at = steps.iter().map(|s| s.ended_at).max().unwrap_or(0);

        (started_at, ended_at)
    }

    pub fn save_panels(&mut self, panel_ids: Vec<i64>) {
        self.deliberation_requests.with_mut(|req| {
            req.panel_ids = panel_ids;
        });
    }

    pub fn save_committees(&mut self, roles: Vec<DeliberationUserCreateRequest>) {
        self.deliberation_requests.with_mut(|req| {
            req.roles = roles;
        });
    }

    pub fn save_basic_info(&mut self, basic_info: DeliberationBasicInfoCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.basic_infos = vec![basic_info];
        });
    }

    pub fn save_sample_survey(&mut self, sample_surveys: DeliberationSampleSurveyCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.sample_surveys = vec![sample_surveys];
        });
    }

    pub fn save_content(&mut self, content: DeliberationContentCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.contents = vec![content];
        });
    }

    pub fn save_discussion(&mut self, discussion: DeliberationDiscussionCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.deliberation_discussions = vec![discussion];
        });
    }

    pub fn save_final_survey(&mut self, final_survey: DeliberationFinalSurveyCreateRequest) {
        self.deliberation_requests.with_mut(|req| {
            req.final_surveys = vec![final_survey];
        });
    }

    pub async fn temporary_save(&self) {
        let DeliberationCreateRequest {
            started_at,
            ended_at,
            thumbnail_image,
            title,
            description,
            project_area,
            resource_ids,
            survey_ids,
            roles,
            panel_ids,
            steps,
            elearning,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            drafts,
        } = self.deliberation_requests();
        let org = self.user.get_selected_org();
        if org.is_none() {
            btracing::e!(self.lang, ApiError::OrganizationNotFound);
            return;
        }

        let org_id = org.unwrap().id;

        match Deliberation::get_client(config::get().api_url)
            .create(
                org_id,
                started_at,
                ended_at,
                thumbnail_image,
                title,
                description,
                project_area,
                resource_ids,
                survey_ids,
                roles,
                panel_ids,
                steps,
                elearning,
                basic_infos,
                sample_surveys,
                contents,
                deliberation_discussions,
                final_surveys,
                drafts,
            )
            .await
        {
            Ok(_) => {
                tracing::debug!("success to create deliberation");
            }
            Err(e) => {
                btracing::error!("failed to create deliberation: {}", e.translate(&self.lang));
            }
        }
    }
}
