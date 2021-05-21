use crate::core::DbConnection;
use crate::errors;

use database::user::InsertableUser;
use database::user::UpdateableUser;
use database::user::User;
use database::user::UserSettings;

use auth::Wrapper as Auth;
use diesel::prelude::*;

use rocket::State;
use rocket_contrib::json::Json;

#[get("/settings")]
pub async fn get_user_settings(
    db: State<'_, DbConnection>,
    user: Auth,
) -> Result<Json<UserSettings>, errors::DimError> {
    Ok(Json(
        User::get_one_unchecked(&*db, user.0.claims.get_user())
            .await?
            .settings,
    ))
}

#[post("/settings", format = "json", data = "<new_settings>")]
pub async fn post_user_settings(
    db: State<'_, DbConnection>,
    user: Auth,
    new_settings: Json<UserSettings>,
) -> Result<Json<UserSettings>, errors::DimError> {
    let update_user = UpdateableUser {
        settings: Some(new_settings.clone()),
        ..Default::default()
    };

    update_user.update(&*db, user.0.claims.get_user()).await?;

    Ok(new_settings)
}
