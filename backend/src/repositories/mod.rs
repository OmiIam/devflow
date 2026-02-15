//! Data access layer built with SQLx.
//!
//! Repositories isolate all SQL queries so services operate over strongly
//! typed functions instead of raw SQL strings.  This makes it easier to mock
//! dependencies in tests and keep the rest of the codebase unaware of SQLx
//! specifics.

pub mod user_repository;
