use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString, PartialEq, Eq)]
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
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalTargetType {
    Keyword,
    Product,
    ProductCategory,
    Theme,
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString, PartialEq, Clone, Copy)]
#[strum(serialize_all = "UPPERCASE")] // 确保默认匹配大写
pub enum SPGlobalCurrencyCode {
    #[strum(serialize = "AED", serialize = "AE")]
    AED, // 阿联酋迪拉姆

    #[strum(serialize = "AUD", serialize = "AU")]
    AUD, // 澳大利亚元

    #[strum(serialize = "BRL", serialize = "BR")]
    BRL, // 巴西雷亚尔

    #[strum(serialize = "CAD", serialize = "CA")]
    CAD, // 加拿大元

    #[strum(serialize = "CHF", serialize = "CH", serialize = "LI")]
    CHF, // 瑞士法郎 (瑞士、列支敦士登)

    #[strum(serialize = "CNY", serialize = "CN")]
    CNY, // 人民币

    #[strum(serialize = "DKK", serialize = "DK", serialize = "GL")]
    DKK, // 丹麦克朗 (丹麦、格陵兰)

    #[strum(serialize = "EGP", serialize = "EG")]
    EGP, // 埃及镑

    // 欧元对应多个国家，这里列举主要代码
    #[strum(
        serialize = "EUR",
        serialize = "EU",
        serialize = "DE",
        serialize = "FR",
        serialize = "IT",
        serialize = "ES"
    )]
    EUR, // 欧元

    #[strum(serialize = "GBP", serialize = "GB")]
    GBP, // 英镑

    #[strum(serialize = "INR", serialize = "IN")]
    INR, // 印度卢比

    #[strum(serialize = "JPY", serialize = "JP")]
    JPY, // 日元

    #[strum(serialize = "MXN", serialize = "MX")]
    MXN, // 墨西哥比索

    #[strum(serialize = "MXP")]
    MXP, // 墨西哥比索 (旧制，通常不关联现代国家代码)

    #[strum(serialize = "NGN", serialize = "NG")]
    NGN, // 尼日利亚奈拉

    #[strum(serialize = "NOK", serialize = "NO")]
    NOK, // 挪威克朗

    #[strum(serialize = "NZD", serialize = "NZ")]
    NZD, // 新西兰元

    #[strum(serialize = "PLN", serialize = "PL")]
    PLN, // 波兰兹罗提

    #[strum(serialize = "SAR", serialize = "SA")]
    SAR, // 沙特里亚尔

    #[strum(serialize = "SEK", serialize = "SE")]
    SEK, // 瑞典克朗

    #[strum(serialize = "SGD", serialize = "SG")]
    SGD, // 新加坡元

    #[strum(serialize = "TRY", serialize = "TR")]
    TRY, // 土耳其里拉

    #[strum(
        serialize = "USD",
        serialize = "US",
        serialize = "EC",
        serialize = "SV"
    )]
    USD, // 美元 (美国、厄瓜多尔、萨尔瓦多等)

    #[strum(serialize = "ZAR", serialize = "ZA")]
    ZAR, // 南非兰特
}

impl SPGlobalCurrencyCode {
    /// 核心逻辑：尝试将国家代码或货币字符串转换为枚举
    pub fn from_country_code(code: &str) -> Option<Self> {
        // 使用 EnumString 提供的 FromStr，不区分大小写
        Self::from_str(code.trim()).ok()
    }
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalKeywordMatchType {
    Broad,
    Exact,
    Phrase,
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalThemeMatchType {
    KeywordsCloseMatch, // 关键词和广告紧密匹配
    KeywordsLooseMatch, // 关键词和广告松散匹配
    ProductComplements, // 与广告产品互补的产品。
    ProductSubstitutes, // 可替代广告宣传产品的其他产品。
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalTargetLevel {
    AdGroup,
    Campaign,
}

#[derive(Debug, Serialize, Deserialize, EnumString, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SPGlobalUpdateState {
    Enabled,
    Paused,
}
