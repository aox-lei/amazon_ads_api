use std::sync::Arc;

use amazon_ads_api::v3::product_ads::{
    CreateProductAds, ListProductAds, ListProductAdsFilter, ProductAdsItemForCreate,
};
mod common;

#[tokio::test]
async fn list_product_ads_test() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let filter = ListProductAdsFilter::builder()
        .include_extended_data_fields(true)
        .ad_group_id_filter(vec!["546821283664002"])
        .build();
    let response = ListProductAds::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(response.unwrap());
}

#[tokio::test]
async fn create_product_ads_test() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);

    let item = ProductAdsItemForCreate::builder()
        .ad_group_id("481941107236736")
        .campaign_id("494576620738611")
        .asin("B0CGDKMGXZ")
        .sku("KM-94373")
        .state(amazon_ads_api::v3::product_ads::StateEnumForCreate::Enabled)
        .build();
    let response = CreateProductAds::builder()
        .ads_client(ads_client)
        .product_ads(vec![item])
        .build()
        .fetch()
        .await
        .unwrap();
    dbg!(&response);
}
