use super::enums::{
    SPCurrencyCode, SPDeliveryReason, SPDeliveryStatus, SPKeywordMatchType, SPMarketplace,
    SPProductIdType, SPProductMatchType, SPState, SPTargetLevel, SPTargetType, SPThemeMatchType,
    SPUpdateState,
};
use crate::ads_v1::types::ErrorsIndex;
use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

// region ListTargets
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSPTargesResponse {
    pub next_token: Option<String>,
    pub targets: Option<Vec<SPTarget>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPTarget {
    pub ad_group_id: Option<String>,

    pub ad_product: String,
    pub bid: Option<SPTargetBid>,
    pub campaign: Option<String>,

    #[serde(rename = "creationDateTime")]
    pub creation_datetime: DateTime<Utc>,
    pub global_target_id: Option<String>,

    #[serde(rename = "lastUpdatedDateTime")]
    pub last_updated_datetime: Option<DateTime<Utc>>,
    pub marketplace_scope: String,
    pub marketplaces: Vec<SPMarketplace>,
    pub negative: bool,
    pub state: SPState,
    pub status: Option<SPStatus>,
    pub tags: Option<Vec<SPTag>>,
    pub target_details: Option<SPTargetDetail>,
    pub target_id: String,
    pub target_level: SPTargetLevel,
    pub target_type: SPTargetType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPTargetBid {
    pub bid: Option<f64>,
    pub currency_code: SPCurrencyCode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPStatus {
    pub delivery_reason: Option<Vec<SPDeliveryReason>>,
    pub delivery_status: SPDeliveryStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SPTargetDetail {
    KeywordTarget(SPKeywordTarget),
    LocationTarget(SPLocationTarget),
    ProductCategoryTarget(SPProductCategoryTarget),
    ProductTarget(SPProductTarget),
    ThemeTarget(SPThemeTarget),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPKeywordTarget {
    pub keyword: String,
    pub match_type: SPKeywordMatchType,
    pub native_language_keyword: Option<String>,
    pub native_language_locale: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPLocationTarget {
    pub location_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCategoryTarget {
    pub product_category_refinement: SPProductCategoryRefinementValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCategoryRefinementValue {
    pub product_category_refinement: SPProductCategoryRefinement,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCategoryRefinement {
    pub product_age_range_id: Option<String>,
    pub product_age_range_id_resolved: Option<String>,
    pub product_brand_id: Option<String>,
    pub product_brand_id_resolved: Option<String>,
    pub product_category_id: Option<String>,
    pub product_category_id_resolved: Option<String>,
    pub product_genre_id: Option<String>,
    pub product_genre_id_resolved: Option<String>,
    pub product_price_greater_than: Option<f64>,
    pub product_price_less_than: Option<f64>,
    pub product_prime_shipping_eligible: Option<bool>,
    pub product_rating_greater_than: Option<f64>,
    pub product_rating_less_than: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductTarget {
    pub match_type: SPProductMatchType,
    pub product: SPProductValue,
    pub product_id_type: SPProductIdType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductValue {
    pub product_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPThemeTarget {
    pub match_type: SPThemeMatchType,
}

// endregion

// region UpdateTargets
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SPTargetUpdate {
    #[builder(start_fn)]
    pub target_id: String,
    #[builder(field)]
    pub bid: Option<SPUpdateTargetBid>,
    #[builder(field)]
    pub state: Option<SPUpdateState>,
    pub tags: Option<Vec<SPCreateTag>>,
}

impl<S: s_p_target_update_builder::State> SPTargetUpdateBuilder<S> {
    pub fn bid(mut self, bid: f64) -> Self {
        let sp_update_target_bid = SPUpdateTargetBid { bid: Some(bid) };
        self.bid = Some(sp_update_target_bid);
        self
    }

    pub fn state(mut self, state: &str) -> Self {
        let state = SPUpdateState::from_str(state).unwrap();
        self.state = Some(state);
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPUpdateTargetBid {
    bid: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateTag {
    pub key: String,
    pub value: String,
}

// endregion

// region OperationTargetResponse
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationTargetResponse {
    pub error: Option<Vec<ErrorsIndex>>,
    pub success: Option<Vec<SPTargetMultiStatusSuccess>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPTargetMultiStatusSuccess {
    pub index: usize,
    pub target: SPTarget,
}
// endregion
