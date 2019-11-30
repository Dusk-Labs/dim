use jsonwebtoken::{decode, encode, Algorithm, Header, TokenData, Validation};
use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};
use serde::{Deserialize, Serialize};
use time::get_time;

// Change this to something more secure but this should do for testing
static KEY: &[u8; 16] = &[
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
];
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserRolesToken {
    // issued at
    iat: i64,
    exp: i64,
    user: String,
    roles: Vec<String>,
}

pub struct Wrapper(TokenData<UserRolesToken>);

#[derive(Debug)]
pub enum JWTError {
    Missing,
    Invalid,
    InvalidKey,
    BadCount,
}

impl UserRolesToken {
    pub fn is_expired(&self) -> bool {
        let now = get_time().sec;
        now >= self.exp
    }

    pub fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.user == claimed_user
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}

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

pub fn jwt_check(token: String) -> Result<TokenData<UserRolesToken>, jsonwebtoken::errors::Error> {
    decode::<UserRolesToken>(&token, KEY, &Validation::new(Algorithm::HS512))
}

impl<'a, 'r> FromRequest<'a, 'r> for Wrapper {
    type Error = JWTError;

    //#[cfg(not(debug_assertions))]
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

    /* DISABLED FOR DEBUG BUILDS
    #[cfg(debug_assertions)]
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
    */
}
