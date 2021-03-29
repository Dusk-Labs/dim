use crate::core::DbConnection;
use crate::errors;
use auth::{jwt_generate, Wrapper as Auth};

use database::progress::Progress;
use database::user::verify;
use database::user::InsertableUser;
use database::user::Login;
use database::user::User;

use diesel::prelude::*;
use rocket::http::Cookie;
use rocket::http::Cookies;
use rocket_contrib::json::{Json, JsonValue};

#[post("/login", data = "<new_login>")]
pub fn login(
    conn: DbConnection,
    new_login: Json<Login>,
    cookies: Cookies,
) -> Result<JsonValue, errors::AuthError> {
    use database::schema::users::dsl::*;
    let user: (String, String, String) = users
        .filter(username.eq(&new_login.username))
        .first(conn.as_ref())?;

    if verify(user.0.clone(), user.1.clone(), new_login.password.clone()) {
        let token = jwt_generate(user.0, user.2.split(",").map(|x| x.to_string()).collect());

        return Ok(json!({
            "token": token,
        }));
    }

    Err(errors::AuthError::WrongPassword)
}

#[get("/whoami")]
pub fn whoami(conn: DbConnection, user: Auth) -> JsonValue {
    json!({
        "username": user.0.claims.get_user(),
        "picture": "https://i.redd.it/3n1if40vxxv31.png",
        "spentWatching": Progress::get_total_time_spent_watching(&conn, user.0.claims.get_user()).unwrap_or(0) / 3600
    })
}

#[get("/admin_exists")]
pub fn admin_exists(conn: DbConnection) -> Result<JsonValue, errors::DimError> {
    Ok(json!({
        "exists": !User::get_all(conn.as_ref())?.is_empty()
    }))
}

#[post("/register", data = "<new_user>")]
pub fn register(conn: DbConnection, new_user: Json<Login>) -> Result<JsonValue, errors::AuthError> {
    let users_empty = User::get_all(&conn)?.is_empty();

    if !users_empty
        && (new_user.invite_token.is_none()
            || !new_user.invite_token_valid(conn.as_ref()).unwrap_or(false))
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
    }
    .insert(conn.as_ref())?;

    if users_empty {
        new_user.invalidate_token(conn.as_ref())?;
    }

    Ok(json!({ "username": res }))
}

#[get("/invites")]
pub fn get_all_invites(conn: DbConnection, user: Auth) -> Result<JsonValue, errors::AuthError> {
    if user.0.claims.has_role("owner") {
        return Ok(json!({
            "invites": Login::get_all_invites(conn.as_ref())?
        }));
    }

    Err(errors::AuthError::Unauthorized)
}

#[post("/new_invite")]
pub fn generate_invite(conn: DbConnection, user: Auth) -> Result<JsonValue, errors::AuthError> {
    if !user.0.claims.has_role("owner") {
        return Err(errors::AuthError::Unauthorized);
    }

    Ok(json!({
        "token": Login::new_invite(conn.as_ref())?
    }))
}
