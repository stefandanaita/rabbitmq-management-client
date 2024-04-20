use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct PermissionApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl PermissionApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_permissions(&self) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/permissions", self.api_url),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<RabbitMqPermission, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/permissions/{}/{}", self.api_url, vhost, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/permissions/{}/{}", self.api_url, vhost, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_topic_permissions(
        &self,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/topic-permissions", self.api_url),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_topic_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<RabbitMqTopicPermission, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/topic-permissions/{}/{}", self.api_url, vhost, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn delete_topic_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/topic-permissions/{}/{}", self.api_url, vhost, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqPermission {
    vhost: String,
    user: String,
    configure: String,
    write: String,
    read: String,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqTopicPermission {}
