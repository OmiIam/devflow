use crate::utils::{jwt, SharedAppState};
use axum::http::StatusCode;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<SharedAppState> for AuthenticatedUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedAppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".into(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid auth header".into()))?;

        let claims = jwt::validate_token(token, &state.config.auth)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

        Ok(AuthenticatedUser {
            user_id: claims.sub,
        })
    }
}
