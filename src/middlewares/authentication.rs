use async_trait::async_trait;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

pub struct AuthenticationMiddleware {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl Middleware for AuthenticationMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        next.run(req, extensions).await
    }
}
