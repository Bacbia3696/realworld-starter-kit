use serde::{Deserialize, Serialize};

// ====== User ===============
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserResponse {
    pub user: UserDto,
}
// ===========================================

// ==== Registration =====
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisterUserRequest {
    pub user: RegisterUserDto,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RegisterUserResponse {
    pub user: UserDto,
}
// ===========================================

// ==== Login =====
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginUserRequest {
    pub user: LoginUserDto,
}
// ===========================================

// ==== Update User =====
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}
// ===========================================
