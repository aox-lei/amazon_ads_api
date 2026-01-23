use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::types::enums::{Locale, MatchType, SortDimension};
use crate::client::AdsClient;
use crate::util::wrap_include_optional;
use anyhow::Result;
use bon::Builder;
use chrono::{DateTime, Utc};
use serde_with::skip_serializing_none;
use std::sync::Arc;

#[derive(Builder)]
#[builder(on(String, into))]
pub struct KeywordRecommendations {
    ads_client: Arc<AdsClient>,
    filter: KeywordRecommendationsFilter,
}

impl KeywordRecommendations {
    pub async fn fetch(self) -> Result<Vec<KeywordRecommendationsResponse>> {
        let filter = serde_json::to_value(&self.filter)?;
        let response = self
            .ads_client
            .post()
            .path("/sp/targets/keywords/recommendations")
            .json_body(filter)
            .content_type("application/vnd.spkeywordsrecommendation.v3+json")
            .call()
            .await?;
        let data = response
            .json::<Vec<KeywordRecommendationsResponse>>()
            .await?;
        Ok(data)
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum KeywordRecommendationsFilter {
    ForASINS(KeywordRecommendationsFilterForASINS),
    ForAdGroup(KeywordRecommendationsFilterForAdGroup),
}

impl From<KeywordRecommendationsFilterForASINS> for KeywordRecommendationsFilter {
    fn from(filter: KeywordRecommendationsFilterForASINS) -> Self {
        KeywordRecommendationsFilter::ForASINS(filter)
    }
}

impl From<KeywordRecommendationsFilterForAdGroup> for KeywordRecommendationsFilter {
    fn from(filter: KeywordRecommendationsFilterForAdGroup) -> Self {
        KeywordRecommendationsFilter::ForAdGroup(filter)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[serde(rename_all = "camelCase")]
pub struct KeywordRecommendationsFilterForASINS {
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|item| item.to_string()).collect())]
    asins: Vec<String>,
    #[builder(default = "KEYWORDS_FOR_ASINS".to_string())]
    recommendation_type: String,
    targets: Option<Vec<TargetsFilter>>,
    locale: Option<Locale>,
    #[builder(default = 200)]
    max_recommendations: i32,
    sort_dimension: Option<SortDimension>,
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[serde(rename_all = "camelCase")]
pub struct KeywordRecommendationsFilterForAdGroup {
    ad_group_id: String,
    campagin_id: String,
    #[builder(default = "KEYWORDS_FOR_ADGROUP".to_string())]
    recommendation_type: String,
    targets: Option<Vec<TargetsFilter>>,
    locale: Option<Locale>,
    #[builder(default = 200)]
    max_recommendations: i32,
    sort_dimension: Option<SortDimension>,
}

#[derive(Serialize, Debug, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TargetsFilter {
    bid: Option<f64>,
    keyword: Option<String>,
    match_type: Option<MatchType>,
    user_selected_keyword: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeywordRecommendationsResponse {
    pub rank: Option<usize>,
    pub suggested_bid: Option<SuggestedBid>,
    pub translation: Option<String>,
    pub bid: Option<f64>,
    pub keyword: Option<String>,
    pub match_type: Option<MatchType>,
    pub user_selected_keyword: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestedBid {
    pub bid_rec_id: Option<String>,
    pub reange_end: Option<f64>,
    pub range_start: Option<f64>,
    pub suggested: Option<f64>,
}
