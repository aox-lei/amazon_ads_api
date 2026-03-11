use super::reporting_type::{
    Column, CreateReportStatus, SpCampaignsColumns, SpTargetingColumn, TimeUnit,
};
use crate::client::AdsClient;
use bon::Builder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use anyhow::Result;
use flate2::read::GzDecoder;
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::sync::Arc;

// region 创建报告
#[derive(Builder)]
pub struct CreateReport {
    ads_client: Arc<AdsClient>,
    filter: CreateReportFilter,
}

impl CreateReport {
    pub async fn fetch(self) -> Result<ReportResponse> {
        let filter = serde_json::to_value(&self.filter)?;

        let res = self
            .ads_client
            .post()
            .path("/reporting/reports")
            .json_body(filter)
            .content_type("application/vnd.createasyncreportrequest.v3+json")
            .call()
            .await?;
        let res_data = res.json::<ReportResponse>().await?;
        Ok(res_data)
    }
}

#[skip_serializing_none]
#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportFilter {
    #[builder(setters(vis="", name=configuration_inner))]
    configuration: AsyncReportConfiguration,
    start_date: NaiveDate,
    end_date: NaiveDate,
    name: Option<String>, // 报告的名称
}

impl<S: create_report_filter_builder::State> CreateReportFilterBuilder<S> {
    pub fn by_campaigns(
        self,
        time_unit: TimeUnit,
        columns: Vec<SpCampaignsColumns>,
    ) -> CreateReportFilterBuilder<create_report_filter_builder::SetConfiguration<S>>
    where
        S::Configuration: create_report_filter_builder::IsUnset,
    {
        let converted_columns: Vec<Column> = columns
            .into_iter()
            .map(Column::SpCampaigns)
            .collect();

        let report_configuration = AsyncReportConfiguration::builder()
            .columns(converted_columns)
            .report_type_id("spCampaigns")
            .time_unit(time_unit)
            .group_by(vec!["campaign"])
            .build();
        self.configuration_inner(report_configuration)
    }

    pub fn by_targeting(
        self,
        time_unit: TimeUnit,
        columns: Vec<SpTargetingColumn>,
    ) -> CreateReportFilterBuilder<create_report_filter_builder::SetConfiguration<S>>
    where
        S::Configuration: create_report_filter_builder::IsUnset,
    {
        let converted_columns: Vec<Column> = columns
            .into_iter()
            .map(Column::SpTargeting)
            .collect();

        let report_configuration = AsyncReportConfiguration::builder()
            .columns(converted_columns)
            .report_type_id("spTargeting")
            .time_unit(time_unit)
            .group_by(vec!["targeting"])
            .build();
        self.configuration_inner(report_configuration)
    }
}

#[skip_serializing_none]
#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct AsyncReportConfiguration {
    #[builder(default="SPONSORED_PRODUCTS".to_string())]
    ad_product: String,
    columns: Vec<Column>,
    filters: Option<Vec<AsyncReportFilter>>,

    #[builder(default="GZIP_JSON".to_string())]
    format: String,
    #[builder(with=|items:Vec<&str>| items.into_iter().map(|item| item.to_string()).collect())]
    group_by: Vec<String>,
    report_type_id: String,
    time_unit: TimeUnit,
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncReportFilter {
    field: Option<String>,
    values: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportResponse {
    pub report_id: String,                   // 报告的id
    pub start_date: NaiveDate,               // 报告开始时间
    pub end_date: NaiveDate,                 // 报告结束时间
    pub failure_reason: Option<String>,      // 失败原因
    pub file_size: Option<f64>,              // 文件大小
    pub generated_at: Option<DateTime<Utc>>, // 报告生成日期
    pub name: Option<String>,                // 报告名称
    pub status: CreateReportStatus,          // 报告创建状态
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub url: Option<String>,                   // 生成报告的链接
    pub url_expires_at: Option<DateTime<Utc>>, // 链接过期时间
}

// region 获取报告状态
#[derive(Builder)]
#[builder(on(String, into))]
pub struct GetReport {
    ads_client: Arc<AdsClient>,
    report_id: String,
}

impl GetReport {
    pub async fn fetch(&self) -> Result<ReportResponse> {
        let res = self
            .ads_client
            .get()
            .path(&format!("/reporting/reports/{}", self.report_id))
            .call()
            .await?;
        Ok(res.json::<ReportResponse>().await?)
    }

    pub async fn fetch_document(&self, url: &str) -> Result<Value> {
        let res = reqwest::get(url).await?;
        let compressed_data = res.bytes().await?;
        let decoder = GzDecoder::new(&compressed_data[..]);
        let json_value: Value = serde_json::from_reader(decoder)?;
        Ok(json_value)
    }
}
