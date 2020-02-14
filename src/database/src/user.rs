use diesel::prelude::*;
use diesel::result::Error as DieselError;
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};

static PBKDF2_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
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

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub invite_token: Option<String>,
}

pub fn hash(salt: String, s: String) -> String {
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        100_000,
        &salt.as_bytes(),
        s.as_bytes(),
        &mut to_store,
    );
    base64::encode(&to_store)
}

pub fn verify(salt: String, password: String, attempted_password: String) -> bool {
    let real_pwd = base64::decode(&password).unwrap();
    if let Ok(_) = pbkdf2::verify(
        PBKDF2_ALG,
        100_000,
        &salt.as_bytes(),
        attempted_password.as_bytes(),
        real_pwd.as_slice(),
    ) {
        return true;
    }
    false
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
    pub fn get_all(conn: &diesel::PgConnection) -> Result<Vec<Self>, DieselError> {
        use crate::schema::users;
        users::table
            .select((users::dsl::username, users::dsl::roles))
            .load::<Self>(conn)
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
    pub fn get_one(
        conn: &diesel::PgConnection,
        uname: String,
        pw: String,
    ) -> Result<Self, DieselError> {
        use crate::schema::users;
        users::table
            .filter(
                users::dsl::username
                    .eq(uname.clone())
                    .and(users::dsl::password.eq(hash(uname, pw))),
            )
            .select((users::dsl::username, users::dsl::roles))
            .first::<Self>(conn)
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
    pub fn delete(conn: &diesel::PgConnection, uname: String) -> Result<usize, DieselError> {
        use crate::schema::users;
        diesel::delete(users::table.filter(users::dsl::username.eq(uname))).execute(conn)
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
    pub fn insert(self, conn: &diesel::PgConnection) -> Result<String, DieselError> {
        use crate::schema::users;

        diesel::insert_into(users::table)
            .values((
                users::dsl::username.eq(self.username.clone()),
                users::dsl::password.eq(hash(self.username, self.password)),
                users::dsl::roles.eq(self.roles),
            ))
            .returning(users::dsl::username)
            .get_result(conn)
    }
}

impl Login {
    pub fn invite_token_valid(&self, conn: &diesel::PgConnection) -> Result<bool, DieselError> {
        use crate::schema::invites;

        if let Some(ref x) = self.invite_token {
            return diesel::select(diesel::dsl::exists(
                invites::table.filter(invites::token.eq(x)),
            ))
            .get_result(conn);
        }
        Ok(false)
    }

    pub fn invalidate_token(&self, conn: &diesel::PgConnection) -> Result<usize, DieselError> {
        use crate::schema::invites;

        if let Some(ref x) = self.invite_token {
            return diesel::delete(invites::table.filter(invites::token.eq(x))).execute(conn);
        }

        Ok(0usize)
    }

    pub fn new_invite(conn: &diesel::PgConnection) -> Result<String, DieselError> {
        use crate::schema::invites;

        let token = uuid::Uuid::new_v4().to_hyphenated().to_string();

        diesel::insert_into(invites::table)
            .values(invites::token.eq(token))
            .returning(invites::token)
            .get_result(conn)
    }

    pub fn get_all_invites(conn: &diesel::PgConnection) -> Result<Vec<String>, DieselError> {
        use crate::schema::invites;

        invites::table
            .select(invites::dsl::token)
            .load::<String>(conn)
    }
}
