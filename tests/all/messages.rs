use crate::context::TestContext;
use rabbitmq_management_client::api::binding::{
    BindingApi, RabbitMqBindingDestinationType, RabbitMqBindingRequest,
};
use rabbitmq_management_client::api::exchange::{ExchangeApi, RabbitMqExchangeRequest};
use rabbitmq_management_client::api::message::{
    MessageApi, RabbitMqGetMessagesAckMode, RabbitMqGetMessagesEncoding,
    RabbitMqGetMessagesOptions, RabbitMqMessageEncoding, RabbitMqPublishMessageRequest,
};
use rabbitmq_management_client::api::queue::{QueueApi, RabbitMqQueueRequest};
use std::collections::HashMap;

#[tokio::test]
async fn can_publish_message_to_exchange() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create the exchange
    ctx.rabbitmq
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

    // Publish message to exchange
    let published = ctx
        .rabbitmq
        .publish_message(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqPublishMessageRequest {
                properties: Default::default(),
                routing_key: "test-queue-routing".to_string(),
                payload: "first-message".to_string(),
                payload_encoding: RabbitMqMessageEncoding::String,
            },
        )
        .await
        .expect("failed to publish the message");

    // There is no queue bound to the exchange,
    // therefore the message won't get routed.
    assert!(!published.routed);

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_consume_messages_from_queue() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    // Create the exchange
    ctx.rabbitmq
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

    // Publish message to exchange
    let published = ctx
        .rabbitmq
        .publish_message(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqPublishMessageRequest {
                properties: Default::default(),
                routing_key: "test-queue-routing".to_string(),
                payload: "first-message".to_string(),
                payload_encoding: RabbitMqMessageEncoding::String,
            },
        )
        .await
        .expect("failed to publish the message");

    assert!(published.routed);

    ctx.rabbitmq
        .publish_message(
            vhost.name.clone(),
            "test-exchange".to_string(),
            RabbitMqPublishMessageRequest {
                properties: Default::default(),
                routing_key: "test-queue-routing".to_string(),
                payload: "second-message".to_string(),
                payload_encoding: RabbitMqMessageEncoding::String,
            },
        )
        .await
        .expect("failed to publish the message");

    // Read the message from the queue
    let messages = ctx
        .rabbitmq
        .get_messages(
            vhost.name.clone(),
            "test-queue".to_string(),
            RabbitMqGetMessagesOptions {
                count: 5,
                ack_mode: RabbitMqGetMessagesAckMode::AckRequeueTrue,
                encoding: RabbitMqGetMessagesEncoding::Auto,
                truncate: None,
            },
        )
        .await
        .expect("failed to consume the message");

    assert_eq!(messages.len(), 2);

    let first_message = messages.first().unwrap();
    assert_eq!(first_message.routing_key, "test-queue-routing");
    assert_eq!(first_message.payload, "first-message");

    let second_message = messages.last().unwrap();
    assert_eq!(second_message.routing_key, "test-queue-routing");
    assert_eq!(second_message.payload, "second-message");

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}
