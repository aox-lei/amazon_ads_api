use amazon_ads_api::region;
#[tokio::test]
async fn region_test() {
    dbg!(region::AmazonRegion::NA.auth_url());
    dbg!(region::AmazonRegion::from_country_code("SS").map(|region| region.auth_url()));
}
