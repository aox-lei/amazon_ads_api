use super::enums::{SPDeliveryReason, SPDeliveryStatus, SPMarketplace, SPProductIdType, SPState};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPAd {
    pub ad_group_id: String,
    pub ad_id: String,

    #[serde_inline_default(String::from("SPONSORED_PRODUCTS"))]
    pub ad_product: String,
    #[serde_inline_default(String::from("PRODUCT_AD"))]
    pub ad_type: String,

    pub campaign_id: String,

    #[serde(rename = "creationDateTime")]
    pub creation_datetime: DateTime<Utc>,
    pub creative: SPCreative,
    pub global_ad_id: Option<String>,

    #[serde(rename = "lastUpdatedDateTime")]
    pub last_updated_datetime: DateTime<Utc>,

    #[serde_inline_default(String::from("SINGLE_MARKETPLACE"))]
    pub marketplace_scope: String,

    pub marketplaces: Vec<SPMarketplace> ,
    pub state: SPState,
    pub status: SPStatus,
    pub tags: Vec<SPTag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreative {
    pub product_creative: SPProductCreative,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCreative {
    pub product_creative_settings: SPProductCreativeSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCreativeSettings {
    pub advertised_product: SPAdvertisedProducts,
    pub headline: Option<String>,
    pub spotlight_vides: Option<SPSpotlightVideoSettings>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPAdvertisedProducts {
    pub global_store_setting: Option<SPGlobalStoreSettings>,
    pub product_id: String,
    pub product_id_type: SPProductIdType,
    pub resolved_product_id: Option<String>,
    pub resolved_product_id_type: Option<SPProductIdType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalStoreSettings {
    pub catalog_source_marketplace: Option<SPMarketplace>,
    pub product_id: String,
    pub product_id_type: SPProductIdType,
    pub resolved_product_id: Option<String>,
    pub resolved_product_id_type: Option<SPProductIdType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPSpotlightVideoSettings {
    pub optimize_text: bool,
    pub videos: Vec<SPVideo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPVideo {
    pub asset_id: String,
    pub asset_version: String,
    pub description: Option<String>,
    pub headline: Option<String>,
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
