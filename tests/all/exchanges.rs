use crate::context::TestContext;
use rabbitmq_management_client::api::exchange::{ExchangeApi, RabbitMqExchangeRequest};
use rabbitmq_management_client::api::{RabbitMqPagination, RabbitMqPaginationFilter};
use rabbitmq_management_client::errors::RabbitMqClientError;

#[tokio::test]
async fn can_list_exchanges() {
    let ctx = TestContext::new();

    let exchanges = ctx
        .rabbitmq
        .list_exchanges(None, None)
        .await
        .expect("failed to list exchanges");

    assert!(!exchanges.items.is_empty());
}

#[tokio::test]
async fn can_list_exchanges_paginated() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    for i in 0..20 {
        ctx.rabbitmq
            .create_exchange(
                vhost.name.clone(),
                format!("test-exchange_{i}"),
                RabbitMqExchangeRequest {
                    kind: "direct".to_string(),
                    auto_delete: true,
                    durable: false,
                    internal: false,
                },
            )
            .await
            .expect("failed to create exchange");
    }

    let exchanges = ctx
        .rabbitmq
        .list_exchanges(
            Some(vhost.name.clone()),
            Some(RabbitMqPagination {
                page: 1,
                page_size: Some(5),
                filter: None,
            }),
        )
        .await
        .expect("failed to list exchanges");

    assert_eq!(exchanges.items.len(), 5);

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_filter_exchanges() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    for i in 0..5 {
        ctx.rabbitmq
            .create_exchange(
                vhost.name.clone(),
                format!("test-exchange_{i}"),
                RabbitMqExchangeRequest {
                    kind: "direct".to_string(),
                    auto_delete: true,
                    durable: false,
                    internal: false,
                },
            )
            .await
            .expect("failed to create exchange");
    }

    let exchanges = ctx
        .rabbitmq
        .list_exchanges(
            Some(vhost.name.clone()),
            Some(RabbitMqPagination {
                page: 1,
                page_size: None,
                filter: Some(RabbitMqPaginationFilter::StringFilter(
                    "test-exchange_3".to_string(),
                )),
            }),
        )
        .await
        .expect("failed to list exchanges");

    assert_eq!(exchanges.items.len(), 1);

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_regex_filter_exchanges() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    for i in 0..5 {
        ctx.rabbitmq
            .create_exchange(
                vhost.name.clone(),
                format!("test-exchange_{i}"),
                RabbitMqExchangeRequest {
                    kind: "direct".to_string(),
                    auto_delete: true,
                    durable: false,
                    internal: false,
                },
            )
            .await
            .expect("failed to create exchange");
    }

    let exchanges = ctx
        .rabbitmq
        .list_exchanges(
            Some(vhost.name.clone()),
            Some(RabbitMqPagination {
                page: 1,
                page_size: None,
                filter: Some(RabbitMqPaginationFilter::RegexFilter(
                    "(test-exchange_3$|test-exchange_0$)".to_string(),
                )),
            }),
        )
        .await
        .expect("failed to list exchanges");

    assert_eq!(exchanges.items.len(), 2);

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn can_crud_exchange() {
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

    // Get the exchange
    let exchange = ctx
        .rabbitmq
        .get_exchange(vhost.name.clone(), "test-exchange".to_string())
        .await
        .expect("failed to get the exchange back");

    assert_eq!(exchange.kind, "direct");
    assert!(exchange.auto_delete);
    assert!(!exchange.durable);
    assert!(!exchange.internal);

    // Delete the exchange
    ctx.rabbitmq
        .delete_exchange(vhost.name.clone(), "test-exchange".to_string())
        .await
        .expect("failed to delete the exchange");

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn cannot_create_if_exchange_exists() {
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

    // Recreate the exchange
    let result = ctx
        .rabbitmq
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
        .await;
    assert!(matches!(result, Err(RabbitMqClientError::AlreadyExists(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}

#[tokio::test]
async fn returns_not_found() {
    let ctx = TestContext::new();

    let vhost = ctx
        .create_random_vhost()
        .await
        .expect("failed to create vhost");

    let result = ctx
        .rabbitmq
        .delete_exchange(vhost.name.clone(), "doesnotexist".to_string())
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::NotFound(_))));

    ctx.delete_vhost(vhost.name)
        .await
        .expect("failed to delete vhost");
}
