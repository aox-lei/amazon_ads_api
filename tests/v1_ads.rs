use std::sync::Arc;

use amazon_ads_api::ads_v1::sp::ads::{ListAds, ListAdsFilter};
use amazon_ads_api::client;
mod common;

#[tokio::test]
async fn list_ads_test() {
    let credential = common::Credential::default();
    let ads_client = client::AdsClient::builder()
        .seller_id("AUXYJQK8O7TFU")
        .country_code("UK")
        .client_id(&credential.client_id)
        .client_secret(&credential.client_secret)
        .refresh_token(&credential.refresh_token)
        .build();
    let ads_client = Arc::new(ads_client);
    let filter = ListAdsFilter::builder().max_results(10).ad_id_filter(vec!["397867212646997"]).build();
    let response = ListAds::builder()
        .ads_client(ads_client)
        .profile_id(credential.profile_id.unwrap())
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(&response);
}
