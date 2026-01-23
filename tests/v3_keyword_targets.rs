use std::sync::Arc;

use amazon_ads_api::v3::keyword_targets::{
    KeywordRecommendations, KeywordRecommendationsFilterForASINS,
};
mod common;

#[tokio::test]
async fn list_keyword_recommendations() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let filter = KeywordRecommendationsFilterForASINS::builder()
        .asins(vec!["B0FMS7N9W5"])
        .build();
    let api = KeywordRecommendations::builder()
        .ads_client(ads_client)
        .filter(filter.into())
        .build();

    let response = api.fetch().await.unwrap();
    dbg!(&response);
}
