use crate::{
    dto::user::{
        LoginUserDto, LoginUserRequest, LoginUserRespone, RegisterUserDto, RegisterUserRequest,
        RegisterUserResponse,
    },
    errors::ConduitResult,
    repositories::{self, user},
    Claims,
};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

pub fn new_route(registry: repositories::Registry) -> Router {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
        .route("/user", get(get_current).put(update))
        .layer(Extension(registry.user))
}

async fn login(
    Extension(repository): Extension<user::UserRepo>,
    Json(req): Json<LoginUserRequest>,
) -> ConduitResult<Json<LoginUserRespone>> {
    let LoginUserRequest {
        user: LoginUserDto { email, password },
    } = &req;

    let user = repository.get_user(email, password).await?;
    Ok(Json(LoginUserRespone {
        user: user.into_dto("token".to_string()),
    }))
}

async fn register(
    Extension(repository): Extension<user::UserRepo>,
    Json(req): Json<RegisterUserRequest>,
) -> ConduitResult<Json<RegisterUserResponse>> {
    let RegisterUserRequest {
        user:
            RegisterUserDto {
                username,
                email,
                password,
            },
    } = &req;
    let user = repository.create_user(username, email, password).await?;
    Ok(Json(RegisterUserResponse {
        user: user.into_dto("token".to_string()),
    }))
}

async fn get_current(
    Extension(repository): Extension<user::UserRepo>,
    c: Claims,
) -> impl IntoResponse {
    println!("{:?}", c);
    "asd"
}

async fn update() -> impl IntoResponse {
    "asd"
}
