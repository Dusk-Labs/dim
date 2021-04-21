use diesel::prelude::*;
use tokio_diesel::*;

use crate::DatabaseError;
use std::num::NonZeroU32;

use serde::Deserialize;
use serde::Serialize;

use ring::digest;
use ring::pbkdf2;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const HASH_ROUNDS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10_000) };

pub type Credential = [u8; CREDENTIAL_LEN];

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
}

#[derive(Deserialize)]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
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
            .select((users::dsl::username, users::dsl::roles))
            .load_async::<(String, String)>(conn)
            .await?
            .iter()
            .cloned()
            .map(|(username, roles)| Self {
                username,
                roles: roles.split(",").map(|x| x.to_string()).collect(),
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
            .select((users::dsl::username, users::dsl::roles))
            .first_async::<(String, String)>(conn)
            .await
            .map(|(username, roles)| Self {
                username,
                roles: roles.split(",").map(|x| x.to_string()).collect(),
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

impl InsertableUser {
    /// Method consumes a InsertableUser object and inserts the values under it into postgres users
    /// table as a new user
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
            ))
            .execute_async(conn)
            .await?;

        Ok(self.username)
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
