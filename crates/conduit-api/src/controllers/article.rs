use axum::{
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};

use crate::repositories;

pub fn new_route(registry: repositories::Registry) -> Router {
    Router::new()
        .route("/articles", get(test).post(test))
        .route("/articles/:slug", get(test).put(test).delete(test))
        .route("/articles/:slug/comments", get(test).post(test))
        .route("/articles/:slug/comments/:id", delete(test))
        .route("/articles/:slug/favorite", post(test).delete(test))
}

async fn test() -> impl IntoResponse {
    "asd"
}
