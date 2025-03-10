use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use dto::{LandingData, LandingDataGetResponse, LandingDataParam, LandingDataReadActionType};
use models::*;

#[derive(Clone, Debug)]
pub struct LandingController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl LandingController {
    async fn query(&self) -> Result<LandingData> {
        todo!()
    }
}

impl LandingController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_web))
            .with_state(self.clone()))
    }

    pub async fn get_web(
        State(ctrl): State<LandingController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<LandingDataParam>,
    ) -> Result<Json<LandingDataGetResponse>> {
        tracing::debug!("list_web {:?}", q);

        match q {
            LandingDataParam::Read(param)
                if param.action == Some(LandingDataReadActionType::FindOne) =>
            {
                Ok(Json(LandingDataGetResponse::Read(ctrl.query().await?)))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}
