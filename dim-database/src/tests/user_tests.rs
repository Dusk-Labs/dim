use dim_auth::generate_key;
use dim_auth::set_key_fallible;

use crate::get_conn_memory;
use crate::user;
use crate::user::Login;
use crate::user::Roles;
use crate::user::User;
use crate::write_tx;

pub async fn insert_user(conn: &mut crate::Transaction<'_>) -> User {
    let invite = Login::new_invite(&mut *conn).await.unwrap();
    let user = user::InsertableUser {
        username: "test".into(),
        password: "test".into(),
        roles: Roles(vec!["User".into()]),
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
            roles: Roles(vec!["User".into()]),
            prefs: Default::default(),
            claimed_invite: invite,
        };

        user.insert(&mut *conn).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_one() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();

    let result = user::User::authenticate(&mut tx, "test".into(), "test".into()).await;
    assert!(result.is_err());

    let user = insert_user(&mut tx).await;
    assert_eq!(user.username, "test");
    let result = user::User::authenticate(&mut tx, "test".into(), "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());
    assert_eq!(result.roles, Roles(vec!["User".to_string()]));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();

    let result = user::User::get_all(&mut tx).await.unwrap();
    assert!(result.is_empty());

    insert_many(&mut tx, 10).await;

    let result = user::User::get_all(&mut tx).await.unwrap();
    assert_eq!(result.len(), 10);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_delete() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();
    let uname = insert_user(&mut tx).await;
    let result = user::User::authenticate(&mut tx, uname.username.clone(), "test".into())
        .await
        .unwrap();
    assert_eq!(result.username, "test".to_string());

    let rows = user::User::delete(&mut tx, uname.id).await.unwrap();
    assert_eq!(rows, 1);

    let result = user::User::authenticate(&mut tx, uname.username, "test".into()).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invites() {
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();

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

#[tokio::test(flavor = "multi_thread")]
async fn test_cookie_encoding() {
    let _ = set_key_fallible(generate_key());
    let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
    let mut tx = write_tx(&mut conn).await.unwrap();

    let user = insert_user(&mut tx).await;
    let token = Login::create_cookie(user.id);
    let token2 = Login::create_cookie(user.id);
    assert_ne!(token, token2);
    let uid = Login::verify_cookie(token).unwrap();
    assert_eq!(uid, user.id);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invalid_cookie() {
    let _ = set_key_fallible(generate_key());
    let res = Login::verify_cookie(String::new());
    assert!(res.is_err());
    let res = Login::verify_cookie(String::from("ansd9uid89as"));
    assert!(res.is_err());
    let res = Login::verify_cookie(String::from("bXl1c2VyaWQ="));
    assert!(res.is_err());
}
