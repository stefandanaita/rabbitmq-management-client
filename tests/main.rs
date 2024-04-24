use crate::context::TestContext;
use rabbitmq_management_client::api::overview::RabbitMqClusterName;

pub mod context;
mod vhosts;

#[tokio::test]
async fn can_get_cluster_overview() {
    let ctx = TestContext::new();

    let overview = ctx.rabbitmq.apis.overview.get_overview().await.unwrap();

    assert_eq!(overview.cluster_name, "rabbit@rabbitmq-management-client");
}

#[tokio::test]
async fn can_set_cluster_name() {
    let ctx = TestContext::new();

    // Get original name
    let original_name = ctx
        .rabbitmq
        .apis
        .overview
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name");
    assert_eq!(&original_name.name, "rabbit@rabbitmq-management-client");

    // Change the cluster name
    ctx.rabbitmq
        .apis
        .overview
        .set_cluster_name(RabbitMqClusterName {
            name: "test@rabbitmq-management-client".to_string(),
        })
        .await
        .expect("failed to set the cluster name");

    // Get the new name
    let new_name = ctx
        .rabbitmq
        .apis
        .overview
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name after change");
    assert_eq!(new_name.name, "test@rabbitmq-management-client");

    // Reset the name
    ctx.rabbitmq
        .apis
        .overview
        .set_cluster_name(original_name)
        .await
        .expect("failed to set the cluster name");
    let reset_name = ctx
        .rabbitmq
        .apis
        .overview
        .get_cluster_name()
        .await
        .expect("failed to get the cluster name after reset");
    assert_eq!(reset_name.name, "rabbit@rabbitmq-management-client");
}

#[tokio::test]
async fn can_list_nodes() {
    let ctx = TestContext::new();

    let nodes = ctx
        .rabbitmq
        .apis
        .nodes
        .list_nodes()
        .await
        .expect("failed to get the list of nodes");

    assert_eq!(nodes.len(), 1);
    assert_eq!(
        nodes.first().unwrap().name,
        "rabbit@rabbitmq-management-client"
    );
}
