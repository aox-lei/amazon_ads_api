use std::sync::Arc;

use crate::middleware::AuthMiddleware;
use crate::region::AmazonRegion;
use bon::bon;
use http::HeaderValue;
use reqwest::header;
use reqwest::header::HeaderMap;
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
        profile_id: Option<&str>,
        account_id: Option<&str>,
    ) -> Self {
        let auth_middleware = AuthMiddleware {
            seller_id: seller_id.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
        };
        let mut headers = HeaderMap::new();
        if let Some(profile_id) = profile_id {
            headers.insert(
                "Amazon-Advertising-API-Scope",
                HeaderValue::from_str(profile_id).unwrap(),
            );
        }
        if let Some(account_id) = account_id {
            headers.insert(
                "Amazon-Ads-AccountId",
                HeaderValue::from_str(account_id).unwrap(),
            );
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_str("application/json").unwrap(),
            );
        }

        let http = ClientBuilder::new(
            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        )
        .with(auth_middleware)
        .build();
        let region = AmazonRegion::from_country_code(country_code).expect("country code failed");
        Self {
            inner: Arc::new(http),
            region,
        }
    }

    #[builder]
    pub async fn post(
        &self,
        path: &str,
        json_body: serde_json::Value,
        content_type: Option<&str>,
    ) -> Result<Response> {
        let url = self.url(path);
        let json_string = serde_json::to_string(&json_body)?;

        let mut req_builder = self.inner.post(url);
        if let Some(content_type) = content_type {
            req_builder = req_builder.header(header::CONTENT_TYPE, content_type);
        }

        let res = req_builder
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
