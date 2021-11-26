use crate::get_conn_memory;
use crate::user;
use crate::user::Login;

pub async fn insert_user(conn: &mut crate::Transaction<'_>) -> String {
    let invite = Login::new_invite(&mut *conn).await.unwrap();
    let user = user::InsertableUser {
        username: "test".into(),
        password: "test".into(),
        roles: vec!["User".into()],
        prefs: Default::default(),
        claimed_invite: invite,
    };

    user.insert(&mut *conn).await.unwrap()
}

pub async fn insert_many(conn: &mut crate::Transaction<'_>, n: usize) {
    for i in 0..n {
        let invite = Login::new_invite(&mut *conn).await.unwrap();
        let user = user::InsertableUser {
            username: format!("test{}", i),
            password: "test".into(),
            roles: vec!["User".into()],
            prefs: Default::default(),
            claimed_invite: invite,
        };

        user.insert(&mut *conn).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let conn = get_conn_memory().await.unwrap().write();
    let mut tx = conn.begin().await.unwrap();

    let result = user::User::get_one(&mut tx, "test".into(), "test".into()).await;
    assert!(result.is_err());

    let uname = insert_user(&mut tx).await;
    let result = user::User::get_one(&mut tx, uname, "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());
    assert_eq!(&result.roles, &["User".to_string()]);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let conn = get_conn_memory().await.unwrap().write();
    let mut tx = conn.begin().await.unwrap();

    let result = user::User::get_all(&mut tx).await.unwrap();
    assert!(result.is_empty());

    insert_many(&mut tx, 10).await;

    let result = user::User::get_all(&mut tx).await.unwrap();
    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let conn = get_conn_memory().await.unwrap().write();
    let mut tx = conn.begin().await.unwrap();
    let uname = insert_user(&mut tx).await;
    let result = user::User::get_one(&mut tx, uname.clone(), "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());

    let rows = user::User::delete(&mut tx, uname.clone()).await.unwrap();
    assert_eq!(rows, 1);

    let result = user::User::get_one(&mut tx, uname, "test".into()).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invites() {
    let conn = get_conn_memory().await.unwrap().write();
    let mut tx = conn.begin().await.unwrap();

    let result = user::Login::get_all_invites(&mut tx).await.unwrap();
    assert!(result.is_empty());

    let invite = user::Login::new_invite(&mut tx).await.unwrap();
    let result = user::Login::get_all_invites(&mut tx).await.unwrap();
    assert_eq!(&result, &[invite.clone()]);

    let result = user::Login {
        invite_token: Some(invite.clone()),
        ..Default::default()
    }
    .invite_token_valid(&mut tx)
    .await
    .unwrap();
    assert!(result);

    let result = user::Login {
        invite_token: Some("TESTTESTTEST".into()),
        ..Default::default()
    }
    .invite_token_valid(&mut tx)
    .await
    .unwrap();
    assert!(!result);

    let result = user::Login {
        invite_token: Some(invite.clone()),
        ..Default::default()
    }
    .invalidate_token(&mut tx)
    .await
    .unwrap();
    assert_eq!(result, 1);

    let result = user::Login::get_all_invites(&mut tx).await.unwrap();
    assert!(result.is_empty());

    let result = user::Login {
        invite_token: Some(invite),
        ..Default::default()
    }
    .invalidate_token(&mut tx)
    .await
    .unwrap();
    assert_eq!(result, 0);
}
