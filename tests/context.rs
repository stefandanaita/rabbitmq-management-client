use rabbitmq_management_client::config::RabbitMqConfiguration;
use rabbitmq_management_client::{RabbitMqClient, RabbitMqClientBuilder};

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
}

fn test_config() -> RabbitMqConfiguration {
    RabbitMqConfiguration {
        rabbitmq_api_url: RABBITMQ_API_URL.to_string(),
        rabbitmq_username: RABBITMQ_USERNAME.to_string(),
        rabbitmq_password: RABBITMQ_PASSWORD.to_string(),
    }
}
