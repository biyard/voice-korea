use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, RequestBuilder,
};
lazy_static! {
    static ref API_ENDPOINT: &'static str =
        option_env!("NONCE_LAB_API_ENDPOINT").expect(" \"NONCE_LAB_API_ENDPOINT\" required");
    static ref API_TOKEN: &'static str =
        option_env!("NONCE_LAB_API_TOKEN").expect(" \"NONCE_LAB_API_TOKEN\" required");
}

pub struct NonceLabClient {
    base_url: String,
    pub client: Client,
}

impl NonceLabClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", API_TOKEN.clone())).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            base_url: API_ENDPOINT.to_string(),
        })
    }

    pub fn _get(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url.clone(), endpoint))
    }

    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url.clone(), endpoint))
    }
}
