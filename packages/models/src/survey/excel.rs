#![allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/v2/organizations/:org-id/surveys/:survey-id/responses", read_action = download_excel, database = skip)]
pub struct SurveyResponseExcel {
    pub url: String,
}
