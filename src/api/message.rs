use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

        let messages = handle_response::<Vec<RabbitMqMessageObject>>(response).await?;

        Ok(messages
            .into_iter()
            .map(|m| RabbitMqMessage {
                payload_bytes: m.payload_bytes,
                redelivered: m.redelivered,
                exchange: m.exchange,
                routing_key: m.routing_key,
                message_count: m.message_count,
                payload: m.payload,
                payload_encoding: m.payload_encoding,
                properties: match m.properties {
                    RabbitMqMessageProps::Empty() => None,
                    RabbitMqMessageProps::Properties(p) => Some(p),
                },
            })
            .collect())
    }
}

#[derive(Debug, Serialize)]
pub struct RabbitMqPublishMessageRequest {
    pub properties: RabbitMqMessageProperties,
    pub routing_key: String,
    pub payload: String,
    pub payload_encoding: RabbitMqMessageEncoding,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RabbitMqMessageEncoding {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RabbitMqMessageDeliveryMode {
    NonPersistent = 1,
    Persistent = 2,
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
struct RabbitMqMessageObject {
    pub payload_bytes: u64,
    pub redelivered: bool,
    pub exchange: String,
    pub routing_key: String,
    pub message_count: u64,
    pub payload: String,
    pub payload_encoding: RabbitMqMessageEncoding,
    pub properties: RabbitMqMessageProps,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RabbitMqMessageProps {
    Properties(RabbitMqMessageProperties),
    Empty(),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqMessage {
    pub payload_bytes: u64,
    pub redelivered: bool,
    pub exchange: String,
    pub routing_key: String,
    pub message_count: u64,
    pub payload: String,
    pub payload_encoding: RabbitMqMessageEncoding,
    pub properties: Option<RabbitMqMessageProperties>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RabbitMqMessageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<RabbitMqMessageDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, RabbitMqMessageHeader>>,
    #[serde(flatten)]
    pub extra_properties: HashMap<String, RabbitMqMessageProperty>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RabbitMqMessageHeader {
    String(String),
    Number(Decimal),
    Boolean(bool),
    HashMap(HashMap<String, RabbitMqMessageHeader>),
    List(Vec<RabbitMqMessageHeader>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RabbitMqMessageProperty {
    String(String),
    Number(Decimal),
    Boolean(bool),
}

impl Display for RabbitMqMessageProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RabbitMqMessageProperty::String(s) => write!(f, "{}", s),
            RabbitMqMessageProperty::Number(d) => write!(f, "{}", d),
            RabbitMqMessageProperty::Boolean(b) => write!(f, "{}", b),
        }
    }
}
