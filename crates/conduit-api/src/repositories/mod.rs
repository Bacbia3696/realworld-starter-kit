use sqlx::PgPool;

pub mod article;
pub mod comment;
pub mod profile;
pub mod tag;
pub mod user;

mod models;

#[derive(Clone)]
pub struct Registry {
    pub user: user::UserRepo,
}

impl Registry {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user: user::UserRepo::new(pool),
        }
    }
}
