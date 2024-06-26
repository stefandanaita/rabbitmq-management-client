use crate::api::binding::BindingApi;
use crate::api::connection::ConnectionApi;
use crate::api::exchange::ExchangeApi;
use crate::api::node::NodeApi;
use crate::api::overview::OverviewApi;
use crate::api::permission::PermissionApi;
use crate::api::policy::PolicyApi;
use crate::api::queue::QueueApi;
use crate::api::user::UserApi;
use crate::api::vhost::VhostApi;
use crate::config::RabbitMqConfiguration;
use crate::errors::RabbitMqClientError;
use crate::middlewares::authentication::AuthenticationMiddleware;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub mod api;
pub mod config;
pub mod errors;
mod middlewares;

#[derive(Clone)]
pub struct RabbitMqClient {
    pub api_url: String,
    pub client: ClientWithMiddleware,
    pub apis: RabbitMqApis,
}

#[derive(Clone)]
pub struct RabbitMqApis {
    pub bindings: BindingApi,
    pub connections: ConnectionApi,
    pub exchanges: ExchangeApi,
    pub nodes: NodeApi,
    pub overview: OverviewApi,
    pub permissions: PermissionApi,
    pub policies: PolicyApi,
    pub queues: QueueApi,
    pub users: UserApi,
    pub vhosts: VhostApi,
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

    pub fn build(self) -> Result<RabbitMqClient, RabbitMqClientError> {
        let client_builder = match self.preset_client {
            None => ClientBuilder::new(reqwest::Client::new()),
            Some(c) => ClientBuilder::from_client(c),
        };

        let client = client_builder
            .with(AuthenticationMiddleware {
                username: self.config.rabbitmq_username,
                password: self.config.rabbitmq_password,
            })
            .build();

        Ok(RabbitMqClient {
            apis: build_apis(self.config.rabbitmq_api_url.clone(), client.clone()),
            api_url: self.config.rabbitmq_api_url,
            client,
        })
    }
}

fn build_apis(url: String, client: ClientWithMiddleware) -> RabbitMqApis {
    RabbitMqApis {
        bindings: BindingApi::new(url.clone(), client.clone()),
        connections: ConnectionApi::new(url.clone(), client.clone()),
        exchanges: ExchangeApi::new(url.clone(), client.clone()),
        nodes: NodeApi::new(url.clone(), client.clone()),
        overview: OverviewApi::new(url.clone(), client.clone()),
        permissions: PermissionApi::new(url.clone(), client.clone()),
        policies: PolicyApi::new(url.clone(), client.clone()),
        queues: QueueApi::new(url.clone(), client.clone()),
        users: UserApi::new(url.clone(), client.clone()),
        vhosts: VhostApi::new(url.clone(), client.clone()),
    }
}
