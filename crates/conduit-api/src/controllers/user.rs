use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    config::AppConfig,
    dto::{
        self,
        user::{
            LoginUserDto, LoginUserRequest, RegisterUserDto, RegisterUserRequest,
            UpdateUserRequest, UserResponse,
        },
    },
    errors::ConduitResult,
    extractors::authentication::UserID,
    jwt::{self, Claims},
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
    c: Option<UserID>,
) -> impl IntoResponse {
    println!("{:?}", c);
    "asd"
}

async fn update(
    Extension(repository): Extension<user::UserRepo>,
    c: UserID,
    Json(req): Json<UpdateUserRequest>,
) -> ConduitResult<Json<UserResponse>> {
    let user_id = "asd";
    todo!()
}
