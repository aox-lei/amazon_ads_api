use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

// 时间单位
#[derive(Debug, Serialize, Deserialize, EnumString, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    Daily,   // 每日数据
    Summary, // 摘要数据
}

#[derive(Debug, Serialize, Deserialize, EnumString, AsRefStr, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateReportStatus {
    Completed,
    Failed,
    Pending,
    Processing,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] // 如果你希望解析 JSON 时自动匹配，不带标签
pub enum Column {
    SpCampaigns(SpCampaignsColumns),
    SpTargeting(SpTargetingColumn),
}

#[derive(Serialize, Deserialize, Debug, AsRefStr, Display, EnumString, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SpCampaignsColumns {
    /// 曝光量: 广告被显示的次数
    Impressions,

    /// 点击量: 顾客点击广告的次数
    Clicks,
    Cost, // 成本
    /// 广告花费: 实际消耗的金额 (常用字段名为 cost 或 spend)
    Spend,

    /// 点击率 (CTR): clicks / impressions
    ClickThroughRate,

    /// 平均点击成本 (CPC): spend / clicks
    CostPerClick,

    /// 加入列表次数: 顾客将商品加入心愿单或列表
    AddToList,

    // --- 归因订单量 (Purchases) ---
    /// 1天/7天/14天/30天内的总订单量 (点击广告后全店产生的订单)
    Purchases1d,
    Purchases7d,
    Purchases14d,
    Purchases30d,

    /// 1天/7天/14天/30天内同SKU订单量 (仅限广告推广的产品)
    PurchasesSameSku1d,
    PurchasesSameSku7d,
    PurchasesSameSku14d,
    PurchasesSameSku30d,

    // --- 归因销量 (Units Sold) ---
    /// 1天/7天/14天/30天内的总销量 (件数)
    UnitsSoldClicks1d,
    UnitsSoldClicks7d,
    UnitsSoldClicks14d,
    UnitsSoldClicks30d,

    /// 1天/7天/14天/30天内同SKU销量 (件数)
    UnitsSoldSameSku1d,
    UnitsSoldSameSku7d,
    UnitsSoldSameSku14d,
    UnitsSoldSameSku30d,

    // --- 归因销售额 (Sales/Revenue) ---
    /// 1天/7天/14天/30天内的总销售额 (全店成交金额)
    Sales1d,
    Sales7d,
    Sales14d,
    Sales30d,

    /// 1天/7天/14天/30天内同SKU销售额 (仅限广告产品成交额)
    AttributedSalesSameSku1d,
    AttributedSalesSameSku7d,
    AttributedSalesSameSku14d,
    AttributedSalesSameSku30d,

    // --- Kindle/书籍类特有指标 (KDP) ---
    /// 合格借阅量: 通过 Kindle Unlimited 借阅的次数
    QualifiedBorrows,

    /// 版税合格借阅量: 达到计费门槛的借阅
    RoyaltyQualifiedBorrows,

    /// 14天内 Kindle 电子书被阅读的标准页数 (KENP)
    KindleEditionNormalizedPagesRead14d,

    /// 14天内 Kindle 电子书产生的预估版税收入
    KindleEditionNormalizedPagesRoyalties14d,

    // --- 维度与上下文 (Dimensions & Context) ---
    /// 统计日期 (YYYYMMDD)
    Date,
    /// 统计开始日期
    StartDate,
    /// 统计结束日期
    EndDate,
    /// 广告系列竞价策略 (如 Fixed, Dynamic Down, Dynamic Up/Down)
    CampaignBiddingStrategy,

    // 按照campaign分组的指标 - 基本数据
    CampaignName,
    CampaignId,
    CampaignStatus,
    CampaignBudgetAmount,       // 你设置的总预算数值。
    CampaignBudgetType,         // 预算类型
    CampaignBudgetCurrencyCode, // 预算币种

    // 按照campaign分组的指标 - 自动化预算规则
    CampaignRuleBasedBudgetAmount, // 动态指标。如果触发了预设规则（如节日促销或流量高峰），系统自动调整后的实际预算金额。
    CampaignApplicableBudgetRuleId, // 当前正在生效的自动化预算规则的唯一标识。
    CampaignApplicableBudgetRuleName, // 你为该规则设置的名字（例如“Prime Day 预算翻倍规则”）。

    // 按照campaign分组的指标 - 竞争力指标 (Placement)
    TopOfSearchImpressionShare, // 核心指标。你的广告在所有符合条件的“搜索结果第一页最顶端”展示次数中所占的比例。

    // 按照广告组分组的指标
    AdGroupName, // 广告组名称
    AdGroupId,   // 广告组 ID
}

#[derive(Serialize, Deserialize, Debug, AsRefStr, Display, EnumString, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SpTargetingColumn {
    // --- 基础维度 (Dimensions) ---
    Date,                       // 报告日期
    StartDate,                  // 开始日期
    EndDate,                    // 结束日期
    PortfolioId,                // 组合 ID
    CampaignName,               // 广告活动名称
    CampaignId,                 // 广告活动 ID
    CampaignBudgetType,         // 预算类型 (如: daily)
    CampaignBudgetAmount,       // 预算金额
    CampaignBudgetCurrencyCode, // 币种 (如: USD, GBP)
    CampaignStatus,             // 广告活动状态
    AdGroupName,                // 广告组名称
    AdGroupId,                  // 广告组 ID
    KeywordId,                  // 关键词 ID
    Keyword,                    // 关键词文本
    KeywordBid,                 // 关键词出价
    KeywordType,                // 关键词类型
    MatchType,                  // 匹配类型 (Exact, Phrase, Broad)
    Targeting,                  // 投放对象
    TopOfSearchImpressionShare, // 搜索结果顶部曝光份额

    // --- 核心指标 (Core Metrics) ---
    Impressions,      // 曝光量
    Clicks,           // 点击量
    Cost,             // 花费 (Spend)
    CostPerClick,     // 平均点击花费 (CPC)
    ClickThroughRate, // 点击率 (CTR)
    AddToList,        // 添加到列表次数

    // --- Kindle 相关指标 ---
    QualifiedBorrows,                         // 合规借阅数
    RoyaltyQualifiedBorrows,                  // 稿酬合规借阅数
    KindleEditionNormalizedPagesRead14d,      // 14天 Kindle 标准页读取数
    KindleEditionNormalizedPagesRoyalties14d, // 14天 Kindle 标准页读取稿酬

    // --- 订单量指标 (Purchases) ---
    Purchases1d,         // 1天总订单量
    Purchases7d,         // 7天总订单量
    Purchases14d,        // 14天总订单量
    Purchases30d,        // 30天总订单量
    PurchasesSameSku1d,  // 1天同 SKU 订单量
    PurchasesSameSku7d,  // 7天同 SKU 订单量
    PurchasesSameSku14d, // 14天同 SKU 订单量
    PurchasesSameSku30d, // 30天同 SKU 订单量

    // --- 销量指标 (Units Sold) ---
    UnitsSoldClicks1d,   // 1天总销售件数
    UnitsSoldClicks7d,   // 7天总销售件数
    UnitsSoldClicks14d,  // 14天总销售件数
    UnitsSoldClicks30d,  // 30天总销售件数
    UnitsSoldSameSku1d,  // 1天同 SKU 销售件数
    UnitsSoldSameSku7d,  // 7天同 SKU 销售件数
    UnitsSoldSameSku14d, // 14天同 SKU 销售件数
    UnitsSoldSameSku30d, // 30天同 SKU 销售件数
    UnitsSoldOtherSku7d, // 7天其他 SKU 销售件数

    // --- 销售额指标 (Sales / Revenue) ---
    Sales1d,                   // 1天总销售额
    Sales7d,                   // 7天总销售额
    Sales14d,                  // 14天总销售额
    Sales30d,                  // 30天总销售额
    AttributedSalesSameSku1d,  // 1天同 SKU 销售额
    AttributedSalesSameSku7d,  // 7天同 SKU 销售额
    AttributedSalesSameSku14d, // 14天同 SKU 销售额
    AttributedSalesSameSku30d, // 30天同 SKU 销售额
    SalesOtherSku7d,           // 7天其他 SKU 销售额

    // --- 广告效率指标 (Efficiency) ---
    AcosClicks7d,  // 7天广告销售成本比 (ACoS)
    AcosClicks14d, // 14天广告销售成本比 (ACoS)
    RoasClicks7d,  // 7天广告支出回报率 (ROAS)
    RoasClicks14d, // 14天广告支出回报率 (ROAS)
}
