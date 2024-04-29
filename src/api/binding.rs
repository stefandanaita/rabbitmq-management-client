use crate::api::_generic::{handle_empty_response, handle_response};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BindingApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl BindingApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_bindings(
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

    pub async fn list_exchange_bindings(
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

    pub async fn create_binding(
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
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqBinding {
    pub source: String,
    pub vhost: String,
    pub destination: String,
    pub destination_type: String,
    pub routing_key: String,
    pub properties_key: String,
}

#[derive(Debug)]
pub enum RabbitMqBindingDestinationType {
    Exchange,
    Queue,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqBindingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
}
