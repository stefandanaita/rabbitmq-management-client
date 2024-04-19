use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;

pub struct ClusterNameApi {
    client: ClientWithMiddleware,
}

impl ClusterNameApi {
    pub fn new(client: ClientWithMiddleware) -> Self { Self { client } }

    pub async fn get_cluster_name(&self) -> Result<RabbitMqClusterName, RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::GET,
                "http://localhost:15672/api/cluster-name",
            )
            .send()
            .await?;

        handle_response(response).await
    }

    pub async fn set_cluster_name(&self, body: RabbitMqClusterName) -> Result<(), RabbitMqClientError> {
        let response = self
            .client
            .request(
                reqwest::Method::PUT,
                "http://localhost:15672/api/cluster-name",
            )
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RabbitMqClusterName {
    pub name: String,
}
