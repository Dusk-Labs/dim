use crate::core::DbConnection;
use auth::jwt_generate;
use diesel::prelude::*;
use dim_database::user::Login;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[post("/login", data = "<new_login>")]
pub fn login(conn: DbConnection, new_login: Json<Login>) -> Result<JsonValue, Status> {
    use dim_database::schema::users::dsl::*;

    let user: (String, String, Vec<String>) = users
        .filter(username.eq(&new_login.username))
        .first(&*conn)
        .unwrap();
    let hash = user.1;

    if hash == new_login.password {
        return Ok(json!({
            "token": jwt_generate(user.0, user.2)
        }));
    }

    Err(Status::NotFound)
}
