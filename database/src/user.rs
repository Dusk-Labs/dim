use crate::DatabaseError;
use std::num::NonZeroU32;

use serde::Deserialize;
use serde::Serialize;

use ring::digest;
use ring::pbkdf2;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1_000) };

pub type Credential = [u8; CREDENTIAL_LEN];

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

// NOTE: Figure out the bug with this not being a valid postgres type
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Role {
    Owner,
    User,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub roles: Vec<String>,
    pub password: String,
}

impl User {
    /// Method gets all entries from the table users.
    ///
    /// # Arguments
    ///
    /// * `conn` - postgres connection
    pub async fn get_all(conn: &crate::DbConnection) -> Result<Vec<Self>, DatabaseError> {
        Ok(sqlx::query!("SELECT * FROM users")
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|user| Self {
                username: user.username.unwrap(),
                roles: user.roles.split(',').map(ToString::to_string).collect(),
                password: user.password,
            })
            .collect())
    }

    pub async fn get(conn: &crate::DbConnection, username: &str) -> Result<Self, DatabaseError> {
        Ok(sqlx::query!(
            "SELECT * from users
                WHERE username = ?",
            username
        )
        .fetch_one(conn)
        .await
        .map(|u| Self {
            username: u.username.unwrap(),
            roles: u.roles.split(',').map(ToString::to_string).collect(),
            password: u.password,
        })?)
    }

    /// Method gets one entry from the table users based on the username supplied and password.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `uname` - username we wish to target and delete
    /// * `pw_hash` - hash of the password for the user we are trying to access
    pub async fn get_one(
        conn: &crate::DbConnection,
        uname: String,
        pw: String,
    ) -> Result<Self, DatabaseError> {
        let hash = hash(uname.clone(), pw);
        let user = sqlx::query!(
            "SELECT * FROM users WHERE username = ? AND password = ?",
            uname,
            hash,
        )
        .fetch_one(conn)
        .await?;

        Ok(Self {
            username: user.username.unwrap(),
            roles: user.roles.split(',').map(ToString::to_string).collect(),
            password: user.password,
        })
    }

    /// Method deletes a entry from the table users and returns the number of rows deleted.
    /// NOTE: Return should always be 1
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    /// * `uname` - username we wish to target and delete
    pub async fn delete(conn: &crate::DbConnection, uname: String) -> Result<usize, DatabaseError> {
        Ok(sqlx::query!("DELETE FROM users WHERE username = ?", uname)
            .execute(conn)
            .await?
            .rows_affected() as usize)
    }
}

#[derive(Deserialize)]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}

impl InsertableUser {
    /// Method consumes a InsertableUser object and inserts the values under it into postgres users
    /// table as a new user
    ///
    /// # Arguments
    /// * `self` - instance of InsertableUser which gets consumed
    /// * `conn` - postgres connection
    pub async fn insert(self, conn: &crate::DbConnection) -> Result<String, DatabaseError> {
        let Self {
            username,
            password,
            roles,
        } = self;

        let password = hash(username.clone(), password);
        let roles = roles.join(",");

        sqlx::query!(
            "INSERT INTO users (username, password, roles) VALUES ($1, $2, $3)",
            username,
            password,
            roles
        )
        .execute(conn)
        .await?;

        Ok(username)
    }
}

#[derive(Deserialize, Default)]
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
        let tok = match &self.invite_token {
            None => return Ok(false),
            Some(t) => t,
        };

        Ok(
            sqlx::query!("SELECT token FROM invites WHERE token = ?", tok)
                .fetch_optional(conn)
                .await?
                .is_some(),
        )
    }

    pub async fn invalidate_token(
        &self,
        conn: &crate::DbConnection,
    ) -> Result<usize, DatabaseError> {
        if let Some(tok) = &self.invite_token {
            Ok(sqlx::query!("DELETE FROM invites WHERE token = ?", tok)
                .execute(conn)
                .await?
                .rows_affected() as usize)
        } else {
            Ok(0)
        }
    }

    pub async fn new_invite(conn: &crate::DbConnection) -> Result<String, DatabaseError> {
        let token = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let _ = sqlx::query!("INSERT INTO invites (token) VALUES ($1)", token)
            .execute(conn)
            .await?;

        Ok(token)
    }

    pub async fn get_all_invites(conn: &crate::DbConnection) -> Result<Vec<String>, DatabaseError> {
        Ok(sqlx::query!("SELECT token from invites")
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|t| t.token)
            .collect())
    }
}
