use std::sync::Arc;

use amazon_ads_api::ads_v1::sp::ads::{CreateAds, DelAds, ListAds, ListAdsFilter};
mod common;

#[tokio::test]
async fn list_ads_test() {
    let (ads_client, profile_id) = common::get_ads_client();
    let ads_client = Arc::new(ads_client);
    let filter = ListAdsFilter::builder()
        .ad_group_id_filter(vec!["546821283664002"])
        .max_results(10)
        .build();
    let response = ListAds::builder()
        .ads_client(ads_client)
        .profile_id(profile_id)
        .filter(filter)
        .build()
        .fetch()
        .call()
        .await;
    dbg!(&response);
}

#[tokio::test]
async fn del_ads_test() {
    let (ads_client, profile_id) = common::get_ads_client();
    let ads_client = Arc::new(ads_client);
    let response = DelAds::builder()
        .ads_client(ads_client)
        .profile_id(profile_id)
        .ad_ids(vec!["332526858188251"])
        .build()
        .fetch()
        .await;
    dbg!(&response);
}

#[tokio::test]
async fn create_ads_test() {
    let (ads_client, profile_id) = common::get_ads_client();
    let ads_client = Arc::new(ads_client);
    let product = CreateAds::by_skus()
        .ad_group_id("546821283664002")
        .skus(vec!["KM-HBW0Jj-Green-"])
        .call();
    let api = CreateAds::builder()
        .ads(product)
        .ads_client(ads_client)
        .profile_id(profile_id)
        .build();
    let res = api.fetch().await;
    dbg!(&res);
}
