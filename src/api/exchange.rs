use crate::api::_generic::{handle_empty_response, handle_response};
use crate::api::binding::RabbitMqBinding;
use crate::api::pagination::{RabbitMqPaginatedResponse, RabbitMqPaginationRequest};
use crate::api::{RabbitMqPagination, RabbitMqPaginationFilter};
use crate::errors::RabbitMqClientError;
use crate::RabbitMqClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ExchangeApi {
    async fn list_exchanges(
        &self,
        vhost: Option<String>,
        pagination: Option<RabbitMqPagination>,
    ) -> Result<RabbitMqPaginatedResponse<RabbitMqExchange>, RabbitMqClientError>;

    async fn get_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<RabbitMqExchange, RabbitMqClientError>;

    async fn create_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn update_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError>;

    async fn delete_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<(), RabbitMqClientError>;

    async fn list_source_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError>;

    async fn list_destination_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError>;
}

#[async_trait]
impl ExchangeApi for RabbitMqClient {
    async fn list_exchanges(
        &self,
        vhost: Option<String>,
        pagination: Option<RabbitMqPagination>,
    ) -> Result<RabbitMqPaginatedResponse<RabbitMqExchange>, RabbitMqClientError> {
        let pagination: RabbitMqPaginationRequest = pagination.unwrap_or_default().into();

        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/exchanges/{}",
                    self.api_url,
                    vhost.unwrap_or_default()
                ),
            )
            .query(&pagination)
            .send()
            .await?;

        handle_response(response).await
    }

    async fn get_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<RabbitMqExchange, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn create_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let exchanges = self
            .list_exchanges(
                Some(vhost.clone()),
                Some(RabbitMqPagination {
                    page: 1,
                    page_size: None,
                    filter: Some(RabbitMqPaginationFilter::RegexFilter(format!(
                        "({exchange}$)"
                    ))),
                }),
            )
            .await?;

        if let Some(existing) = exchanges.items.iter().find(|e| e.name == exchange) {
            return Err(RabbitMqClientError::AlreadyExists(format!(
                "{} exchange",
                existing.name
            )));
        }

        self.update_exchange(vhost, exchange, request).await
    }

    async fn update_exchange(
        &self,
        vhost: String,
        exchange: String,
        request: RabbitMqExchangeRequest,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .json(&request)
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn delete_exchange(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                format!("{}/api/exchanges/{}/{}", self.api_url, vhost, exchange),
            )
            .send()
            .await?;

        handle_empty_response(response).await
    }

    async fn list_source_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/exchanges/{}/{}/bindings/source",
                    self.api_url, vhost, exchange
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    async fn list_destination_bindings(
        &self,
        vhost: String,
        exchange: String,
    ) -> Result<Vec<RabbitMqBinding>, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!(
                    "{}/api/exchanges/{}/{}/bindings/destination",
                    self.api_url, vhost, exchange
                ),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchange {
    pub auto_delete: bool,
    pub durable: bool,
    pub internal: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub user_who_performed_action: String,
    pub vhost: String,
    pub message_stats: Option<RabbitMqExchangeMessageStats>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchangeMessageStats {
    pub publish_in: Option<i64>,
    pub publish_out: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RabbitMqExchangeRequest {
    #[serde(rename = "type")]
    pub kind: String,
    pub auto_delete: bool,
    pub durable: bool,
    pub internal: bool,
}
