use crate::{models::user::User, utils::AppState};
use sqlx::PgPool;
use uuid::Uuid;

/// Repository responsible for reading/writing users.
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        email: &str,
        password_hash: &str,
        name: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"INSERT INTO users (id, email, password_hash, name)
               VALUES ($1, $2, $3, $4)
               RETURNING *"#,
        )
        .bind(user_id)
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .fetch_one(self.pool)
        .await
    }
}

impl AppState {
    pub fn user_repo(&self) -> UserRepository<'_> {
        UserRepository::new(&self.db_pool)
    }
}
