mod config;
mod endpoints;
mod injector;
mod models;
mod registry;
mod server;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    server::run_server().await
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,axum=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
