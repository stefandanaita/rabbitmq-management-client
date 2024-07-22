use crate::api::_generic::{handle_empty_response, handle_response};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait PermissionApi {
    async fn list_permissions(&self) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError>;

    async fn get_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<RabbitMqPermission, RabbitMqClientError>;

    async fn delete_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<(), RabbitMqClientError>;

    async fn list_topic_permissions(
        &self,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError>;

    async fn get_topic_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<RabbitMqTopicPermission, RabbitMqClientError>;

    async fn delete_topic_permission(
        &self,
        vhost: String,
        user: String,
    ) -> Result<(), RabbitMqClientError>;
}

#[async_trait]
impl PermissionApi for RabbitMqClient {
    async fn list_permissions(&self) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
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

    async fn get_permission(
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

    async fn delete_permission(
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

        handle_empty_response(response).await
    }

    async fn list_topic_permissions(
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

    async fn get_topic_permission(
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

    async fn delete_topic_permission(
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

        handle_empty_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqPermission {
    pub vhost: String,
    pub user: String,
    pub configure: String,
    pub write: String,
    pub read: String,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqTopicPermission {}
