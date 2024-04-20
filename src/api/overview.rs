use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

pub struct OverviewApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl OverviewApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn get_overview(&self) -> Result<RabbitMqOverview, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/overview", self.api_url),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_cluster_name(&self) -> Result<RabbitMqClusterName, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/cluster-name", self.api_url),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn set_cluster_name(
        &self,
        request: RabbitMqClusterName,
    ) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                format!("{}/api/cluster-name", self.api_url),
            )
            .json(&request)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqOverview {
    pub management_version: String,
    pub rates_mode: String,
    pub exchange_types: Vec<RabbitMqExchangeType>,
    pub product_version: String,
    pub product_name: String,
    pub rabbitmq_version: String,
    pub cluster_name: String,
    pub erlang_version: String,
    pub erlang_full_version: String,
    pub release_series_support_status: String,
    pub disable_stats: bool,
    pub is_op_policy_updating_enabled: bool,
    pub enable_queue_totals: bool,
    pub churn_rates: RabbitMqChurnRates,
    pub object_totals: RabbitMqObjectTotals,
    pub listeners: Vec<RabbitMqListener>,
    pub contexts: Vec<RabbitMqContext>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchangeType {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqContext {
    pub node: Option<String>,
    pub description: String,
    pub path: String,
    pub cowboy_opts: String,
    pub port: String,
    pub protocol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqListener {
    pub node: String,
    pub protocol: String,
    pub ip_address: String,
    pub port: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqChurnRates {
    pub channel_closed: i64,
    pub channel_created: i64,
    pub connection_closed: i64,
    pub connection_created: i64,
    pub queue_created: i64,
    pub queue_declared: i64,
    pub queue_deleted: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqObjectTotals {
    pub channels: i64,
    pub connections: i64,
    pub consumers: i64,
    pub exchanges: i64,
    pub queues: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RabbitMqClusterName {
    pub name: String,
}
