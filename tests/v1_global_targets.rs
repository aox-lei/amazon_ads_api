use std::sync::Arc;

use amazon_ads_api::ads_v1::sp_global::targets::{
    ListGlobalTargets, ListGlobalTargetsFilter, UpdateGlobalTarget,
};
use amazon_ads_api::ads_v1::sp_global::types::enums::SPGlobalTargetType;
use amazon_ads_api::ads_v1::sp_global::types::targets::SPGlobalTargetUpdate;
mod common;

#[tokio::test]
async fn list_global_targets_test() {
    let ads_client = common::get_ads_client()
        .account_id(&common::account_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let filter = ListGlobalTargetsFilter::builder()
        .ad_group_id_filter(vec!["4999865406785808299"])
        .target_type_filter(vec![SPGlobalTargetType::Theme])
        .build();
    let response = ListGlobalTargets::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build()
        .fetch()
        .call()
        .await;
    dbg!(&response);
}

#[tokio::test]
async fn update_global_target_test() {
    let ads_client = common::get_ads_client()
        .account_id(&common::account_id())
        .call();
    let ads_client = Arc::new(ads_client);
    let body = SPGlobalTargetUpdate::builder("4999920531756562841")
        .bid("GB", 0.1)
        .build();

    let api = UpdateGlobalTarget::builder()
        .targets(vec![body])
        .ads_client(ads_client)
        .build();
    let res = api.fetch().await;
    dbg!(&res);
}
