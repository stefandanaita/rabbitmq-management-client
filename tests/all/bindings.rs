use crate::context::TestContext;
use rabbitmq_management_client::api::binding::{
    RabbitMqBindingDestinationType, RabbitMqBindingRequest,
};
use rabbitmq_management_client::api::exchange::RabbitMqExchangeRequest;
use rabbitmq_management_client::api::queue::RabbitMqQueueRequest;
use rabbitmq_management_client::errors::RabbitMqClientError;
use std::collections::HashMap;

#[tokio::test]
async fn can_create_and_list_bindings() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Create another exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange2".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Bind the exchanges together
    let binding_id = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            RabbitMqBindingRequest {
                routing_key: Some("test-routing".to_string()),
                arguments: None,
            },
        )
        .await
        .expect("failed to create binding");

    // List bindings
    let bindings = ctx
        .rabbitmq
        .apis
        .bindings
        .list_bindings(Some(vhost.name.clone()))
        .await
        .expect("failed to list bindings");

    // Find the binding and assert
    let filtered = bindings
        .iter()
        .find(|b| binding_id.eq(format!("{}/{}", b.destination, b.properties_key).as_str()));
    assert!(filtered.is_some());

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_filter_bindings() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Create a queue
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: true,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create exchange");

    // Bind the exchanges together
    let binding_id = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "test-exchange".to_string(),
            "test-queue".to_string(),
            RabbitMqBindingDestinationType::Queue,
            RabbitMqBindingRequest {
                routing_key: Some("test-queue-routing".to_string()),
                arguments: Some(HashMap::from([("foo".to_string(), "bar".to_string())])),
            },
        )
        .await
        .expect("failed to create binding");

    // List bindings
    let bindings = ctx
        .rabbitmq
        .apis
        .bindings
        .filter_bindings(
            vhost.name.clone(),
            "test-exchange".to_string(),
            "test-queue".to_string(),
            RabbitMqBindingDestinationType::Queue,
        )
        .await
        .expect("failed to list bindings");

    // Find the binding and assert
    let filtered = bindings
        .iter()
        .find(|b| binding_id.eq(format!("{}/{}", b.destination, b.properties_key).as_str()));
    assert!(filtered.is_some());

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_get_and_delete_binding() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Create another exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange2".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Bind the exchanges together
    let binding_id = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            RabbitMqBindingRequest {
                routing_key: Some("test-routing".to_string()),
                arguments: None,
            },
        )
        .await
        .expect("failed to create binding");

    let binding = ctx
        .rabbitmq
        .apis
        .bindings
        .get_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            binding_id.clone().split("/").last().unwrap().to_string(),
        )
        .await
        .expect("failed to get the binding");

    assert_eq!(binding.routing_key, "test-routing");

    // Delete the binding
    ctx.rabbitmq
        .apis
        .bindings
        .delete_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            binding_id.split("/").last().unwrap().to_string(),
        )
        .await
        .expect("failed to delete binding");

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn errors_not_found() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    let result = ctx
        .rabbitmq
        .apis
        .bindings
        .get_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            "does_not_exist".to_string(),
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_create_binding_if_source_exchange_not_found() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Bind the exchanges together
    let result = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "does_not_exist".to_string(),
            "test-exchange".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            RabbitMqBindingRequest {
                routing_key: Some("test-routing".to_string()),
                arguments: None,
            },
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_create_binding_if_destination_exchange_not_found() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    let result = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "test-exchange".to_string(),
            "does_not_exist".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            RabbitMqBindingRequest {
                routing_key: Some("test-routing".to_string()),
                arguments: None,
            },
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_create_binding_if_destination_queue_not_found() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    let result = ctx
        .rabbitmq
        .apis
        .bindings
        .create_binding(
            vhost.name.clone(),
            "test-exchange".to_string(),
            "does_not_exist".to_string(),
            RabbitMqBindingDestinationType::Queue,
            RabbitMqBindingRequest {
                routing_key: Some("test-routing".to_string()),
                arguments: None,
            },
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_delete_binding_not_found() {
    let ctx = TestContext::new();

    // Create the vhost
    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create an exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    // Create another exchange
    ctx.rabbitmq
        .apis
        .exchanges
        .create_exchange(
            vhost.name.clone(),
            "test-exchange2".to_string(),
            RabbitMqExchangeRequest {
                kind: "direct".to_string(),
                auto_delete: true,
                durable: false,
                internal: false,
            },
        )
        .await
        .expect("failed to create exchange");

    let result = ctx
        .rabbitmq
        .apis
        .bindings
        .delete_binding(
            vhost.name.clone(),
            "test-exchange1".to_string(),
            "test-exchange2".to_string(),
            RabbitMqBindingDestinationType::Exchange,
            "does_not_exist".to_string(),
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}
