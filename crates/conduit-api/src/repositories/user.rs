use crate::{
    errors::{ConduitError, ConduitResult},
    repositories::models::UserEntity,
};
use sqlx::{error::ErrorKind, PgPool};

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepo {
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        hashed_password: &str,
    ) -> ConduitResult<UserEntity> {
        sqlx::query_as!(
            UserEntity,
            r#"insert into "user"(username, email, password) values($1, $2, $3) returning *"#,
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            if let Some(err) = err.as_database_error() {
                if err.kind() == ErrorKind::UniqueViolation {
                    return ConduitError::ExistedUser.into();
                }
            }
            ConduitError::from(err).into()
        })
    }

    pub async fn get_user(&self, email: &str, password: &str) -> ConduitResult<UserEntity> {
        sqlx::query_as!(
            UserEntity,
            r#"select id, username, email, password, bio, image, updated_at, created_at from "user" where email = $1 and password = $2"#,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| {
            if let sqlx::Error::RowNotFound = err {
                ConduitError::InvalidCredential.into()
            } else {
                ConduitError::from(err).into()
            }
        })
    }
}
