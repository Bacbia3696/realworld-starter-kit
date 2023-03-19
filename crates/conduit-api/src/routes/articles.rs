use axum::routing::{delete, get, post};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/articles", get(list).post(create))
        .route("/articles/feed", get(feed))
        .route("/articles/:slug", get(get_by_slug).post(create))
        .route("/articles/:slug/favorite", post(favorite))
        .route("/articles/:slug/comments", post(add_comment))
        .route("/articles/:slug/comments/:id", delete(delete_comment))
}

async fn list() {}
async fn feed() {}
async fn get_by_slug() {}
async fn update() {}
async fn create() {}
async fn add_comment() {}
async fn favorite() {}
async fn delete_comment() {}
