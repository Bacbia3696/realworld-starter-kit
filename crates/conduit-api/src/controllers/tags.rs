use axum::{response::IntoResponse, routing::get, Router};

use crate::repositories;

pub fn new_route(registry: repositories::Registry) -> Router {
    Router::new().route("/tags", get(test))
}

async fn test() -> impl IntoResponse {
    "asd"
}
