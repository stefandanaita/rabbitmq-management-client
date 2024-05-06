use reqwest_middleware::ClientWithMiddleware;
use rabbitmq_management_client::api::vhost::{RabbitMqVhost, RabbitMqVhostRequest};
use rabbitmq_management_client::config::RabbitMqConfiguration;
use rabbitmq_management_client::errors::RabbitMqClientError;
use rabbitmq_management_client::{RabbitMqClient, RabbitMqClientBuilder};
use uuid::Uuid;

const RABBITMQ_API_URL: &str = "http://localhost:15672";
const RABBITMQ_USERNAME: &str = "guest";
const RABBITMQ_PASSWORD: &str = "guest";

pub struct TestContext {
    pub rabbitmq: RabbitMqClient,
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TestContext {
    pub fn new() -> Self {
        let rmq = RabbitMqClientBuilder::new(test_config()).build().unwrap();

        Self { rabbitmq: rmq }
    }

    pub fn new_with_preset_client(client: ClientWithMiddleware) -> Self {
        let rmq = RabbitMqClientBuilder::new(test_config())
            .preset_client(client)
            .build()
            .unwrap();

        Self { rabbitmq: rmq }
    }

    pub async fn create_random_vhost(&self) -> Result<RabbitMqVhost, RabbitMqClientError> {
        let id = Uuid::new_v4().to_string();

        self.rabbitmq
            .apis
            .vhosts
            .create_vhost(RabbitMqVhostRequest {
                name: id.clone(),
                description: Some(format!("{} testing vhost", id.clone())),
                tags: vec![],
                tracing: false,
            })
            .await?;

        self.rabbitmq.apis.vhosts.get_vhost(id).await
    }

    pub async fn delete_vhost(&self, name: String) -> Result<(), RabbitMqClientError> {
        self.rabbitmq.apis.vhosts.delete_vhost(name).await
    }
}

fn test_config() -> RabbitMqConfiguration {
    RabbitMqConfiguration {
        rabbitmq_api_url: RABBITMQ_API_URL.to_string(),
        rabbitmq_username: RABBITMQ_USERNAME.to_string(),
        rabbitmq_password: RABBITMQ_PASSWORD.to_string(),
    }
}
