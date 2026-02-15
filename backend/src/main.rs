//! The main entry point for the DevFlow backend application.
//!
//! This binary is responsible for:
//! 1.  Initializing the application's configuration.
//! 2.  Setting up the logger (`tracing`).
//! 3.  Establishing a connection to the database.
//! 4.  Configuring and launching the Axum web server.
//!
//! It uses the `devflow_backend` library crate for all the core application logic.

use anyhow::Context;
use axum::Router;
use devflow_backend::{
    config::AppConfig,
    db::create_pool,
    routes::api_router,
    utils::{AppState, SharedAppState},
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// The `#[tokio::main]` attribute is a macro that transforms this `async fn main`
/// into a synchronous `fn main` that initializes the tokio runtime and runs the
/// future returned by the async main function.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "devflow_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Loading DevFlow backend");

    let config =
        AppConfig::load().context("Unable to load configuration (check environment variables)")?;
    let db_pool = create_pool(&config.database, config.environment)
        .await
        .context("Failed to establish database pool")?;

    let shared_state: SharedAppState = AppState::new(config.clone(), db_pool).into();

    let addr: SocketAddr = config
        .server
        .address()
        .parse()
        .context("Server host/port produced invalid socket address")?;

    let app = Router::new()
        .merge(api_router())
        .with_state::<SharedAppState>(())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::clone(&shared_state));

    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("Failed to start server")?;

    Ok(())
}
