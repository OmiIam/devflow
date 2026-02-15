//! Password hashing helpers built on the Argon2 algorithm.
//!
//! We isolate hashing/verification logic so that the rest of the codebase never
//! manipulates raw passwords.  This module uses Argon2id with parameters that
//! balance security and performance for server-side hashing.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use thiserror::Error;

/// Errors that can occur while hashing or verifying passwords.
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("hashing failed: {0}")]
    HashError(String),
    #[error("invalid password")]
    InvalidPassword,
}

/// Hash a plaintext password using Argon2id.
pub fn hash_password(plaintext: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plaintext.as_bytes(), &salt)
        .map_err(|err| PasswordError::HashError(err.to_string()))?;
    Ok(hash.to_string())
}

/// Verify a plaintext password against a stored Argon2 hash.
pub fn verify_password(hash: &str, candidate: &str) -> Result<(), PasswordError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|err| PasswordError::HashError(err.to_string()))?;
    Argon2::default()
        .verify_password(candidate.as_bytes(), &parsed_hash)
        .map_err(|_| PasswordError::InvalidPassword)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing_and_verification_round_trip() {
        let hash = hash_password("SupersafePass123!").expect("hashing should succeed");
        assert!(verify_password(&hash, "SupersafePass123!").is_ok());
        assert!(verify_password(&hash, "wrongpassword").is_err());
    }
}
