use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString, PartialEq,Eq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalAdStateFilter {
    Enabled,
    Paused,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, AsRefStr, PartialEq, Eq)]
pub enum SPGlobalState {
    ARCHIVED,
    ENABLED,
    PAUSED,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalCreateState {
    Enabled,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, EnumString)]
pub enum SPGlobalMarketplace {
    AE,
    AU,
    BE,
    BR,
    CA,
    DE,
    EG,
    ES,
    FR,
    #[strum(serialize = "GB", serialize = "UK")]
    GB,
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
    US,
}

#[derive(Debug, Default, Serialize, Deserialize, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalProductIdType {
    #[default]
    Asin,
    Sku,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalDeliveryReason {
    // 广告主相关
    AdvertiserArchived,
    AdvertiserOutOfBudget,
    AdvertiserOutOfPostpayCreditLimit,
    AdvertiserOutOfPostpayMonthlyBudget,
    AdvertiserOutOfPrepayBalance,
    AdvertiserPaused,
    AdvertiserPaymentFailure,
    AdvertiserPolicingPendingReview,
    AdvertiserPolicingSuspended,

    // 广告个体相关
    AdArchived,
    AdCreationFailed,
    AdCreationInProgress,
    AdGroupArchived,
    AdGroupIncomplete,
    AdGroupLowBid,
    AdGroupPaused,
    AdGroupPendingReview,
    AdGroupPolicingPendingReview,
    AdGroupRejected,
    AdIneligible,
    AdMissingDecoration,
    AdMissingImage,
    AdNotDelivering,
    AdPaused,
    AdPolicingPendingReview,
    AdPolicingSuspended,

    // 品牌与推广活动
    BrandIneligible,
    CampaignArchived,
    CampaignEndDateReached,
    CampaignIncomplete,
    CampaignOutOfBudget,
    CampaignPaused,
    CampaignPendingReview,
    CampaignPendingStartDate,
    CampaignRejected,

    // 素材与落地页
    CreativeMissingAsset,
    CreativePendingReview,
    CreativeRejected,
    LandingPageIneligible,
    LandingPageNotAvailable,

    // 审核/违规策略相关 (Moderation)
    ModerationAdultNoveltyPolicyViolation,
    ModerationAdultProductPolicyViolation,
    ModerationAdultSoftlinesPolicyViolation,
    ModerationClaimWeightlossPolicyViolation,
    ModerationContentNudityPolicyViolation,
    ModerationContentProvocativePolicyViolation,
    ModerationContentSmokingPolicyViolation,
    ModerationCriticalEventsPolicyViolation,
    ModerationError404,
    ModerationGraphicalSexualImagesPolicyViolation,
    ModerationHfssProductPolicyViolation,
    ModerationLanguageOffensivePolicyViolation,
    ModerationNotCompliantToAdPolicy,
    ModerationSmokingRelatedPolicyViolation,

    // 库存与购买限制
    NotBuyable,
    NotInBuybox,
    NotInPolicy,
    NoInventory,
    NoPurchasableOffer,
    OutOfRewardBudget,
    OutOfStock,

    // 组合与目标 (Portfolio/Target)
    PirRuleExcluded,
    PortfolioArchived,
    PortfolioEndDateReached,
    PortfolioOutOfBudget,
    PortfolioPaused,
    PortfolioPendingStartDate,

    // 安全扫描
    SecurityScanPendingReview,
    SecurityScanRejected,

    // 其他
    SpendLimitExceeded,
    StatusUnavailable,
    TargetArchived,
    TargetBlocked,
    TargetPaused,
    TargetPolicingSuspended,
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalDeliveryStatus {
    Delivering,
    NotDelivering,
    Unavailable,
}
