//! `devflow_backend` is the core library for the DevFlow application.
//!
//! This library contains all the business logic, data models, and components
//! necessary to run the DevFlow API. The main binary in `src/main.rs` will
//! use this library to configure and start the web server.
//!
//! By structuring the project this way, we achieve a clear separation between
//! the application's core logic (the library) and its entry point (the binary).
//! This makes the code more modular, easier to test, and reusable.
//!
//! ## Modules
//!
//! The library is organized into the following modules:
//!
//! - `config`: Application configuration management.
//! - `db`: Database connection and pooling.
//! - `dto`: Data Transfer Objects for API requests and responses.
//! - `errors`: Custom error types for the application.
//! - `handlers`: Axum route handlers that process HTTP requests.
//! - `middleware`: Custom middleware for authentication, logging, etc.
//! - `models`: Database entity models.
//! - `repositories`: Data access layer for database interactions.
//! - `routes`: API route definitions.
//! - `services`: Business logic layer.
//! - `utils`: Shared utility functions (e.g., JWT, password hashing).

// This is the main entry point for the library.
// Public modules are declared here so the binary crate and integration tests
// can import from a single namespace (e.g. `devflow_backend::routes::api_router`).
pub mod config;
pub mod db;
pub mod dto;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;
