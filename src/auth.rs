use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use cached::proc_macro::cached;
use cached::TimedCache;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
}

#[cached(
    result = true,
    ty = "TimedCache<String, AccessTokenResponse>",
    create = "{ cached::TimedCache::with_lifespan_and_capacity(Duration::from_secs(3500), 100) }",
    key = "String",
    convert = r#"{ format!("{}", _seller_id) }"#
)]
pub async fn get_access_token(
    _seller_id: &str,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<AccessTokenResponse> {
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("refresh_token", refresh_token);

    // 2. 发送 POST 请求
    let response = client
        .post("https://api.amazon.com/auth/o2/token")
        .form(&params)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.json::<AccessTokenResponse>().await?)
}
