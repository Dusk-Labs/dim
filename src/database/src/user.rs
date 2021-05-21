use diesel::prelude::*;
use tokio_diesel::*;

use crate::DatabaseError;
use std::collections::HashMap;
use std::num::NonZeroU32;

use serde::Deserialize;
use serde::Serialize;

use ring::digest;
use ring::pbkdf2;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10_000) };

pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Theme of the app
    theme: Theme,
    /// Defines whether the sidebar should be collapsed or not
    is_sidebar_compact: bool,
    show_card_names: bool,
    /// If this contains a string then the filebrowser/explorer will default to this path instead of `/`.
    filebrowser_default_path: Option<String>,
    filebrowser_list_view: bool,
    /// If a file has subtitles then the subtitles with this language will be selected.
    default_subtitle_language: Option<String>,
    /// If a file has audio then the audio track with this language will be selected, otherwise the first one.
    default_audio_language: Option<String>,
    /// Any other external args.
    external_args: HashMap<String, String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            is_sidebar_compact: false,
            show_card_names: true,
            filebrowser_default_path: None,
            filebrowser_list_view: true,
            default_subtitle_language: Some("english".into()),
            default_audio_language: Some("english".into()),
            external_args: HashMap::new(),
        }
    }
}

// NOTE: Figure out the bug with this not being a valid postgres type
#[derive(Serialize, Deserialize, Debug, DbEnum, Eq, PartialEq)]
pub enum Role {
    Owner,
    User,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub username: String,
    pub roles: Vec<String>,
    pub profile_picture: String,
    pub settings: UserSettings,
}

pub fn hash(salt: String, s: String) -> String {
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        HASH_ROUNDS,
        &salt.as_bytes(),
        s.as_bytes(),
        &mut to_store,
    );
    base64::encode(&to_store)
}

pub fn verify(salt: String, password: String, attempted_password: String) -> bool {
    let real_pwd = base64::decode(&password).unwrap();

    pbkdf2::verify(
        PBKDF2_ALG,
        HASH_ROUNDS,
        &salt.as_bytes(),
        attempted_password.as_bytes(),
        real_pwd.as_slice(),
    )
    .is_ok()
}

impl User {
    /// Method gets all entries from the table users.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::user::{User, InsertableUser};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let new_user = InsertableUser {
    ///     username: "test_get_all".to_string(),
    ///     password: "test_get_all".to_string(),
    ///     roles: vec!["user".to_string()],
    /// };
    ///
    /// let res = new_user.insert(&conn).unwrap();
    /// assert_eq!(res, "test_get_all".to_string());
    ///
    /// let user = User::get_all(&conn).unwrap();
    /// assert!(user.len() > 0usize);
    ///
    /// let _ = User::delete(&conn, "test_get_all".to_string()).unwrap();
    pub async fn get_all(conn: &crate::DbConnection) -> Result<Vec<Self>, DatabaseError> {
        use crate::schema::users;

        Ok(users::table
            .select((
                users::dsl::username,
                users::dsl::roles,
                users::dsl::profile_picture,
                users::dsl::settings,
            ))
            .load_async::<(String, String, String, String)>(conn)
            .await?
            .iter()
            .cloned()
            .map(|(username, roles, profile_picture, settings)| Self {
                username,
                profile_picture,
                roles: roles.split(",").map(|x| x.to_string()).collect(),
                // Should never panic because we arent ever inserting arbitrary invalid data into the field.
                settings: serde_json::from_str(&settings).unwrap(),
            })
            .collect())
    }

    /// Method gets one entry from the table users based on the username supplied and password.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `uname` - username we wish to target and delete
    /// * `pw_hash` - hash of the password for the user we are trying to access
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::user::{User, hash, InsertableUser};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let new_user = InsertableUser {
    ///     username: "test_get_one".to_string(),
    ///     password: "test_get_one".to_string(),
    ///     roles: vec!["user".to_string()],
    /// };
    ///
    /// let res = new_user.insert(&conn).unwrap();
    /// assert_eq!(res, "test_get_one".to_string());
    ///
    /// let user = User::get_one(
    ///     &conn,
    ///     "test_get_one".to_string(),
    ///     "test_get_one".to_string()
    /// ).unwrap();
    /// assert_eq!(user.username, "test_get_one".to_string());
    /// assert_eq!(user.roles, vec!["user".to_string()]);
    ///
    /// let err_rows = User::get_one(&conn, "random".to_string(), "random".to_string());
    /// assert!(err_rows.is_err());
    ///
    /// let _ = User::delete(&conn, "test_get_one".to_string()).unwrap();
    pub async fn get_one(
        conn: &crate::DbConnection,
        uname: String,
        pw: String,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::users;
        Ok(users::table
            .filter(
                users::dsl::username
                    .eq(uname.clone())
                    .and(users::dsl::password.eq(hash(uname, pw))),
            )
            .select((
                users::dsl::username,
                users::dsl::roles,
                users::dsl::profile_picture,
                users::dsl::settings,
            ))
            .first_async::<(String, String, String, String)>(conn)
            .await
            .map(|(username, roles, profile_picture, settings)| Self {
                username,
                profile_picture,
                roles: roles.split(",").map(|x| x.to_string()).collect(),
                settings: serde_json::from_str(&settings).unwrap(),
            })?)
    }

    pub async fn get_one_unchecked(
        conn: &crate::DbConnection,
        uname: String,
    ) -> Result<Self, DatabaseError> {
        use crate::schema::users;

        Ok(users::table
            .filter(users::dsl::username.eq(uname))
            .select((
                users::dsl::username,
                users::dsl::roles,
                users::dsl::profile_picture,
                users::dsl::settings,
            ))
            .first_async::<(String, String, String, String)>(conn)
            .await
            .map(|(username, roles, profile_picture, settings)| Self {
                username,
                profile_picture,
                roles: roles.split(",").map(|x| x.to_string()).collect(),
                settings: serde_json::from_str(&settings).unwrap(),
            })?)
    }

    /// Method deletes a entry from the table users and returns the number of rows deleted.
    /// NOTE: Return should always be 1
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `uname` - username we wish to target and delete
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::user::{User, InsertableUser};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let new_user = InsertableUser {
    ///     username: "test_del".to_string(),
    ///     password: "test_del".to_string(),
    ///     roles: vec!["user".to_string()],
    /// };
    ///
    /// let res = new_user.insert(&conn).unwrap();
    /// assert_eq!(res, "test_del".to_string());
    ///
    /// let rows = User::delete(&conn, "test_del".to_string()).unwrap();
    /// assert_eq!(rows, 1usize);
    ///
    /// let err_rows = User::delete(&conn, "random".to_string()).unwrap();
    /// assert_eq!(err_rows, 0usize);
    pub async fn delete(conn: &crate::DbConnection, uname: String) -> Result<usize, DatabaseError> {
        use crate::schema::users;
        Ok(
            diesel::delete(users::table.filter(users::dsl::username.eq(uname)))
                .execute_async(conn)
                .await?,
        )
    }
}

#[derive(Deserialize)]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub profile_picture: String,
    pub settings: UserSettings,
}

impl Default for InsertableUser {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            roles: vec!["User".into()],
            profile_picture: "https://i.redd.it/3n1if40vxxv31.png".into(),
            settings: UserSettings::default(),
        }
    }
}

impl InsertableUser {
    /// Method consumes a InsertableUser object and inserts the values under it into postgres users table as a new user
    ///
    /// # Arguments
    /// * `self` - instance of InsertableUser which gets consumed
    /// * `conn` - postgres connection
    ///
    /// # Example
    /// ```
    /// use database::get_conn_devel as get_conn;
    /// use database::user::{User, InsertableUser, Role};
    ///
    /// let conn = get_conn().unwrap();
    ///
    /// let new_user = InsertableUser {
    ///     username: "test_insert".to_string(),
    ///     password: "test_insert".to_string(),
    ///     roles: vec!["user".to_string()],
    /// };
    ///
    /// let res = new_user.insert(&conn).unwrap();
    /// assert_eq!(res, "test_insert".to_string());
    ///
    /// let user = User::get_one(&conn, "test_insert".to_string(), "test_insert".to_string()).unwrap();
    /// assert_eq!(user.username, "test_insert".to_string());
    /// assert_eq!(user.roles, vec!["user".to_string()]);
    ///
    /// let _ = User::delete(&conn, "test_insert".to_string()).unwrap();
    /// ```
    pub async fn insert(self, conn: &crate::DbConnection) -> Result<String, DatabaseError> {
        use crate::schema::users;

        diesel::insert_into(users::table)
            .values((
                users::dsl::username.eq(self.username.clone()),
                users::dsl::password.eq(hash(self.username.clone(), self.password)),
                users::dsl::roles.eq(self.roles.join(",")),
                users::dsl::profile_picture.eq(self.profile_picture.clone()),
                users::dsl::settings.eq(serde_json::to_string(&self.settings).unwrap()),
            ))
            .execute_async(conn)
            .await?;

        Ok(self.username)
    }
}

#[derive(Deserialize, Default)]
pub struct UpdateableUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub profile_picture: Option<String>,
    pub settings: Option<UserSettings>,
}

impl UpdateableUser {
    pub async fn update(
        self,
        conn: &crate::DbConnection,
        _username: String,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::users;

        let entry = User::get_one_unchecked(conn, _username).await?;

        #[derive(Clone, Default, AsChangeset, Debug)]
        #[table_name = "users"]
        struct InnerUser {
            username: Option<String>,
            password: Option<String>,
            profile_picture: Option<String>,
            settings: Option<String>,
        }

        let username = self.username.clone().unwrap_or(entry.username.clone());

        let values = InnerUser {
            username: self.username,
            password: self.password.map(|x| hash(username, x)),
            profile_picture: self.profile_picture,
            settings: self.settings.and_then(|x| serde_json::to_string(&x).ok()),
        };

        Ok(
            diesel::update(users::dsl::users.filter(users::dsl::username.eq(&entry.username)))
                .set(values)
                .execute_async(conn)
                .await?,
        )
    }
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub invite_token: Option<String>,
}

impl Login {
    pub async fn invite_token_valid(
        &self,
        conn: &crate::DbConnection,
    ) -> Result<bool, DatabaseError> {
        use crate::schema::invites;

        if let Some(x) = self.invite_token.clone() {
            return Ok(diesel::select(diesel::dsl::exists(
                invites::table.filter(invites::token.eq(x)),
            ))
            .get_result_async(conn)
            .await?);
        }
        Ok(false)
    }

    pub async fn invalidate_token(
        &self,
        conn: &crate::DbConnection,
    ) -> Result<usize, DatabaseError> {
        use crate::schema::invites;

        if let Some(x) = self.invite_token.clone() {
            return Ok(diesel::delete(invites::table.filter(invites::token.eq(x)))
                .execute_async(conn)
                .await?);
        }

        Ok(0usize)
    }

    pub async fn new_invite(conn: &crate::DbConnection) -> Result<String, DatabaseError> {
        use crate::schema::invites;

        let token = uuid::Uuid::new_v4().to_hyphenated().to_string();

        diesel::insert_into(invites::table)
            .values(invites::token.eq(token.clone()))
            .execute_async(conn)
            .await?;

        Ok(token)
    }

    pub async fn get_all_invites(conn: &crate::DbConnection) -> Result<Vec<String>, DatabaseError> {
        use crate::schema::invites;

        Ok(invites::table
            .select(invites::dsl::token)
            .load_async::<String>(conn)
            .await?)
    }
}
