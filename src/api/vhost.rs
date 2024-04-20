use crate::api::_generic::handle_response;
use crate::api::permission::{RabbitMqPermission, RabbitMqTopicPermission};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct VhostApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl VhostApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_vhosts(&self) -> Result<Vec<RabbitMqVhost>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, format!("{}/api/vhosts", self.api_url))
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn create_vhost(
        &self,
        request: RabbitMqVhostRequest,
    ) -> Result<(), RabbitMqClientError> {
        let vhosts = self.list_vhosts().await?;
        if let Some(existing) = vhosts.iter().find(|v| v.name == request.name.clone()) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} vhost",
                existing.name
            )));
        }

        self.update_vhost(request).await
    }

    pub async fn update_vhost(
        &self,
        request: RabbitMqVhostRequest,
    ) -> Result<(), RabbitMqClientError> {
        #[derive(Debug, Serialize)]
        struct RequestBody {
            description: Option<String>,
            tags: String,
            tracing: bool,
        }

        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/vhosts/{}", self.api_url, request.name),
            )
            .json(&RequestBody {
                description: request.description,
                tags: request.tags.join(","),
                tracing: request.tracing,
            })
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_vhost(&self, vhost: String) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/vhosts/{}", self.api_url, vhost),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn start_vhost_on_node(
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

        handle_response(response).await
    }

    pub async fn list_vhost_permissions(
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

    pub async fn list_vhost_topic_permissions(
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
    cluster_state: HashMap<String, String>,
    default_queue_type: String,
    description: String,
    metadata: RabbitMqVhostMetadata,
    name: String,
    tags: Vec<String>,
    tracing: bool,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqVhostMetadata {
    description: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqVhostRequest {
    name: String,
    description: Option<String>,
    tags: Vec<String>,
    tracing: bool,
}
