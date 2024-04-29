use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::binding::RabbitMqBinding;
use crate::errors::RabbitMqClientError;
use chrono::{DateTime, Utc};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueueApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl QueueApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_queues(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqQueue>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/queues/{}", self.api_url, vhost.unwrap_or_default()),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_queue(
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

    pub async fn get_queue_bindings(
        &self,
        vhost: String,
        name: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/queues/{}/{}/definitions", self.api_url, vhost, name),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn create_queue(
        &self,
        vhost: String,
        queue: String,
        request: RabbitMqQueueRequest,
    ) -> Result<(), RabbitMqClientError> {
        let queues = self.list_queues(Some(vhost.clone())).await?;
        if let Some(existing) = queues.iter().find(|q| q.name == queue) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} queue",
                existing.name
            )));
        }

        self.update_queue(vhost, queue, request).await
    }

    pub async fn update_queue(
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

    pub async fn delete_queue(
        &self,
        vhost: String,
        name: String,
    ) -> Result<(), RabbitMqClientError> {
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

    pub async fn purge_queue(
        &self,
        vhost: String,
        name: String,
    ) -> Result<(), RabbitMqClientError> {
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

    pub async fn set_queue_actions(
        &self,
        vhost: String,
        queue: String,
        actions: RabbitMqQueueActionsRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}/api/queues/{}/{}/actions", self.api_url, vhost, queue),
            )
            .json(&actions)
            .send()
            .await?;

        handle_empty_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueue {
    pub name: String,
    pub node: String,
    pub operator_policy: String,
    pub policy: String,
    pub state: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub vhost: String,
    pub auto_delete: bool,
    pub consumer_capacity: i64,
    pub consumer_utilisation: i64,
    pub consumers: i64,
    pub durable: bool,
    pub exclusive: bool,
    pub exclusive_consumer_tag: Option<String>,
    pub idle_since: Option<DateTime<Utc>>,
    pub memory: i64,
    pub messages: i64,
    pub message_stats: RabbitMqQueueMessageStats,
    pub message_bytes: i64,
    pub message_bytes_paged_out: i64,
    pub message_bytes_persistent: i64,
    pub message_bytes_ram: i64,
    pub message_bytes_ready: i64,
    pub message_bytes_unacknowledged: i64,
    pub messages_paged_out: i64,
    pub messages_persistent: i64,
    pub messages_ram: i64,
    pub messages_read: i64,
    pub messages_ready_ram: i64,
    pub messages_unacknowledged: i64,
    pub messages_unacknowledged_ram: i64,
    pub reductions: i64,
    pub recoverable_slaves: Option<Vec<String>>,
    pub slave_nodes: Option<Vec<String>>,
    pub synchronised_slave_nodes: Option<Vec<String>>,
    pub garbage_collection: RabbitMqQueueGarbageCollection,
    pub effective_policy_definition: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueueMessageStats {
    pub ack: i64,
    pub deliver: i64,
    pub deliver_get: i64,
    pub deliver_no_ack: i64,
    pub get: i64,
    pub get_empty: i64,
    pub get_no_ack: i64,
    pub publish: i64,
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
pub struct RabbitMqQueueActionsRequest {
    pub actions: RabbitMqQueueAction,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RabbitMqQueueAction {
    Sync,
    CancelSync,
}
