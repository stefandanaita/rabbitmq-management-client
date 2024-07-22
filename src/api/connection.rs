use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait ConnectionApi {
    async fn list_connections(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqConnection>, RabbitMqClientError>;
}

#[async_trait]
impl ConnectionApi for RabbitMqClient {
    async fn list_connections(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqConnection>, RabbitMqClientError> {
        let endpoint = match vhost {
            None => format!("{}/api/connections", self.api_url),
            Some(vhost) => format!("{}/api/vhosts/{}/connections", self.api_url, vhost),
        };

        let response = self
            .client
            .request(reqwest::Method::GET, &endpoint)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqConnection {}
