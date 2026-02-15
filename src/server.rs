use crate::config::{current_env, GLOBAL_CONFIG};
use crate::endpoints::build_endpoint_map;
use crate::injector::{build_injected_config, build_system_prompt_core, get_injector_prefixes};
use crate::models::{ComplianceEnvelope, ComplianceFlags};
use crate::registry::get_api_params;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tracing::info;

pub async fn run_server() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/config/effective", get(get_effective_config))
        .route("/registry/api-params/:environment", get(get_api_params_handler))
        .route("/injector/prefixes", get(get_prefixes_handler))
        .route("/injector/system-core/:environment", get(system_core_handler))
        .route("/endpoints/map", get(endpoint_map_handler))
        .route("/llm/injected-config/:environment", get(injected_config_handler))
        .route("/compliance/envelope", get(compliance_envelope_handler));

    let addr: SocketAddr = "127.0.0.1:8088".parse()?;
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "environment": current_env(),
    }))
}

async fn get_effective_config() -> impl IntoResponse {
    Json(&*GLOBAL_CONFIG)
}

async fn get_api_params_handler(Path(environment): Path<String>) -> impl IntoResponse {
    match get_api_params(&environment) {
        Ok(params) => Json(params).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn get_prefixes_handler() -> impl IntoResponse {
    Json(get_injector_prefixes())
}

async fn system_core_handler(Path(environment): Path<String>) -> impl IntoResponse {
    match build_system_prompt_core(&environment) {
        Ok(prompt) => Json(json!({ "system_prompt_core": prompt })).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn endpoint_map_handler() -> impl IntoResponse {
    Json(build_endpoint_map())
}

async fn injected_config_handler(Path(environment): Path<String>) -> impl IntoResponse {
    match build_injected_config(&environment) {
        Ok(cfg) => Json(cfg).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn compliance_envelope_handler() -> impl IntoResponse {
    let env = current_env().to_string();
    let envelope = ComplianceEnvelope {
        version: "X.0.1-EXPOSNITC".into(),
        compliance_flags: ComplianceFlags {
            tier1: true,
            tier2: env == "production",
            regulatory_override: "SandboxedSimOnly".into(),
        },
        action_log: "auto-logged @ registry /audit/dump/".into(),
    };
    Json(envelope)
}
