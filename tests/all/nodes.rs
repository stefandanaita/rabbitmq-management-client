use crate::context::TestContext;

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
    assert_eq!(nodes.first().unwrap().name, "rabbit@rabbitmq");
}

#[tokio::test]
async fn can_get_node() {
    let ctx = TestContext::new();

    let node = ctx
        .rabbitmq
        .apis
        .nodes
        .get_node("rabbit@rabbitmq".to_string())
        .await
        .expect("failed to get the list of nodes");

    assert_eq!(node.name, "rabbit@rabbitmq");
    assert!(node.running);
}

#[tokio::test]
async fn can_get_node_memory() {
    let ctx = TestContext::new();

    let memory = ctx
        .rabbitmq
        .apis
        .nodes
        .get_node_memory("rabbit@rabbitmq".to_string())
        .await
        .expect("failed to get the list of nodes");

    assert_eq!(memory.strategy, "rss");
}
