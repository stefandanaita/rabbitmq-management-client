use crate::context::TestContext;
use rabbitmq_management_client::api::user::{
    RabbitMqUserCreateRequest, RabbitMqUserTag, RabbitMqUsersBulkDeleteRequest, UserApi,
};
use rabbitmq_management_client::errors::RabbitMqClientError;
use uuid::Uuid;

#[tokio::test]
async fn can_list_users() {
    let ctx = TestContext::new();

    let users = ctx
        .rabbitmq
        .list_users()
        .await
        .expect("failed to list users");

    assert!(!users.is_empty());
}

#[tokio::test]
async fn can_crud_users() {
    let ctx = TestContext::new();

    let admin_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: admin_uuid.clone().to_string(),
            password: Some("fake_admin_password".to_string()),
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![RabbitMqUserTag::Administrator],
        })
        .await
        .expect("failed to create admin user");

    let admin = ctx
        .rabbitmq
        .get_user(admin_uuid.clone().to_string())
        .await
        .expect("failed to get admin user");

    assert_eq!(admin.name, admin_uuid.clone().to_string());
    assert!(admin.tags.contains(&RabbitMqUserTag::Administrator));

    ctx.rabbitmq
        .delete_user(admin_uuid.clone().to_string())
        .await
        .expect("failed to delete admin user");

    // Getting the admin user should error
    let deleted_admin = ctx.rabbitmq.get_user(admin_uuid.to_string()).await;

    assert!(matches!(
        deleted_admin,
        Err(RabbitMqClientError::NotFound(_))
    ));

    let mnm_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: mnm_uuid.clone().to_string(),
            password: None,
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![RabbitMqUserTag::Management, RabbitMqUserTag::Monitoring],
        })
        .await
        .expect("failed to create the management & monitoring user");

    let mnm = ctx
        .rabbitmq
        .get_user(mnm_uuid.clone().to_string())
        .await
        .expect("failed to get mnm user");

    assert_eq!(mnm.name, mnm_uuid.clone().to_string());
    assert!(mnm.tags.contains(&RabbitMqUserTag::Management));
    assert!(mnm.tags.contains(&RabbitMqUserTag::Monitoring));

    ctx.rabbitmq
        .delete_user(mnm_uuid.clone().to_string())
        .await
        .expect("failed to delete the management & monitoring user");

    // Getting the admin user should error
    let deleted_mnm = ctx.rabbitmq.get_user(mnm_uuid.to_string()).await;

    assert!(matches!(deleted_mnm, Err(RabbitMqClientError::NotFound(_))));
}

#[tokio::test]
async fn can_create_empty_password_hash_user() {
    let ctx = TestContext::new();

    let user_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: user_uuid.clone().to_string(),
            password: None,
            password_hash: Some("".to_string()),
            hashing_algorithm: None,
            tags: vec![RabbitMqUserTag::Administrator],
        })
        .await
        .expect("failed to create user");

    let user = ctx
        .rabbitmq
        .get_user(user_uuid.clone().to_string())
        .await
        .expect("failed to get user");

    assert_eq!(user.name, user_uuid.clone().to_string());
    assert_eq!(user.password_hash, "");

    ctx.rabbitmq
        .delete_user(user_uuid.to_string())
        .await
        .expect("failed to delete user");
}

#[tokio::test]
async fn can_bulk_delete_users() {
    let ctx = TestContext::new();

    let original_users = ctx
        .rabbitmq
        .list_users()
        .await
        .expect("failed to list users");
    dbg!(&original_users);

    let foo_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: foo_uuid.clone().to_string(),
            password: None,
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![],
        })
        .await
        .expect("failed to create user");

    let bar_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: bar_uuid.clone().to_string(),
            password: None,
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![],
        })
        .await
        .expect("failed to create user");

    let baz_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: baz_uuid.clone().to_string(),
            password: None,
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![],
        })
        .await
        .expect("failed to create user");

    let qux_uuid = Uuid::new_v4();
    ctx.rabbitmq
        .create_user(RabbitMqUserCreateRequest {
            name: qux_uuid.clone().to_string(),
            password: None,
            password_hash: None,
            hashing_algorithm: None,
            tags: vec![],
        })
        .await
        .expect("failed to create user");

    let new_users = ctx
        .rabbitmq
        .list_users()
        .await
        .expect("failed to list users");
    dbg!(&new_users);

    assert_eq!(original_users.len(), new_users.len() - 4);

    ctx.rabbitmq
        .bulk_delete_users(RabbitMqUsersBulkDeleteRequest {
            users: vec![
                foo_uuid.to_string(),
                bar_uuid.to_string(),
                baz_uuid.to_string(),
                qux_uuid.to_string(),
            ],
        })
        .await
        .expect("failed to bulk delete users");

    let after_delete_users = ctx
        .rabbitmq
        .list_users()
        .await
        .expect("failed to list users");

    assert_eq!(original_users.len(), after_delete_users.len());
}
