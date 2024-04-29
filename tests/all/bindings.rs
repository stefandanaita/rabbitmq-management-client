use rabbitmq_management_client::api::binding::{RabbitMqBindingDestinationType, RabbitMqBindingRequest};
use rabbitmq_management_client::api::exchange::RabbitMqExchangeRequest;
use crate::context::TestContext;

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
    let binding = ctx.rabbitmq
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
    let bindings = ctx.rabbitmq
        .apis
        .bindings
        .list_bindings(Some(vhost.name.clone()))
        .await
        .expect("failed to list bindings");

    // Find the binding and assert
    let filtered = bindings.iter().find(|b| {
        binding.eq(format!("{}/{}", b.destination, b.properties_key).as_str())
    });
    assert!(filtered.is_some());

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}
