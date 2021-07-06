use crate::get_conn_memory;
use crate::user;

pub async fn insert_user(conn: &crate::DbConnection) -> String {
    let user = user::InsertableUser {
        username: "test".into(),
        password: "test".into(),
        roles: vec!["User".into()],
        prefs: Default::default(),
    };

    user.insert(conn).await.unwrap()
}

pub async fn insert_many(conn: &crate::DbConnection, n: usize) {
    for i in 0..n {
        let user = user::InsertableUser {
            username: format!("test{}", i),
            password: "test".into(),
            roles: vec!["User".into()],
            prefs: Default::default(),
        };

        user.insert(conn).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let ref conn = get_conn_memory().await.unwrap();

    let result = user::User::get_one(conn, "test".into(), "test".into()).await;
    assert!(result.is_err());

    let uname = insert_user(conn).await;
    let result = user::User::get_one(conn, uname, "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());
    assert_eq!(&result.roles, &["User".to_string()]);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let ref conn = get_conn_memory().await.unwrap();

    let result = user::User::get_all(conn).await.unwrap();
    assert!(result.is_empty());

    insert_many(conn, 10).await;

    let result = user::User::get_all(conn).await.unwrap();
    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let ref conn = get_conn_memory().await.unwrap();
    let uname = insert_user(conn).await;
    let result = user::User::get_one(conn, uname.clone(), "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());

    let rows = user::User::delete(conn, uname.clone()).await.unwrap();
    assert_eq!(rows, 1);

    let result = user::User::get_one(conn, uname, "test".into()).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invites() {
    let ref conn = get_conn_memory().await.unwrap();

    let result = user::Login::get_all_invites(conn).await.unwrap();
    assert!(result.is_empty());

    let invite = user::Login::new_invite(conn).await.unwrap();
    let result = user::Login::get_all_invites(conn).await.unwrap();
    assert_eq!(&result, &[invite.clone()]);

    let result = user::Login {
        invite_token: Some(invite.clone()),
        ..Default::default()
    }
    .invite_token_valid(conn)
    .await
    .unwrap();
    assert!(result);

    let result = user::Login {
        invite_token: Some("TESTTESTTEST".into()),
        ..Default::default()
    }
    .invite_token_valid(conn)
    .await
    .unwrap();
    assert!(!result);

    let result = user::Login {
        invite_token: Some(invite.clone()),
        ..Default::default()
    }
    .invalidate_token(conn)
    .await
    .unwrap();
    assert_eq!(result, 1);

    let result = user::Login::get_all_invites(conn).await.unwrap();
    assert!(result.is_empty());

    let result = user::Login {
        invite_token: Some(invite),
        ..Default::default()
    }
    .invalidate_token(conn)
    .await
    .unwrap();
    assert_eq!(result, 0);
}
