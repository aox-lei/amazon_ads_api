use std::sync::Arc;

use amazon_ads_api::client;
use amazon_ads_api::sp::product_ads::{
    CreateProductAds, ListProductAds, ListProductAdsFilter, ProductAdsItemForCreate,
};
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
    let filter = ListProductAdsFilter::builder()
        .ad_group_id_filter(vec!["aaaa"])
        .include_extended_data_fields(true)
        .build();
    let response = ListProductAds::builder()
        .ads_client(ads_client)
        .profile_id(credential.profile_id.unwrap())
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(response.unwrap().product_ads.len());
}

#[tokio::test]
async fn create_product_ads_test() {
    let credential = common::Credential::default();
    let ads_client = client::AdsClient::builder()
        .seller_id("aaaa")
        .country_code("UK")
        .client_id(&credential.client_id)
        .client_secret(&credential.client_secret)
        .refresh_token(&credential.refresh_token)
        .build();
    let ads_client = Arc::new(ads_client);

    let item = ProductAdsItemForCreate::builder()
        .ad_group_id("515228709275405")
        .campaign_id("358885870824039")
        .asin("B0FHQ2PTZD")
        .sku("KM-bn7k3g-2pcs-Diameter354inH315in")
        .state(amazon_ads_api::sp::product_ads::StateEnumForCreate::Enabled)
        .build();
    let response = CreateProductAds::builder()
        .ads_client(ads_client)
        .profile_id(credential.profile_id.unwrap())
        .product_ads(vec![item])
        .build()
        .fetch()
        .await
        .unwrap();
    dbg!(&response);
}
