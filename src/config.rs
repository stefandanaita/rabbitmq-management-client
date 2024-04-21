use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqConfiguration {
    pub rabbitmq_api_url: String,
    pub rabbitmq_username: String,
    pub rabbitmq_password: String,
}
