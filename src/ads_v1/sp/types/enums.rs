use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub enum SPMarketplace {
    AE,
    AU,
    BE,
    BR,
    CA,
    DE,
    EG,
    ES,
    FR,
    GB,
    IE,
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
    ZA,
}

#[derive(Debug, Deserialize, AsRefStr, PartialEq, Eq)]
pub enum SPState {
    ARCHIVED,
    ENABLED,
    PAUSED,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPCreateState {
    Enabled,
    Paused,
}

#[derive(Debug, Default, Serialize, Deserialize, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SPProductIdType {
    #[default]
    Asin,
    Sku,
}

#[derive(Debug, Deserialize, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPDeliveryStatus {
    Delivering,
    NotDelivering,
    Unavailable,
}

/// 广告系统投放状态详情
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPDeliveryReason {
    // === 广告主级别 (ADVERTISER) ===
    /// 广告主账号已归档
    AdvertiserArchived,
    /// 广告主预算耗尽
    AdvertiserOutOfBudget,
    /// 超过后付费信用额度
    AdvertiserOutOfPostpayCreditLimit,
    /// 超过后付费月度预算
    AdvertiserOutOfPostpayMonthlyBudget,
    /// 预付费余额不足
    AdvertiserOutOfPrepayBalance,
    /// 广告主账号已暂停
    AdvertiserPaused,
    /// 广告主支付失败
    AdvertiserPaymentFailure,
    /// 广告主合规性审核中
    AdvertiserPolicingPendingReview,
    /// 广告主账号因违规被封禁
    AdvertiserPolicingSuspended,

    // === 广告活动级别 (CAMPAIGN / PORTFOLIO) ===
    /// 广告活动已归档
    CampaignArchived,
    /// 已达到广告活动结束日期
    CampaignEndDateReached,
    /// 广告活动信息不完整
    CampaignIncomplete,
    /// 广告活动预算耗尽
    CampaignOutOfBudget,
    /// 广告活动已暂停
    CampaignPaused,
    /// 广告活动审核中
    CampaignPendingReview,
    /// 广告活动等待开始日期
    CampaignPendingStartDate,
    /// 广告活动被拒绝
    CampaignRejected,
    /// 组合(Portfolio)已归档
    PortfolioArchived,
    /// 组合已达结束日期
    PortfolioEndDateReached,
    /// 组合预算耗尽
    PortfolioOutOfBudget,
    /// 组合已暂停
    PortfolioPaused,
    /// 组合等待开始日期
    PortfolioPendingStartDate,

    // === 广告组级别 (AD_GROUP) ===
    /// 广告组已归档
    AdGroupArchived,
    /// 广告组信息不完整
    AdGroupIncomplete,
    /// 广告组出价过低
    AdGroupLowBid,
    /// 广告组已暂停
    AdGroupPaused,
    /// 广告组审核中
    AdGroupPendingReview,
    /// 广告组合规性审核中
    AdGroupPolicingPendingReview,
    /// 广告组被拒绝
    AdGroupRejected,

    // === 具体广告/素材级别 (AD / CREATIVE / EXTENSION) ===
    /// 广告已归档
    AdArchived,
    /// 广告创建失败
    AdCreationFailed,
    /// 广告创建中
    AdCreationInProgress,
    /// 广告插件/扩展已归档
    AdExtensionArchived,
    /// 广告插件已暂停
    AdExtensionPaused,
    /// 广告插件合规性审核中
    AdExtensionPolicingPendingReview,
    /// 广告插件被封禁
    AdExtensionPolicingSuspended,
    /// 广告不符合投放条件
    AdIneligible,
    /// 广告缺少修饰信息
    AdMissingDecoration,
    /// 广告缺少图片
    AdMissingImage,
    /// 广告未在投放中
    AdNotDelivering,
    /// 广告已暂停
    AdPaused,
    /// 广告合规审核中
    AdPolicingPendingReview,
    /// 广告被封禁
    AdPolicingSuspended,
    /// 创意缺少资产(图片/视频等)
    CreativeMissingAsset,
    /// 创意审核中
    CreativePendingReview,
    /// 创意被拒绝
    CreativeRejected,

    // === 政策与审核级别 (MODERATION / POLICY) ===
    /// 违反成人新奇物品政策
    ModerationAdultNoveltyPolicyViolation,
    /// 违反成人用品政策
    ModerationAdultProductPolicyViolation,
    /// 违反成人软文/服装政策
    ModerationAdultSoftlinesPolicyViolation,
    /// 违反减肥声明政策
    ModerationClaimWeightlossPolicyViolation,
    /// 违反裸露内容政策
    ModerationContentNudityPolicyViolation,
    /// 违反挑逗性内容政策
    ModerationContentProvocativePolicyViolation,
    /// 违反吸烟内容政策
    ModerationContentSmokingPolicyViolation,
    /// 违反重大事件/危机政策
    ModerationCriticalEventsPolicyViolation,
    /// 落地页 404 错误
    ModerationError404,
    /// 违反露骨色情图像政策
    ModerationGraphicalSexualImagesPolicyViolation,
    /// 违反高脂高糖(HFSS)食品政策
    ModerationHfssProductPolicyViolation,
    /// 违反冒犯性语言政策
    ModerationLanguageOffensivePolicyViolation,
    /// 不符合通用广告政策
    ModerationNotCompliantToAdPolicy,
    /// 违反吸烟相关政策
    ModerationSmokingRelatedPolicyViolation,
    /// 安全扫描审核中
    SecurityScanPendingReview,
    /// 安全扫描未通过
    SecurityScanRejected,

    // === 商品与库存状态 (STOCK / BUYBOX) ===
    /// 品牌不符合条件
    BrandIneligible,
    /// 落地页不符合条件
    LandingPageIneligible,
    /// 落地页无法访问
    LandingPageNotAvailable,
    /// 不可购买
    NotBuyable,
    /// 不在黄金购物车(Buybox)内
    NotInBuybox,
    /// 不符合政策
    NotInPolicy,
    /// 无库存
    NoInventory,
    /// 无可购买的报价
    NoPurchasableOffer,
    /// 库存耗尽
    OutOfStock,

    // === 目标与预算限制 (TARGET / SPEND) ===
    /// 目标(关键词/人群)已归档
    TargetArchived,
    /// 目标被拦截
    TargetBlocked,
    /// 目标已暂停
    TargetPaused,
    /// 目标因合规被封禁
    TargetPolicingSuspended,
    /// PIR 规则排除
    PirRuleExcluded,
    /// 奖励预算耗尽
    OutOfRewardBudget,
    /// 超过支出限制
    SpendLimitExceeded,

    // === 其他状态 ===
    /// 状态不可用
    StatusUnavailable,
    /// 其他原因
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SPAdStateFilter {
    Enabled,
    Paused,
    Archived,
}
