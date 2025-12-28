use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

pub struct AuthMiddleware {
    pub seller_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let token_body = crate::auth::get_access_token(
            &self.seller_id,
            &self.client_id,
            &self.client_secret,
            &self.refresh_token,
        )
        .await
        .map_err(reqwest_middleware::Error::Middleware)?;

        let auth_header = format!("Bearer {}", token_body.access_token);
        req.headers_mut()
            .insert(reqwest::header::AUTHORIZATION, auth_header.parse().unwrap());
        req.headers_mut().insert(
            "Amazon-Advertising-API-ClientId",
            self.client_id.parse().unwrap(),
        );
        next.run(req, extensions).await
    }
}
