pub type Result<T> = std::result::Result<T, ServerFnError>;

use crate::utils::metadata::get_ext_from_name;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::{GetObjectUriRequest, GetObjectUriResponse, MetadataRequest};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::config;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetadataApi {}

impl MetadataApi {
    pub fn init() {
        let srv = Self {};
        use_context_provider(|| srv);
    }

    pub async fn upload_metadata(&self, req: MetadataRequest) -> Result<String> {
        let client = reqwest::Client::new();
        let res = match client
            .post(format!("{}/metadata/v2/put-uri", &*config::get().api_url))
            .json(&GetObjectUriRequest {
                filenames: vec![req.file_name.clone()],
            })
            .send()
            .await
        {
            Ok(v) => match v.json::<GetObjectUriResponse>().await {
                Ok(response) => Ok(response),
                Err(e) => {
                    tracing::error!("Failed to deserialize response: {}", e);
                    Err(ServerFnError::new(format!(
                        "upload metadata failed: deserialization error: {:?}",
                        e
                    )))
                }
            },
            Err(e) => {
                tracing::error!("Failed to upload metadata: network error {}", e);
                Err(ServerFnError::new(format!(
                    "upload metadata failed: network error: {:?}",
                    e
                )))
            }
        }?;

        let presigned_uri = res.presigned_uris[0].clone();
        let uri = res.uris[0].clone();

        tracing::debug!(
            "presigned_uri: {} Request body size: {}",
            presigned_uri.clone(),
            req.bytes.len()
        );

        let ext = get_ext_from_name(&req.file_name.clone()).unwrap();
        let content_type = HeaderValue::from_str(&format!("image/{}", ext)).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, content_type);

        match reqwest::Client::new()
            .put(presigned_uri.clone())
            .headers(headers)
            .body(req.bytes)
            .send()
            .await
        {
            Ok(_) => Ok(uri.clone()),
            Err(e) => {
                tracing::error!("Failed to upload metadata {:?}", e);
                return Err(ServerFnError::new(format!(
                    "upload metadata failed: {:?}",
                    e
                )));
            }
        }
    }
}
