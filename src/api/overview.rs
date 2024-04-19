use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct OverviewApi {
    client: ClientWithMiddleware,
}

impl OverviewApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn get_overview(&self) -> Result<RabbitMqOverview, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, "http://localhost:15672/api/overview")
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqOverview {
    management_version: String,
    rates_mode: String,
    exchange_types: Vec<RabbitMqExchangeType>,
    product_version: String,
    product_name: String,
    rabbitmq_version: String,
    cluster_name: String,
    erlang_version: String,
    erlang_full_version: String,
    release_series_support_status: String,
    disable_stats: bool,
    is_op_policy_updating_enabled: bool,
    enable_queue_totals: bool,
    churn_rates: RabbitMqChurnRates,
    object_totals: RabbitMqObjectTotals,
    listeners: Vec<RabbitMqListener>,
    contexts: Vec<RabbitMqContext>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqExchangeType {
    name: String,
    description: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqContext {
    node: Option<String>,
    description: String,
    path: String,
    cowboy_opts: String,
    port: String,
    protocol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqListener {
    node: String,
    protocol: String,
    ip_address: String,
    port: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqChurnRates {
    channel_closed: i64,
    channel_created: i64,
    connection_closed: i64,
    connection_created: i64,
    queue_created: i64,
    queue_declared: i64,
    queue_deleted: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqObjectTotals {
    channels: i64,
    connections: i64,
    consumers: i64,
    exchanges: i64,
    queues: i64,
}
