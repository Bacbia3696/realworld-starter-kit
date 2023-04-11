use std::{env, sync::Arc};

use axum::{response::IntoResponse, routing::get};
use clap::Parser;
use tower_http::trace::TraceLayer;

use crate::{
    config::AppConfig,
    controllers::{profile, user},
};

mod config;
mod controllers;
mod dto;
mod errors;
mod extractors;
mod jwt;
mod repositories;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init();

    let config = Arc::new(AppConfig::parse());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    let service_registry = repositories::Registry::new(pool);

    let rout = axum::Router::new()
        .route("/", get(root))
        .merge(user::new_route(service_registry.clone(), config))
        .merge(profile::new_route(service_registry.clone()));

    let app = axum::Router::new()
        .nest("/api", rout)
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
