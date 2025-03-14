use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
        Extension, Json,
    },
};
use by_types::QueryResponse;
use deliberation_resources::deliberation_resource::{
    DeliberationResource, DeliberationResourceType,
};
use deliberation_surveys::DeliberationSurvey;
use deliberation_user::{DeliberationUser, DeliberationUserCreateRequest};
use discussion_resources::DiscussionResource;
use discussions::{Discussion, DiscussionCreateRequest};
use models::{
    deliberation::{
        Deliberation, DeliberationAction, DeliberationByIdAction, DeliberationCreateRequest,
        DeliberationGetResponse, DeliberationParam, DeliberationQuery, DeliberationQueryActionType,
        DeliberationRepository, DeliberationRepositoryUpdateRequest, DeliberationSummary,
        DeliberationUpdateRequest,
    },
    step::{Step, StepRepository},
    *,
};
use panel_deliberations::PanelDeliberation;
use sqlx::postgres::PgRow;
use step::StepCreateRequest;

use crate::controllers::v2::organizations::OrganizationPath;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationPath {
    pub org_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationParentPath {
    pub org_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationController {
    repo: DeliberationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    step: StepRepository,
}

impl DeliberationController {
    pub async fn update(
        &self,
        org_id: i64,
        id: i64,
        DeliberationUpdateRequest {
            started_at,
            ended_at,
            project_area,
            title,
            description,
            resource_ids,
            survey_ids,
            roles,
            panel_ids,
            steps,
            elearning,
            discussions,
        }: DeliberationUpdateRequest,
    ) -> Result<Deliberation> {
        if started_at >= ended_at {
            return Err(ApiError::ValidationError(
                "started_at should be less than ended_at".to_string(),
            )
            .into());
        }

        let du = DeliberationUser::get_repository(self.pool.clone());
        let dr = DeliberationResource::get_repository(self.pool.clone());
        let ds = DeliberationSurvey::get_repository(self.pool.clone());
        let d = Discussion::get_repository(self.pool.clone());
        let discussion_resource_repo = DiscussionResource::get_repository(self.pool.clone());
        let pd = PanelDeliberation::get_repository(self.pool.clone());

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .update_with_tx(
                &mut *tx,
                id,
                DeliberationRepositoryUpdateRequest {
                    org_id: Some(org_id),
                    started_at: Some(started_at),
                    ended_at: Some(ended_at),
                    project_area: Some(project_area),
                    title: Some(title),
                    description: Some(description),
                },
            )
            .await?
            .ok_or(ApiError::DeliberationException)?;

        // resource
        let prev_resources = DeliberationResource::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(DeliberationResource::from)
            .fetch_all(&mut *tx)
            .await?;

        for resource in prev_resources {
            let _ = dr.delete_with_tx(&mut *tx, resource.id).await?;
        }

        for resource_id in resource_ids {
            dr.insert_with_tx(
                &mut *tx,
                deliberation.id,
                resource_id,
                DeliberationResourceType::Reference,
            )
            .await?
            .ok_or(ApiError::DeliberationResourceException)?;
        }

        for resource_id in elearning {
            dr.insert_with_tx(
                &mut *tx,
                deliberation.id,
                resource_id,
                DeliberationResourceType::Elearning,
            )
            .await?
            .ok_or(ApiError::DeliberationResourceException)?;
        }

        // surveys
        let prev_surveys = DeliberationSurvey::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(DeliberationSurvey::from)
            .fetch_all(&mut *tx)
            .await?;

        for survey in prev_surveys {
            let _ = ds.delete_with_tx(&mut *tx, survey.id).await?;
        }

        for survey_id in survey_ids {
            ds.insert_with_tx(&mut *tx, deliberation.id, survey_id)
                .await?
                .ok_or(ApiError::DeliberationSurveyException)?;
        }

        // members
        let prev_users = DeliberationUser::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(DeliberationUser::from)
            .fetch_all(&mut *tx)
            .await?;

        for user in prev_users {
            let _ = du.delete_with_tx(&mut *tx, user.id).await?;
        }

        for DeliberationUserCreateRequest { user_id, role } in roles {
            du.insert_with_tx(&mut *tx, user_id, org_id, deliberation.id, role)
                .await?
                .ok_or(ApiError::DeliberationUserException)?;
        }

        //panel
        let prev_panels = PanelDeliberation::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(PanelDeliberation::from)
            .fetch_all(&mut *tx)
            .await?;

        for panel in prev_panels {
            let _ = pd.delete_with_tx(&mut *tx, panel.id).await?;
        }

        for panel_id in panel_ids {
            pd.insert_with_tx(&mut *tx, panel_id, deliberation.id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        //step
        let prev_steps = Step::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(Step::from)
            .fetch_all(&mut *tx)
            .await?;

        for step in prev_steps {
            let _ = self.step.delete_with_tx(&mut *tx, step.id).await?;
        }

        for StepCreateRequest {
            ended_at,
            step_type,
            started_at,
            name,
        } in steps
        {
            self.step
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    step_type,
                    name,
                    started_at,
                    ended_at,
                )
                .await?
                .ok_or(ApiError::DeliberationStepException)?;
        }

        //discussion
        let prev_discussions = Discussion::query_builder()
            .deliberation_id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_all(&mut *tx)
            .await?;

        for discussion in prev_discussions {
            let _ = d.delete_with_tx(&mut *tx, discussion.id).await?;

            let prev_discussion_resources = DiscussionResource::query_builder()
                .discussion_id_equals(discussion.id)
                .query()
                .map(DiscussionResource::from)
                .fetch_all(&mut *tx)
                .await?;

            for resource in prev_discussion_resources {
                let _ = discussion_resource_repo
                    .delete_with_tx(&mut *tx, resource.id)
                    .await?;
            }
        }

        for DiscussionCreateRequest {
            description,
            ended_at,
            name,
            resources,
            started_at,
        } in discussions
        {
            let discussion = d
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    started_at,
                    ended_at,
                    name,
                    description,
                    None,
                )
                .await?
                .ok_or(ApiError::DeliberationDiscussionException)?;

            for resource_id in resources {
                discussion_resource_repo
                    .insert_with_tx(&mut *tx, discussion.id, resource_id)
                    .await?
                    .ok_or(ApiError::DiscussionResourceException)?;
            }
        }

        tx.commit().await?;
        Ok(deliberation)
    }

    pub async fn create(
        &self,
        org_id: i64,
        DeliberationCreateRequest {
            started_at,
            ended_at,
            project_area,
            title,
            description,
            panel_ids,
            resource_ids,
            survey_ids,
            roles,
            steps,
            elearning,
            discussions,
        }: DeliberationCreateRequest,
    ) -> Result<Deliberation> {
        if started_at >= ended_at {
            return Err(ApiError::ValidationError(
                "started_at should be less than ended_at".to_string(),
            )
            .into());
        }

        let du = DeliberationUser::get_repository(self.pool.clone());
        let dr = DeliberationResource::get_repository(self.pool.clone());
        let ds = DeliberationSurvey::get_repository(self.pool.clone());
        let d = Discussion::get_repository(self.pool.clone());
        let discussion_resource_repo = DiscussionResource::get_repository(self.pool.clone());
        let pd = PanelDeliberation::get_repository(self.pool.clone());

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .insert_with_tx(
                &mut *tx,
                org_id,
                started_at,
                ended_at,
                project_area,
                title,
                description,
            )
            .await?
            .ok_or(ApiError::DeliberationException)?;

        for DeliberationUserCreateRequest { user_id, role } in roles {
            du.insert_with_tx(&mut *tx, user_id, org_id, deliberation.id, role)
                .await?
                .ok_or(ApiError::DeliberationUserException)?;
        }

        for resource_id in resource_ids {
            dr.insert_with_tx(
                &mut *tx,
                deliberation.id,
                resource_id,
                DeliberationResourceType::Reference,
            )
            .await?
            .ok_or(ApiError::DeliberationResourceException)?;
        }

        for resource_id in elearning {
            dr.insert_with_tx(
                &mut *tx,
                deliberation.id,
                resource_id,
                DeliberationResourceType::Elearning,
            )
            .await?
            .ok_or(ApiError::DeliberationResourceException)?;
        }

        for survey_id in survey_ids {
            ds.insert_with_tx(&mut *tx, deliberation.id, survey_id)
                .await?
                .ok_or(ApiError::DeliberationSurveyException)?;
        }

        for StepCreateRequest {
            ended_at,
            step_type,
            started_at,
            name,
        } in steps
        {
            self.step
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    step_type,
                    name,
                    started_at,
                    ended_at,
                )
                .await?
                .ok_or(ApiError::DeliberationStepException)?;
        }

        for DiscussionCreateRequest {
            description,
            ended_at,
            name,
            resources,
            started_at,
        } in discussions
        {
            let discussion = d
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    started_at,
                    ended_at,
                    name,
                    description,
                    None,
                )
                .await?
                .ok_or(ApiError::DeliberationDiscussionException)?;

            for resource_id in resources {
                discussion_resource_repo
                    .insert_with_tx(&mut *tx, discussion.id, resource_id)
                    .await?
                    .ok_or(ApiError::DiscussionResourceException)?;
            }
        }

        for id in panel_ids {
            pd.insert_with_tx(&mut *tx, id, deliberation.id)
                .await?
                .ok_or(ApiError::DeliberationPanelException)?;
        }

        tx.commit().await?;

        Ok(deliberation)
    }

    pub async fn query(
        &self,
        org_id: i64,
        DeliberationQuery { size, bookmark, .. }: DeliberationQuery,
    ) -> Result<QueryResponse<DeliberationSummary>> {
        let mut total_count: i64 = 0;
        let items: Vec<DeliberationSummary> = Deliberation::query_builder()
            .org_id_equals(org_id)
            .limit(size as i32)
            .page(bookmark.unwrap_or("1".to_string()).parse::<i32>().unwrap())
            .with_count()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool.clone());
        let step = Step::get_repository(pool.clone());
        Self { pool, repo, step }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                post(Self::act_deliberation_by_id).get(Self::get_deliberation_by_id), // .post(Self::act_deliberation_by_id)
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_deliberation).get(Self::get_deliberation),
            )
            .with_state(self.clone()))
    }

    pub async fn search_by(
        &self,
        org_id: i64,
        q: DeliberationQuery,
    ) -> Result<Json<DeliberationGetResponse>> {
        let mut total_count: i64 = 0;

        let items = DeliberationSummary::query_builder()
            .org_id_equals(org_id)
            .title_contains(q.clone().title.unwrap_or_default())
            .limit(q.size())
            .page(q.page())
            .query()
            .map(|r: PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(DeliberationGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn act_deliberation_by_id(
        State(ctrl): State<DeliberationController>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationByIdAction>,
    ) -> Result<Json<Deliberation>> {
        match body {
            DeliberationByIdAction::Update(params) => {
                Ok(Json(ctrl.update(org_id, id, params).await?))
            }
        }
    }

    pub async fn act_deliberation(
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("act_deliberation {} {:?}", org_id, body);

        match body {
            DeliberationAction::Create(param) => Ok(Json(ctrl.create(org_id, param).await?)),
        }
    }

    pub async fn get_deliberation_by_id(
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", org_id, id);
        // FIXME: {"DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    // pub async fn get_deliberation_by_id(
    //     State(ctrl): State<DeliberationController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    // ) -> Result<Json<Deliberation>> {
    //     tracing::debug!("get_deliberation {} {:?}", org_id, id);
    //     Ok(Json(
    //         Deliberation::query_builder()
    //             .id_equals(id)
    //             .org_id_equals(org_id)
    //             .query()
    //             .map(Deliberation::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_deliberation(
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<DeliberationParam>,
    ) -> Result<Json<DeliberationGetResponse>> {
        tracing::debug!("list_deliberation {} {:?}", org_id, param);

        match param {
            // "DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
            DeliberationParam::Query(q) => match q.action {
                Some(DeliberationQueryActionType::SearchBy) => ctrl.search_by(org_id, q).await,
                None => {
                    return Ok(Json(DeliberationGetResponse::Query(
                        ctrl.query(org_id, q).await?,
                    )));
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{
        deliberation::{Deliberation, DeliberationQuery},
        ProjectArea,
    };

    use crate::tests::{setup, TestContext};

    #[tokio::test]
    async fn test_deliberation_empty() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let cli = Deliberation::get_client(&endpoint);
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                ProjectArea::City,
                format!("test deliberation {now}"),
                "test description".to_string(),
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            )
            .await;
        assert!(res.is_ok());

        let res = cli.query(org_id, DeliberationQuery::new(10)).await.unwrap();

        assert_eq!(res.items.len(), 1)
    }
}
