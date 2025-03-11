use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use dto::{LandingData, LandingDataGetResponse, LandingDataParam, LandingDataReadActionType};
use models::{
    deliberation_project::DeliberationProjectSummary, organization::OrganizationSummary,
    review::ReviewSummary, *,
};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct LandingController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl LandingController {
    async fn query(&self) -> Result<LandingData> {
        let project_query = DeliberationProjectSummary::query_builder()
            .order_by_created_at_desc()
            .limit(10);
        let organization_query = OrganizationSummary::query_builder()
            .order_by_created_at_desc()
            .limit(10);
        let review_query = ReviewSummary::query_builder()
            .order_by_created_at_desc()
            .limit(10);

        let mut tx = self.pool.begin().await?;

        let projects = project_query
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&mut *tx)
            .await?;

        let organizations = organization_query
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&mut *tx)
            .await?;

        let reviews = review_query
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(LandingData {
            projects,
            organizations,
            reviews,
        })
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
