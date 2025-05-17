use argonautica::{Hasher, Verifier};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::error::ApiError;

pub fn hash_password(password:&str, secret:&str) -> Result<String,ApiError> {
    let mut hasher = Hasher::default();

   hasher
        .with_password(password)
        .with_secret(secret)
        .hash()
        .map_err(|_| ApiError::InternalError)?;
 
}

pub fn verify_password(hash:&str, password:&str, secret:&str) -> Result<bool,ApiError> {
    let mut verifier = Verifier::default();

    verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret(secret)
        .verify()
        .map_err(|_| ApiError::InternalError)
}

pub fn create_jwt(username:&str,secret:&str) -> Result<String,ApiError>{
    let key: hmac::Hmac<Sha256> = hmac::NewMac::new_from_slice(secret.as_bytes())
    .map_err(|_| ApiError::InternalError)?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let exp = now + 60 * 60 * 24; // 24 hours

  let claims = BTreeMap::new();
  claims.insert("sub", username);
  claims.insert("exp", &exp.to_string());

  claims.sign_with_key(&key).map_err(|_| ApiError::InternalError)

}