use axum::routing::get;

pub fn router() -> axum::Router {
    axum::Router::new().route("/tags", get(list))
}

async fn list() {}
