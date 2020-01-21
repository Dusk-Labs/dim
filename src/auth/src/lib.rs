use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome,
};
use serde::{Deserialize, Serialize};
use time::get_time;

/// This is the secret key with which we sign the JWT tokens.
// TODO: Generate this at first run to ensure security
static KEY: &[u8; 16] = &[
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
];
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

/// Struct holds info needed for JWT to function correctly
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserRolesToken {
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
pub fn jwt_check(token: String) -> Result<TokenData<UserRolesToken>, jsonwebtoken::errors::Error> {
    decode::<UserRolesToken>(&token, KEY, &Validation::new(Algorithm::HS512))
}

impl<'a, 'r> FromRequest<'a, 'r> for Wrapper {
    type Error = JWTError;

    /*
    fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Wrapper(TokenData {
            header: Header::new(Algorithm::HS512),
            claims: UserRolesToken {
                iat: 999_999,
                exp: 999_999,
                user: "Test User".to_string(),
                roles: vec!["Owner".to_string()],
            },
        }))
    }
    */

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, JWTError::Missing)),
            1 => match jwt_check(keys[0].to_string()) {
                Ok(k) => Outcome::Success(Wrapper(k)),
                Err(_) => Outcome::Failure((Status::BadRequest, JWTError::InvalidKey)),
            },
            _ => Outcome::Failure((Status::BadRequest, JWTError::BadCount)),
        }
    }
}
