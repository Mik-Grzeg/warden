use std::net::SocketAddr;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

use axum::{response::IntoResponse, Json};
use axum::{routing::get, Router};

pub async fn health() -> impl IntoResponse {
    Json("healthy")
}

fn init_tracing() {
    let logger = tracing_subscriber::fmt::layer().compact();
    let env_filter = EnvFilter::try_from_default_env()
        .or(EnvFilter::try_new("debug"))
        .unwrap();

    let collector = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(collector).unwrap();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/health", get(health))
                .into_make_service(),
        )
        .await
        .unwrap()
}
