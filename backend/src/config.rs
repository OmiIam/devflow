//! Application configuration loading utilities.
//!
//! Axum services typically need to know which port to listen on,
//! how to connect to the database, and which secrets to use when
//! signing JWTs.  Hard-coding those values in `main.rs` makes tests
//! brittle and deployments unsafe, so we load them at runtime from
//! environment variables and optional configuration files.
//!
//! The `config` crate aggregates `.env` values, environment variables,
//! and (eventually) layered files like `config/development.toml`.

use config::{builder::DefaultState, Config, ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

/// High-level configuration consumed by the Axum application.
///
/// Each nested struct keeps related values together so we can pass only the
/// pieces a subsystem needs (for example, repositories only require the
/// `DatabaseConfig`).
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Which environment the process thinks it is running in.
    pub environment: EnvironmentKind,
    /// Settings required for binding the HTTP server.
    pub server: ServerConfig,
    /// Database connection information and pool sizing.
    pub database: DatabaseConfig,
    /// Authentication-specific configuration such as JWT secrets.
    pub auth: AuthConfig,
}

impl AppConfig {
    /// Load configuration by merging environment variables and optional files.
    ///
    /// # Errors
    /// Returns `ConfigError` when the configuration file cannot be read or when
    /// deserialization fails due to missing/invalid keys.  The caller should
    /// surface that failure during startup instead of calling `unwrap()`.
    pub fn load() -> Result<Self, ConfigError> {
        let builder = Config::builder();
        Self::build_with_sources(builder)?.try_deserialize()
    }

    fn build_with_sources(builder: ConfigBuilder<DefaultState>) -> Result<Config, ConfigError> {
        builder
            // Optional base file for shared defaults (future use).
            .add_source(File::with_name("config/base").required(false))
            // Environment-specific override file, e.g., `config/development.toml`.
            .add_source(
                File::with_name(&format!("config/{}", EnvironmentKind::default())).required(false),
            )
            // Convert env vars like DEVFLOW__SERVER__PORT into nested keys.
            .add_source(Environment::with_prefix("DEVFLOW").separator("__"))
            .build()
    }
}

/// Which environment the process is running in.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum EnvironmentKind {
    #[default]
    Development,
    Test,
    Production,
}

impl fmt::Display for EnvironmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            EnvironmentKind::Development => "development",
            EnvironmentKind::Test => "test",
            EnvironmentKind::Production => "production",
        };
        write!(f, "{as_str}")
    }
}

/// HTTP server level configuration (address/port).
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_host")]
    pub host: String,
    #[serde(default = "default_server_port")]
    pub port: u16,
}

fn default_server_host() -> String {
    // Bind to all interfaces by default so Docker/containers can reach the API.
    "0.0.0.0".to_string()
}

fn default_server_port() -> u16 {
    8000
}

impl ServerConfig {
    /// Convenience helper returning `host:port`.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Database connection settings used by SQLx.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// Full postgres connection string (e.g. postgres://user:pass@host/db).
    #[serde(default = "default_database_url")]
    pub url: String,
    /// Maximum number of pooled PostgreSQL connections.
    #[serde(default = "default_pool_size")]
    pub max_connections: u32,
}

fn default_database_url() -> String {
    // Development-friendly default; production should override via environment.
    "postgres://postgres:password@localhost:5432/devflow".to_string()
}

fn default_pool_size() -> u32 {
    5
}

/// Authentication related settings.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// Secret key for signing JWTs.
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    /// Expiration of issued tokens, stored in hours for readability.
    #[serde(default = "default_jwt_expiry_hours")]
    pub jwt_expiry_hours: u16,
}

fn default_jwt_secret() -> String {
    "insecure-development-secret-change-me".to_string()
}

fn default_jwt_expiry_hours() -> u16 {
    24 * 7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_config_provides_default_address() {
        let cfg = ServerConfig {
            host: default_server_host(),
            port: default_server_port(),
        };
        assert_eq!(cfg.address(), "0.0.0.0:8000");
    }
}
