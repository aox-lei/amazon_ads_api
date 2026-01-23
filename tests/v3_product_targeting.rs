use std::sync::Arc;

use amazon_ads_api::v3::product_targeting::ListNegativeTargetsBrandsSearch;
mod common;

#[tokio::test]
async fn list_brand_search() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let api = ListNegativeTargetsBrandsSearch::builder()
        .ads_client(ads_client)
        .keyword("apple")
        .build();

    let response = api.fetch().await.unwrap();
    dbg!(&response);
}
