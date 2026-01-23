use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::ads_v1::types::ErrorsIndex;
use crate::client::AdsClient;
use crate::util::{wrap_include, wrap_include_optional};
use anyhow::Result;
use bon::{bon, Builder};

use super::types::enums::SPGlobalMarketplace;
use super::types::{
    ads::{SPGlobalAd, SPGlobalAdCreate, SPGlobalAdMultiStatusSuccess, SPGlobalAdPartialIndex},
    enums::SPGlobalAdStateFilter,
};
use serde_with::skip_serializing_none;
use std::str::FromStr;
use std::sync::Arc;

// region ListAds
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

// endregion

// region CreateAds
#[derive(Builder)]
#[builder(on(String, into))]
pub struct CreateAds {
    ads_client: Arc<AdsClient>,
    ads: Vec<SPGlobalAdCreate>,
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
    pub fn by_asins(
        ad_group_id: &str,
        country_codes: Vec<&str>,
        asins: Vec<&str>,
    ) -> Vec<SPGlobalAdCreate> {
        let marketplaces: Vec<_> = country_codes
            .iter()
            .map(|country_code| SPGlobalMarketplace::from_str(country_code).unwrap())
            .collect();

        asins
            .into_iter()
            .map(|asin| {
                SPGlobalAdCreate::builder(ad_group_id)
                    .asin(country_codes.clone(), asin)
                    .marketplaces(marketplaces.clone())
                    .build()
            })
            .collect()
    }
    #[builder]
    pub fn by_skus(
        ad_group_id: &str,
        country_codes: Vec<&str>,
        skus: Vec<&str>,
    ) -> Vec<SPGlobalAdCreate> {
        let marketplaces: Vec<_> = country_codes
            .iter()
            .map(|country_code| SPGlobalMarketplace::from_str(country_code).unwrap())
            .collect();

        skus.into_iter()
            .map(|sku| {
                SPGlobalAdCreate::builder(ad_group_id)
                    .sku(country_codes.clone(), sku)
                    .marketplaces(marketplaces.clone())
                    .build()
            })
            .collect()
    }
}

// endregion

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

    #[serde(serialize_with = "wrap_include")]
    #[builder(default = vec!["GLOBAL".to_string()])]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub marketplace_scope_filter: Vec<String>,

    #[builder(default = 1000)]
    max_results: i32,

    #[serde(serialize_with = "wrap_include_optional")]
    state_filter: Option<Vec<SPGlobalAdStateFilter>>,
}

// endregion

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListAdsResponse {
    pub ads: Option<Vec<SPGlobalAd>>,
    pub next_token: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperationAdsResponse {
    pub error: Option<Vec<ErrorsIndex>>,
    pub partial_success: Option<Vec<SPGlobalAdPartialIndex>>,
    pub success: Option<Vec<SPGlobalAdMultiStatusSuccess>>,
}
