//! # User Model & DTOs
//!
//! This module contains the core `User` domain model, which represents the
//! data as it is stored in the database. It also includes the `UserResponse`
//! Data Transfer Object (DTO), which defines the safe, public representation
//! of a user that can be sent to clients.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// # The User Domain Model
///
/// This struct represents a user record in the database.
///
/// It derives `FromRow` to allow `sqlx` to map database rows directly into this struct.
///
/// ## Security
/// This struct includes the `password_hash`. It **must not** derive `Serialize`.
/// It is an internal representation and should never be exposed to the client.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// # Public User Response DTO
///
/// This struct defines the publicly safe representation of a user.
/// It excludes sensitive fields like `password_hash`.
///
/// It derives `Serialize` so it can be converted to JSON and sent in API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// # Conversion from Domain Model to DTO
///
/// This implementation of the `From` trait provides a clean and explicit way
/// to convert our internal `User` model into the public `UserResponse` DTO.
/// This is a key part of ensuring we don't accidentally leak sensitive data.
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
