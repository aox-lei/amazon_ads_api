use amazon_ads_api::client;
use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct Credential {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub profile_id: Option<String>,
}

impl Default for Credential {
    fn default() -> Self {
        dotenv().ok();

        let client_id =
            env::var("AMAZON_CLIENT_ID").expect("Missing AMAZON_CLIENT_ID in .env file");
        let client_secret =
            env::var("AMAZON_CLIENT_SECRET").expect("Missing AMAZON_CLIENT_SECRET in .env file");
        let refresh_token =
            env::var("AMAZON_REFRESH_TOKEN").expect("Missing AMAZON_REFRESH_TOKEN in .env file");
        Self {
            client_id,
            client_secret,
            refresh_token,
            profile_id: Some("2019288267937185".to_string()),
        }
    }
}

#[allow(dead_code)]
pub fn get_ads_client() -> (client::AdsClient, String) {
    let credential = Credential::default();
    let ads_client = client::AdsClient::builder()
        .seller_id("AUXYJQK8O7TFU")
        .country_code("UK")
        .client_id(&credential.client_id)
        .client_secret(&credential.client_secret)
        .refresh_token(&credential.refresh_token)
        .build();
    (ads_client, credential.profile_id.unwrap())
}
