use std::sync::Arc;

use amazon_ads_api::products::{ProductMetadata, ProductMetadataFilter};
mod common;

#[tokio::test]
async fn list_product() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let filter = ProductMetadataFilter::builder()
        .check_eligibility(false)
        .check_item_details(false)
        .build();
    let api = ProductMetadata::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build();

    let response = api.fetch().call().await.unwrap();
    dbg!(&response);
}
