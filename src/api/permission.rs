use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct PermissionApi {
    client: ClientWithMiddleware,
}

impl PermissionApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn list_permissions(&self) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                "http://localhost:15672/api/permissions",
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
                format!("http://localhost:15672/api/permissions/{}/{}", vhost, user),
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
                format!("http://localhost:15672/api/permissions/{}/{}", vhost, user),
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
                "http://localhost:15672/api/topic-permissions",
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
                format!(
                    "http://localhost:15672/api/topic-permissions/{}/{}",
                    vhost, user
                ),
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
                format!(
                    "http://localhost:15672/api/topic-permissions/{}/{}",
                    vhost, user
                ),
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
