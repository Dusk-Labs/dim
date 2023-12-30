use crate::DatabaseError;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::SystemTime;

use dim_auth::user_cookie_decode;
use dim_auth::user_cookie_generate;
use dim_auth::AuthError;
use serde::Deserialize;
use serde::Serialize;

use ring::digest;
use ring::pbkdf2;
use sqlx::Decode;
use sqlx::Encode;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1_000) };

pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum Theme {
    Light,
    Dark,
    Black,
}

pub fn default_theme() -> Theme {
    Theme::Dark
}

pub fn default_true() -> bool {
    true
}

pub fn default_false() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultVideoQuality {
    /// Represents DirectPlay quality
    DirectPlay,
    /// Represents a default video quality made up of resolution and bitrate.
    Resolution(u64, u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Theme of the app
    #[serde(default = "default_theme")]
    theme: Theme,
    /// Defines whether the sidebar should be collapsed or not
    #[serde(default = "default_false")]
    is_sidebar_compact: bool,
    #[serde(default = "default_true")]
    show_card_names: bool,
    /// If this contains a string then the filebrowser/explorer will default to this path instead of `/`.
    filebrowser_default_path: Option<String>,
    #[serde(default = "default_true")]
    filebrowser_list_view: bool,
    /// If a file has subtitles then the subtitles with this language will be selected.
    default_subtitle_language: Option<String>,
    /// If a file has audio then the audio track with this language will be selected, otherwise the first one.
    default_audio_language: Option<String>,
    /// Represents the default video quality for user.
    pub default_video_quality: DefaultVideoQuality,
    /// Any other external args.
    #[serde(default)]
    external_args: HashMap<String, String>,
    /// Whether hovercards are hidden or not
    #[serde(default)]
    show_hovercards: bool,
    /// Whether to auto play next video
    enable_autoplay: bool,
}

impl<DB: sqlx::Database> sqlx::Type<DB> for UserSettings
where
    Vec<u8>: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<u8> as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> Decode<'r, DB> for UserSettings
where
    &'r [u8]: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_slice(value).unwrap_or_default())
    }
}

impl<'q, DB: sqlx::Database> Encode<'q, DB> for UserSettings
where
    Vec<u8>: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let val = serde_json::to_vec(self).unwrap_or_default();
        <Vec<u8> as Encode<DB>>::encode(val, buf)
    }
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
            show_hovercards: true,
            default_video_quality: DefaultVideoQuality::DirectPlay,
            enable_autoplay: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Role {
    Owner,
    User,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserID(pub(crate) i64);

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Roles(pub Vec<String>);

impl<DB: sqlx::Database> sqlx::Type<DB> for Roles
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> Decode<'r, DB> for Roles
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value).unwrap_or_default())
    }
}

impl<'q, DB: sqlx::Database> Encode<'q, DB> for Roles
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let val = serde_json::to_string(self).unwrap_or_default();
        <String as Encode<DB>>::encode(val, buf)
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub roles: Roles,
    pub prefs: UserSettings,
    pub picture: Option<i64>,
}

impl User {
    /// Method gets all entries from the table users.
    pub async fn get_all(conn: &mut crate::Transaction<'_>) -> Result<Vec<Self>, DatabaseError> {
        Ok(
            sqlx::query!(
                r#"SELECT id as "id: UserID", username, roles as "roles: Roles", prefs as "prefs: UserSettings", picture FROM users"#
            )
            .fetch_all(&mut **conn)
            .await?
            .into_iter()
            .map(|user| Self {
                id: user.id,
                username: user.username,
                roles: user.roles,
                prefs: user.prefs,
                picture: user.picture,
            })
            .collect(),
        )
    }

    pub async fn get_by_id(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query!(
            r#"SELECT id as "id: UserID", username, roles as "roles: Roles", prefs as "prefs: UserSettings", picture from users
                WHERE id = ?"#,
            uid
        )
        .fetch_one(&mut **conn)
        .await
        .map(|u| Self {
            id: u.id,
            username: u.username,
            roles: u.roles,
            prefs: u.prefs,
            picture: u.picture,
        })?)
    }

    pub async fn get(
        conn: &mut crate::Transaction<'_>,
        username: &str,
    ) -> Result<Self, DatabaseError> {
        Ok(sqlx::query!(
            r#"SELECT id as "id: UserID", username, roles as "roles: Roles", prefs as "prefs: UserSettings", picture from users
                WHERE username = ?"#,
            username
        )
        .fetch_one(&mut **conn)
        .await
        .map(|u| Self {
            id: u.id,
            username: u.username,
            roles: u.roles,
            prefs: u.prefs,
            picture: u.picture,
        })?)
    }

    /// Method gets one entry from the table users based on the username supplied and password.
    ///
    /// # Arguments
    /// * `uname` - username we wish to target and delete
    /// * `pw_hash` - hash of the password for the user we are trying to access
    pub async fn authenticate(
        conn: &mut crate::Transaction<'_>,
        uname: String,
        pw: String,
    ) -> Result<Self, DatabaseError> {
        let hash = hash(uname.clone(), pw);
        let user = sqlx::query!(
            r#"SELECT id as "id: UserID", username, roles as "roles: Roles", prefs as "prefs: UserSettings", picture FROM users WHERE username = ? AND password = ?"#,
            uname,
            hash,
        )
        .fetch_one(&mut **conn)
        .await?;

        Ok(Self {
            id: user.id,
            username: user.username,
            roles: user.roles,
            prefs: user.prefs,
            picture: user.picture,
        })
    }

    /// Method gets users password from the table users based on the user
    ///
    /// # Arguments
    /// * `conn` - DBTransaction
    pub async fn get_pass(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<String, DatabaseError> {
        let pass = sqlx::query!("SELECT password FROM users WHERE id = ?", self.id,)
            .fetch_one(&mut **conn)
            .await
            .map(|r| r.password)?;

        Ok(pass)
    }

    /// Method deletes a entry from the table users and returns the number of rows deleted.
    /// NOTE: Return should always be 1
    ///
    /// # Arguments
    /// * `uname` - username we wish to target and delete
    pub async fn delete(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM users WHERE id = ?", uid)
            .execute(&mut **conn)
            .await?
            .rows_affected() as usize)
    }

    /// Method resets the password for a user to a new password.
    ///
    /// # Arguments
    /// * `&` - db &ection
    /// * `password` - new password.
    pub async fn set_password(
        &self,
        conn: &mut crate::Transaction<'_>,
        password: String,
    ) -> Result<usize, DatabaseError> {
        let hash = hash(self.username.clone(), password);

        Ok(sqlx::query!(
            "UPDATE users SET password = $1 WHERE username = ?2",
            hash,
            self.username
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }

    pub async fn set_username(
        conn: &mut crate::Transaction<'_>,
        old_username: String,
        new_username: String,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "UPDATE users SET username = $1 WHERE users.username = ?2",
            new_username,
            old_username
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }

    pub async fn set_picture(
        conn: &mut crate::Transaction<'_>,
        uid: UserID,
        asset_id: i64,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "UPDATE users SET picture = $1 WHERE users.id = ?2",
            asset_id,
            uid
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.0.contains(&role.to_string())
    }

    pub fn roles(&self) -> Roles {
        self.roles.clone()
    }
}

#[derive(Deserialize)]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub roles: Roles,
    pub prefs: UserSettings,
    pub claimed_invite: String,
}

impl InsertableUser {
    /// Method consumes a InsertableUser object and inserts the values under it into database
    /// table as a new user
    pub async fn insert(self, conn: &mut crate::Transaction<'_>) -> Result<User, DatabaseError> {
        let Self {
            username,
            password,
            roles,
            prefs,
            claimed_invite,
        } = self;

        let password = hash(username.clone(), password);

        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (username, password, prefs, claimed_invite, roles) VALUES ($1, $2, $3, $4, $5) returning id as "id: UserID",username,roles as "roles: Roles",prefs as "prefs: UserSettings",picture"#,
            username,
            password,
            prefs,
            claimed_invite,
            roles
        ).fetch_one(&mut **conn)
        .await?;
        Ok(user)
    }
}

#[derive(Deserialize)]
pub struct UpdateableUser {
    pub prefs: Option<UserSettings>,
}

impl UpdateableUser {
    pub async fn update(
        &self,
        conn: &mut crate::Transaction<'_>,
        user: UserID,
    ) -> Result<usize, DatabaseError> {
        if let Some(prefs) = &self.prefs {
            return Ok(sqlx::query!(
                "UPDATE users SET prefs = $1 WHERE users.id = ?",
                prefs,
                user
            )
            .execute(&mut **conn)
            .await?
            .rows_affected() as usize);
        }

        Ok(0)
    }
}

#[derive(Deserialize, Default)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub invite_token: Option<String>,
}

impl Login {
    /// Will return whether the token is valid and hasnt been claimed yet.
    pub async fn invite_token_valid(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<bool, DatabaseError> {
        let tok = match &self.invite_token {
            None => return Ok(false),
            Some(t) => t,
        };

        Ok(sqlx::query!(
            "SELECT id FROM invites
                          WHERE id NOT IN (
                              SELECT claimed_invite FROM users
                          )
                          AND id = ?",
            tok
        )
        .fetch_optional(&mut **conn)
        .await?
        .is_some())
    }

    pub async fn invalidate_token(
        &self,
        conn: &mut crate::Transaction<'_>,
    ) -> Result<usize, DatabaseError> {
        if let Some(tok) = &self.invite_token {
            Ok(sqlx::query!("DELETE FROM invites WHERE id = ?", tok)
                .execute(&mut **conn)
                .await?
                .rows_affected() as usize)
        } else {
            Ok(0)
        }
    }

    pub async fn new_invite(conn: &mut crate::Transaction<'_>) -> Result<String, DatabaseError> {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let token = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let _ = sqlx::query!(
            "INSERT INTO invites (id, date_added) VALUES ($1, $2)",
            token,
            ts
        )
        .execute(&mut **conn)
        .await?;

        Ok(token)
    }

    pub async fn get_all_invites(
        conn: &mut crate::Transaction<'_>,
    ) -> Result<Vec<String>, DatabaseError> {
        Ok(sqlx::query!("SELECT id from invites")
            .fetch_all(&mut **conn)
            .await?
            .into_iter()
            .map(|t| t.id)
            .collect())
    }

    pub async fn delete_token(
        conn: &mut crate::Transaction<'_>,
        token: String,
    ) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!(
            "DELETE FROM invites
                WHERE id NOT IN (
                    SELECT claimed_invite FROM users
                ) AND id = ?",
            token
        )
        .execute(&mut **conn)
        .await?
        .rows_affected() as usize)
    }

    pub fn create_cookie(id: UserID) -> String {
        user_cookie_generate(id.0)
    }

    pub fn verify_cookie(cookie: String) -> Result<UserID, AuthError> {
        Ok(UserID(user_cookie_decode(cookie)?))
    }
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
