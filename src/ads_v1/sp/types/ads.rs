use super::enums::{
    SPCreateState, SPDeliveryReason, SPDeliveryStatus, SPMarketplace, SPProductIdType, SPState,
};
use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
// region SPAd 相关
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

    pub marketplaces: Vec<SPMarketplace>,
    pub state: SPState,
    pub status: Option<SPStatus>,
    pub tags: Vec<SPTag>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SPAdCreate {
    #[builder(start_fn)]
    pub ad_group_id: String,

    #[builder(field)]
    pub creative: SPCreateCreative,

    #[builder(default = "SPONSORED_PRODUCTS")]
    pub ad_product: String,

    #[builder(default = "PRODUCT_AD")]
    pub ad_type: String,

    #[builder(default=SPCreateState::Enabled)]
    pub state: SPCreateState,
    pub tags: Option<Vec<SPCreateTag>>,
}

impl<S: s_p_ad_create_builder::State> SPAdCreateBuilder<S> {
    pub fn asin(mut self, asin: &str) -> Self {
        let products = SPCreateAdvertisedProducts::builder()
            .product_id(asin)
            .product_id_type(SPProductIdType::Asin)
            .build();
        self.creative = SPCreateCreative {
            product_creative: SPCreateProductCreative {
                product_creative_settings: SPCreateProductCreativeSettings::builder()
                    .advertised_product(products)
                    .build(),
            },
        };
        self
    }
    pub fn sku(mut self, asin: &str) -> Self {
        let products = SPCreateAdvertisedProducts::builder()
            .product_id(asin)
            .product_id_type(SPProductIdType::Sku)
            .build();
        self.creative = SPCreateCreative {
            product_creative: SPCreateProductCreative {
                product_creative_settings: SPCreateProductCreativeSettings::builder()
                    .advertised_product(products)
                    .build(),
            },
        };
        self
    }
}

// endregion

// region SPCreative

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreative {
    pub product_creative: SPProductCreative,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateCreative {
    pub product_creative: SPCreateProductCreative,
}

// --- SPProductCreative -------------------
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCreative {
    pub product_creative_settings: SPProductCreativeSettings,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateProductCreative {
    pub product_creative_settings: SPCreateProductCreativeSettings,
}

// --- SPProductCreativeSettings -------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPProductCreativeSettings {
    pub advertised_product: SPAdvertisedProducts,
    pub headline: Option<String>,
    pub spotlight_vides: Option<SPSpotlightVideoSettings>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateProductCreativeSettings {
    pub advertised_product: SPCreateAdvertisedProducts,
    pub headline: Option<String>,
    pub spotlight_vides: Option<SPCreateSpotlightVideoSettings>,
}

// --- SPAdvertisedProducts -------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPAdvertisedProducts {
    pub global_store_setting: Option<SPGlobalStoreSettings>,
    pub product_id: String,
    pub product_id_type: SPProductIdType,
    pub resolved_product_id: Option<String>,
    pub resolved_product_id_type: Option<SPProductIdType>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct SPCreateAdvertisedProducts {
    pub global_store_setting: Option<SPCreateGlobalStoreSettings>,
    pub product_id: String,
    pub product_id_type: SPProductIdType,
}

// --- SPGlobalStoreSettings -------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalStoreSettings {
    pub catalog_source_marketplace: Option<SPMarketplace>,
    pub product_id: String,
    pub product_id_type: SPProductIdType,
    pub resolved_product_id: Option<String>,
    pub resolved_product_id_type: Option<SPProductIdType>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateGlobalStoreSettings {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateSpotlightVideoSettings {
    pub optimize_text: bool,
    pub videos: Vec<SPCreateVideo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPVideo {
    pub asset_id: String,
    pub asset_version: String,
    pub description: Option<String>,
    pub headline: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateVideo {
    pub asset_id: String,
    pub asset_version: String,
    pub description: Option<String>,
    pub headline: Option<String>,
}

// endregion

// region SPStatus

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPStatus {
    pub delivery_reason: Option<Vec<SPDeliveryReason>>,
    pub delivery_status: SPDeliveryStatus,
}

// endregion

// region SPTag

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPTag {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPCreateTag {
    pub key: String,
    pub value: String,
}

// endregion

// region Error

// endregion

// region SPAdMultiStatusSuccess

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPAdMultiStatusSuccess {
    pub ad: SPAd,
    pub index: i32,
}

// endregion
