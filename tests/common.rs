use amazon_ads_api::client;
use bon::builder;
use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct Credential {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
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
        }
    }
}

#[allow(dead_code)]
#[builder]
pub fn get_ads_client(account_id: Option<&str>, profile_id: Option<&str>) -> client::AdsClient {
    let credential = Credential::default();
    let ads_client = client::AdsClient::builder()
        .seller_id("AUXYJQK8O7TFU")
        .country_code("UK")
        .client_id(&credential.client_id)
        .client_secret(&credential.client_secret)
        .refresh_token(&credential.refresh_token)
        .maybe_account_id(account_id)
        .maybe_profile_id(profile_id)
        .build();
    ads_client
}

#[allow(dead_code)]
pub fn account_id() -> String {
    dotenv().ok();
    env::var("AMAZON_ACCOUNT_ID").expect("Missing AMAZON_ACCOUNT_ID in .env file")
}

#[allow(dead_code)]
pub fn profile_id() -> String {
    dotenv().ok();
    env::var("AMAZON_PROFILE_ID").expect("Missing AMAZON_PROFILE_ID in .env file")
}
