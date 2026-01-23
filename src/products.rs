use crate::client::AdsClient;
use anyhow::Result;
use bon::{bon, builder, Builder};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::sync::Arc;
use strum::AsRefStr;

#[derive(Builder)]
pub struct ProductMetadata {
    ads_client: Arc<AdsClient>,
    filter: ProductMetadataFilter,
}

#[bon]
impl ProductMetadata {
    #[builder]
    pub async fn fetch(
        self,
        #[builder(default = 1)] page_index: i32,
        #[builder(default = 300)] page_size: i32,
    ) -> Result<ProductMetadataResponse> {
        let mut filter = serde_json::to_value(&self.filter)?;
        filter["pageIndex"] = page_index.into();
        filter["pageSize"] = page_size.into();

        let res = self
            .ads_client
            .post()
            .path("/product/metadata")
            .json_body(filter)
            .call()
            .await?;

        let data = res.json::<ProductMetadataResponse>().await?;
        Ok(data)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ProductMetadataFilter {
    // SP SB SD
    #[builder(default="SP".to_string())]
    ad_type: String,

    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    asins: Option<Vec<String>>,

    // 是否检查广告资格
    #[builder(default = false)]
    check_eligibility: bool,

    // 是否需要提供商品的详情
    #[builder(default = true)]
    check_item_details: bool,

    // 是否只返回与GlobalStore相关的GlobalStore商品信息
    #[builder(default = false)]
    is_global_store_selection: bool,

    // 语言, 默认是市场语言
    locale: Option<String>,

    // 要搜索的字符串
    search_str: Option<String>,

    // 只支持卖家SP, 不能与ASIN换个search_str同时使用
    #[builder(with=|item:Vec<&str>| item.into_iter().map(|item| item.to_string()).collect::<Vec<String>>())]
    skus: Option<Vec<String>>,

    // CREATED_DATE SUGGESTED
    #[builder(default="SUGGESTED".to_string())]
    sort_by: String,

    // ASC DESC
    #[builder(default="DESC".to_string())]
    sort_order: String,
}


// region 响应

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductMetadataResponse {
    #[serde(rename = "ProductMetadataList")]
    pub product_metadata_list: Vec<ProductMetadataModel>,
    pub cursor_token: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductMetadataModel {
    pub asin: Option<String>,
    pub availability: Option<StockAvailability>,
    // 基准价格
    pub basis_price: Option<BasisPrice>,
    // 畅销榜排名
    pub best_seller_rank: Option<String>,

    //品牌
    pub brand: Option<String>,

    // 类别
    pub category: Option<String>,

    //该产品的首次上架日期
    pub created_date: Option<String>,

    // 资格状态
    pub eligibility_status: Option<EligibilityStatus>,

    //GlobalStore设置
    pub global_store_setting: Option<GlobalStoreSetting>,

    pub image_url: Option<String>,

    // 不符合资格的状态标识符列表
    pub ineligibility_codes: Option<Vec<String>>,

    // 不符合广告刊登条件的原因列表
    pub ineligibility_reasons: Option<Vec<String>>,

    // 支付价格
    pub price_to_pay: Option<PriceToPay>,

    pub sku: Option<String>,

    pub title: Option<String>,
    // 变体asin列表
    pub variation_list: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasisPrice {
    pub amount: Option<f64>,
    pub currenct: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStoreSetting {
    pub catalog_source_country_code: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceToPay {
    pub amount: Option<f64>,
    pub currenct: Option<String>,
}

// region 库存可用性
#[derive(Debug, Serialize, Deserialize, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum StockAvailability {
    InStock,       // 有货
    InStockScarce, // 库存稀少
    OutOfStock,    // 缺货
    Preorder,      // 预购
    Leadtime,      // 需要交货周期
    AvailableDate, // 目前不可用, 未来可用
}

#[derive(Debug, Serialize, Deserialize, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityStatus {
    Eligible,   // 符合条件
    Ineligible, // 不符合
}
