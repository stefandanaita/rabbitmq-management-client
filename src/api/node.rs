use crate::api::_generic::handle_response;
use crate::api::overview::{RabbitMqContext, RabbitMqExchangeType};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct NodeApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl NodeApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
    }

    pub async fn list_nodes(&self) -> Result<Vec<RabbitMqNode>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, format!("{}/api/nodes", self.api_url))
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_node(&self, node: String) -> Result<RabbitMqNode, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/nodes/{}", self.api_url, node),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_node_memory(
        &self,
        node: String,
    ) -> Result<RabbitMqNodeMemory, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("{}/api/nodes/{}/memory", self.api_url, node),
            )
            .send()
            .await?;

        let wrapper: RabbitMqNodeMemoryWrapper = handle_response(response).await?;

        Ok(wrapper.memory)
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNode {
    pub os_pid: String,
    pub fd_total: i64,
    pub sockets_total: i64,
    pub mem_limit: i64,
    pub mem_alarm: bool,
    pub disk_free_limit: i64,
    pub disk_free_alarm: bool,
    pub proc_total: i64,
    pub rates_mode: String,
    pub uptime: i64,
    pub run_queue: i64,
    pub processors: i64,
    pub exchange_types: Vec<RabbitMqExchangeType>,
    pub auth_mechanisms: Vec<RabbitMqNodeAuthMechanism>,
    pub applications: Vec<RabbitMqNodeApplication>,
    pub contexts: Vec<RabbitMqContext>,
    pub log_files: Vec<String>,
    pub db_dir: String,
    pub config_files: Vec<String>,
    pub net_ticktime: i64,
    pub enabled_plugins: Vec<String>,
    pub mem_calculation_strategy: String,
    pub name: String,
    pub running: bool,
    #[serde(rename = "type")]
    pub kind: String,
    pub mem_used: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeAuthMechanism {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeApplication {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeMemoryWrapper {
    memory: RabbitMqNodeMemory,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeMemory {
    pub connection_readers: i64,
    pub connection_writers: i64,
    pub connection_channels: i64,
    pub connection_other: i64,
    pub queue_procs: i64,
    pub queue_slave_procs: i64,
    pub quorum_queue_procs: i64,
    pub quorum_queue_dlx_procs: i64,
    pub stream_queue_procs: i64,
    pub stream_queue_replica_reader_procs: i64,
    pub stream_queue_coordinator_procs: i64,
    pub plugins: i64,
    pub other_proc: i64,
    pub metrics: i64,
    pub mgmt_db: i64,
    pub mnesia: i64,
    pub quorum_ets: i64,
    pub other_ets: i64,
    pub binary: i64,
    pub msg_index: i64,
    pub code: i64,
    pub atom: i64,
    pub other_system: i64,
    pub allocated_unused: i64,
    pub reserved_unallocated: i64,
    pub strategy: String,
    pub total: RabbitMqNodeMemoryTotal,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeMemoryTotal {
    pub erlang: i64,
    pub rss: i64,
    pub allocated: i64,
}
