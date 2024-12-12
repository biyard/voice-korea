use models::prelude::{AdminSurveyCompleteRequest, AdminSurveyCompleteResponse};
use reqwest::{header::HeaderMap, Client};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct Watcher {
    endpoint: String,
    req_client: Client,
}

impl Watcher {
    pub fn new() -> Result<Self> {
        let endpoint = option_env!("API_DOMAIN")
            .unwrap_or("http://localhost:3000")
            .to_string();
        let mut headers = HeaderMap::new();
        headers.insert(
            "SERVER-KEY",
            option_env!("INTERNAL_SERVER_KEY")
                .unwrap_or("server-key")
                .parse()?,
        );
        let req_client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Watcher {
            endpoint,
            req_client,
        })
    }

    pub async fn finalize_survey(&self, date: String) -> Result<AdminSurveyCompleteResponse> {
        let data = AdminSurveyCompleteRequest { ended_at: date };
        let res = self
            .req_client
            .request(
                reqwest::Method::PATCH,
                format!("{}/m1/survey", self.endpoint),
            )
            .json(&data)
            .send()
            .await?;
        let res = res.error_for_status()?.json().await?;
        Ok(res)
    }
}
