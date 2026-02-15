//! DTOs for `/auth/*` endpoints.
//!
//! Each struct derives `Validate` to ensure invalid input never reaches the
//! service layer.  This mirrors the “document → tests → implementation” flow:
//! we document the contract here, add unit tests to ensure validation behaves,
//! and only then wire handlers/services.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Payload for `POST /auth/register`.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12, message = "password must be at least 12 characters"))]
    pub password: String,
}

/// Payload for `POST /auth/login`.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 12))]
    pub password: String,
}

/// Response body shared by register/login endpoints.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: AuthUserDto,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user: AuthUserDto,
}

/// Response for `/auth/logout`.
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

/// Public user representation returned to clients.
#[derive(Debug, Serialize)]
pub struct AuthUserDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_request_rejects_short_password() {
        let req = RegisterRequest {
            name: "Dev".into(),
            email: "dev@example.com".into(),
            password: "short".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn login_request_requires_email_format() {
        let req = LoginRequest {
            email: "not-an-email".into(),
            password: "supersecurepassword".into(),
        };
        assert!(req.validate().is_err());
    }
}
