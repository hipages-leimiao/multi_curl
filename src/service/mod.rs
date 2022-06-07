use async_trait::async_trait;
use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

pub struct ApiClient {
    client: Client,
}
type RequestHeaderMap = HeaderMap<HeaderValue>;
#[async_trait]
pub(crate) trait Requests {
    async fn send_post(
        &self,
        url: &str,
        body: String,
        headers: RequestHeaderMap,
    ) -> Result<Bytes, reqwest::Error>;
}

impl ApiClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    pub fn common_headers(token: &str) -> RequestHeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::USER_AGENT, "rust-agent".parse().unwrap());
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        headers
    }
}

#[async_trait]
impl Requests for ApiClient {
    async fn send_post(
        &self,
        url: &str,
        body: String,
        headers: RequestHeaderMap,
    ) -> Result<Bytes, reqwest::Error> {
        self.client
            .post(url)
            .body(body)
            .headers(headers)
            .send()
            .await?
            .bytes()
            .await
    }
}
