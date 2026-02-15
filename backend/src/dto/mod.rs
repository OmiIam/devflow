//! Data Transfer Objects (DTOs) used to validate and serialize HTTP payloads.
//!
//! By keeping DTOs separate from our database/domain models we can:
//! - Enforce request validation rules without polluting the persistence structs.
//! - Hide internal fields (e.g., password hashes) from API responses.
//! - Provide stable contracts for the frontend independent of DB changes.
//!
//! Every DTO should derive `serde::Deserialize`/`serde::Serialize` as needed and,
//! when user input is involved, `validator::Validate`.

pub mod auth;
pub mod task;
