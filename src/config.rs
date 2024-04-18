#[derive(Debug, Clone)]
pub struct RabbitMqClientConfig {
    pub rabbitmq_api_url: String,
    pub rabbitmq_username: String,
    pub rabbitmq_password: String,
}
