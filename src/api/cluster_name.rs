use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

pub struct ClusterNameApi {
    api_url: String,
    client: ClientWithMiddleware,
}

impl ClusterNameApi {
    pub fn new(api_url: String, client: ClientWithMiddleware) -> Self {
        Self { api_url, client }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RabbitMqClusterName {
    pub name: String,
}
