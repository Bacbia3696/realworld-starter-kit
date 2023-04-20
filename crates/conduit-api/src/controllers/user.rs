use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    config::AppConfig,
    dto::user::{
        LoginUserDto, LoginUserRequest, RegisterUserDto, RegisterUserRequest, UpdateUserRequest,
        UserDto, UserResponse,
    },
    errors::ConduitResult,
    extractors::authentication::UserID,
    jwt,
    repositories::{self, user},
};

pub fn new_route(registry: repositories::Registry, config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
        .route("/user", get(get_current).put(update))
        .layer(Extension(registry.user))
        .layer(Extension(jwt::JwtService::new(config)))
}

async fn login(
    Extension(jwt_service): Extension<jwt::JwtService>,
    Extension(repository): Extension<user::UserRepo>,
    Json(req): Json<LoginUserRequest>,
) -> ConduitResult<Json<UserResponse>> {
    let LoginUserRequest {
        user: LoginUserDto { email, password },
    } = &req;

    let user = repository.get_user(email, password).await?;
    let token = jwt_service.new_token(user.id, &user.email)?;
    Ok(Json(UserResponse {
        user: user.into_dto(token),
    }))
}

async fn register(
    Extension(repository): Extension<user::UserRepo>,
    Json(req): Json<RegisterUserRequest>,
) -> ConduitResult<Json<UserResponse>> {
    let RegisterUserRequest {
        user:
            RegisterUserDto {
                username,
                email,
                password,
            },
    } = &req;
    let user = repository.create_user(username, email, password).await?;
    Ok(Json(UserResponse {
        user: user.into_dto("token".to_string()),
    }))
}

async fn get_current(
    Extension(repository): Extension<user::UserRepo>,
    user_id: UserID,
) -> ConduitResult<Json<UserResponse>> {
    let current_user = repository.get_user_by_id(user_id.0).await?;
    Ok(Json(UserResponse {
        user: UserDto {
            id: user_id.0,
            username: current_user.username,
            email: current_user.email,
            bio: current_user.bio,
            image: current_user.image,
            token: String::new(),
        },
    }))
}

async fn update(
    Extension(repository): Extension<user::UserRepo>,
    user_id: UserID,
    Json(req): Json<UpdateUserRequest>,
) -> ConduitResult<Json<UserResponse>> {
    let mut current_user = repository.get_user_by_id(user_id.0).await?;
    let user = req.user;
    if let Some(username) = user.username {
        current_user.username = username
    }
    if let Some(password) = user.password {
        current_user.password = password
    }
    if let Some(bio) = user.bio {
        current_user.bio = Some(bio)
    }
    if let Some(image) = user.image {
        current_user.image = Some(image)
    }
    if let Some(email) = user.email {
        current_user.email = email
    }
    repository.update_user(&current_user).await?;
    Ok(Json(UserResponse {
        user: UserDto {
            id: current_user.id,
            username: current_user.username,
            email: current_user.email,
            bio: current_user.bio,
            image: current_user.image,
            token: "".to_owned(),
        },
    }))
}
