use serde::{Deserialize, Serialize};

use crate::client::AdsClient;
use anyhow::Result;
use bon::Builder;
use list_product_ads_filter_builder::{
    IsUnset, SetAdGroupIdFilter, SetAdIdFilter, SetCampaignIdFilter, State,
};
use serde_with::skip_serializing_none;
use std::sync::Arc;

// ==============================================================================
// ListProductAds 请求类
// ==============================================================================

#[derive(Builder)]
pub struct ListProductAds {
    ads_client: Arc<AdsClient>,
    profile_id: String,
    filter: ListProductAdsFilter,
}

impl ListProductAds {
    pub async fn fetch(self) -> Result<ListProductAdsResponse> {
        let filter = serde_json::to_value(&self.filter)?;
        let response = self
            .ads_client
            .post("/sp/productAds/list", &self.profile_id, filter)
            .await?;
        let data = response.json::<ListProductAdsResponse>().await?;
        Ok(data)
    }
}

// ==============================================================================
// ListProductAds 过滤器
// ==============================================================================
#[skip_serializing_none]
#[derive(Serialize, Builder, Debug)]
pub struct ListProductAdsFilter {
    pub ad_group_id_filter: Option<AdGroupIdFilter>,
    pub ad_id_filter: Option<AdIdFilter>,
    pub campaign_id_filter: Option<CampaignIdFilter>,
    pub include_extended_data_fields: Option<bool>,
    pub max_results: Option<usize>, // 默认1000, 最大1000
    pub next_token: Option<String>,
    pub state_filter: Option<StateEnum>,
}

impl<S: State> ListProductAdsFilterBuilder<S> {
    pub fn ad_group_ids(
        self,
        ad_group_ids: Vec<&str>,
    ) -> ListProductAdsFilterBuilder<SetAdGroupIdFilter<S>>
    where
        S::AdGroupIdFilter: IsUnset,
    {
        let ad_group_ids: Vec<String> = ad_group_ids.into_iter().map(String::from).collect();
        self.ad_group_id_filter(AdGroupIdFilter::builder().include(ad_group_ids).build())
    }

    pub fn ad_ids(self, ad_ids: Vec<&str>) -> ListProductAdsFilterBuilder<SetAdIdFilter<S>>
    where
        S::AdIdFilter: IsUnset,
    {
        let ad_ids: Vec<String> = ad_ids.into_iter().map(String::from).collect();
        self.ad_id_filter(AdIdFilter::builder().include(ad_ids).build())
    }

    pub fn campaign_ids(
        self,
        campaign_ids: Vec<&str>,
    ) -> ListProductAdsFilterBuilder<SetCampaignIdFilter<S>>
    where
        S::CampaignIdFilter: IsUnset,
    {
        let campaign_ids: Vec<String> = campaign_ids.into_iter().map(String::from).collect();
        self.campaign_id_filter(CampaignIdFilter::builder().include(campaign_ids).build())
    }
}

#[derive(Serialize, Builder, Debug)]
pub struct AdGroupIdFilter {
    include: Vec<String>,
}
#[derive(Serialize, Builder, Debug)]
pub struct AdIdFilter {
    include: Vec<String>,
}
#[derive(Serialize, Builder, Debug)]

pub struct CampaignIdFilter {
    include: Vec<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StateEnum {
    Archived,
    Enabled,
    Enabling,
    Other,
    Paused,
    Proposed,
    UserDeleted,
}

// ==============================================================================
// Response Model
// ==============================================================================
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListProductAdsResponse {
    pub next_token: Option<String>,
    pub product_ads: Vec<ProductAdsItem>,
    pub total_results: usize,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductAdsItemState {
    Archived,
    Enabled,
    Enabling,
    Other,
    Paused,
    Proposed,
    UserDeleted,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAdsItem {
    pub ad_group_id: String,
    pub ad_id: String,
    pub campaign_id: String,
    pub asin: String,
    pub sku: String,
    pub state: ProductAdsItemState,
}
