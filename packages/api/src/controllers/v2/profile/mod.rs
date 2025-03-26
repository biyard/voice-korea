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
    dto::{ProfileData, ProfileDataGetResponse, ProfileDataParam},
};
use models::{dto::ProfileDataReadActionType, *};

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct ProfileController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ProfileController {
    async fn query(&self, user_id: i64) -> Result<ProfileData> {
        let mut tx = self.pool.begin().await?;

        let user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_one(&mut *tx)
            .await?;

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

        Ok(ProfileData {
            designed_projects,
            participated_projects,
            user,
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
        Query(q): Query<ProfileDataParam>,
    ) -> Result<Json<ProfileDataGetResponse>> {
        tracing::debug!("list_projects {:?}", q);

        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        match q {
            ProfileDataParam::Read(param)
                if param.action == Some(ProfileDataReadActionType::Find) =>
            {
                Ok(Json(ProfileDataGetResponse::Read(
                    ctrl.query(user_id).await?,
                )))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}
