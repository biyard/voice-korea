use models::*;
use surveys::_id::responses::SurveyResponseController;

use super::{resources::v1::bucket::MetadataControllerV1, reviews::v1::ReviewControllerV1};

pub mod surveys {
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
                "/users",
                crate::controllers::v1::users::UserControllerV1::route(pool.clone())?,
            )
            .nest(
                "/surveys/:survey-id/responses",
                SurveyResponseController::route(pool.clone())?,
            )
            .nest(
                "/organizations",
                crate::controllers::organizations::v2::OrganizationControllerV2::route(
                    pool.clone(),
                )?,
            )
            .nest("/reviews", ReviewControllerV1::route(pool.clone())?)
            .nest("/metadata", MetadataControllerV1::route(pool.clone())?))
    }
}
