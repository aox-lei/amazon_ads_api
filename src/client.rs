use std::sync::Arc;

use crate::middleware::AuthMiddleware;
use crate::region::AmazonRegion;
use bon::bon;
use reqwest::header;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

#[derive(Debug)]
pub struct AdsClient {
    inner: Arc<ClientWithMiddleware>,
    region: AmazonRegion,
}
use anyhow::Result;

#[bon]
impl AdsClient {
    #[builder]
    pub fn new(
        country_code: &str,
        seller_id: &str,
        client_id: &str,
        client_secret: &str,
        refresh_token: &str,
    ) -> Self {
        let auth_middleware = AuthMiddleware {
            seller_id: seller_id.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
        };
        let http = ClientBuilder::new(reqwest::Client::new())
            .with(auth_middleware)
            .build();
        let region = AmazonRegion::from_country_code(country_code).expect("country code failed");
        Self {
            inner: Arc::new(http),
            region,
        }
    }

    pub async fn post(
        &self,
        path: &str,
        profile_id: &str,
        json_body: serde_json::Value,
    ) -> Result<Response> {
        let url = self.url(path);
        let json_string = serde_json::to_string(&json_body)?;
        let media_type = "application/vnd.spProductAd.v3+json";
        let res = self
            .inner
            .post(url)
            .header("Amazon-Advertising-API-Scope", profile_id)
            .header(header::CONTENT_TYPE, media_type)
            .header(header::ACCEPT, media_type)
            .body(json_string)
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.region.api_endpoint(), path)
    }
}
