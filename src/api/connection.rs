use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct ConnectionApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl ConnectionApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_connections(
        &self,
        vhost: Option<String>,
    ) -> Result<RabbitMqConnection, RabbitMqClientError> {
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
