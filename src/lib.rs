use crate::config::RabbitMqClientConfig;
use crate::errors::RabbitMqClientError;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

mod api;
mod config;
mod errors;
mod middlewares;

pub struct RabbitMqClient {
    pub api_url: String,
    pub client: ClientWithMiddleware,
}

pub struct RabbitMqClientBuilder {
    config: RabbitMqClientConfig,
    preset_client: Option<ClientWithMiddleware>,
}

impl RabbitMqClientBuilder {
    pub fn new(config: RabbitMqClientConfig) -> Self {
        Self {
            config,
            preset_client: None,
        }
    }

    pub fn preset_client(mut self, client: ClientWithMiddleware) -> Self {
        self.preset_client = Some(client);
        self
    }

    pub fn build(self) -> Result<RabbitMqClient, RabbitMqClientError> {
        let client: ClientWithMiddleware = self
            .preset_client
            .unwrap_or_else(|| ClientBuilder::new(reqwest::Client::new()).build());

        Ok(RabbitMqClient {
            client,
            api_url: self.config.rabbitmq_api_url,
        })
    }
}
