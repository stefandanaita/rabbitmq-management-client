use crate::context::TestContext;
use rabbitmq_management_client::api::binding::{
    RabbitMqBindingDestinationType, RabbitMqBindingRequest,
};
use rabbitmq_management_client::api::exchange::RabbitMqExchangeRequest;
use rabbitmq_management_client::api::queue::{RabbitMqQueueAction, RabbitMqQueueRequest};
use rabbitmq_management_client::errors::RabbitMqClientError;
use std::collections::HashMap;

#[tokio::test]
async fn can_list_queues() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create a couple of queues
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue1".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: false,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create queue1");

    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue2".to_string(),
            RabbitMqQueueRequest {
                auto_delete: true,
                durable: true,
                arguments: Some(HashMap::from([("foo".to_string(), "bar".to_string())])),
                node: None,
            },
        )
        .await
        .expect("failed to create queue2");

    let queues = ctx
        .rabbitmq
        .apis
        .queues
        .list_queues(Some(vhost.name.clone()))
        .await
        .expect("failed to list queues");

    assert_eq!(queues.len(), 2);

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_crud_queue() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create a couple of queues
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: false,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create queue");

    // Get the queue
    let queue = ctx
        .rabbitmq
        .apis
        .queues
        .get_queue(vhost.name.clone(), "test-queue".to_string())
        .await
        .expect("failed to get the queue");

    assert_eq!(queue.name, "test-queue");

    // Delete the queue
    ctx.rabbitmq
        .apis
        .queues
        .delete_queue(vhost.name.clone(), "test-queue".to_string())
        .await
        .expect("failed to delete queue");

    // Getting the queue should error
    let result = ctx
        .rabbitmq
        .apis
        .queues
        .get_queue(vhost.name.clone(), "test-queue".to_string())
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_create_queue_that_already_exists() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create a couple of queues
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: false,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create queue1");

    let result = ctx
        .rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: true,
                durable: true,
                arguments: Some(HashMap::from([("foo".to_string(), "bar".to_string())])),
                node: None,
            },
        )
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::AlreadyExists(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_get_queue_bindings() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create the exchange
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

    // Create the queue
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: false,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create queue1");

    // Bind the exchange and the queue
    ctx.rabbitmq
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

    // Get queue bindings
    let bindings = ctx
        .rabbitmq
        .apis
        .queues
        .get_queue_bindings(vhost.name.clone(), "test-queue".to_string())
        .await
        .expect("failed to get queue bindings");

    assert!(bindings
        .iter()
        .any(|b| b.source == "test-exchange" && b.routing_key == "test-queue-routing"));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_set_queue_action() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create the queue
    ctx.rabbitmq
        .apis
        .queues
        .create_queue(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueRequest {
                auto_delete: false,
                durable: false,
                arguments: None,
                node: None,
            },
        )
        .await
        .expect("failed to create queue1");

    // Set the sync action
    ctx.rabbitmq
        .apis
        .queues
        .set_queue_actions(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueAction::Sync,
        )
        .await
        .expect("failed to set sync action");

    // Set the cancel_sync action
    ctx.rabbitmq
        .apis
        .queues
        .set_queue_actions(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqQueueAction::CancelSync,
        )
        .await
        .expect("failed to set sync action");

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}
