use crate::api::_generic::handle_response;
use crate::api::binding::RabbitMqBinding;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ExchangeApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl ExchangeApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
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
                    "{}/api/exchanges/{}",
                    self.api_url,
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
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn create_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let exchanges = self.list_exchanges(Some(vhost.clone())).await?;
        if let Some(existing) = exchanges.iter().find(|v| v.name == exchange) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} policy",
                existing.name
            )));
        }

        self.update_exchange(vhost, exchange, request).await
    }

    pub async fn update_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .json(&request)
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
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn publish_message(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeMessagePublishRequest,
    ) -> Result<RabbitMqExchangeMessagePublishResponse, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "{}/api/exchanges/{}/{}/publish",
                    self.api_url, vhost, exchange
                ),
            )
            .json(&request)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_source_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/exchanges/{}/{}/bindings/source",
                    self.api_url, vhost, exchange
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_destination_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/exchanges/{}/{}/bindings/destination",
                    self.api_url, vhost, exchange
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
