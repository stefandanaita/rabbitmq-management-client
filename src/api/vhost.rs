use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::permission::{RabbitMqPermission, RabbitMqTopicPermission};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait VhostApi {
    async fn list_vhosts(&self) -> Result<Vec<RabbitMqVhost>, RabbitMqClientError>;

    async fn get_vhost(&self, vhost: String) -> Result<RabbitMqVhost, RabbitMqClientError>;

    async fn create_vhost(&self, request: RabbitMqVhostRequest) -> Result<(), RabbitMqClientError>;

    async fn update_vhost(&self, request: RabbitMqVhostRequest) -> Result<(), RabbitMqClientError>;

    async fn delete_vhost(&self, vhost: String) -> Result<(), RabbitMqClientError>;

    async fn start_vhost_on_node(
        &self,
        vhost: String,
        node: String,
    ) -> Result<(), RabbitMqClientError>;

    async fn list_vhost_permissions(
        &self,
        vhost: String,
    ) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError>;

    async fn list_vhost_topic_permissions(
        &self,
        vhost: String,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError>;
}

#[async_trait]
impl VhostApi for RabbitMqClient {
    async fn list_vhosts(&self) -> Result<Vec<RabbitMqVhost>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, format!("{}/api/vhosts", self.api_url))
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_vhost(&self, vhost: String) -> Result<RabbitMqVhost, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/vhosts/{}", self.api_url, vhost),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn create_vhost(&self, request: RabbitMqVhostRequest) -> Result<(), RabbitMqClientError> {
        let vhosts = self.list_vhosts().await?;
        if let Some(existing) = vhosts.iter().find(|v| v.name == request.name.clone()) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} vhost",
                existing.name
            )));
        }

        self.update_vhost(request).await
    }

    async fn update_vhost(&self, request: RabbitMqVhostRequest) -> Result<(), RabbitMqClientError> {
        #[derive(Debug, Serialize)]
        struct RequestBody {
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            tags: String,
            tracing: bool,
        }

        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/vhosts/{}", self.api_url, request.name),
            )
            .json(&RequestBody {
                description: request.description,
                tags: request.tags.join(","),
                tracing: request.tracing,
            })
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn delete_vhost(&self, vhost: String) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/vhosts/{}", self.api_url, vhost),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn start_vhost_on_node(
        &self,
        vhost: String,
        node: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!("{}/api/vhosts/{}/start/{}", self.api_url, vhost, node),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn list_vhost_permissions(
        &self,
        vhost: String,
    ) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/vhosts/{}/permissions", self.api_url, vhost),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn list_vhost_topic_permissions(
        &self,
        vhost: String,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/vhosts/{}/topic-permissions", self.api_url, vhost),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqVhost {
    pub cluster_state: HashMap<String, String>,
    pub default_queue_type: String,
    pub description: String,
    pub metadata: RabbitMqVhostMetadata,
    pub name: String,
    pub tags: Vec<String>,
    pub tracing: bool,
    #[serde(default)]
    pub messages: i64,
    #[serde(default)]
    pub messages_ready: i64,
    #[serde(default)]
    pub messages_unacknowledged: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqVhostMetadata {
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqVhostRequest {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub tracing: bool,
}
