#![allow(dead_code, unused)]
use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::{
    deliberation_report::{
        DeliberationReport, DeliberationReportAction, DeliberationReportByIdAction,
        DeliberationReportCreateRequest, DeliberationReportRepository,
        DeliberationReportRepositoryUpdateRequest, DeliberationReportUpdateRequest,
    },
    *,
};

use crate::{controllers::v2::organizations::OrganizationPath, utils::app_claims::AppClaims};

#[derive(Clone, Debug)]
pub struct DeliberationReportController {
    repo: DeliberationReportRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationReportController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationReport::get_repository(pool.clone());
        Self { pool, repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_report_by_id))
            .route("/", post(Self::act_report))
            .with_state(self.clone()))
    }

    pub async fn act_report_by_id(
        State(ctrl): State<DeliberationReportController>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<DeliberationReportByIdAction>,
    ) -> Result<Json<DeliberationReport>> {
        match body {
            DeliberationReportByIdAction::Update(params) => {
                Ok(Json(ctrl.update(org_id, id, params).await?))
            }
        }
    }

    pub async fn act_report(
        State(ctrl): State<DeliberationReportController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationReportAction>,
    ) -> Result<Json<DeliberationReport>> {
        tracing::debug!("act_draft {} {:?}", org_id, body);

        match body {
            DeliberationReportAction::Create(param) => {
                Ok(Json(ctrl.create(org_id, auth, param).await?))
            }
        }
    }
}

impl DeliberationReportController {
    pub async fn update(
        &self,
        org_id: i64,
        report_id: i64,
        params: DeliberationReportUpdateRequest,
    ) -> Result<DeliberationReport> {
        let res = self
            .repo
            .update(
                report_id,
                DeliberationReportRepositoryUpdateRequest {
                    org_id: None,
                    deliberation_id: None,
                    user_id: None,
                    title: Some(params.title),
                    description: Some(params.description),
                    status: None,
                },
            )
            .await?;

        Ok(res)
    }

    pub async fn create(
        &self,
        org_id: i64,
        auth: Option<Authorization>,
        param: DeliberationReportCreateRequest,
    ) -> Result<DeliberationReport> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(
                org_id,
                param.deliberation_id,
                user_id,
                param.title,
                param.description,
                param.status,
            )
            .await?;
        Ok(res)
    }
}
