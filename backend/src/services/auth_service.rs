use crate::{
    config::AuthConfig,
    dto::auth::{AuthResponse, AuthUserDto, LoginRequest, RegisterRequest},
    models::user::User,
    repositories::user_repository::UserRepository,
    utils::{jwt, password},
};
use sqlx;
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

/// Auth-specific business logic.
pub struct AuthService<'a> {
    repo: UserRepository<'a>,
    auth_config: &'a AuthConfig,
}

impl<'a> AuthService<'a> {
    pub fn new(repo: UserRepository<'a>, auth_config: &'a AuthConfig) -> Self {
        Self { repo, auth_config }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AuthServiceError> {
        req.validate().map_err(AuthServiceError::ValidationFailed)?;

        if self.repo.find_by_email(&req.email).await?.is_some() {
            return Err(AuthServiceError::EmailAlreadyExists);
        }

        let password_hash = password::hash_password(&req.password)?;
        let user = self
            .repo
            .create(Uuid::new_v4(), &req.email, &password_hash, &req.name)
            .await?;

        self.build_auth_response(&user)
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AuthServiceError> {
        req.validate().map_err(AuthServiceError::ValidationFailed)?;
        let user = self
            .repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AuthServiceError::InvalidCredentials)?;

        password::verify_password(&user.password_hash, &req.password)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;

        self.build_auth_response(&user)
    }

    fn build_auth_response(&self, user: &User) -> Result<AuthResponse, AuthServiceError> {
        let token = jwt::generate_token(user.id, self.auth_config)?;
        let dto = AuthUserDto {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        };
        Ok(AuthResponse { user: dto, token })
    }
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("validation failed")]
    ValidationFailed(#[from] validator::ValidationErrors),
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("password error")]
    Password(#[from] password::PasswordError),
    #[error("token error")]
    Token(#[from] jwt::JwtError),
}
