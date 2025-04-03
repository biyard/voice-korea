#![allow(dead_code, unused)]
use bdk::prelude::*;
use by_macros::DioxusController;
use models::{
    deliberation::{Deliberation, DeliberationCreateRequest},
    step::StepCreateRequest,
    step_type::StepType,
    *,
};

use crate::{
    config,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    popup_service: Signal<PopupService>,
    current_step: Signal<CurrentStep>,
    user: LoginService,

    deliberation_requests: Signal<DeliberationCreateRequest>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentStep {
    SettingInfo,          // Setting Deliberation Overview
    CompositionCommittee, // Composition Deliberation Committee
    CompositionPanel,     // Composition Participant Panel
    DeliberationSchedule, // Deliberation Procedure and Schedule
    Preview,              // Final Recommendation

    EditContent,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let popup_service: PopupService = use_context();

        let ctrl = Self {
            lang,
            user,
            popup_service: use_signal(|| popup_service),
            current_step: use_signal(|| CurrentStep::SettingInfo),
            deliberation_requests: use_signal(|| DeliberationCreateRequest::default()),
        };
        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn change_request(&mut self, req: DeliberationCreateRequest) {
        tracing::debug!("req: {:?}", req);
        self.deliberation_requests.set(req);
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn use_service() -> Self {
        use_context()
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
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
}
