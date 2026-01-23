use crate::client::AdsClient;
use anyhow::Result;
use bon::Builder;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Builder)]
#[builder(on(String, into))]
pub struct ListNegativeTargetsBrandsSearch {
    ads_client: Arc<AdsClient>,
    keyword: String,
}

impl ListNegativeTargetsBrandsSearch {
    pub async fn fetch(self) -> Result<Vec<ListNegativeTargetsBrandsSearchItem>> {
        let filter = json!({
            "keyword": self.keyword,
        });
        let res = self
            .ads_client
            .post()
            .path("/sp/negativeTargets/brands/search")
            .json_body(filter)
            .content_type("application/vnd.spproducttargeting.v3+json")
            .call()
            .await?;
        let data = res
            .json::<Vec<ListNegativeTargetsBrandsSearchItem>>()
            .await?;
        Ok(data)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListNegativeTargetsBrandsSearchItem {
    pub id: String,
    pub name: String,
}
