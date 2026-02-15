use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers::auth_handler, utils::SharedAppState};

pub fn router() -> Router<SharedAppState> {
    Router::new()
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/me", get(auth_handler::me))
        .route("/auth/logout", post(auth_handler::logout))
}
