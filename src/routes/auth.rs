use crate::core::DbConnection;
use crate::errors;
use auth::jwt_generate;
use database::user::Login;
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

#[post("/login", data = "<new_login>")]
pub fn login(conn: DbConnection, new_login: Json<Login>) -> Result<JsonValue, errors::AuthError> {
    use database::schema::users::dsl::*;

    let user: (String, String, Vec<String>) = users
        .filter(username.eq(&new_login.username))
        .first(conn.as_ref())?;

    let hash = user.1;

    if hash == new_login.password {
        return Ok(json!({
            "token": jwt_generate(user.0, user.2)
        }));
    }

    Err(errors::AuthError::FailedAuth)
}
