use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorsIndex {
    pub errors: Vec<Error>,
    pub index: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: ErrorCode,
    pub field_location: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Display, AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // 基础 HTTP/系统错误
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    ContentTooLarge,
    TooManyRequests,
    InternalError,
    ActionNotSupported,

    // 资源状态与限制
    ActiveResourceLimitExceeded,
    TotalResourceLimitExceeded,
    ArchivedParentCannotCreate,
    ArchivedParentCannotEdit,
    ArchivedResourceCannotEdit,
    AutocreatedEntityCannotEdit,
    ResourceIsInTerminalState,
    FeatureDiscontinued,

    // 字段校验：存在性与唯一性
    DuplicateFieldValueFound,
    DuplicateResourceIdFound,
    FieldValueIsNotEmpty, // 注意：根据上下文可能是逻辑反转，这里按字面意思
    FieldValueIsNull,
    FieldValueIsInvalid,
    FieldValueIsOutOrRange,
    FieldValueNotFound,
    FieldValueNotUnique,
    ResourceIdNotFound,
    ResourceIsEmail, // 对应 RESOURCE_IS_EMPTY 的可能误读，或 RESOURCE_IS_NULL
    ResourceIsNull,
    ResourceIsEmpty,

    // 字段校验：范围与内容
    FieldSizeIsAboveMaximumLimit,
    FieldSizeIsBelowMinimumLimit,
    FieldSizeIsOutOfRange,
    FieldValueIsAboveMaximumLimit,
    FieldValueIsBelowMinimumLimit,
    FieldValueContainsBlocklistedWords,
    FieldValueContainsInvalidCharacters,
    FieldValueMismatch,
    FieldValueMustBeEmptyOrNull,
    FieldValueCannotEdit,

    // 日期与时长
    DateCannotBeInPast,
    DateCannotBeNull,
    DateTooSoon,
    DurationTooShort,

    // 业务逻辑限制
    GlobalAttributeUpdateRestrictedPortfolio,
    GlobalAttributeUpdateRestrictedState,
    GlobalCampaignSingleAdgroupLimit,
    PaymentIssue,
    ProductIneligible,
    ResourceDoesNotBelongToParent,
    UnsupportedMarketplace,

    // 兜底方案
    #[serde(other)]
    Unknown,
}
