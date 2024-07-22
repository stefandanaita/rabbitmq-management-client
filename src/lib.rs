use crate::{
    config::RabbitMqConfiguration, errors::RabbitMqClientError,
    middlewares::authentication::AuthenticationMiddleware,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub mod api;
pub mod config;
pub mod errors;
mod middlewares;

#[derive(Clone)]
pub struct RabbitMqClient {
    pub api_url: String,
    pub client: ClientWithMiddleware,
}

pub struct RabbitMqClientBuilder {
    config: RabbitMqConfiguration,
    preset_client: Option<ClientWithMiddleware>,
}

impl RabbitMqClientBuilder {
    pub fn new(config: RabbitMqConfiguration) -> Self {
        Self {
            config,
            preset_client: None,
        }
    }

    pub fn preset_client(mut self, client: ClientWithMiddleware) -> Self {
        self.preset_client = Some(client);
        self
    }

    #[tracing::instrument(skip(self))]
    pub fn build(self) -> Result<RabbitMqClient, RabbitMqClientError> {
        let client_builder = match self.preset_client {
            None => ClientBuilder::new(reqwest::Client::new()),
            Some(c) => ClientBuilder::from_client(c),
        };

        if self.config.rabbitmq_username.is_empty() || self.config.rabbitmq_password.is_empty() {
            return Err(RabbitMqClientError::MissingCredentials);
        }

        let client = client_builder
            .with(AuthenticationMiddleware {
                username: self.config.rabbitmq_username,
                password: self.config.rabbitmq_password,
            })
            .build();

        Ok(RabbitMqClient {
            api_url: self.config.rabbitmq_api_url,
            client,
        })
    }
}
