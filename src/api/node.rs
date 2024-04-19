use crate::api::_generic::handle_response;
use crate::api::overview::{RabbitMqContext, RabbitMqExchangeType};
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

pub struct NodeApi {
    client: ClientWithMiddleware,
}

impl NodeApi {
    pub fn new(client: ClientWithMiddleware) -> Self {
        Self { client }
    }

    pub async fn get_nodes(&self) -> Result<Vec<RabbitMqNode>, RabbitMqClientError> {
        let response = self
            .client
            .request(reqwest::Method::GET, "http://localhost:15672/api/nodes")
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_node(&self, node_name: String) -> Result<RabbitMqNode, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("http://localhost:15672/api/nodes/{}", node_name),
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn get_node_memory(
        &self,
        node_name: String,
    ) -> Result<RabbitMqNodeMemory, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                format!("http://localhost:15672/api/nodes/{}/memory", node_name),
            )
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNode {
    os_pid: String,
    fd_total: i64,
    sockets_total: i64,
    mem_limit: i64,
    mem_alarm: bool,
    disk_free_limit: i64,
    disk_free_alarm: bool,
    proc_total: i64,
    rates_mode: String,
    uptime: i64,
    run_queue: i64,
    processors: i64,
    exchange_types: Vec<RabbitMqExchangeType>,
    auth_mechanisms: Vec<RabbitMqNodeAuthMechanism>,
    applications: Vec<RabbitMqNodeApplication>,
    contexts: Vec<RabbitMqContext>,
    log_files: Vec<String>,
    db_dir: String,
    config_files: Vec<String>,
    net_ticktime: i64,
    enabled_plugins: Vec<String>,
    mem_calculation_strategy: String,
    name: String,
    running: bool,
    #[serde(rename = "type")]
    kind: String,
    mem_used: i64,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeAuthMechanism {
    name: String,
    description: String,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeApplication {
    name: String,
    description: String,
    version: String,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeMemory {
    connection_readers: i64,
    connection_writers: i64,
    connection_channels: i64,
    connection_other: i64,
    queue_procs: i64,
    queue_slave_procs: i64,
    quorum_queue_procs: i64,
    quorum_queue_dlx_procs: i64,
    stream_queue_procs: i64,
    stream_queue_replica_reader_procs: i64,
    stream_queue_coordinator_procs: i64,
    plugins: i64,
    other_proc: i64,
    metrics: i64,
    mgmt_db: i64,
    mnesia: i64,
    quorum_ets: i64,
    other_ets: i64,
    binary: i64,
    msg_index: i64,
    code: i64,
    atom: i64,
    other_system: i64,
    allocated_unused: i64,
    reserved_unallocated: i64,
    strategy: String,
    total: RabbitMqNodeMemoryTotal,
}

#[derive(Debug, Deserialize)]
pub struct RabbitMqNodeMemoryTotal {
    erlang: i64,
    rss: i64,
    allocated: i64,
}
