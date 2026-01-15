use std::sync::Arc;

use amazon_ads_api::ads_v1::sp_global::ads::{CreateAds, ListAds, ListAdsFilter};
mod common;

#[tokio::test]
async fn list_ads_test() {
    let (ads_client, _profile_id) = common::get_ads_client();
    let ads_client = Arc::new(ads_client);
    let filter = ListAdsFilter::builder()
        .max_results(10)
        // .ad_group_id_filter(vec!["5000096132751368993", "485766861743528", "546821283664002", "4999899225094945252"])
        // .ad_id_filter(vec!["397867212646997"])
        .build();
    let response = ListAds::builder()
        .ads_client(ads_client)
        .account_id("amzn1.ads-account.g.a8a482m7xu81z4dallvt606cc")
        .filter(filter)
        .build()
        .fetch()
        .call()
        .await;
    dbg!(&response);
}

#[tokio::test]
async fn create_ads_test() {
    let (ads_client, _profile_id) = common::get_ads_client();
    let ads_client = Arc::new(ads_client);
    let product = CreateAds::by_skus()
        .ad_group_id("4999899225094945252")
        .country_codes(vec!["UK"])
        .skus(vec!["KM-QqwSUL-White-30x120cm"])
        .call();
    let api = CreateAds::builder()
        .ads(product)
        .ads_client(ads_client)
        .account_id("amzn1.ads-account.g.a8a482m7xu81z4dallvt606cc")
        .build();
    let res = api.fetch().await;
    dbg!(&res);
}
