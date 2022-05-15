use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::Aead;
use aes_gcm::AeadInPlace;
use aes_gcm::Aes256Gcm;
use aes_gcm::NewAead;

use displaydoc::Display;
use once_cell::sync::OnceCell;
use rand::Rng;
use rand::RngCore;
use serde::Serialize;
use std::convert::TryInto;
use thiserror::Error;

const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

/// This is the secret key with which we sign the cookies.
// TODO: Generate this at first run to ensure security
static KEY: OnceCell<[u8; 32]> = OnceCell::new();

pub fn generate_key() -> [u8; 32] {
    rand::thread_rng().gen()
}

pub fn set_key(k: [u8; 32]) {
    KEY.set(k).expect("Failed to set secret_key")
}

/// This function should only be called from tests
pub fn set_key_fallible(k: [u8; 32]) {
    let _ = KEY.set(k);
}

fn get_key() -> &'static [u8; 32] {
    KEY.get().expect("key must be initialized")
}

#[derive(Clone, Debug, Display, Error, Serialize)]
pub enum AuthError {
    /// Token is not base64 encoded.
    BadBase64,
    /// Token data is too short.
    ShortData,
    /// Failed to decrypt token.
    DecryptError,
    /// Token plaintext does not contain a UserID.
    PlainTextNoti64,
}

/// Function encrypts a UserID with a nonce and returns it as a base64 string to be used as a cookie/token.
pub fn user_cookie_generate(user: i64) -> String {
    // Create a vec to hold the [nonce | cookie value].
    let cookie_val = &user.to_be_bytes();
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
pub fn user_cookie_decode(cookie: String) -> Result<i64, AuthError> {
    let data = base64::decode(cookie).map_err(|_| AuthError::BadBase64)?;
    if data.len() <= NONCE_LEN {
        return Err(AuthError::ShortData);
    }
    let (nonce, cipher) = data.split_at(NONCE_LEN);
    let aead = Aes256Gcm::new(GenericArray::from_slice(get_key()));
    let plaintext = aead
        .decrypt(GenericArray::from_slice(nonce), cipher)
        .map_err(|_| AuthError::DecryptError)?;

    Ok(i64::from_be_bytes(
        plaintext
            .try_into()
            .map_err(|_| AuthError::PlainTextNoti64)?,
    ))
}
