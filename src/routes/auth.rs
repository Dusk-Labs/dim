use crate::core::DbConnection;
use crate::errors;
use auth::jwt_generate;
use database::schema::users::dsl::*;
use database::user::{hash, InsertableUser, Login};
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

#[post("/login", data = "<new_login>")]
pub fn login(conn: DbConnection, new_login: Json<Login>) -> Result<JsonValue, errors::AuthError> {
    let user: (String, String, Vec<String>) = users
        .filter(username.eq(&new_login.username))
        .first(conn.as_ref())?;

    let user_passwd = user.1;

    if user_passwd == hash(new_login.password.clone()) {
        return Ok(json!({
            "token": jwt_generate(user.0, user.2)
        }));
    }

    Err(errors::AuthError::FailedAuth)
}

#[post("/register", data = "<new_user>")]
pub fn register(conn: DbConnection, new_user: Json<Login>) -> Result<JsonValue, errors::AuthError> {
    let res = InsertableUser {
        username: new_user.username.clone(),
        password: hash(new_user.password.clone()),
        roles: vec!["user".to_string()],
    }
    .insert(conn.as_ref())?;

    Ok(json!({ "username": res }))
}
