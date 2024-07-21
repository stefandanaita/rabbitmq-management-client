use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::binding::RabbitMqBinding;
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait QueueApi {
    async fn list_queues(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqQueue>, RabbitMqClientError>;

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
    async fn list_queues(
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
    pub arguments: HashMap<String, String>,
    pub state: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub vhost: String,
    pub auto_delete: bool,
    pub durable: bool,
    pub exclusive: bool,
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
pub struct RabbitMqQueueActionRequest {
    pub action: RabbitMqQueueAction,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RabbitMqQueueAction {
    Sync,
    CancelSync,
}
