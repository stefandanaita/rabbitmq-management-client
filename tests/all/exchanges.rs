use crate::context::TestContext;

#[tokio::test]
async fn can_list_exchanges() {
    let ctx = TestContext::new();

    let exchanges = ctx
        .rabbitmq
        .apis
        .exchanges
        .list_exchanges(None)
        .await
        .expect("failed to list exchanges");

    assert!(!exchanges.is_empty());
}
