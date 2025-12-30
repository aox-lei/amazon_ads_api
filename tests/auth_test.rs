use amazon_ads_api::auth;
mod common;

#[tokio::test]
async fn auth_test() {
    let credential = common::Credential::default();
    let data = auth::get_access_token(
        "aaaaa",
        credential.client_id.as_str(),
        credential.client_secret.as_str(),
        credential.refresh_token.as_str(),
    )
    .await;
    dbg!(&data);
}
