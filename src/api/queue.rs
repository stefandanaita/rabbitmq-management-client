use crate::api::_generic::handle_response;
use crate::api::binding::RabbitMqBinding;
use crate::errors::RabbitMqClientError;
use chrono::{DateTime, Utc};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct QueueApi {
    client: ClientWithMiddleware,
}

impl QueueApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn list_queues(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqQueue>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "http://localhost:15672/api/queues/{}",
                    vhost.unwrap_or_default()
                ),
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
                format!("http://localhost:15672/api/queues/{}/{}", vhost, name),
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
                format!(
                    "http://localhost:15672/api/queues/{}/{}/definitions",
                    vhost, name
                ),
            )
            .send()
            .await?;

        handle_response(response).await
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
                format!("http://localhost:15672/api/queues/{}/{}", vhost, name),
            )
            .send()
            .await?;

        handle_response(response).await
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
                format!(
                    "http://localhost:15672/api/queues/{}/{}/contents",
                    vhost, name
                ),
            )
            .send()
            .await?;

        handle_response(response).await
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
                format!(
                    "http://localhost:15672/api/queues/{}/{}/actions",
                    vhost, queue
                ),
            )
            .json(&actions)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueue {
    name: String,
    node: String,
    operator_policy: String,
    policy: String,
    state: String,
    #[serde(rename = "type")]
    kind: String,
    vhost: String,
    auto_delete: bool,
    consumer_capacity: i64,
    consumer_utilisation: i64,
    consumers: i64,
    durable: bool,
    exclusive: bool,
    exclusive_consumer_tag: Option<String>,
    idle_since: Option<DateTime<Utc>>,
    memory: i64,
    messages: i64,
    message_stats: RabbitMqQueueMessageStats,
    message_bytes: i64,
    message_bytes_paged_out: i64,
    message_bytes_persistent: i64,
    message_bytes_ram: i64,
    message_bytes_ready: i64,
    message_bytes_unacknowledged: i64,
    messages_paged_out: i64,
    messages_persistent: i64,
    messages_ram: i64,
    messages_read: i64,
    messages_ready_ram: i64,
    messages_unacknowledged: i64,
    messages_unacknowledged_ram: i64,
    reductions: i64,
    recoverable_slaves: Option<Vec<String>>,
    slave_nodes: Option<Vec<String>>,
    synchronised_slave_nodes: Option<Vec<String>>,
    garbage_collection: RabbitMqQueueGarbageCollection,
    effective_policy_definition: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueueMessageStats {
    ack: i64,
    deliver: i64,
    deliver_get: i64,
    deliver_no_ack: i64,
    get: i64,
    get_empty: i64,
    get_no_ack: i64,
    publish: i64,
    redeliver: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqQueueGarbageCollection {
    fullsweep_after: i64,
    max_heap_size: i64,
    min_bin_vheap_size: i64,
    min_heap_size: i64,
    minor_gcs: i64,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqQueueActionsRequest {
    actions: RabbitMqQueueAction,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RabbitMqQueueAction {
    Sync,
    CancelSync,
}
