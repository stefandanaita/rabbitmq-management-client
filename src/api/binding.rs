use crate::api::_generic::{handle_empty_response, handle_response};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait BindingApi {
    async fn list_bindings(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError>;

    async fn filter_bindings(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError>;

    async fn get_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        properties_key: String,
    ) -> Result<RabbitMqBinding, RabbitMqClientError>;

    async fn create_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        request: RabbitMqBindingRequest,
    ) -> Result<String, RabbitMqClientError>;

    async fn delete_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        properties_key: String,
    ) -> Result<(), RabbitMqClientError>;
}

#[async_trait]
impl BindingApi for RabbitMqClient {
    async fn list_bindings(
        &self,
        vhost: Option<String>,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/bindings/{}",
                    self.api_url,
                    vhost.unwrap_or_default()
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn filter_bindings(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let destination_type = match destination_type {
            RabbitMqBindingDestinationType::Exchange => "e",
            RabbitMqBindingDestinationType::Queue => "q",
        };

        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/bindings/{}/e/{}/{}/{}",
                    self.api_url, vhost, source, destination_type, destination
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        properties_key: String,
    ) -> Result<RabbitMqBinding, RabbitMqClientError> {
        let destination_type = match destination_type {
            RabbitMqBindingDestinationType::Exchange => "e",
            RabbitMqBindingDestinationType::Queue => "q",
        };

        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/bindings/{}/e/{}/{}/{}/{}",
                    self.api_url, vhost, source, destination_type, destination, properties_key
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn create_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        request: RabbitMqBindingRequest,
    ) -> Result<String, RabbitMqClientError> {
        let destination_type = match destination_type {
            RabbitMqBindingDestinationType::Exchange => "e",
            RabbitMqBindingDestinationType::Queue => "q",
        };

        let response = self
            .client
            .request(
                reqwest::Method::POST,
                format!(
                    "{}/api/bindings/{}/e/{}/{}/{}",
                    self.api_url, vhost, source, destination_type, destination
                ),
            )
            .json(&request)
            .send()
            .await?;

        let location = match response.headers().get("Location") {
            None => None,
            Some(location) => {
                let location = location
                    .to_str()
                    .map_err(|e| RabbitMqClientError::UnexpectedResponse(e.to_string()))?
                    .to_string();
                Some(location)
            }
        };

        handle_empty_response(response).await?;

        if let Some(location) = location {
            Ok(location)
        } else {
            Err(RabbitMqClientError::UnexpectedResponse(
                "Binding Location header not present".to_string(),
            ))
        }
    }

    async fn delete_binding(
        &self,
        vhost: String,
        source: String,
        destination: String,
        destination_type: RabbitMqBindingDestinationType,
        properties_key: String,
    ) -> Result<(), RabbitMqClientError> {
        let destination_type = match destination_type {
            RabbitMqBindingDestinationType::Exchange => "e",
            RabbitMqBindingDestinationType::Queue => "q",
        };

        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!(
                    "{}/api/bindings/{}/e/{}/{}/{}/{}",
                    self.api_url, vhost, source, destination_type, destination, properties_key
                ),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqBinding {
    pub source: String,
    pub vhost: String,
    pub destination: String,
    pub destination_type: RabbitMqBindingDestinationType,
    pub routing_key: String,
    pub properties_key: String,
    pub arguments: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub enum RabbitMqBindingDestinationType {
    #[serde(rename = "exchange")]
    Exchange,
    #[serde(rename = "queue")]
    Queue,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqBindingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
}
