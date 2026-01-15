use amazon_ads_api::ads_v1::sp_global::ads::OperationAdsResponse;
#[tokio::test]
async fn test() {
    let json_text = r#"{
  "error": [],
  "partialSuccess": [],
  "success": [
    {
      "ad": {
        "adGroupId": "4999899225094945252",
        "adId": "5000061705296824111",
        "adProduct": "SPONSORED_PRODUCTS",
        "adType": "PRODUCT_AD",
        "campaignId": "4999868670239876433",
        "creationDateTime": "2026-01-15T14:19:47.704Z",
        "creative": {
          "productCreative": {
            "productCreativeSettings": {
              "advertisedProduct": {
                "marketplaceSettings": [
                  {
                    "marketplace": "GB",
                    "productId": "0M-OneColor-60x35x100cm236x137x393"
                  }
                ],
                "productIdType": "SKU"
              }
            }
          }
        },
        "lastUpdatedDateTime": "2026-01-15T14:19:47.704Z",
        "marketplaceConfigurations": [
          {
            "marketplace": "GB",
            "overrides": {
              "state": "ENABLED"
            }
          }
        ],
        "marketplaceScope": "GLOBAL",
        "marketplaces": [
          "GB"
        ],
        "state": "ENABLED"
      },
      "index": 0
    }
  ]
}"#;
let res:OperationAdsResponse = serde_json::from_str(json_text).unwrap();
dbg!(res);
}
