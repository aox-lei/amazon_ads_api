use super::enums::{
    SPGlobalCreateState, SPGlobalDeliveryReason, SPGlobalDeliveryStatus, SPGlobalMarketplace,
    SPGlobalProductIdType, SPGlobalState,
};
use crate::ads_v1::types::Error;
use bon::{builder, Builder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_with::skip_serializing_none;
use std::str::FromStr;

// region SPGlobalAd
#[serde_inline_default]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct SPGlobalAd {
    pub ad_group_id: String,
    pub ad_id: String,

    #[serde_inline_default(String::from("SPONSORED_PRODUCTS"))]
    pub ad_product: String,
    #[serde_inline_default(String::from("PRODUCT_AD"))]
    pub ad_type: String,

    pub campaign_id: String,

    #[serde(rename = "creationDateTime")]
    pub creation_datetime: DateTime<Utc>,
    pub creative: SPGlobalCreative,
    pub global_ad_id: Option<String>,

    #[serde(rename = "lastUpdatedDateTime")]
    pub last_updated_datetime: DateTime<Utc>,

    #[serde_inline_default(String::from("SINGLE_MARKETPLACE"))]
    pub marketplace_scope: String,

    pub marketplaces: Vec<SPGlobalMarketplace>,
    pub state: SPGlobalState,
    pub status: Option<SPGlobalStatus>,
    pub tags: Vec<SPGlobalTag>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]

pub struct SPGlobalAdCreate {
    #[builder(start_fn)]
    pub ad_group_id: String,

    #[builder(field)]
    pub creative: SPGlobalCreateCreative,

    #[builder(default="SPONSORED_PRODUCTS".to_string())]
    pub ad_product: String,

    #[builder(default="PRODUCT_AD".to_string())]
    pub ad_type: String,

    pub marketplace_configurations: Option<SPGlobalCreateMarketplaceAdConfigurations>,

    #[builder(default="GLOBAL".to_string())]
    pub marketplace_scope: String,

    pub marketplaces: Vec<SPGlobalMarketplace>,

    #[builder(default=SPGlobalCreateState::Enabled)]
    pub state: SPGlobalCreateState,
    
    pub tags: Option<Vec<SPGlobalCreateTag>>,
}

impl<S: s_p_global_ad_create_builder::State> SPGlobalAdCreateBuilder<S> {
    pub fn asin(mut self, country_codes: Vec<&str>, asin: &str) -> Self {
        let marketplace_settings: Vec<_> = country_codes
            .into_iter()
            .map(|country_code| {
                SPGlobalCreateAdvertisedProductMarketplaceSetting::builder()
                    .marketplace(SPGlobalMarketplace::from_str(country_code).unwrap())
                    .product_id(asin)
                    .build()
            })
            .collect();
        let products = SPGlobalCreateAdvertisedProducts::builder()
            .marketplace_settings(marketplace_settings)
            .product_id_type(SPGlobalProductIdType::Asin)
            .build();
        self.creative = SPGlobalCreateCreative {
            product_creative: SPGlobalCreateProductCreative {
                product_creative_settings: SPGlobalCreateProductCreativeSettings {
                    advertised_product: products,
                },
            },
        };
        self
    }

    pub fn sku(mut self, country_codes: Vec<&str>, asin: &str) -> Self {
        let marketplace_settings: Vec<_> = country_codes
            .into_iter()
            .map(|country_code| {
                SPGlobalCreateAdvertisedProductMarketplaceSetting::builder()
                    .marketplace(SPGlobalMarketplace::from_str(country_code).unwrap())
                    .product_id(asin)
                    .build()
            })
            .collect();
        let products = SPGlobalCreateAdvertisedProducts::builder()
            .marketplace_settings(marketplace_settings)
            .product_id_type(SPGlobalProductIdType::Sku)
            .build();
        self.creative = SPGlobalCreateCreative {
            product_creative: SPGlobalCreateProductCreative {
                product_creative_settings: SPGlobalCreateProductCreativeSettings {
                    advertised_product: products,
                },
            },
        };
        self
    }
}

// endregion

// region SPGlobalCreative
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreative {
    pub product_creative: SPGlobalProductCreative,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateCreative {
    pub product_creative: SPGlobalCreateProductCreative,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalProductCreative {
    pub product_creative_settings: SPGlobalProductCreativeSettings,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateProductCreative {
    pub product_creative_settings: SPGlobalCreateProductCreativeSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalProductCreativeSettings {
    pub advertised_product: SPGlobalAdvertisedProducts,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateProductCreativeSettings {
    pub advertised_product: SPGlobalCreateAdvertisedProducts,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalAdvertisedProducts {
    pub marketplace_settings: Option<Vec<SPGlobalAdvertisedProductMarketplaceSetting>>,
    pub product_id_type: SPGlobalProductIdType,
    pub resolved_product_id_type: Option<SPGlobalProductIdType>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Default, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateAdvertisedProducts {
    pub marketplace_settings: Option<Vec<SPGlobalCreateAdvertisedProductMarketplaceSetting>>,
    pub product_id_type: SPGlobalProductIdType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalAdvertisedProductMarketplaceSetting {
    pub global_store_setting: Option<SPGlobalGlobalStoreSettings>,
    pub marketplace: SPGlobalMarketplace,
    pub product_id: String,
    pub resolved_product_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct SPGlobalCreateAdvertisedProductMarketplaceSetting {
    pub global_store_setting: Option<SPGlobalCreateGlobalStoreSettings>,
    pub marketplace: SPGlobalMarketplace,
    pub product_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalGlobalStoreSettings {
    pub catalog_source_marketplace: Option<SPGlobalMarketplace>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateGlobalStoreSettings {
    pub catalog_source_marketplace: Option<SPGlobalMarketplace>,
}

// endregion

// region SPGlobalStatus

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalStatus {
    pub delivery_reason: Option<Vec<SPGlobalDeliveryReason>>,
    pub delivery_status: SPGlobalDeliveryStatus,
}
// endregion

// region SPGlobalTag

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateTag {
    pub key: String,
    pub value: String,
}

// endregion

//region SPGlobalCreateMarketplaceAdConfigurations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateMarketplaceAdConfigurations {
    marketplace: SPGlobalMarketplace,
    overrides: SPGlobalCreateMarketplaceAdFieldOverrides,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalCreateMarketplaceAdFieldOverrides {
    state: Option<SPGlobalState>,
    tags: Option<Vec<SPGlobalCreateTag>>,
}
// endregion

//region SPGlobalAdPartialIndex
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalAdPartialIndex {
    pub ad: SPGlobalAd,
    pub errors: Vec<Error>,
    pub index: i32,
}
// endregion

//region SPGlobalAdMultiStatusSuccess
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SPGlobalAdMultiStatusSuccess {
    pub ad: SPGlobalAd,
    pub errors: Vec<Error>,
    pub index: i32,
}

// endregion
