<div align="center">
 <img src="https://raw.githubusercontent.com/stefandanaita/rabbitmq-management-client/main/logo.webp" width="300" style="border-radius: 10%" />
</div>

<br />

[![Crates.io](https://img.shields.io/crates/v/rabbitmq-management-client.svg)](https://crates.io/crates/rabbitmq-management-client)
[![CI](https://github.com/stefandanaita/rabbitmq-management-client/workflows/CI/badge.svg)](https://github.com/stefandanaita/rabbitmq-management-client/actions)

# RabbitMQ Management Client

A comprehensive Rust client for the RabbitMQ Management API, providing async/await support for managing RabbitMQ clusters, virtual hosts, users, queues, exchanges, and more.

## Features

- **Async/await support** with `tokio`
- **Full API coverage** for RabbitMQ Management HTTP API
- **Type-safe** request/response handling with `serde`
- **Pagination support** for large datasets
- **Sorting and filtering** options
- **Authentication middleware** with automatic credential handling
- **Comprehensive error handling**
- **Custom HTTP client** support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rabbitmq-management-client = "0.5.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use rabbitmq_management_client::{
    RabbitMqClientBuilder, 
    config::RabbitMqConfiguration,
    api::{queue::QueueApi, exchange::ExchangeApi, overview::OverviewApi}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the client
    let config = RabbitMqConfiguration {
        rabbitmq_api_url: "http://localhost:15672/api".to_string(),
        rabbitmq_username: "guest".to_string(),
        rabbitmq_password: "guest".to_string(),
    };

    // Build the client
    let client = RabbitMqClientBuilder::new(config).build()?;

    // Get cluster overview
    let overview = client.get_overview().await?;
    println!("RabbitMQ Version: {}", overview.rabbitmq_version);
    println!("Cluster Name: {}", overview.cluster_name);

    Ok(())
}
```

## API Reference

### Overview API

Get cluster information and statistics:

```rust
use rabbitmq_management_client::api::overview::OverviewApi;

// Get cluster overview
let overview = client.get_overview().await?;

// Get cluster name
let cluster_name = client.get_cluster_name().await?;

// Set cluster name
client.set_cluster_name(RabbitMqClusterName {
    name: "my-cluster".to_string()
}).await?;
```

### Virtual Hosts API

Manage virtual hosts:

```rust
use rabbitmq_management_client::api::vhost::{VhostApi, RabbitMqVhostRequest};

// List all virtual hosts
let vhosts = client.list_vhosts().await?;

// Get specific virtual host
let vhost = client.get_vhost("my-vhost".to_string()).await?;

// Create virtual host
client.create_vhost(
    "new-vhost".to_string(),
    RabbitMqVhostRequest {
        description: Some("My new virtual host".to_string()),
        tags: Some("production".to_string()),
        ..Default::default()
    }
).await?;

// Delete virtual host
client.delete_vhost("old-vhost".to_string()).await?;
```

### Queue API

Manage queues with full CRUD operations:

```rust
use rabbitmq_management_client::api::{
    queue::{QueueApi, RabbitMqQueueRequest},
    RabbitMqRequestOptions, RabbitMqPagination
};

// List queues with pagination
let options = RabbitMqRequestOptions {
    pagination: Some(RabbitMqPagination {
        page: 1,
        page_size: 10,
        ..Default::default()
    }),
    ..Default::default()
};
let queues = client.list_queues(Some("/".to_string()), Some(options)).await?;

// Get specific queue
let queue = client.get_queue("/".to_string(), "my-queue".to_string()).await?;

// Create queue
client.create_queue(
    "/".to_string(),
    "new-queue".to_string(),
    RabbitMqQueueRequest {
        durable: Some(true),
        auto_delete: Some(false),
        arguments: Some(std::collections::HashMap::new()),
        ..Default::default()
    }
).await?;

// Delete queue
client.delete_queue("/".to_string(), "old-queue".to_string()).await?;

// Purge queue
client.purge_queue("/".to_string(), "my-queue".to_string()).await?;
```

### Exchange API

Manage exchanges:

```rust
use rabbitmq_management_client::api::{
    exchange::{ExchangeApi, RabbitMqExchangeRequest, RabbitMqExchangeType}
};

// List exchanges
let exchanges = client.list_exchanges(Some("/".to_string()), None).await?;

// Create exchange
client.create_exchange(
    "/".to_string(),
    "my-exchange".to_string(),
    RabbitMqExchangeRequest {
        exchange_type: RabbitMqExchangeType::Direct,
        durable: Some(true),
        auto_delete: Some(false),
        internal: Some(false),
        arguments: Some(std::collections::HashMap::new()),
    }
).await?;

// Get exchange bindings
let bindings = client.list_source_bindings(
    "/".to_string(), 
    "my-exchange".to_string()
).await?;
```

### User Management API

Manage RabbitMQ users:

```rust
use rabbitmq_management_client::api::user::{UserApi, RabbitMqUserRequest, RabbitMqUserTag};

// List users
let users = client.list_users().await?;

// Create user
client.create_user(
    "newuser".to_string(),
    RabbitMqUserRequest {
        password: Some("password123".to_string()),
        tags: Some(vec![RabbitMqUserTag::Management]),
        ..Default::default()
    }
).await?;

// Update user
client.update_user(
    "newuser".to_string(),
    RabbitMqUserRequest {
        tags: Some(vec![RabbitMqUserTag::Administrator]),
        ..Default::default()
    }
).await?;

// Delete user
client.delete_user("olduser".to_string()).await?;
```

### Permissions API

Manage user permissions:

```rust
use rabbitmq_management_client::api::permission::{PermissionApi, RabbitMqPermissionRequest};

// List permissions
let permissions = client.list_permissions().await?;

// Set user permissions
client.create_permission(
    "/".to_string(),
    "username".to_string(),
    RabbitMqPermissionRequest {
        configure: ".*".to_string(),
        write: ".*".to_string(),
        read: ".*".to_string(),
    }
).await?;
```

### Bindings API

Manage queue and exchange bindings:

```rust
use rabbitmq_management_client::api::binding::{BindingApi, RabbitMqBindingRequest};

// Create binding
client.create_binding(
    "/".to_string(),
    "my-exchange".to_string(),
    "my-queue".to_string(),
    RabbitMqBindingRequest {
        routing_key: Some("my.routing.key".to_string()),
        arguments: Some(std::collections::HashMap::new()),
    }
).await?;
```

### Message API

Publish and get messages:

```rust
use rabbitmq_management_client::api::message::{MessageApi, RabbitMqMessageRequest};

// Publish message
client.publish_message(
    "/".to_string(),
    "my-exchange".to_string(),
    RabbitMqMessageRequest {
        routing_key: Some("my.routing.key".to_string()),
        payload: "Hello, World!".to_string(),
        payload_encoding: "string".to_string(),
        properties: Some(std::collections::HashMap::new()),
        ..Default::default()
    }
).await?;

// Get messages from queue
let messages = client.get_messages(
    "/".to_string(),
    "my-queue".to_string(),
    Some(10), // count
    Some(false), // requeue
    Some("auto".to_string()), // encoding
    Some(30) // truncate
).await?;
```

### Node API

Monitor cluster nodes:

```rust
use rabbitmq_management_client::api::node::NodeApi;

// List all nodes
let nodes = client.list_nodes().await?;

// Get specific node
let node = client.get_node("rabbit@hostname".to_string()).await?;
```

## Advanced Usage

### Custom HTTP Client

You can provide your own HTTP client with custom middleware:

```rust
use reqwest_middleware::ClientBuilder;

let custom_client = ClientBuilder::new(reqwest::Client::new())
    // Add your custom middleware here
    .build();

let client = RabbitMqClientBuilder::new(config)
    .preset_client(custom_client)
    .build()?;
```

### Pagination and Sorting

Many list operations support pagination and sorting:

```rust
use rabbitmq_management_client::api::{
    RabbitMqRequestOptions, 
    RabbitMqPagination, 
    RabbitMqSorting,
    RabbitMqPaginationFilter
};

let options = RabbitMqRequestOptions {
    pagination: Some(RabbitMqPagination {
        page: 1,
        page_size: 50,
        filter: Some(RabbitMqPaginationFilter {
            name: Some("test".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }),
    sorting: Some(RabbitMqSorting {
        sort: Some("name".to_string()),
        sort_reverse: Some(false),
    }),
    disable_stats: Some(true), // Improve performance by disabling stats
};

let queues = client.list_queues(None, Some(options)).await?;
```

### Error Handling

The library provides comprehensive error types:

```rust
use rabbitmq_management_client::errors::RabbitMqClientError;

match client.get_queue("/".to_string(), "nonexistent".to_string()).await {
    Ok(queue) => println!("Queue found: {:?}", queue),
    Err(RabbitMqClientError::NotFound) => println!("Queue not found"),
    Err(RabbitMqClientError::Unauthorized) => println!("Invalid credentials"),
    Err(RabbitMqClientError::NetworkError(e)) => println!("Network error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Configuration

The `RabbitMqConfiguration` struct supports environment-based configuration:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    #[serde(flatten)]
    rabbitmq: RabbitMqConfiguration,
}

// Load from environment variables or config file
let config: Config = envy::from_env()?; // Using envy crate
let client = RabbitMqClientBuilder::new(config.rabbitmq).build()?;
```

## Testing

The library includes comprehensive integration tests that run against a real RabbitMQ instance. To run tests:

```bash
# Start RabbitMQ with management plugin
docker-compose -f tests/docker-compose.yaml up -d

# Run tests
cargo test

# Clean up
docker-compose -f tests/docker-compose.yaml down
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please read our [Code of Conduct](CODE_OF_CONDUCT.md) and check the [issues](https://github.com/stefandanaita/rabbitmq-management-client/issues) page for ways to contribute.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed list of changes in each release.