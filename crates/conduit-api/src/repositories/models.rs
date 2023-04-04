use crate::dto::user::UserDto;
use sqlx::types::time::OffsetDateTime;

#[derive(Debug)]
pub struct UserEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            username: self.username,
            bio: Some(self.bio),
            image: Some(self.image),
            token,
        }
    }
}

pub struct Profile {}
pub struct Tag {}
pub struct Article {}
pub struct Comment {}
