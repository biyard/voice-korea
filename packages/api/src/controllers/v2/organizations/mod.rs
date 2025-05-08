pub mod _id;
use _id::*;

use by_axum::axum::extract::{Query, State};
use by_axum::axum::{Extension, Json};
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
        routing::get,
    },
};
use by_types::DatabaseConfig;
use models::organization::{
    OrganizationGetResponse, OrganizationParam, OrganizationQuery, OrganizationQueryActionType,
    OrganizationQueryBy, OrganizationSorter, OrganizationSummary,
};
use models::{QueryResponse, Result};
use models::{User, UserReadAction};
use reqwest::StatusCode;
use sqlx::postgres::{PgPoolOptions, PgRow};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct OrganizationPath {
    pub org_id: i64,
}

#[derive(Clone, Debug)]
pub struct OrganizationController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl OrganizationController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = OrganizationController { pool: pool.clone() };
        let no_auth = by_axum::axum::Router::new()
            .route("/", get(Self::get_organizations))
            .with_state(ctrl.clone());

        let auth = by_axum::axum::Router::new()
            .nest(
                "/:org-id/deliberations",
                deliberations::DeliberationController::new(pool.clone()).route()?,
            )
            .nest(
                "/:org-id/drafts",
                reports::DeliberationReportController::new(pool.clone()).route()?,
            )
            .nest(
                "/:org-id/surveys",
                surveys::SurveyControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/panels",
                panels::PanelControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/resources",
                resources::ResourceControllerV1::route(pool.clone())?,
            )
            .nest(
                "/:org-id/members",
                members::OrganizationMemberController::new(pool.clone()).route(),
            )
            .nest(
                "/:org-id/groups",
                groups::GroupController::route(pool.clone())?,
            )
            .nest(
                "/:org-id/invitations",
                invitations::InvitationControllerV2::route(pool.clone())?,
            )
            .layer(middleware::from_fn(authorize_organization));

        Ok(auth.merge(no_auth))
    }
}

impl OrganizationController {
    pub async fn get_organizations(
        State(ctrl): State<OrganizationController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<OrganizationParam>,
    ) -> Result<Json<OrganizationGetResponse>> {
        tracing::debug!("get_organizations {:?}", q);

        match q {
            OrganizationParam::Query(param) => match param.action {
                Some(OrganizationQueryActionType::Search) => Ok(Json(
                    OrganizationGetResponse::Query(ctrl.search(auth, param).await?),
                )),
                _ => Ok(Json(OrganizationGetResponse::Query(
                    ctrl.query(auth, param).await?,
                ))),
            },
            OrganizationParam::Custom(param) => Ok(Json(OrganizationGetResponse::Query(
                ctrl.custom_query(auth, param).await?,
            ))),
        }
    }
}

impl OrganizationController {
    async fn search(
        &self,
        _auth: Option<Authorization>,
        OrganizationQuery { name, .. }: OrganizationQuery,
    ) -> Result<QueryResponse<OrganizationSummary>> {
        let mut total_count = 0;
        let items: Vec<OrganizationSummary> = OrganizationSummary::query_builder()
            .name_contains(name.unwrap_or_default())
            .with_count()
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.get("total_count");
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn custom_query(
        &self,
        _auth: Option<Authorization>,
        param: OrganizationQueryBy,
    ) -> Result<QueryResponse<OrganizationSummary>> {
        let mut total_count = 0;

        let mut builder = OrganizationSummary::query_builder().limit(100).page(1);

        if param.sorter == OrganizationSorter::Newest {
            builder = builder.order_by_created_at_desc();
        } else {
            builder = builder.order_by_created_at_asc();
        }

        let items: Vec<OrganizationSummary> = builder
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: OrganizationQuery,
    ) -> Result<QueryResponse<OrganizationSummary>> {
        let mut total_count = 0;

        let builder = OrganizationSummary::query_builder()
            .limit(param.size())
            .page(param.page());

        let items: Vec<OrganizationSummary> = builder
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

#[allow(dead_code)]
pub async fn authorize_organization(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    tracing::debug!("request: {:?}", req);
    let auth = req.extensions().get::<Option<Authorization>>();
    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.unwrap();

    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.clone().unwrap();

    let user_id = match auth {
        Authorization::Bearer { claims } => claims.sub,
        _ => {
            tracing::debug!("Authorization header is not Bearer");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    tracing::debug!("request: {:?} {:?}", user_id, req.uri().path());

    let org_id = req.uri().path().split("/").collect::<Vec<_>>();

    if org_id.len() < 2 {
        return Ok(next.run(req).await);
    }

    let org_id = org_id[1].to_string();

    tracing::debug!("org_id: {}", org_id);

    let conf = crate::config::get();
    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let repo = User::get_repository(pool);

    let user_id = user_id.parse::<i64>().unwrap();
    let org_id = org_id.parse::<i64>().unwrap();

    match repo
        .find_one(&UserReadAction::new().find_by_id(user_id))
        .await
    {
        Ok(user) => {
            if !user.orgs.iter().any(move |o| o.id == org_id) {
                tracing::error!("User is not member of organization");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Err(e) => {
            tracing::error!("Failed to find user: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    return Ok(next.run(req).await);
}
