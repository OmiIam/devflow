//! # Authentication DTOs
//!
//! This module contains the Data Transfer Objects (DTOs) used for
//! user authentication requests (registration and login).
//!
//! DTOs are responsible for defining the expected structure of incoming
//! request bodies and applying validation rules to them.

use serde::Deserialize;
use validator::Validate;

/// # DTO for User Registration
///
/// Defines the shape of the request body for `POST /auth/register`.
///
/// It derives `Deserialize` to be extracted from the request body by Axum,
/// and `Validate` to apply the validation rules defined on its fields.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "must be at least 8 characters long"))]
    pub password: String,
}

/// # DTO for User Login
///
/// Defines the shape of the request body for `POST /auth/login`.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "sorry, only email addresses here"))]
    pub email: String,

    // We don't validate password length on login, only that it's present.
    #[validate(length(min = 1, message = "password insertion will not be skipped"))]
    pub password: String,
}
