use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
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
use discussions::Discussion;
use models::{
    deliberation::{
        Deliberation, DeliberationAction, DeliberationCreateRequest, DeliberationGetResponse,
        DeliberationParam, DeliberationQuery, DeliberationQueryActionType, DeliberationRepository,
        DeliberationSummary,
    },
    deliberation_basic_info_members::deliberation_basic_info_member::DeliberationBasicInfoMember,
    deliberation_basic_info_resources::deliberation_basic_info_resource::DeliberationBasicInfoResource,
    deliberation_basic_info_surveys::deliberation_basic_info_survey::DeliberationBasicInfoSurvey,
    deliberation_basic_infos::deliberation_basic_info::{
        DeliberationBasicInfo, DeliberationBasicInfoCreateRequest,
    },
    deliberation_content_members::deliberation_content_member::DeliberationContentMember,
    deliberation_contents::deliberation_content::{
        DeliberationContent, DeliberationContentCreateRequest,
    },
    deliberation_discussion_members::deliberation_discussion_member::DeliberationDiscussionMember,
    deliberation_discussion_resources::deliberation_discussion_resource::DeliberationDiscussionResource,
    deliberation_discussions::deliberation_discussion::{
        DeliberationDiscussion, DeliberationDiscussionCreateRequest,
    },
    deliberation_draft_members::deliberation_draft_member::DeliberationDraftMember,
    deliberation_draft_resources::deliberation_draft_resource::DeliberationDraftResource,
    deliberation_draft_surveys::deliberation_draft_survey::DeliberationDraftSurvey,
    deliberation_drafts::deliberation_draft::{DeliberationDraft, DeliberationDraftCreateRequest},
    deliberation_final_survey_members::deliberation_final_survey_member::DeliberationFinalSurveyMember,
    deliberation_final_survey_surveys::deliberation_final_survey_survey::DeliberationFinalSurveySurvey,
    deliberation_final_surveys::deliberation_final_survey::{
        DeliberationFinalSurvey, DeliberationFinalSurveyCreateRequest,
    },
    deliberation_sample_survey_members::deliberation_sample_survey_member::DeliberationSampleSurveyMember,
    deliberation_sample_survey_surveys::deliberation_sample_survey_survey::DeliberationSampleSurveySurvey,
    deliberation_sample_surveys::deliberation_sample_survey::{
        DeliberationSampleSurvey, DeliberationSampleSurveyCreateRequest,
    },
    discussion_groups::DiscussionGroup,
    elearnings::elearning::Elearning,
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
            thumbnail_image,
            basic_infos,
            sample_surveys,
            contents,
            deliberation_discussions,
            final_surveys,
            drafts,
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
        let pd = PanelDeliberation::get_repository(self.pool.clone());

        let basic_info = DeliberationBasicInfo::get_repository(self.pool.clone());
        let basic_info_member = DeliberationBasicInfoMember::get_repository(self.pool.clone());
        let basic_info_resource = DeliberationBasicInfoResource::get_repository(self.pool.clone());
        let basic_info_survey = DeliberationBasicInfoSurvey::get_repository(self.pool.clone());

        let sample_survey = DeliberationSampleSurvey::get_repository(self.pool.clone());
        let sample_survey_member =
            DeliberationSampleSurveyMember::get_repository(self.pool.clone());
        let sample_survey_survey =
            DeliberationSampleSurveySurvey::get_repository(self.pool.clone());

        let deliberation_contents = DeliberationContent::get_repository(self.pool.clone());
        let deliberation_contents_member =
            DeliberationContentMember::get_repository(self.pool.clone());
        let elearning_repo = Elearning::get_repository(self.pool.clone());

        let discussion_repo = DeliberationDiscussion::get_repository(self.pool.clone());
        let discussion_member = DeliberationDiscussionMember::get_repository(self.pool.clone());
        let discussion_resource = DeliberationDiscussionResource::get_repository(self.pool.clone());
        let disc_repo = Discussion::get_repository(self.pool.clone());
        let disc_group = DiscussionGroup::get_repository(self.pool.clone());
        let disc_res = DiscussionResource::get_repository(self.pool.clone());

        let final_repo = DeliberationFinalSurvey::get_repository(self.pool.clone());
        let final_member = DeliberationFinalSurveyMember::get_repository(self.pool.clone());
        let final_survey = DeliberationFinalSurveySurvey::get_repository(self.pool.clone());

        let draft_repo = DeliberationDraft::get_repository(self.pool.clone());
        let draft_member = DeliberationDraftMember::get_repository(self.pool.clone());
        let draft_survey = DeliberationDraftSurvey::get_repository(self.pool.clone());
        let draft_resource = DeliberationDraftResource::get_repository(self.pool.clone());

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .insert_with_tx(
                &mut *tx,
                org_id,
                started_at,
                ended_at,
                thumbnail_image,
                title,
                description,
                project_area,
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

        for DeliberationBasicInfoCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            surveys,
        } in basic_infos
        {
            if started_at >= ended_at {
                return Err(ApiError::ValidationError(
                    "started_at should be less than ended_at".to_string(),
                )
                .into());
            }

            let info = basic_info
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                )
                .await?
                .ok_or(ApiError::DeliberationBasicInfoException)?;

            for user_id in users {
                let _ = basic_info_member
                    .insert_with_tx(&mut *tx, user_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for resource_id in resources {
                let _ = basic_info_resource
                    .insert_with_tx(&mut *tx, resource_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }

            for survey_id in surveys {
                let _ = basic_info_survey
                    .insert_with_tx(&mut *tx, survey_id, info.id)
                    .await?
                    .ok_or(ApiError::DeliberationBasicInfoException)?;
            }
        }

        for DeliberationSampleSurveyCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            estimate_time,
            point,
            users,
            surveys,
        } in sample_surveys
        {
            if started_at >= ended_at {
                return Err(ApiError::ValidationError(
                    "started_at should be less than ended_at".to_string(),
                )
                .into());
            }

            let sample = sample_survey
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                    estimate_time,
                    point,
                )
                .await?
                .ok_or(ApiError::DeliberationSampleSurveyException)?;

            for user_id in users {
                let _ = sample_survey_member
                    .insert_with_tx(&mut *tx, user_id, sample.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }

            for survey_id in surveys {
                let _ = sample_survey_survey
                    .insert_with_tx(&mut *tx, survey_id, sample.id)
                    .await?
                    .ok_or(ApiError::DeliberationSampleSurveyException)?;
            }
        }

        for DeliberationContentCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            questions,
            users,
            elearnings,
        } in contents
        {
            if started_at >= ended_at {
                return Err(ApiError::ValidationError(
                    "started_at should be less than ended_at".to_string(),
                )
                .into());
            }

            let content = deliberation_contents
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                    questions,
                )
                .await?
                .ok_or(ApiError::DeliberationLearningException)?;

            for user_id in users {
                let _ = deliberation_contents_member
                    .insert_with_tx(&mut *tx, user_id, content.id)
                    .await?
                    .ok_or(ApiError::DeliberationLearningException)?;
            }

            for elearning in elearnings {
                let _ = elearning_repo
                    .insert_with_tx(
                        &mut *tx,
                        content.id,
                        elearning.title,
                        elearning.resources,
                        elearning.necessary,
                    )
                    .await?
                    .ok_or(ApiError::DeliberationLearningException)?;
            }
        }

        for DeliberationDiscussionCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            discussions,
        } in deliberation_discussions
        {
            if started_at >= ended_at {
                return Err(ApiError::ValidationError(
                    "started_at should be less than ended_at".to_string(),
                )
                .into());
            }

            let discussion = discussion_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                )
                .await?
                .ok_or(ApiError::DeliberationDiscussionException)?;

            for user_id in users {
                let _ = discussion_member
                    .insert_with_tx(&mut *tx, user_id, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            for resource_id in resources {
                let _ = discussion_resource
                    .insert_with_tx(&mut *tx, resource_id, discussion.id)
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;
            }

            for disc in discussions {
                let d = disc_repo
                    .insert_with_tx(
                        &mut *tx,
                        deliberation.id,
                        disc.started_at,
                        disc.ended_at,
                        disc.name,
                        disc.description,
                        disc.maximum_count,
                        None,
                    )
                    .await?
                    .ok_or(ApiError::DeliberationDiscussionException)?;

                for user_id in disc.users {
                    let _ = disc_group
                        .insert_with_tx(&mut *tx, d.id, user_id)
                        .await?
                        .ok_or(ApiError::DeliberationDiscussionException)?;
                }

                for res_id in disc.resources {
                    let _ = disc_res
                        .insert_with_tx(&mut *tx, d.id, res_id)
                        .await?
                        .ok_or(ApiError::DeliberationDiscussionException)?;
                }
            }
        }

        for DeliberationFinalSurveyCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            estimate_time,
            point,
            users,
            surveys,
        } in final_surveys
        {
            let d = final_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                    estimate_time,
                    point,
                )
                .await?
                .ok_or(ApiError::DeliberationFinalSurveyException)?;

            for user_id in users {
                let _ = final_member
                    .insert_with_tx(&mut *tx, user_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }

            for survey_id in surveys {
                let _ = final_survey
                    .insert_with_tx(&mut *tx, survey_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalSurveyException)?;
            }
        }

        for DeliberationDraftCreateRequest {
            started_at,
            ended_at,
            title,
            description,
            users,
            resources,
            surveys,
        } in drafts
        {
            let d = draft_repo
                .insert_with_tx(
                    &mut *tx,
                    started_at,
                    ended_at,
                    title,
                    description,
                    deliberation.id,
                )
                .await?
                .ok_or(ApiError::DeliberationFinalRecommendationException)?;

            for user_id in users {
                let _ = draft_member
                    .insert_with_tx(&mut *tx, user_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
            }

            for survey_id in surveys {
                let _ = draft_survey
                    .insert_with_tx(&mut *tx, survey_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
            }

            for resource_id in resources {
                let _ = draft_resource
                    .insert_with_tx(&mut *tx, resource_id, d.id)
                    .await?
                    .ok_or(ApiError::DeliberationFinalRecommendationException)?;
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
                get(Self::get_deliberation_by_id), // .post(Self::act_deliberation_by_id)
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
                "".to_string(),
                format!("test deliberation {now}"),
                "test description".to_string(),
                ProjectArea::City,
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
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
