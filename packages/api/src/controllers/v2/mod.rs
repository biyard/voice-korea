use deliberations::{ProjectControllerV2, _id::responses::DeliberationResponseController};
use models::*;
use reviews::ReviewControllerV2;
use surveys::_id::responses::SurveyResponseController;

use super::resources::v1::bucket::MetadataControllerV1;

pub mod reviews;

pub mod surveys {
    pub mod _id {
        pub mod responses;
    }
}

pub mod organizations {
    pub mod _id;
}

pub mod deliberations;

#[derive(Clone, Debug)]
pub struct Version2Controller {}

impl Version2Controller {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/surveys/:survey-id/responses",
                SurveyResponseController::route(pool.clone())?,
            )
            .nest(
                "/organizations",
                crate::controllers::organizations::v2::OrganizationController::route(pool.clone())?,
            )
            .nest("/reviews", ReviewControllerV2::route(pool.clone())?)
            .nest("/metadata", MetadataControllerV1::route(pool.clone())?)
            .nest(
                "/deliberations/:deliberation-id/responses",
                DeliberationResponseController::route(pool.clone())?,
            )
            .nest("/projects", ProjectControllerV2::route(pool.clone())?))
    }
}
