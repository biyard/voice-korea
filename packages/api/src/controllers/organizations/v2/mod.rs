pub mod deliberations;

use by_axum::{
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
    },
};
use by_types::DatabaseConfig;
use deliberations::DeliberationController;
use models::*;
use reqwest::StatusCode;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct OrganizationControllerV2 {}

impl OrganizationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/:org-id/deliberations",
                DeliberationController::new(pool.clone()).route()?,
            )
            .nest(
                "/:org-id/surveys",
                crate::controllers::survey::v2::SurveyControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/panels",
                crate::controllers::panels::v2::PanelControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/resources",
                crate::controllers::resources::v1::ResourceControllerV1::route(pool.clone())?,
            )
            .nest(
                "/:org-id/members",
                crate::controllers::members::v2::MemberControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/groups",
                crate::controllers::groups::v2::GroupControllerV2::route(pool.clone())?,
            )
            .nest(
                "/:org-id/invitations",
                crate::controllers::invitations::v2::InvitationControllerV2::route(pool.clone())?,
            )
            .layer(middleware::from_fn(authorize_organization)))
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
