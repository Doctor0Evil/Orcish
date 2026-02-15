mod config;
mod endpoints;
mod injector;
mod models;
mod registry;
mod server;

use crate::config::OrcishConfig;
use server::run_server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let config = OrcishConfig::load();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    tracing::info!(
        "Booting ORCISH node – environment={}, host={}, port={}",
        config.environment,
        config.http.host,
        config.http.port
    );

    run_server(&config.http.host, config.http.port).await;
}
