use crate::endpoints::*;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/config/effective", get(effective_config_handler))
        .route("/registry/api-params/:environment", get(api_params_handler))
        .route("/injector/prefixes", get(injector_prefixes_handler))
        .route("/injector/system-core/:environment", get(system_core_handler))
        .route("/endpoints/map", get(endpoints_map_handler))
        .route("/llm/injected-config/:environment", get(llm_injected_config_handler))
        .layer(TraceLayer::new_for_http())
}

pub async fn run_server(host: &str, port: u16) {
    let app = build_router();
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("invalid socket address for Orcish");

    tracing::info!("orcish-node listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("failed to bind orcish socket"),
        app,
    )
    .await
    .expect("orcish server crashed");
}
