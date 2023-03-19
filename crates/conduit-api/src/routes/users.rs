use axum::routing::{get, post};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/users/login", post(login))
        .route("/users", post(registration))
        .route("/user", get(current_user).put(update))
}

async fn login() {}
async fn registration() {}
async fn current_user() {}
async fn update() {}
