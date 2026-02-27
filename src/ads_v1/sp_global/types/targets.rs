use super::enums::{
    SPGlobalCurrencyCode, SPGlobalDeliveryReason, SPGlobalDeliveryStatus, SPGlobalKeywordMatchType,
    SPGlobalMarketplace, SPGlobalState, SPGlobalTargetLevel, SPGlobalTargetType,
    SPGlobalThemeMatchType, SPGlobalUpdateState,
};
use crate::ads_v1::types::Error;
use crate::ads_v1::types::ErrorsIndex;
use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

// region ListTargets
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSPGlobalTargesResponse {
    pub next_token: Option<String>,
    pub targets: Option<Vec<SPGlobalTarget>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTarget {
    pub ad_group_id: Option<String>,

    pub ad_product: String,
    pub bid: Option<SPGlobalTargetBid>,
    pub campaign: Option<String>,

    #[serde(rename = "creationDateTime")]
    pub creation_datetime: DateTime<Utc>,

    #[serde(rename = "lastUpdatedDateTime")]
    pub last_updated_datetime: Option<DateTime<Utc>>,
    pub marketplace_configurations: Option<Vec<SPGlobalMarketplaceTargetConfigurations>>,
    pub marketplace_score: String,
    pub marketplaces: Vec<Vec<SPGlobalMarketplace>>,
    pub negative: bool,
    pub state: SPGlobalState,
    pub status: Option<SPGlobalStatus>,
    pub tags: Option<Vec<SPGlobalTag>>,
    pub target_details: Option<SPGlobalTargetDetail>,
    pub target_id: String,
    pub target_level: SPGlobalTargetLevel,
    pub target_type: SPGlobalTargetType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTargetBid {
    pub marketplace_settings: Option<Vec<SPGlobalTargetBidMarketplaceSetting>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTargetBidMarketplaceSetting {
    pub bid: Option<f64>,
    pub currency_code: SPGlobalCurrencyCode,
    pub marketplace: SPGlobalMarketplace,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalMarketplaceTargetConfigurations {
    pub marketplace: SPGlobalMarketplace,
    pub overrides: SPGlobalMarketplaceTargetFieldOverrides,
    pub target_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalMarketplaceTargetFieldOverrides {
    pub state: Option<SPGlobalState>,
    pub tags: Option<Vec<SPGlobalTag>>,
    pub target_details: Option<SPGlobalTargetDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalStatus {
    pub delivery_reason: Option<Vec<SPGlobalDeliveryReason>>,
    pub delivery_status: SPGlobalDeliveryStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SPGlobalTargetDetail {
    KeywordTarget(SPGlobalKeywordTarget),
    ThemeTarget(SPGlobalThemeTarget),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalKeywordTarget {
    pub keyword: String,
    pub match_type: SPGlobalKeywordMatchType,
    pub native_language_keyword: Option<String>,
    pub native_language_locale: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalThemeTarget {
    pub match_type: SPGlobalThemeMatchType,
}

// endregion

// region UpdateTargets
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTargetUpdate {
    #[builder(start_fn)]
    pub target_id: String,
    #[builder(field)]
    pub bid: Option<SPGlobalUpdateTargetBid>,

    pub marketplace_configurations: Option<Vec<SPGlobalCreateMarketplaceTargetConfigurations>>,
    pub marketplaces: Option<Vec<SPGlobalMarketplace>>,
    pub state: Option<SPGlobalUpdateState>,
    pub tags: Option<Vec<SPGlobalCreateTag>>,
}

impl<S: s_p_global_target_update_builder::State> SPGlobalTargetUpdateBuilder<S> {
    pub fn bid(mut self, country_code: &str, bid: f64) -> Self {
        let update_target_bid = self.bid.get_or_insert_with(|| SPGlobalUpdateTargetBid {
            marketplace_settings: Some(vec![]),
        });
        if let Some(ref mut marketplace_settings) = update_target_bid.marketplace_settings {
            marketplace_settings.push(SPGlobalCreateTargetBidMarketplaceSetting {
                bid: Some(bid),
                currency_code: SPGlobalCurrencyCode::from_country_code(country_code)
                    .expect("无法识别的国家代码"),
                marketplace: SPGlobalMarketplace::from_str(country_code).expect("无效的国家代码"),
            });
        }
        self
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateMarketplaceTargetConfigurations {
    marketplace: SPGlobalMarketplace,
    overrides: SPGlobalCreateMarketplaceTargetFieldOverrides,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateMarketplaceTargetFieldOverrides {
    state: Option<SPGlobalState>,
    tags: Option<Vec<SPGlobalCreateTag>>,
    target_details: Option<SPGlobalCreateTargetDetail>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalUpdateTargetBid {
    marketplace_settings: Option<Vec<SPGlobalCreateTargetBidMarketplaceSetting>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateTargetBidMarketplaceSetting {
    bid: Option<f64>,
    currency_code: SPGlobalCurrencyCode,
    marketplace: SPGlobalMarketplace,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SPGlobalCreateTargetDetail {
    KeywordTarget(SPGlobalCreateKeywordTarget),
    ThemeTarget(SPGlobalCreateThemeTarget),
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateKeywordTarget {
    keyword: String,
    match_type: SPGlobalKeywordMatchType,
    native_language_keyword: Option<String>,
    native_language_locale: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateThemeTarget {
    match_type: SPGlobalThemeMatchType,
}

// endregion

// region OperationTargetResponse
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationGlobalTargetResponse {
    pub error: Option<Vec<ErrorsIndex>>,
    pub partial_success: Option<Vec<SPGlobalTargetPartialIndex>>,
    pub success: Option<Vec<SPGlobalTargetMultiStatusSuccess>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTargetPartialIndex {
    pub errors: Option<Vec<Error>>,
    pub index: usize,
    pub target: SPGlobalTarget,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTargetMultiStatusSuccess {
    pub index: usize,
    pub target: SPGlobalTarget,
}
// endregion
