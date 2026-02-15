use crate::{
    dto::auth::{AuthResponse, LoginRequest, LogoutResponse, MeResponse, RegisterRequest},
    middleware::auth::AuthenticatedUser,
    services::auth_service::{AuthService, AuthServiceError},
    utils::SharedAppState,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn register(
    State(state): State<SharedAppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let service = AuthService::new(state.user_repo(), &state.config.auth);
    service
        .register(payload)
        .await
        .map(Json)
        .map_err(auth_error_to_response)
}

pub async fn login(
    State(state): State<SharedAppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let service = AuthService::new(state.user_repo(), &state.config.auth);
    service
        .login(payload)
        .await
        .map(Json)
        .map_err(auth_error_to_response)
}

pub async fn me(
    State(state): State<SharedAppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<Json<MeResponse>, (StatusCode, String)> {
    let repo = state.user_repo();
    let user = repo
        .find_by_id(user_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()))?
        .ok_or((StatusCode::UNAUTHORIZED, "User not found".into()))?;

    let dto = MeResponse {
        user: crate::dto::auth::AuthUserDto {
            id: user.id,
            name: user.name,
            email: user.email,
        },
    };
    Ok(Json(dto))
}

pub async fn logout(_user: AuthenticatedUser) -> Json<LogoutResponse> {
    Json(LogoutResponse {
        message: "success".into(),
    })
}

fn auth_error_to_response(err: AuthServiceError) -> (StatusCode, String) {
    match err {
        AuthServiceError::ValidationFailed(_) => (StatusCode::BAD_REQUEST, "Invalid input".into()),
        AuthServiceError::EmailAlreadyExists => {
            (StatusCode::CONFLICT, "Email already exists".into())
        }
        AuthServiceError::InvalidCredentials => {
            (StatusCode::UNAUTHORIZED, "Invalid credentials".into())
        }
        AuthServiceError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
        }
        AuthServiceError::Password(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Password error".into())
        }
        AuthServiceError::Token(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Token error".into()),
    }
}
