use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct BindingApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl BindingApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_bindings(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/bindings/{}",
                    self.api_url,
                    vhost.unwrap_or_default()
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqBinding {
    source: String,
    vhost: String,
    destination: String,
    destination_type: String,
    routing_key: String,
    properties_key: String,
}
