use async_trait::async_trait;
use http::HeaderValue;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use base64::{Engine as _, engine::{general_purpose}};

pub struct AuthenticationMiddleware {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl Middleware for AuthenticationMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let creds = general_purpose::STANDARD.encode(format!("{}:{}", self.username, self.password));

        let mut header_value = HeaderValue::from_str(format!("Basic {}", creds).as_str())
            .map_err(|e| reqwest_middleware::Error::Middleware(e.into()))?;
        header_value.set_sensitive(true);
        req.headers_mut().insert("Authorization", header_value);

        next.run(req, extensions).await
    }
}
