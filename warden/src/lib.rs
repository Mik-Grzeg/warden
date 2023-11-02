use std::net::SocketAddr;

use tracing_subscriber::{prelude::*, EnvFilter, Registry};

pub mod controller;
mod handlers;

fn init_tracing() {
    let logger = tracing_subscriber::fmt::layer().compact();
    let env_filter = EnvFilter::try_from_default_env()
        .or(EnvFilter::try_new("info"))
        .unwrap();

    let collector = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(collector).unwrap();
}

pub async fn app() -> anyhow::Result<()> {
    // initialize tracing
    init_tracing();

    // build our application with a route
    let app = handlers::router::router();

    let controller = controller::run();

    // run our app with hyper, listening globally on port 8000
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("Listening on {}", addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    tokio::join!(controller, server).1?;
    Ok(())
}
