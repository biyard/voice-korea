pub mod landing;
pub mod organizations;
pub mod projects;

use deliberations::_id::responses::DeliberationResponseController;
use models::*;
use surveys::_id::responses::SurveyResponseController;

use super::{resources::v1::bucket::MetadataControllerV1, reviews::v1::ReviewControllerV1};

pub mod surveys {
    pub mod _id {
        pub mod responses;
    }
}

pub mod deliberations {
    pub mod _id {
        pub mod responses;
    }
}

#[derive(Clone, Debug)]
pub struct Version2Controller {}

impl Version2Controller {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/landing",
                landing::LandingController::new(pool.clone()).route()?,
            )
            .nest(
                "/projects",
                projects::DeliberationProjectController::new(pool.clone()).route()?,
            )
            .nest(
                "/surveys/:survey-id/responses",
                SurveyResponseController::route(pool.clone())?,
            )
            .nest(
                "/organizations",
                organizations::OrganizationController::route(pool.clone())?,
            )
            .nest("/reviews", ReviewControllerV1::route(pool.clone())?)
            .nest("/metadata", MetadataControllerV1::route(pool.clone())?)
            .nest(
                "/deliberations/:deliberation-id/responses",
                DeliberationResponseController::route(pool.clone())?,
            ))
    }
}
