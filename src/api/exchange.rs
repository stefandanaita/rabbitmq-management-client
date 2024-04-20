use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::binding::RabbitMqBinding;

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
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}",
                    vhost, exchange
                ),
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
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}",
                    vhost, exchange
                ),
            )
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}",
                    vhost, exchange
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn publish_message(
        &self,
        vhost: String,
        exchange: String,
        body: RabbitMqExchangeMessagePublishRequest,
    ) -> Result<RabbitMqExchangeMessagePublishResponse, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}/publish",
                    vhost, exchange
                ),
            )
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_source_bindings(&self, vhost: String, exchange: String) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}/bindings/source",
                    vhost, exchange
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_destination_bindings(&self, vhost: String, exchange: String) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "http://localhost:15672/api/exchanges/{}/{}/bindings/destination",
                    vhost, exchange
                ),
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
    message_stats: Option<RabbitMqExchangeMessageStats>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchangeMessageStats {
    publish_in: i64,
    publish_out: i64,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqExchangeRequest {
    #[serde(rename = "type")]
    kind: String,
    auto_delete: Option<bool>,
    durable: Option<bool>,
    internal: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqExchangeMessagePublishRequest {
    properties: HashMap<String, String>,
    routing_key: String,
    payload: String,
    payload_encoding: RabbitMqMessageEncoding,
}

#[derive(Debug, Serialize)]
pub enum RabbitMqMessageEncoding {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchangeMessagePublishResponse {
    routed: bool,
}
