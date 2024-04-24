use crate::context::TestContext;
use rabbitmq_management_client::api::vhost::RabbitMqVhostRequest;
use rabbitmq_management_client::errors::RabbitMqClientError;

#[tokio::test]
async fn can_list_vhosts() {
    let ctx = TestContext::new();

    let vhosts = ctx
        .rabbitmq
        .apis
        .vhosts
        .list_vhosts()
        .await
        .expect("failed to list vhosts");

    let default_vhost = vhosts
        .into_iter()
        .find(|vh| vh.name == "/")
        .expect("could not find the default vhost");
    assert_eq!(default_vhost.description, "Default virtual host");
}

#[tokio::test]
async fn can_crud_vhost() {
    let ctx = TestContext::new();

    // Create the vhost
    ctx.rabbitmq
        .apis
        .vhosts
        .create_vhost(RabbitMqVhostRequest {
            name: "test-vhost".to_string(),
            description: Some("testing vhost".to_string()),
            tags: vec!["test1".to_string(), "test2".to_string()],
            tracing: true,
        })
        .await
        .expect("failed to create vhost");

    // Get the newly created vhost
    let new_vhost = ctx
        .rabbitmq
        .apis
        .vhosts
        .get_vhost("test-vhost".to_string())
        .await
        .expect("failed to get the newly created vhost");

    assert_eq!(new_vhost.name, "test-vhost");
    assert_eq!(new_vhost.description, "testing vhost");
    assert_eq!(
        new_vhost.tags,
        vec!["test1".to_string(), "test2".to_string(),]
    );
    assert!(new_vhost.tracing);

    // Delete the vhost
    ctx.rabbitmq
        .apis
        .vhosts
        .delete_vhost("test-vhost".to_string())
        .await
        .expect("failed to delete the newly created vhost");

    // Should fail to get the vhost again
    let deleted_vhost = ctx
        .rabbitmq
        .apis
        .vhosts
        .get_vhost("test-vhost".to_string())
        .await;

    assert!(matches!(
        deleted_vhost,
        Err(RabbitMqClientError::NotFound(_))
    ));
}

#[tokio::test]
async fn cannot_create_if_vhosts_exists() {
    let ctx = TestContext::new();

    let result = ctx
        .rabbitmq
        .apis
        .vhosts
        .create_vhost(RabbitMqVhostRequest {
            name: "/".to_string(),
            description: None,
            tags: vec![],
            tracing: false,
        })
        .await;

    assert!(matches!(result, Err(RabbitMqClientError::AlreadyExists(_))));
}
