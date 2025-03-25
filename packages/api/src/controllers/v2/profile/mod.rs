use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use models::dto::{
    ProfileProjectsData, ProfileProjectsDataGetResponse, ProfileProjectsDataParam,
    ProfileProjectsDataReadActionType,
};
use models::*;

#[derive(Clone, Debug)]
pub struct ProfileController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ProfileController {
    async fn query(&self, user_id: i64) -> Result<ProfileProjectsData> {
        let _user_id = user_id;

        Ok(ProfileProjectsData {
            designed_projects: vec![],
            participated_projects: vec![],
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
                if param.action == Some(ProfileProjectsDataReadActionType::FindOne) =>
            {
                Ok(Json(ProfileProjectsDataGetResponse::Read(
                    ctrl.query(user_id).await?,
                )))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}
