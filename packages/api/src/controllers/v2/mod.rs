pub mod landing;
pub mod metadata;
pub mod organizations;
pub mod projects;
pub mod reviews;

use deliberations::_id::responses::DeliberationResponseController;
use models::*;
use surveys::_id::responses::SurveyResponseController;

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
                "/metadata",
                metadata::MetadataControllerV1::route(pool.clone())?,
            )
            .nest(
                "/landing",
                landing::LandingController::new(pool.clone()).route()?,
            )
            .nest(
                "/projects",
                projects::DeliberationProjectController::new(pool.clone()).route()?,
            )
            .nest(
                "/organizations",
                organizations::OrganizationController::route(pool.clone())?,
            )
            .nest("/reviews", reviews::ReviewController::route(pool.clone())?)
            .nest(
                "/surveys/:survey-id/responses",
                SurveyResponseController::route(pool.clone())?,
            )
            .nest(
                "/deliberations/:deliberation-id/responses",
                DeliberationResponseController::route(pool.clone())?,
            ))
    }
}
