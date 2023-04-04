use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}

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
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LoginUserRespone {
    pub user: UserDto,
}
// ===========================================

// ==== Update User =====
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateUserDto {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateUserRequest {
    pub user: LoginUserDto,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateUserResponse {
    pub user: UserDto,
}
// ===========================================
