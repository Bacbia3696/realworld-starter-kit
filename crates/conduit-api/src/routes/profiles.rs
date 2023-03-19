use axum::routing::{delete, get, post, put};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", get(root))
        .route("/profiles/:username/follow", post(follow_user))
        .route("/profiles/:username/follow", delete(unfollow_user))
        .route("/", get(root))
        .route("/", get(root))
}

async fn root() {}
async fn follow_user() {}
async fn unfollow_user() {}
