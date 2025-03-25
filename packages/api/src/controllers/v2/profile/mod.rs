use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use models::{
    deliberation::Deliberation,
    deliberation_response::DeliberationResponse,
    deliberation_user::DeliberationUser,
    dto::{ProfileProjectsData, ProfileProjectsDataGetResponse, ProfileProjectsDataParam},
};
use models::{dto::ProfileProjectsDataReadActionType, *};

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct ProfileController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ProfileController {
    async fn query(&self, user_id: i64) -> Result<ProfileProjectsData> {
        let mut tx = self.pool.begin().await?;

        let pr_responses = DeliberationResponse::query_builder()
            .user_id_equals(user_id)
            .page(1)
            .limit(30)
            .order_by_created_at_desc()
            .query()
            .map(DeliberationResponse::from)
            .fetch_all(&mut *tx)
            .await?;

        let ds_responses = DeliberationUser::query_builder()
            .user_id_equals(user_id)
            .page(1)
            .limit(30)
            .order_by_created_at_desc()
            .query()
            .map(DeliberationUser::from)
            .fetch_all(&mut *tx)
            .await?;

        let mut participated_projects: Vec<Deliberation> = vec![];
        let mut designed_projects: Vec<Deliberation> = vec![];

        for pr_response in pr_responses {
            let deliberation_id = pr_response.deliberation_id;

            let deliberation = Deliberation::query_builder()
                .id_equals(deliberation_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&mut *tx)
                .await?;

            participated_projects.push(deliberation);
        }

        for ds_response in ds_responses {
            let deliberation_id = ds_response.deliberation_id;

            let deliberation = Deliberation::query_builder()
                .id_equals(deliberation_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&mut *tx)
                .await?;

            designed_projects.push(deliberation);
        }

        Ok(ProfileProjectsData {
            designed_projects,
            participated_projects,
        })
    }
}

impl ProfileController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/projects", get(Self::get_projects))
            .with_state(self.clone()))
    }

    pub async fn get_projects(
        State(ctrl): State<ProfileController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<ProfileProjectsDataParam>,
    ) -> Result<Json<ProfileProjectsDataGetResponse>> {
        tracing::debug!("list_projects {:?}", q);

        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        match q {
            ProfileProjectsDataParam::Read(param)
                if param.action == Some(ProfileProjectsDataReadActionType::Find) =>
            {
                Ok(Json(ProfileProjectsDataGetResponse::Read(
                    ctrl.query(user_id).await?,
                )))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}
