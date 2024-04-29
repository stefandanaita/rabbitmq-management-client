use crate::api::_generic::{handle_empty_response, handle_response};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PolicyApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl PolicyApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_policies(
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

    pub async fn get_policy(
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

    pub async fn create_policy(
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

    pub async fn update_policy(
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

    pub async fn delete_policy(
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
