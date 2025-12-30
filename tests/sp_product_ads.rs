use std::sync::Arc;

use amazon_ads_api::client;
use amazon_ads_api::sp::product_ads::{ListProductAds, ListProductAdsFilter};
mod common;
#[tokio::test]
async fn list_product_ads_test() {
    let credential = common::Credential::default();
    let ads_client = client::AdsClient::builder()
        .seller_id("aaaa")
        .country_code("UK")
        .client_id(&credential.client_id)
        .client_secret(&credential.client_secret)
        .refresh_token(&credential.refresh_token)
        .build();
    let ads_client = Arc::new(ads_client);
    let filter = ListProductAdsFilter::builder().build();
    let response = ListProductAds::builder()
        .ads_client(ads_client)
        .profile_id(credential.profile_id.unwrap())
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(response.unwrap().product_ads.len());
}
