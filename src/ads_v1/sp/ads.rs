use serde::{Deserialize, Serialize};

use crate::client::AdsClient;
use crate::util::{wrap_include, wrap_include_optional};

use anyhow::Result;
use bon::Builder;

use super::types::{ads::SPAd, enums::SPAdStateFilter};
use serde_with::skip_serializing_none;
use std::sync::Arc;

#[derive(Builder)]
#[builder(on(String, into))]
pub struct ListAds {
    ads_client: Arc<AdsClient>,
    profile_id: String,
    filter: ListAdsFilter,
}

impl ListAds {
    pub async fn fetch(self) -> Result<ListAdsResponse> {
        let filter = serde_json::to_value(&self.filter)?;
        let response = self
            .ads_client
            .post("/adsApi/v1/query/ads", &self.profile_id, filter)
            .await?;
        let data = response.json::<ListAdsResponse>().await?;
        Ok(data)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListAdsFilter {
    #[serde(serialize_with = "wrap_include")]
    #[builder(default = vec!["SPONSORED_PRODUCTS".to_string()])]
    pub ad_product_filter: Vec<String>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub ad_group_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub ad_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub campaign_id_filter: Option<Vec<String>>,

    #[builder(default = 1000)]
    max_results: i32,

    #[serde(serialize_with = "wrap_include_optional")]
    // #[builder(with=|items: Vec<SPAdStateFilter> | items.into_iter().map(|s| s.to_string()).collect())]
    state_filter: Option<Vec<SPAdStateFilter>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListAdsResponse {
    pub ads: Option<Vec<SPAd>>,
    pub next_token: Option<String>,
}
