use std::sync::Arc;

use amazon_ads_api::ads_v1::sp::targets::{ListTargets, ListTargetsFilter, UpdateTarget};
use amazon_ads_api::ads_v1::sp::types::enums::SPTargetType;
use amazon_ads_api::ads_v1::sp::types::targets::SPTargetUpdate;
mod common;

#[tokio::test]
async fn list_targets_test() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let filter = ListTargetsFilter::builder()
        .ad_group_id_filter(vec!["294948230297731"])
        .target_type_filter(vec![SPTargetType::Theme])
        .build();
    let response = ListTargets::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build()
        .fetch()
        .call()
        .await;
    dbg!(&response);
}

#[tokio::test]
async fn update_target_test() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let body = SPTargetUpdate::builder("214060908268872").bid(0.02).build();

    let api = UpdateTarget::builder()
        .targets(vec![body])
        .ads_client(ads_client)
        .build();
    let res = api.fetch().await;
    dbg!(&res);
}
