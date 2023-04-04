use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::repositories;

pub fn new_route(_registry: repositories::Registry) -> Router {
    // .route("/api/profiles/:username", get(root))
    // .route("/api/profiles/:username/follow", post(root).delete(root))
    Router::new()
        .route("/profiles/:username", get(get_by_username))
        .route(
            "/profiles/:username/follow",
            post(get_by_username).delete(get_by_username),
        )
}

async fn get_by_username() -> impl IntoResponse {
    "asd"
}
