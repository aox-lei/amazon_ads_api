use crate::client::AdsClient;
use crate::util::{wrap_include, wrap_include_optional};
use serde::Serialize;
use serde_json::json;

use super::types::enums::SPGlobalTargetType;
use super::types::targets::{
    ListSPGlobalTargesResponse, OperationGlobalTargetResponse, SPGlobalTargetUpdate,
};
use anyhow::Result;
use bon::{bon, builder, Builder};
use serde_with::skip_serializing_none;
use std::sync::Arc;

#[derive(Builder)]
#[builder(on(String, into))]
pub struct ListGlobalTargets {
    ads_client: Arc<AdsClient>,
    filter: ListGlobalTargetsFilter,
}

#[bon]
impl ListGlobalTargets {
    #[builder]
    pub async fn fetch(self, next_token: Option<&str>) -> Result<ListSPGlobalTargesResponse> {
        let mut filter = serde_json::to_value(&self.filter)?;
        if let Some(next_token) = next_token {
            filter["nextToken"] = json!(next_token);
        }

        let response = self
            .ads_client
            .post()
            .path("/adsApi/v1/query/targets")
            .json_body(filter)
            .call()
            .await?;
        let data = response.json::<ListSPGlobalTargesResponse>().await?;
        Ok(data)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListGlobalTargetsFilter {
    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    ad_group_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include")]
    #[builder(default = vec!["SPONSORED_PRODUCTS".to_string()])]
    ad_product_filter: Vec<String>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    campaign_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include")]
    #[builder(default=vec!["GLOBAL".to_string()])]
    marketplace_scope_filter: Vec<String>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    target_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include_optional")]
    target_type_filter: Option<Vec<SPGlobalTargetType>>,
}

#[derive(Builder)]
#[builder(on(String, into))]
pub struct UpdateGlobalTarget {
    ads_client: Arc<AdsClient>,
    targets: Vec<SPGlobalTargetUpdate>,
}

impl UpdateGlobalTarget {
    pub async fn fetch(self) -> Result<OperationGlobalTargetResponse> {
        let json_body = json!({
            "targets": self.targets
        });
        let res = self
            .ads_client
            .post()
            .path("/adsApi/v1/update/targets")
            .json_body(json_body)
            .call()
            .await?;
        Ok(res.json::<OperationGlobalTargetResponse>().await?)
    }
}
