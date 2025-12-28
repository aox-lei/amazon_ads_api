use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmazonRegion {
    NA, // 北美
    EU, // 欧洲
    FE, // 远东（亚太）
}

impl AmazonRegion {
    /// 获取 OAuth 授权地址
    pub fn auth_url(&self) -> &'static str {
        match self {
            AmazonRegion::NA => "https://www.amazon.com/ap/oa",
            AmazonRegion::EU => "https://eu.account.amazon.com/ap/oa",
            AmazonRegion::FE => "https://apac.account.amazon.com/ap/oa",
        }
    }

    /// 获取 API 网关地址 (广告业务实际调用的接口)
    pub fn api_endpoint(&self) -> &'static str {
        match self {
            AmazonRegion::NA => "https://advertising-api.amazon.com",
            AmazonRegion::EU => "https://advertising-api-eu.amazon.com",
            AmazonRegion::FE => "https://advertising-api-fe.amazon.com",
        }
    }

    pub fn from_country_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            // 北美 (NA)
            "US" | "CA" | "MX" | "BR" => Some(AmazonRegion::NA),
            // 欧洲 (EU)
            "UK" | "GB" | "DE" | "FR" | "ES" | "IT" | "NL" | "AE" | "SA" | "PL" | "SE" | "TR"
            | "EG" | "BE" => Some(AmazonRegion::EU),
            // 远东 (FE)
            "JP" | "AU" | "SG" => Some(AmazonRegion::FE),
            _ => None,
        }
    }
}
