use crate::context::TestContext;
use rabbitmq_management_client::api::overview::{OverviewApi, RabbitMqClusterName};
use reqwest_middleware::ClientBuilder;

#[tokio::test]
async fn can_get_cluster_overview() {
    let ctx = TestContext::new();

    let overview = ctx.rabbitmq.get_overview().await.unwrap();

    assert_eq!(overview.cluster_name, "rabbit@rabbitmq");
}

#[tokio::test]
async fn can_set_cluster_name() {
    let ctx = TestContext::new();

    // Get original name
    let original_name = ctx
        .rabbitmq
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name");
    assert_eq!(&original_name.name, "rabbit@rabbitmq");

    // Change the cluster name
    ctx.rabbitmq
        .set_cluster_name(RabbitMqClusterName {
            name: "test@rabbitmq".to_string(),
        })
        .await
        .expect("failed to set the cluster name");

    // Get the new name
    let new_name = ctx
        .rabbitmq
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name after change");
    assert_eq!(new_name.name, "test@rabbitmq");

    // Reset the name
    ctx.rabbitmq
        .set_cluster_name(original_name)
        .await
        .expect("failed to set the cluster name");
    let reset_name = ctx
        .rabbitmq
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name after reset");
    assert_eq!(reset_name.name, "rabbit@rabbitmq");
}

#[tokio::test]
async fn uses_preset_client() {
    let client = ClientBuilder::new(reqwest::Client::new()).build();

    let ctx = TestContext::new_with_preset_client(client);

    let cluster_name = ctx
        .rabbitmq
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name");
    assert_eq!(&cluster_name.name, "rabbit@rabbitmq");
}
