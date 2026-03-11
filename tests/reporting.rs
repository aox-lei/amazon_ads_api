use std::sync::Arc;

use amazon_ads_api::reporting::{CreateReport, CreateReportFilter, GetReport};
use amazon_ads_api::reporting_type::{
    CreateReportStatus, SpCampaignsColumns, SpTargetingColumn, TimeUnit,
};
use chrono::NaiveDate;
use flate2::read::GzDecoder;
use serde_json::Value;

mod common;

#[tokio::test]
async fn create_report() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);

    let filter = CreateReportFilter::builder()
        .by_campaigns(
            TimeUnit::Summary,
            vec![
                SpCampaignsColumns::Impressions,
                SpCampaignsColumns::Clicks,
                SpCampaignsColumns::Cost,
                SpCampaignsColumns::Spend,
                SpCampaignsColumns::ClickThroughRate,
                SpCampaignsColumns::CostPerClick,
                SpCampaignsColumns::Sales1d,
                SpCampaignsColumns::Sales7d,
                SpCampaignsColumns::Sales14d,
                SpCampaignsColumns::Sales30d,
                SpCampaignsColumns::StartDate,
                SpCampaignsColumns::EndDate,
                SpCampaignsColumns::CampaignBiddingStrategy,
                SpCampaignsColumns::CampaignName,
                SpCampaignsColumns::CampaignId,
                SpCampaignsColumns::CampaignBudgetAmount,
                SpCampaignsColumns::CampaignBudgetType,
                SpCampaignsColumns::CampaignBudgetCurrencyCode,
                SpCampaignsColumns::CampaignRuleBasedBudgetAmount,
                SpCampaignsColumns::CampaignApplicableBudgetRuleId,
                SpCampaignsColumns::CampaignApplicableBudgetRuleName,
                SpCampaignsColumns::TopOfSearchImpressionShare,
            ],
        )
        .start_date(NaiveDate::parse_from_str("2026-03-01", "%Y-%m-%d").unwrap())
        .end_date(NaiveDate::parse_from_str("2026-03-09", "%Y-%m-%d").unwrap())
        .build();

    let res = CreateReport::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(&res);
}

#[tokio::test]
async fn create_report_targeting() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);

    let filter = CreateReportFilter::builder()
        .by_targeting(
            TimeUnit::Summary,
            vec![
                SpTargetingColumn::Impressions,
                SpTargetingColumn::Clicks,
                SpTargetingColumn::ClickThroughRate,
                SpTargetingColumn::CostPerClick,
                SpTargetingColumn::Cost,
                SpTargetingColumn::Sales1d,
                SpTargetingColumn::Sales7d,
                SpTargetingColumn::Sales14d,
                SpTargetingColumn::Sales30d,
                SpTargetingColumn::StartDate,
                SpTargetingColumn::EndDate,
                SpTargetingColumn::CampaignName,
                SpTargetingColumn::CampaignId,
                SpTargetingColumn::AdGroupName,
                SpTargetingColumn::AdGroupId,
                SpTargetingColumn::CampaignBudgetAmount,
                SpTargetingColumn::CampaignBudgetType,
                SpTargetingColumn::CampaignBudgetCurrencyCode,
                SpTargetingColumn::TopOfSearchImpressionShare,
                SpTargetingColumn::Targeting,
            ],
        )
        .start_date(NaiveDate::parse_from_str("2026-03-01", "%Y-%m-%d").unwrap())
        .end_date(NaiveDate::parse_from_str("2026-03-09", "%Y-%m-%d").unwrap())
        .build();

    let res = CreateReport::builder()
        .ads_client(ads_client)
        .filter(filter)
        .build()
        .fetch()
        .await;
    dbg!(&res);
}
#[tokio::test]
async fn get_report() {
    let ads_client = common::get_ads_client()
        .profile_id(&common::profile_id())
        .call();
    let ads_client = Arc::new(ads_client);

    let get_report = GetReport::builder()
        .ads_client(ads_client)
        .report_id("ad0ccc40-fba7-436e-a745-6f500cd49048")
        .build();
    let res = get_report.fetch().await.unwrap();
    if res.status == CreateReportStatus::Completed {
        let data = if let Some(url) = res.url {
            Some(get_report.fetch_document(&url).await.unwrap())
        } else {
            None
        };
        dbg!(&data);
    }
}

#[tokio::test]
async fn get_report_document() {
    let res = reqwest::get("https://offline-report-storage-eu-west-1-prod.s3.eu-west-1.amazonaws.com/987e60e1-f2cc-45e9-b723-44ae3e8bc1bc-1773207831409/report-987e60e1-f2cc-45e9-b723-44ae3e8bc1bc-1773207831409.json.gz?X-Amz-Security-Token=IQoJb3JpZ2luX2VjEI3%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FwEaCWV1LXdlc3QtMSJHMEUCIQC1oBn0gVxEfWVTIoUzVzRaVGVpjqFYLhzOPPAVAMpdGAIgdA3nFxEKH2am9H%2BcPPtZshdCeElHauyc0%2FYx0ZyI3Xsq4QUIVhACGgwxMjgxMTczMzA1MTQiDEJ7ea1o0Wa%2FygIyZCq%2BBQOshcV4KG%2Fzo2OHw3NKbCO0a4Dzyss9nHpgZseJLIP1kLbe8TyvWgzAptyErGNjHpKnKHydBxwSLrvrMy4ChcUWhBpQ6FaPO3WvlzEOHXRrJKvTaascn7RMNkiLj4mgrMDvDIE%2FCfMq81KLhpKfeNG%2BaLqXLYiOZLGHcvOhD1Gg4LHnqI9%2BUDnXrUb3qLMNJoaT5Mgy6GBQc2Fgvl72M7lS%2Bk5Ua0Z7ZSAhfkS3fr%2Fea8FsNFVYRkTnHqSw42kAKgYsiNh7eA437huXeNiYSQ%2BicrO19jDOXd783FkeI4DsPaLONTb6A6ePG7s%2Fbw4%2BAqkphK%2BDwRBj0TG0zRkwNuGvm1kfKOK7Jd%2FwSHicOHOkiUmtIzNMZiGnf1saOHdray6%2Fm6xpYa11LPQIPjuCGl1aXcPdx%2BvKUCUmdNROS9cnId8cDUAor4%2BT5jFQK9eC88FzjQTfO%2FiJQo95%2FujHRac1j%2FEWP3qb5WCjLPRZY%2Bl5pr9ckzZixt5oij5z%2F3j6F3Tc8ulCqsfnzcbjDXI7jZXOUiBrvlb2NapGanirM%2Bsf0ClwtDaT1uV%2F3Lg5dqwr3tdkXrWQJL6C1fi4PmMV5aVlGbWblmekp29noOia3YRk3B3FzhqQo28gyemZhhRzd0P5sr0LRHxEN7ziI3vCRa7S0rx2tbd%2F3viG2tLFlA9fNgPmRKiowOn3uQcutov8f7yTy6kCtIW7GXxODPwjuHaa7jz7H4VzsKYlrjeQXNByGLp4LCWvapwGKQlQ9fi6s6%2BZo5uRkXm%2B34DssO88rkUwsikvGv3y8ltuHUSAFpLy%2FmOrAu9ULSQaihgZ%2BRtFGoE11d4%2FTUBhMWFK9lMqZRp9uEzDx1EmcqUkXG7OD4L8d58bQ5HZlokoWxjOnbGHi%2F0wdOxdXbRNIJP958gtJGQs21ZFTwl2X20FuGn6EjCn8MPNBjqpASqlbKgtRyjAnpsAb9C7e5zyCApWzGgeN2kIysdSswyOYdz6vnRX3EFNrk8NEIGTeTQzTsW8jZUqEY783RBIhpK6jSpaDq52H3FgubjIVjQ7XTBhCXD0luPCBCc5PqJMhwoz%2Fu%2F1C1Me14%2BV6dUqaBHrWNF26U34yS6tzMTMhnpJXUktYrQ6Izg%2Bnx%2FRcP5vxQ3jT%2BWio3XDRT%2B5ZnPMLCglf3DQJRaYmVA%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20260311T060024Z&X-Amz-SignedHeaders=host&X-Amz-Expires=3600&X-Amz-Credential=ASIAR3VDDKJJCYDGDSH5%2F20260311%2Feu-west-1%2Fs3%2Faws4_request&X-Amz-Signature=e297e9d0c7a7fafa26a69989f3f061f684c1442d590491c7b89fd411c8b9d32a").await;
    let compressed_data = res.unwrap().bytes().await.unwrap();
    let decoder = GzDecoder::new(&compressed_data[..]);
    let json_value: Value = serde_json::from_reader(decoder).unwrap();
    dbg!(&json_value);
}
