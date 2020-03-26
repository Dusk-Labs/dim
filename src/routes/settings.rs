use crate::{core::DbConnection, errors};
use auth::Wrapper as Auth;
use diesel::prelude::*;

#[get("/user/settings")]
pub fn get_user_settings(db: DbConnection, user: Auth) -> ! {
    unimplemented!();
}

#[post("/user/settings")]
pub fn post_user_settings(db: DbConnection, user: Auth) -> &'static str {
    "hello world"
}
