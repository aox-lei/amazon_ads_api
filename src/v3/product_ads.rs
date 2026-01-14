use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::AdsClient;
use crate::util::wrap_include_optional;
use anyhow::Result;
use bon::Builder;
use chrono::{DateTime, Utc};
use serde_with::skip_serializing_none;
use std::sync::Arc;

// ==============================================================================
// ListProductAds 请求类
// ==============================================================================

#[derive(Builder)]
#[builder(on(String, into))]
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
            .post()
            .path("/sp/productAds/list")
            .profile_id(&self.profile_id)
            .json_body(filter)
            .call()
            .await?;
        let data = response.json::<ListProductAdsResponse>().await?;
        Ok(data)
    }
}

// --- ListProductAds 过滤器 -------------------

#[skip_serializing_none]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListProductAdsFilter {
    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub ad_group_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub ad_id_filter: Option<Vec<String>>,

    #[serde(serialize_with = "wrap_include_optional")]
    #[builder(with=|items: Vec<&str>| items.into_iter().map(|s| s.to_string()).collect())]
    pub campaign_id_filter: Option<Vec<String>>,

    pub include_extended_data_fields: Option<bool>,
    pub max_results: Option<usize>, // 默认1000, 最大1000
    pub next_token: Option<String>,
    pub state_filter: Option<StateEnum>,
}

// --- Response Model 状态枚举 -------------------

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListProductAdsResponse {
    pub next_token: Option<String>,
    pub product_ads: Vec<ProductAdItem>,
    pub total_results: usize,
}
// ==============================================================================
// 添加广告产品
// ==============================================================================
#[derive(Builder, Debug)]
#[builder(on(String, into))]
pub struct CreateProductAds {
    ads_client: Arc<AdsClient>,
    profile_id: String,
    product_ads: Vec<ProductAdsItemForCreate>,
}

impl CreateProductAds {
    pub async fn fetch(self) -> Result<CreateProductAdsResponse> {
        let json_body = json!({
            "productAds": serde_json::to_value(&self.product_ads)?,
        });
        let response = self
            .ads_client
            .post()
            .path("/sp/productAds")
            .profile_id(&self.profile_id)
            .json_body(json_body)
            .call()
            .await?;
        let data = response.json::<CreateProductAdsResponse>().await?;
        Ok(data)
    }
}
// --- 创建广告产品的单个项 -------------------
#[skip_serializing_none]
#[derive(Builder, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ProductAdsItemForCreate {
    pub ad_group_id: String,
    pub asin: String,
    pub campaign_id: String,
    pub sku: String,
    pub custom_text: Option<String>, // 用于为关联的 ASIN 创建自定义文字广告的自定义文本。仅适用于美国市场的 KDP 作者和图书供应商。
    pub state: StateEnumForCreate,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StateEnumForCreate {
    Enabled,
    Paused,
    Proposed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsResponse {
    pub product_ads: CreateProductAdsOperationResponse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsOperationResponse {
    pub error: Option<Vec<CreateProductAdsMutationError>>,
    pub success: Option<Vec<CreateProductAdsMutationSuccess>>,
}

// --- 成功的响应结构 -------------------
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsMutationSuccess {
    pub ad_id: Option<String>,
    pub index: i64,
    pub product_ad: Option<ProductAdItem>,
}

// --- Error的响应结构 -------------------
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsMutationError {
    pub errors: Vec<CreateProductAdsMutationErrorDetail>,
    pub index: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsMutationErrorDetail {
    pub error_type: String,
    pub error_value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateProductAdsErrorValue {
    AdEligibilityError(CreateProductAdsMutationErrorSelector),
    AsinOwnershipError(CreateProductAdsMutationErrorSelector),
    BillingError(CreateProductAdsMutationErrorSelector),
    DuplicateValueError(CreateProductAdsMutationErrorSelector),
    EntityNotFoundError(CreateProductAdsMutationErrorSelector),
    EntityQuotaError(CreateProductAdsMutationErrorSelector),
    EntityStateError(CreateProductAdsMutationErrorSelector),
    InternalServerError(CreateProductAdsMutationErrorSelector),
    MalformedValueError(CreateProductAdsMutationErrorSelector),
    MissingValueError(CreateProductAdsMutationErrorSelector),
    OtherError(CreateProductAdsMutationErrorSelector),
    ParentEntityError(CreateProductAdsMutationErrorSelector),
    ProductIdentifierError(CreateProductAdsMutationErrorSelector),
    RangeError(CreateProductAdsMutationErrorSelector),
    ThrottledError(CreateProductAdsMutationErrorSelector),
    UnsupportedOperationError(CreateProductAdsMutationErrorSelector),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsMutationErrorSelector {
    pub cause: CreateProductAdsErrorCause,
    pub marketplace: Option<MarketplaceEnum>,
    pub message: String,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductAdsErrorCause {
    pub location: String,
    pub trigger: Option<String>,
}

// ==============================================================================
// 基本数据
// ==============================================================================
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductAdItem {
    pub ad_group_id: String,
    pub ad_id: String,
    pub asin: Option<String>,
    pub campaign_id: String,
    pub custom_text: Option<String>,
    pub extended_data: Option<ProductAdItemExtendedData>,
    pub global_ad_id: Option<String>,
    pub sku: Option<String>,
    pub state: StateEnum,
}
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductAdItemExtendedData {
    #[serde(rename = "creationDateTime")]
    pub creation_datetime: Option<DateTime<Utc>>,
    #[serde(rename = "lastUpdateDateTime")]
    pub last_update_datetime: Option<DateTime<Utc>>,
    pub serving_status_details: Option<Vec<ServingStatusDetail>>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServingStatusDetail {
    pub help_url: Option<String>,
    pub message: Option<String>,
    pub name: Option<ServingStatusReason>,
}

// --- 状态枚举 -------------------
#[derive(Serialize, Debug, Deserialize)]
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

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketplaceEnum {
    AE,
    AU,
    BR,
    CA,
    DE,
    EG,
    ES,
    FR,
    IN,
    IT,
    JP,
    MX,
    NL,
    PL,
    SA,
    SE,
    SG,
    TR,
    UK,
    US,
}

// --- 服务状态枚举 -------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServingStatus {
    // 账户/广告主相关
    AccountOutOfBudget,
    AdvertiserAccountOutOfBudget,
    AdvertiserArchived,
    AdvertiserExceedSpendsLimit,
    AdvertiserOutOfBudget,
    AdvertiserPaused,
    AdvertiserPaymentFailure,
    AdvertiserPolicingPendingReview,
    AdvertiserPolicingSuspended,
    AdvertiserStatusEnabled,

    // 广告相关
    AdArchived,
    AdCreationFailed,
    AdCreationOfflineFailed,
    AdCreationOfflineInProgress,
    AdCreationOfflinePending,
    AdEligible,
    AdIneligible,
    AdLandingPageNotAvailable,
    AdMissingDecoration,
    AdMissingImage,
    AdNotBuyable,
    AdNotInBuybox,
    AdNoPurchasableOffer,
    AdOutOfStock,
    AdPaused,
    AdPolicingPendingReview,
    AdPolicingSuspended,
    AdStatusLive,

    // 广告组相关
    AdGroupArchived,
    AdGroupIncomplete,
    AdGroupLowBid,
    AdGroupPaused,
    AdGroupPolicingCreativeRejected,
    AdGroupPolicingPendingReview,
    AdGroupStatusEnabled,

    // 广告活动相关
    CampaignAdsNotDelivering,
    CampaignArchived,
    CampaignEnded,
    CampaignIncomplete,
    CampaignOutOfBudget,
    CampaignPaused,
    CampaignPendingStartDate,
    CampaignStatusEnabled,

    // 通用状态
    Eligible,
    Ended,
    Ineligible,
    LandingPageNotAvailable,
    MissingDecoration,
    MissingImage,
    NotBuyable,
    NotInBuybox,
    NoInventory,
    NoPurchasableOffer,
    OutOfStock,
    PendingReview,
    PendingStartDate,
    Rejected,
    StatusUnavailable,

    // 组合/策略相关
    PirRuleExcluded,
    PortfolioArchived,
    PortfolioEnded,
    PortfolioOutOfBudget,
    PortfolioPaused,
    PortfolioPendingStartDate,
    PortfolioStatusEnabled,
    SecurityScanPendingReview,
    SecurityScanRejected,

    // 定向相关
    TargetingClauseArchived,
    TargetingClauseBlocked,
    TargetingClausePaused,
    TargetingClausePolicingSuspended,
    TargetingClauseStatusLive,

    // 其他
    Other,
}

// --- 服务状态理由枚举 -------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServingStatusReason {
    // 基础详情状态
    AccountOutOfBudgetDetail,
    AdultProduct,
    AdvertiserAccountOutOfBudgetDetail,
    AdvertiserArchivedDetail,
    AdvertiserExceedSpendsLimitDetail,
    AdvertiserOutOfBudgetDetail,
    AdvertiserPausedDetail,
    AdvertiserPaymentFailureDetail,
    AdvertiserPolicingPendingReviewDetail,
    AdvertiserPolicingSuspendedDetail,
    AdvertiserStatusEnabledDetail,

    // 广告详情
    AdArchivedDetail,
    AdCreationOfflineFailed,
    AdCreationOfflineInProgress,
    AdCreationOfflinePending,
    AdPausedDetail,
    AdPolicingPendingReview,
    AdPolicingPendingReviewDetail,
    AdPolicingSuspendedDetail,
    AdStatusLiveDetail,

    // 广告组详情
    AdGroupArchivedDetail,
    AdGroupIncompleteDetail,
    AdGroupLowBidDetail,
    AdGroupPausedDetail,
    AdGroupPolicingCreativeRejectedDetail,
    AdGroupPolicingPendingReviewDetail,
    AdGroupStatusEnabledDetail,

    // 产品/库存相关 (CP = Controlled Placement / Creative Placement)
    AsinQuarantined,
    BrandRemoved,
    CbaNotSupported,
    ClosedGl,
    CpIneligible,
    CpIneligibleAsin,
    CpIneligibleUnknown,
    CpIneligibleVendor,
    InventoryIncomplete,
    ItemMissing,
    NoInventoryDetail,
    OutOfStockDetail,
    SkuDefective,
    VariationParent,
    RestrictedGl,

    // 广告活动与组合
    CampaignAdsNotDeliveringDetail,
    CampaignArchivedDetail,
    CampaignIncompleteDetail,
    CampaignOutOfBudgetDetail,
    CampaignPausedDetail,
    CampaignStatusEnabledDetail,
    PortfolioArchivedDetail,
    PortfolioEndedDetail,
    PortfolioOutOfBudgetDetail,
    PortfolioPausedDetail,
    PortfolioPendingStartDateDetail,
    PortfolioStatusEnabledDetail,

    // 审核 (Moderation) 相关详情
    ModerationAdultNoveltyPvDetail,
    ModerationAdultProductPvDetail,
    ModerationAdultSoftlinesPvDetail,
    ModerationClaimWeightlossPvDetail,
    ModerationContentNudityPvDetail,
    ModerationContentProvocativePvDetail,
    ModerationContentSmokingPvDetail,
    ModerationCriticalEventsPvDetail,
    ModerationError404PvDetail,
    ModerationGraphicalSexualImagesPvDetail,
    ModerationHfssProductPvDetail,
    ModerationLanguageOffensivePvDetail,
    ModerationNotCompliantToAdPolicyPvDetail,
    ModerationSmokingRelatedPvDetail,

    // 资格与着陆页
    EligibleDetail,
    EndedDetail,
    IneligibleCondition,
    LandingPageIneligible,
    LandingPageNotAvailableDetail,
    NotBuyableDetail,
    NotInBuyboxDetail,
    NoPurchasableOfferDetail,
    OfferMissingDetail,

    // 缺失与拒绝
    MissingDecorationDetail,
    MissingImageDetail,
    PendingReviewDetail,
    PendingStartDateDetail,
    RejectedDetail,
    SecurityScanPendingReview,
    SecurityScanRejected,

    // 定向与规则
    PirRuleExcluded,
    StatusUnavailable,
    TargetingClauseArchivedDetail,
    TargetingClauseBlockedDetail,
    TargetingClausePausedDetail,
    TargetingClausePolicingSuspendedDetail,
    TargetingClauseStatusLiveDetail,

    // 兜底处理
    #[serde(other)]
    Other,
}
