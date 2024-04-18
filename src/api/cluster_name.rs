use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;
use crate::api::_generic::handle_response;
use crate::errors::RabbitMqClientError;

#[derive(Clone)]
pub struct ClusterNameApi {
    client: ClientWithMiddleware,
}

impl ClusterNameApi {
    pub fn new(client: ClientWithMiddleware) -> Self { Self { client } }

    pub async fn get_cluster_name(&self) -> Result<ClusterName, RabbitMqClientError> {
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
}

#[derive(Debug, Deserialize)]
pub struct ClusterName {
    pub name: String,
}
