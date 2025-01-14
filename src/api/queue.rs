use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::binding::RabbitMqBinding;
use crate::api::options::pagination::RabbitMqPaginationRequest;
use crate::api::RabbitMqPaginatedResponse;
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::options::RabbitMqRequestOptions;

#[async_trait]
pub trait QueueApi {
    async fn list_queues(
        &self,
        vhost: Option<String>,
        options: Option<RabbitMqRequestOptions>,
    ) -> Result<RabbitMqPaginatedResponse<RabbitMqQueue>, RabbitMqClientError>;

    async fn get_queue(
        &self,
        vhost: String,
        name: String,
    ) -> Result<RabbitMqQueue, RabbitMqClientError>;

    async fn get_queue_bindings(
        &self,
        vhost: String,
        name: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError>;

    async fn create_queue(
        &self,
        vhost: String,
        queue: String,
        request: RabbitMqQueueRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn update_queue(
        &self,
        vhost: String,
        queue: String,
        request: RabbitMqQueueRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn delete_queue(&self, vhost: String, name: String) -> Result<(), RabbitMqClientError>;

    async fn purge_queue(&self, vhost: String, name: String) -> Result<(), RabbitMqClientError>;

    async fn set_queue_actions(
        &self,
        vhost: String,
        queue: String,
        action: RabbitMqQueueAction,
    ) -> Result<(), RabbitMqClientError>;
}

#[async_trait]
impl QueueApi for RabbitMqClient {
    #[tracing::instrument(skip(self))]
    async fn list_queues(
        &self,
        vhost: Option<String>,
        options: Option<RabbitMqRequestOptions>,
    ) -> Result<RabbitMqPaginatedResponse<RabbitMqQueue>, RabbitMqClientError> {
        let options: RabbitMqRequestOptions = options.unwrap_or_default();
        let pagination: RabbitMqPaginationRequest = options.pagination.unwrap_or_default().into();

        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/queues/{}", self.api_url, vhost.unwrap_or_default()),
            )
            .query(&pagination)
            .query(&options.sorting)
            .query(&[("disable_stats", options.disable_stats)])
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_queue(
        &self,
        vhost: String,
        name: String,
    ) -> Result<RabbitMqQueue, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/queues/{}/{}", self.api_url, vhost, name),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_queue_bindings(
        &self,
        vhost: String,
        name: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/queues/{}/{}/bindings", self.api_url, vhost, name),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn create_queue(
        &self,
        vhost: String,
        queue: String,
        request: RabbitMqQueueRequest,
    ) -> Result<(), RabbitMqClientError> {
        match self.get_queue(vhost.clone(), queue.clone()).await {
            Ok(_) => Err(RabbitMqClientError::AlreadyExists(format!(
                "{} queue",
                queue
            ))),
            Err(e) => match e {
                RabbitMqClientError::NotFound(_) => self.update_queue(vhost, queue, request).await,
                _ => Err(e),
            },
        }
    }

    #[tracing::instrument(skip(self))]
    async fn update_queue(
        &self,
        vhost: String,
        queue: String,
        request: RabbitMqQueueRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/queues/{}/{}", self.api_url, vhost, queue),
            )
            .json(&request)
            .send()
            .await?;

        handle_empty_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn delete_queue(&self, vhost: String, name: String) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/queues/{}/{}", self.api_url, vhost, name),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn purge_queue(&self, vhost: String, name: String) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/queues/{}/{}/contents", self.api_url, vhost, name),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn set_queue_actions(
        &self,
        vhost: String,
        queue: String,
        action: RabbitMqQueueAction,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}/api/queues/{}/{}/actions", self.api_url, vhost, queue),
            )
            .json(&RabbitMqQueueActionRequest { action })
            .send()
            .await?;

        handle_empty_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueue {
    pub name: String,
    pub node: String,
    pub arguments: HashMap<String, RabbitMqArgument>,
    pub state: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub vhost: String,
    pub auto_delete: bool,
    pub durable: bool,
    pub exclusive: bool,
    pub consumer_capacity: Option<Decimal>,
    pub consumer_utilisation: Option<Decimal>,
    pub consumers: Option<i64>,
    pub messages: Option<i64>,
    pub messages_ready: Option<i64>,
    pub messages_unacknowledged: Option<i64>,
    pub garbage_collection: Option<RabbitMqQueueGarbageCollection>,
    pub message_stats: Option<RabbitMqQueueMessageStats>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RabbitMqArgument {
    String(String),
    Decimal(Decimal),
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueueMessageStats {
    #[serde(default)]
    pub ack: i64,
    #[serde(default)]
    pub deliver: i64,
    #[serde(default)]
    pub deliver_get: i64,
    #[serde(default)]
    pub deliver_no_ack: i64,
    #[serde(default)]
    pub get: i64,
    #[serde(default)]
    pub get_empty: i64,
    #[serde(default)]
    pub get_no_ack: i64,
    #[serde(default)]
    pub publish: i64,
    #[serde(default)]
    pub redeliver: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueueGarbageCollection {
    pub fullsweep_after: i64,
    pub max_heap_size: i64,
    pub min_bin_vheap_size: i64,
    pub min_heap_size: i64,
    pub minor_gcs: i64,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqQueueRequest {
    pub auto_delete: bool,
    pub durable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqQueueActionRequest {
    pub action: RabbitMqQueueAction,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RabbitMqQueueAction {
    Sync,
    CancelSync,
}
