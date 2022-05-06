use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::AeadInPlace;
use aes_gcm::Aes256Gcm;
use aes_gcm::NewAead;

use once_cell::sync::OnceCell;
use rand::Rng;
use rand::RngCore;
use std::convert::TryInto;

use warp::http::header::AUTHORIZATION;
use warp::reject;
use warp::Filter;
use warp::Rejection;

use crate::user::User;
use crate::user::UserID;
use crate::DbConnection;

pub(crate) const NONCE_LEN: usize = 12;
pub(crate) const TAG_LEN: usize = 16;

/// This is the secret key with which we sign the cookies.
// TODO: Generate this at first run to ensure security
static KEY: OnceCell<[u8; 32]> = OnceCell::new();

pub fn generate_key() -> [u8; 32] {
    rand::thread_rng().gen()
}

pub fn set_key(k: [u8; 32]) {
    KEY.set(k).expect("Failed to set secret_key")
}

fn get_key() -> &'static [u8; 32] {
    KEY.get().expect("key must be initialized")
}

#[derive(Debug)]
pub struct Wrapper(pub User);

impl Wrapper {
    pub fn get_user(&self) -> UserID {
        self.0.id
    }
}

#[derive(Clone, Debug)]
pub enum AuthError {
    Missing,
    Invalid,
    InvalidKey,
    BadCount,
    DBError,
    DBQueryError,
    BadBase64,
    ShortData,
    DecryptError,
    CookieError,
}

impl warp::reject::Reject for AuthError {}

/// Function encrypts a UserID with a nonce and returns it as a base64 string to be used as a cookie/token.
pub fn user_cookie_generate(user: UserID) -> String {
    // Create a vec to hold the [nonce | cookie value].
    let cookie_val = &user.0.to_be_bytes();
    let mut data = vec![0; NONCE_LEN + cookie_val.len() + TAG_LEN];

    // Split data into three: nonce, input/output, tag. Copy input.
    let (nonce, in_out) = data.split_at_mut(NONCE_LEN);
    let (in_out, tag) = in_out.split_at_mut(cookie_val.len());
    in_out.copy_from_slice(cookie_val);

    // Fill nonce piece with random data.
    let mut rng = rand::thread_rng();
    rng.try_fill_bytes(nonce)
        .expect("couldn't random fill nonce");
    let nonce = GenericArray::clone_from_slice(nonce);

    // Perform the actual sealing operation, using the cookie's name as
    // associated data to prevent value swapping.
    let aead = Aes256Gcm::new(GenericArray::from_slice(get_key()));
    let aad_tag = aead
        .encrypt_in_place_detached(&nonce, b"", in_out)
        .expect("encryption failure!");

    // Copy the tag into the tag piece.
    tag.copy_from_slice(&aad_tag);

    // Base64 encode [nonce | encrypted value | tag].
    base64::encode(&data)
}

/// Function decrypts a UserID which was encrypted with `user_cookie_generate`
pub fn user_cookie_decode(cookie: String) -> Result<UserID, AuthError> {
    let data = base64::decode(cookie).map_err(|_| AuthError::BadBase64)?;
    if data.len() <= NONCE_LEN {
        return Err(AuthError::ShortData);
    }
    let (nonce, cipher) = data.split_at(NONCE_LEN);
    let aead = Aes256Gcm::new(GenericArray::from_slice(get_key()));
    let plaintext = aead
        .decrypt(GenericArray::from_slice(nonce), cipher)
        .map_err(|_| AuthError::DecryptError)?;

    Ok(UserID(i64::from_be_bytes(plaintext.try_into().unwrap())))
}

pub fn with_auth(
    conn: DbConnection,
) -> impl Filter<Extract = (Wrapper,), Error = Rejection> + Clone {
    // TODO: Remove
    warp::header(AUTHORIZATION.as_str())
        .and(warp::any().map(move || conn.clone()))
        .and_then(|x, c: DbConnection| async move {
            let mut tx = match c.read().begin().await {
                Ok(tx) => tx,
                Err(_) => return Err(reject::custom(AuthError::DBError)),
            };
            let id = user_cookie_decode(x)?;
            match User::get_by_id(&mut tx, id).await {
                Ok(u) => Ok(Wrapper(u)),
                Err(_) => Err(reject::custom(AuthError::DBQueryError)),
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::{get_conn_memory, tests::user_tests::insert_user, write_tx};

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_cookie_encoding() {
        let _ = KEY.set(generate_key());
        let mut conn = get_conn_memory().await.unwrap().writer().lock_owned().await;
        let mut tx = write_tx(&mut conn).await.unwrap();

        let user = insert_user(&mut tx).await;
        let token = user_cookie_generate(user.id);
        let token2 = user_cookie_generate(user.id);
        assert_ne!(token, token2);
        let uid = user_cookie_decode(token).unwrap();
        assert_eq!(uid, user.id);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_invalid_cookie() {
        let _ = KEY.set(generate_key());
        let res = user_cookie_decode(String::new());
        assert!(res.is_err());
        let res = user_cookie_decode(String::from("ansd9uid89as"));
        assert!(res.is_err());
        let res = user_cookie_decode(String::from("bXl1c2VyaWQ="));
        assert!(res.is_err());
    }
}
