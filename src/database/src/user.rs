use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

// Figure out the bug with this not being a valid postgres type
#[derive(Serialize, Deserialize, Debug, DbEnum, Eq, PartialEq)]
pub enum Role {
    Owner,
    User,
}

#[derive(Queryable)]
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
}

// TODO: replace with a proper hashing function after mocking is done.
pub fn hash(s: String) -> String {
    s
}

impl User {
    /// Method gets all entries from the table users.
    ///
    /// # Arguments
    /// * `conn` - postgres connection
    ///
    /// # Example
    /// ```
    /// use database::get_conn;
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
    /// use database::get_conn;
    /// use database::user::{User, InsertableUser};
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
        pw_hash: String,
    ) -> Result<Self, DieselError> {
        use crate::schema::users;
        users::table
            .filter(
                users::dsl::username
                    .eq(uname)
                    .and(users::dsl::password.eq(pw_hash)),
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
    /// use database::get_conn;
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
    /// use database::get_conn;
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
                users::dsl::username.eq(self.username),
                users::dsl::password.eq(hash(self.password)),
                users::dsl::roles.eq(self.roles),
            ))
            .returning(users::dsl::username)
            .get_result(conn)
    }
}
