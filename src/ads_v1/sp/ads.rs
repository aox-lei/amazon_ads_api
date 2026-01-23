use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::AdsClient;
use crate::util::{wrap_include, wrap_include_optional};

use anyhow::Result;
use bon::{bon, builder, Builder};

use super::types::{
    ads::{SPAd, SPAdCreate, SPAdMultiStatusSuccess},
    enums::SPAdStateFilter,
};
use crate::ads_v1::types::ErrorsIndex;
use serde_with::skip_serializing_none;
use std::sync::Arc;

#[derive(Builder)]
#[builder(on(String, into))]
pub struct ListAds {
    ads_client: Arc<AdsClient>,
    filter: ListAdsFilter,
}
#[bon]
impl ListAds {
    #[builder]
    pub async fn fetch(self, next_token: Option<&str>) -> Result<ListAdsResponse> {
        let mut filter = serde_json::to_value(&self.filter)?;
        if let Some(next_token) = next_token {
            filter["nextToken"] = json!(next_token);
        }
        let response = self
            .ads_client
            .post()
            .path("/adsApi/v1/query/ads")
            .json_body(filter)
            .call()
            .await?;
        let data = response.json::<ListAdsResponse>().await?;
        Ok(data)
    }
}

#[derive(Builder)]
#[builder(on(String, into))]
pub struct DelAds {
    ads_client: Arc<AdsClient>,
    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    ad_ids: Vec<String>,
}

impl DelAds {
    pub async fn fetch(self) -> Result<OperationAdsResponse> {
        let json_body = json!({
            "adIds": self.ad_ids
        });
        let res = self
            .ads_client
            .post()
            .path("/adsApi/v1/delete/ads")
            .json_body(json_body)
            .call()
            .await?;
        let data = res.json::<OperationAdsResponse>().await?;
        Ok(data)
    }
}

#[derive(Builder)]
#[builder(on(String, into))]
pub struct CreateAds {
    ads_client: Arc<AdsClient>,
    ads: Vec<SPAdCreate>,
}
#[bon]
impl CreateAds {
    pub async fn fetch(self) -> Result<OperationAdsResponse> {
        let json_body = json!({
            "ads": self.ads
        });
        let res = self
            .ads_client
            .post()
            .path("/adsApi/v1/create/ads")
            .json_body(json_body)
            .call()
            .await?;
        Ok(res.json::<OperationAdsResponse>().await?)
    }

    #[builder]
    pub fn by_asins(ad_group_id: &str, asins: Vec<&str>) -> Vec<SPAdCreate> {
        asins
            .into_iter()
            .map(|asin| SPAdCreate::builder(ad_group_id).asin(asin).build())
            .collect()
    }
    #[builder]
    pub fn by_skus(ad_group_id: &str, skus: Vec<&str>) -> Vec<SPAdCreate> {
        skus.into_iter()
            .map(|sku| SPAdCreate::builder(ad_group_id).sku(sku).build())
            .collect()
    }
}

// region ListAdsFilter

#[skip_serializing_none]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListAdsFilter {
    #[serde(serialize_with = "wrap_include")]
    #[builder(default = vec!["SPONSORED_PRODUCTS".to_string()])]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
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
    state_filter: Option<Vec<SPAdStateFilter>>,
}

// endregion

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListAdsResponse {
    pub ads: Option<Vec<SPAd>>,
    pub next_token: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperationAdsResponse {
    pub error: Option<Vec<ErrorsIndex>>,
    pub success: Option<Vec<SPAdMultiStatusSuccess>>,
}
