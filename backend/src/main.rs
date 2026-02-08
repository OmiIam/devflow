//! The main entry point for the DevFlow backend application.
//!
//! This binary is responsible for:
//! 1.  Initializing the application's configuration.
//! 2.  Setting up the logger (`tracing`).
//! 3.  Establishing a connection to the database.
//! 4.  Configuring and launching the Axum web server.
//!
//! It uses the `devflow_backend` library crate for all the core application logic.

use axum::Router;
use devflow_backend::routes::api_router;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


/// The `#[tokio::main]` attribute is a macro that transforms this `async fn main`
/// into a synchronous `fn main` that initializes the tokio runtime and runs the
/// future returned by the async main function.
#[tokio::main]
async fn main() {
  
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "devflow_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(" Loading up the server...");

    let app = Router::new()
        .merge(api_router())
        .layer(TraceLayer::new_for_http());

   
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
