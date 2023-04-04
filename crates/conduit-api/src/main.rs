use std::env;

use crate::controllers::{profile, user};
use axum::{
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    routing::get,
    RequestPartsExt, TypedHeader,
};
use errors::{ConduitError, OurReport};
use http::request::Parts;
use tower_http::trace::TraceLayer;

mod config;
mod controllers;
mod dto;
mod errors;
mod repositories;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    let service_registry = repositories::Registry::new(pool);

    let rout = axum::Router::new()
        .route("/", get(root))
        .merge(user::new_route(service_registry.clone()))
        .merge(profile::new_route(service_registry.clone()));

    let app: _ = axum::Router::new()
        .nest("/api", rout)
        // .route("/api/profiles/:username", get(root))
        // .route("/api/profiles/:username/follow", post(root).delete(root))
        // .route("/api/articles/feed", get(root))
        // .route("/api/articles", get(root).post(root))
        // .route("/api/articles/:slug", get(root).put(root).delete(root))
        // .route("/api/articles/:slug/comments", get(root).post(root))
        // .route("/api/articles/:slug/comments/:id", delete(root))
        // .route("/api/articles/:slug/favorite", post(root).delete(root))
        // .route("/api/tags", get(root))
        // Enables logging. Use `RUST_LOG=tower_http=debug`
        .layer(TraceLayer::new_for_http());

    let addr = "127.0.0.1:8081";
    tracing::info!("listening on http://{}...", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> impl IntoResponse {
    tracing::info!("root api");
    "Hello world!"
}

fn init() {
    // load env from .env
    dotenvy::dotenv().expect(".env file not found");
    // tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .init();
    color_eyre::install().unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection = OurReport;

    /// Perform the extraction.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // extract token from authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ConduitError::Unauthorized)?;
        println!("token: {:?}", bearer);
        Ok(Claims {
            sub: "fuck".to_string(),
            company: "com".to_string(),
            exp: 0,
        })
    }
}
