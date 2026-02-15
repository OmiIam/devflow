//! JSON Web Token helpers for issuing and verifying auth tokens.

use crate::config::AuthConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Claims stored in the JWT payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("failed to encode token")]
    Encode(#[from] jsonwebtoken::errors::Error),
    #[error("failed to decode token")]
    Decode(#[source] jsonwebtoken::errors::Error),
}

/// Issue a JWT for the given user id.
pub fn generate_token(user_id: Uuid, config: &AuthConfig) -> Result<String, JwtError> {
    let expiry = Utc::now() + Duration::hours(config.jwt_expiry_hours.into());
    let claims = Claims {
        sub: user_id,
        exp: expiry.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(JwtError::Encode)
}

/// Validate a JWT and return its claims.
pub fn validate_token(token: &str, config: &AuthConfig) -> Result<Claims, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(JwtError::Decode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuthConfig;

    fn config() -> AuthConfig {
        AuthConfig {
            jwt_secret: "test-secret".into(),
            jwt_expiry_hours: 1,
        }
    }

    #[test]
    fn generate_and_validate_round_trip() {
        let cfg = config();
        let user_id = Uuid::new_v4();
        let token = generate_token(user_id, &cfg).expect("token generation");
        let claims = validate_token(&token, &cfg).expect("token validation");
        assert_eq!(claims.sub, user_id);
    }
}
