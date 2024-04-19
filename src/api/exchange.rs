use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use crate::api::policy::{RabbitMqPolicy, RabbitMqPolicyRequest};

pub struct ExchangeApi {
    client: ClientWithMiddleware,
}

impl ExchangeApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn list_exchanges(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqExchange>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "http://localhost:15672/api/exchanges/{}",
                    vhost.unwrap_or_default()
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<RabbitMqExchange, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("http://localhost:15672/api/exchanges/{}/{}", vhost, exchange),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn create_exchange(
        &self,
        vhost: String,
        exchange: String,
        body: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let exchanges = self.list_policies(Some(vhost.clone())).await?;
        if let Some(existing) = exchanges.iter().find(|v| v.name == exchange) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} policy",
                existing.name
            )));
        }

        self.update_exchange(vhost, exchange, body).await
    }

    pub async fn update_exchange(
        &self,
        vhost: String,
        exchange: String,
        body: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("http://localhost:15672/api/exchanges/{}/{}", vhost, exchange),
            )
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_policy(
        &self,
        vhost: String,
        policy: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("http://localhost:15672/api/policies/{}/{}", vhost, policy),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchange {
    auto_delete: bool,
    durable: bool,
    internal: bool,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    user_who_performed_action: String,
    vhost: String,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqExchangeRequest {
    #[serde(rename = "type")]
    kind: String,
    auto_delete: Option<bool>,
    durable: Option<bool>,
    internal: Option<bool>,
}
