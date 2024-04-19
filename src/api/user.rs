use crate::api::_generic::handle_response;
use crate::api::permission::{RabbitMqPermission, RabbitMqTopicPermission};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

pub struct UserApi {
    client: ClientWithMiddleware,
}

impl UserApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn who_am_i(&self) -> Result<RabbitMqWhoAmI, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, "http://localhost:15672/api/whoami")
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_users(&self) -> Result<Vec<RabbitMqUser>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, "http://localhost:15672/api/users")
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_users_without_permissions(
        &self,
    ) -> Result<Vec<RabbitMqUser>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                "http://localhost:15672/api/users/without-permissions",
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn bulk_delete_users(
        &self,
        users: RabbitMqUsersBulkDeleteRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                "http://localhost:15672/api/users/bulk-delete",
            )
            .json(&users)
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_user_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("http://localhost:15672/api/users/{}/permissions", user),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn list_user_topic_permissions(
        &self,
        user: String,
    ) -> Result<Vec<RabbitMqTopicPermission>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!(
                    "http://localhost:15672/api/users/{}/topic-permissions",
                    user
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqWhoAmI {
    name: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqUser {
    name: String,
    password_hash: String,
    hashing_algorithm: String,
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqUsersBulkDeleteRequest {
    users: Vec<String>,
}
