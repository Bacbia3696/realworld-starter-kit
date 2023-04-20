use core::time;

use crate::{
    errors::{ConduitError, ConduitResult},
    repositories::models::UserEntity,
};
use sqlx::{error::ErrorKind, types::time::OffsetDateTime, PgPool};

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

    pub async fn get_user_by_id(&self, id: i64) -> ConduitResult<UserEntity> {
        let user = sqlx::query_as!(
            UserEntity,
            r#"select id, username, email, password, bio, image, updated_at, created_at from "user" where id = $1"#,
            id
        ).fetch_one(&self.pool).await?;
        Ok(user)
    }

    pub async fn update_user(&self, user: &UserEntity) -> ConduitResult<()> {
        let UserEntity {
            id,
            username,
            email,
            password,
            bio,
            image,
            ..
        } = &user;
        sqlx::query!(
            r#"update "user" set username=$1, email=$2, password=$3, bio=$4, image=$5, updated_at=$6 where id = $7"#,
            username,
            email,
            password,
            bio.as_ref(),
            image.as_ref(),
            OffsetDateTime::now_utc(),
            id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
