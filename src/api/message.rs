use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait MessageApi {
    async fn publish_message(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqPublishMessageRequest,
    ) -> Result<RabbitMqPublishMessageResponse, RabbitMqClientError>;

    async fn get_messages(
        &self,
        vhost: String,
        queue: String,
        options: RabbitMqGetMessagesOptions,
    ) -> Result<Vec<RabbitMqMessage>, RabbitMqClientError>;
}

#[async_trait]
impl MessageApi for RabbitMqClient {
    async fn publish_message(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqPublishMessageRequest,
    ) -> Result<RabbitMqPublishMessageResponse, RabbitMqClientError> {
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

    #[tracing::instrument(skip(self))]
    async fn get_messages(
        &self,
        vhost: String,
        queue: String,
        options: RabbitMqGetMessagesOptions,
    ) -> Result<Vec<RabbitMqMessage>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}/api/queues/{}/{}/get", self.api_url, vhost, queue),
            )
            .json(&options)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Serialize)]
pub struct RabbitMqPublishMessageRequest {
    pub properties: HashMap<String, String>,
    pub routing_key: String,
    pub payload: String,
    pub payload_encoding: RabbitMqMessageEncoding,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RabbitMqMessageEncoding {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqPublishMessageResponse {
    pub routed: bool,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqGetMessagesOptions {
    pub count: u32,
    #[serde(rename = "ackmode")]
    pub ack_mode: RabbitMqGetMessagesAckMode,
    pub encoding: RabbitMqGetMessagesEncoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RabbitMqGetMessagesAckMode {
    AckRequeueTrue,
    AckRequeueFalse,
    RejectRequeueTrue,
    RejectRequeueFalse,
}

#[derive(Debug, Serialize)]
pub enum RabbitMqGetMessagesEncoding {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqMessage {
    pub payload_bytes: u64,
    pub redelivered: bool,
    pub exchange: String,
    pub routing_key: String,
    pub message_count: u64,
    pub payload: String,
    pub payload_encoding: RabbitMqMessageEncoding,
}
