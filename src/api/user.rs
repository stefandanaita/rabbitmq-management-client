use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::permission::{RabbitMqPermission, RabbitMqTopicPermission};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait UserApi {
    async fn who_am_i(&self) -> Result<RabbitMqWhoAmI, RabbitMqClientError>;

    async fn list_users(&self) -> Result<Vec<RabbitMqUser>, RabbitMqClientError>;

    async fn list_users_without_permissions(
        &self,
    ) -> Result<Vec<RabbitMqUser>, RabbitMqClientError>;

    async fn bulk_delete_users(
        &self,
        users: RabbitMqUsersBulkDeleteRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn list_user_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError>;

    async fn list_user_topic_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError>;
}

#[async_trait]
impl UserApi for RabbitMqClient {
    #[tracing::instrument(skip(self))]
    async fn who_am_i(&self) -> Result<RabbitMqWhoAmI, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, format!("{}/api/whoami", self.api_url))
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn list_users(&self) -> Result<Vec<RabbitMqUser>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, format!("{}/api/users", self.api_url))
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn list_users_without_permissions(
        &self,
    ) -> Result<Vec<RabbitMqUser>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/users/without-permissions", self.api_url),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn bulk_delete_users(
        &self,
        users: RabbitMqUsersBulkDeleteRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/users/bulk-delete", self.api_url),
            )
            .json(&users)
            .send()
            .await?;

        handle_empty_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn list_user_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/users/{}/permissions", self.api_url, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    #[tracing::instrument(skip(self))]
    async fn list_user_topic_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/users/{}/topic-permissions", self.api_url, user),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqWhoAmI {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqUser {
    pub name: String,
    pub password_hash: String,
    pub hashing_algorithm: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqUsersBulkDeleteRequest {
    pub users: Vec<String>,
}
