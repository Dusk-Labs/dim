use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::Algorithm;
use jsonwebtoken::Header;
use jsonwebtoken::TokenData;
use jsonwebtoken::Validation;

use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Request;
use rocket::Outcome;

use serde::Deserialize;
use serde::Serialize;
use time::get_time;

#[cfg(all(not(debug_assertions), feature = "null_auth"))]
std::compile_error!("Cannot disable authentication for non-devel environments.");

/// This is the secret key with which we sign the JWT tokens.
// TODO: Generate this at first run to ensure security
static KEY: &[u8; 16] = &[
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
];
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

/// Struct holds info needed for JWT to function correctly
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserRolesToken {
    /// Unique, per jwt identifier
    id: u128,
    /// Timestamp when the token was issued.
    iat: i64,
    /// Timestamp when the token expires.
    exp: i64,
    /// Username of the user to whom this token belongs to
    user: String,
    /// The roles of the user, usually owner or user
    // TODO: Use a enum here maybe considering theres like two possibilities lol?
    roles: Vec<String>,
}

#[derive(Debug)]
pub struct Wrapper(pub TokenData<UserRolesToken>);

#[derive(Debug)]
pub enum JWTError {
    Missing,
    Invalid,
    InvalidKey,
    BadCount,
}

impl UserRolesToken {
    /// Method returns whether the token is expired or not.
    pub fn is_expired(&self) -> bool {
        let now = get_time().sec;
        now >= self.exp
    }

    /// Method used to make sure that tokens are generated for different users to avoid collisions
    pub fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.user == claimed_user
    }

    /// Method checks if the user holding this token has a specific role.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    /// Method returns the username from the token
    pub fn get_user(&self) -> String {
        self.user.clone()
    }

    /// Method returns the id of this token
    pub fn get_id(&self) -> u128 {
        self.id
    }
}

/// Function generates a new JWT token and signs it with our KEY
/// # Arguments
/// * `user` - Username for whom we want to generate a token
/// * `roles` - vector of roles we want to give to this user.
///
/// # Example
/// ```
/// use auth::{jwt_generate, jwt_check};
///
/// let token_1 = jwt_generate("test".into(), vec!["owner".into()]);
/// let check_token = jwt_check(token_1).unwrap();
/// ```
pub fn jwt_generate(user: String, roles: Vec<String>) -> String {
    let now = get_time().sec;
    let payload = UserRolesToken {
        id: uuid::Uuid::new_v4().to_u128_le(),
        iat: now,
        exp: now + ONE_WEEK,
        user,
        roles,
    };

    encode(&Header::new(Algorithm::HS512), &payload, KEY).unwrap()
}

/// Function checks the token supplied and validates it
/// # Arguments
/// * `token` - JWT token we want to validate
///
/// # Example
/// ```
/// use auth::{jwt_generate, jwt_check};
///
/// let token_1 = jwt_generate("test".into(), vec!["owner".into()]);
/// let check_token = jwt_check(token_1).unwrap();
///
/// let check_token_2 = jwt_check("testtesttest".into());
/// assert!(check_token_2.is_err());
/// ```
#[cfg(not(feature = "null_auth"))]
pub fn jwt_check(token: String) -> Result<TokenData<UserRolesToken>, jsonwebtoken::errors::Error> {
    decode::<UserRolesToken>(&token, KEY, &Validation::new(Algorithm::HS512))
}

#[cfg(all(debug_assertions, feature = "null_auth"))]
pub fn jwt_check(_: String) -> Result<TokenData<UserRolesToken>, jsonwebtoken::errors::Error> {
    Ok(TokenData {
        header: jsonwebtoken::Header {
            alg: jsonwebtoken::Algorithm::HS512,
            ..Default::default()
        },
        claims: UserRolesToken {
            id: uuid::Uuid::new_v4().to_u128_le(),
            iat: 0,
            exp: i64::MAX,
            user: "Hiro".into(),
            roles: vec!["owner".into()],
        },
    })
}

impl<'a, 'r> FromRequest<'a, 'r> for Wrapper {
    type Error = JWTError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("Authorization").next() {
            Some(k) => match jwt_check(k.into()) {
                Ok(k) => Outcome::Success(Wrapper(k)),
                Err(_) => Outcome::Failure((Status::Unauthorized, JWTError::InvalidKey)),
            },
            None => Outcome::Failure((Status::Unauthorized, JWTError::Missing)),
        }
    }
}
