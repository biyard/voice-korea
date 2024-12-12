use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, RequestBuilder,
};

use crate::utils::error::ApiError;
pub mod models;

use models as nonce_lab_models;

use nonce_lab_models::{
    NonceLabCreateSurveyRequest, NonceLabCreateSurveyResponse, NonceLabGetSurveyDto,
    NonceLabSurveyResultResponse,
};
lazy_static! {
    static ref API_ENDPOINT: &'static str =
        option_env!("NONCE_LAB_API_ENDPOINT").expect(" \"NONCE_LAB_API_ENDPOINT\" required");
    static ref API_TOKEN: &'static str =
        option_env!("NONCE_LAB_API_TOKEN").expect(" \"NONCE_LAB_API_TOKEN\" required");
}

pub struct NonceLabClient {
    base_url: String,
    client: Client,
}

impl NonceLabClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", API_TOKEN.clone())).unwrap(),
        );
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Client Build Failed");

        Self {
            client,
            base_url: API_ENDPOINT.to_string(),
        }
    }

    fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url.clone(), endpoint))
    }

    fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url.clone(), endpoint))
    }

    pub async fn get_survey(&self, nonce_lab_id: u32) -> Result<NonceLabGetSurveyDto, ApiError> {
        let res = self
            .get(&format!("/v1/vendor/survey/{}", nonce_lab_id))
            .send()
            .await
            .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;
        match res.error_for_status() {
            Ok(v) => match v.json().await {
                Ok(v) => Ok(v),
                Err(e) => Err(ApiError::JSONSerdeError(e.to_string())),
            },
            Err(e) => Err(ApiError::ReqwestFailed(e.to_string())),
        }
    }

    pub async fn get_survey_responses(&self, nonce_lab_id: u32) -> Result<Option<u64>, ApiError> {
        let res = self.get_survey(nonce_lab_id).await?;
        if let Some(v) = res.response_count_map {
            Ok(Some(v.values().sum()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_survey_result(
        &self,
        nonce_lab_id: u32,
    ) -> Result<NonceLabSurveyResultResponse, ApiError> {
        let res = self
            .get(&format!("/v1/vendor/survey/{}/result", nonce_lab_id))
            .send()
            .await
            .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;
        match res.error_for_status() {
            Ok(v) => match v.json().await {
                Ok(v) => Ok(v),
                Err(e) => Err(ApiError::JSONSerdeError(e.to_string())),
            },
            Err(e) => Err(ApiError::ReqwestFailed(e.to_string())),
        }
    }

    pub async fn create_survey(
        &self,
        survey_dto: NonceLabCreateSurveyRequest,
    ) -> Result<u32, ApiError> {
        let res = self
            .post("/v1/vendor/survey")
            .json(&survey_dto)
            .send()
            .await
            .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;
        let res: NonceLabCreateSurveyResponse = match res.error_for_status() {
            Ok(v) => match v.json().await {
                Ok(v) => v,
                Err(e) => return Err(ApiError::JSONSerdeError(e.to_string())),
            },
            Err(e) => {
                return Err(ApiError::ReqwestFailed(e.to_string()));
            }
        };
        Ok(res.id)
    }
}
