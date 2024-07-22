use crate::api::_generic::{handle_empty_response, handle_response};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait PolicyApi {
    async fn list_policies(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqPolicy>, RabbitMqClientError>;

    async fn get_policy(
        &self,
        vhost: String,
        policy: String,
    ) -> Result<RabbitMqPolicy, RabbitMqClientError>;

    async fn create_policy(
        &self,
        vhost: String,
        policy: String,
        request: RabbitMqPolicyRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn update_policy(
        &self,
        vhost: String,
        policy: String,
        request: RabbitMqPolicyRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn delete_policy(&self, vhost: String, policy: String)
        -> Result<(), RabbitMqClientError>;
}

#[async_trait]
impl PolicyApi for RabbitMqClient {
    async fn list_policies(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqPolicy>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/policies/{}",
                    self.api_url,
                    vhost.unwrap_or_default()
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_policy(
        &self,
        vhost: String,
        policy: String,
    ) -> Result<RabbitMqPolicy, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/policies/{}/{}", self.api_url, vhost, policy),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn create_policy(
        &self,
        vhost: String,
        policy: String,
        request: RabbitMqPolicyRequest,
    ) -> Result<(), RabbitMqClientError> {
        let policies = self.list_policies(Some(vhost.clone())).await?;
        if let Some(existing) = policies.iter().find(|v| v.name == policy) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} policy",
                existing.name
            )));
        }

        self.update_policy(vhost, policy, request).await
    }

    async fn update_policy(
        &self,
        vhost: String,
        policy: String,
        request: RabbitMqPolicyRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/policies/{}/{}", self.api_url, vhost, policy),
            )
            .json(&request)
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn delete_policy(
        &self,
        vhost: String,
        policy: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/policies/{}/{}", self.api_url, vhost, policy),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqPolicy {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqPolicyRequest {
    pub pattern: String,
    pub definition: HashMap<String, RabbitMqPolicyDefinitionValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "apply-to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum RabbitMqPolicyDefinitionValue {
    String(String),
    Integer(i64),
}
