use crate::core::DbConnection;
use crate::errors;
use auth::{jwt_generate, Wrapper as Auth};

use database::progress::Progress;
use database::user::verify;
use database::user::InsertableUser;
use database::user::Login;
use database::user::User;

use diesel::prelude::*;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use tokio_diesel::*;

#[post("/login", data = "<new_login>")]
pub async fn login(
    conn: State<'_, DbConnection>,
    new_login: Json<Login>,
) -> Result<JsonValue, errors::AuthError> {
    use database::schema::users::dsl::*;
    let uname = new_login.username.clone();
    let user: (String, String, String, String, String) =
        users.filter(username.eq(uname)).first_async(&conn).await?;

    if verify(user.0.clone(), user.1.clone(), new_login.password.clone()) {
        let token = jwt_generate(user.0, user.2.split(",").map(|x| x.to_string()).collect());

        return Ok(json!({
            "token": token,
        }));
    }

    Err(errors::AuthError::WrongPassword)
}

#[get("/whoami")]
pub async fn whoami(conn: State<'_, DbConnection>, user: Auth) -> JsonValue {
    json!({
        "username": user.0.claims.get_user(),
        "picture": "https://i.redd.it/3n1if40vxxv31.png",
        "spentWatching": Progress::get_total_time_spent_watching(&conn, user.0.claims.get_user()).await.unwrap_or(0) / 3600
    })
}

#[get("/admin_exists")]
pub async fn admin_exists(conn: State<'_, DbConnection>) -> Result<JsonValue, errors::DimError> {
    Ok(json!({
        "exists": !User::get_all(&conn).await?.is_empty()
    }))
}

#[post("/register", data = "<new_user>")]
pub async fn register(
    conn: State<'_, DbConnection>,
    new_user: Json<Login>,
) -> Result<JsonValue, errors::AuthError> {
    let users_empty = User::get_all(&conn).await?.is_empty();

    if !users_empty
        && (new_user.invite_token.is_none()
            || !new_user.invite_token_valid(&conn).await.unwrap_or(false))
    {
        return Err(errors::AuthError::NoTokenError);
    }

    let roles = if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
        ..Default::default()
    }
    .insert(&conn)
    .await?;

    if users_empty {
        new_user.invalidate_token(&conn).await?;
    }

    Ok(json!({ "username": res }))
}

#[get("/invites")]
pub async fn get_all_invites(
    conn: State<'_, DbConnection>,
    user: Auth,
) -> Result<JsonValue, errors::AuthError> {
    if user.0.claims.has_role("owner") {
        return Ok(json!({
            "invites": Login::get_all_invites(&conn).await?
        }));
    }

    Err(errors::AuthError::Unauthorized)
}

#[post("/new_invite")]
pub async fn generate_invite(
    conn: State<'_, DbConnection>,
    user: Auth,
) -> Result<JsonValue, errors::AuthError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::AuthError::Unauthorized);
    }

    Ok(json!({
        "token": Login::new_invite(&conn).await?
    }))
}
