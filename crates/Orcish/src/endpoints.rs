use crate::injector::{injector_prefixes, injected_config_block, system_core_prompt};
use crate::models::{
    EndpointMapping, EnvironmentKind, HealthStatus, InjectedConfig,
};
use crate::registry::build_api_params_registry;
use axum::{extract::Path, http::StatusCode, Json};
use std::collections::HashMap;

pub async fn health_handler() -> Json<HealthStatus> {
    Json(HealthStatus::ok(EnvironmentKind::Dev))
}

pub async fn effective_config_handler() -> Json<HashMap<&'static str, String>> {
    // For now this is a shallow projection. Real config comes from src/config.rs.
    let mut map = HashMap::new();
    map.insert("environment", "dev".into());
    map.insert("http_host", "127.0.0.1".into());
    map.insert("http_port", "8088".into());
    Json(map)
}

pub async fn api_params_handler(Path(env): Path<String>) -> Json<serde_json::Value> {
    let env_kind = EnvironmentKind::from_str(&env);
    let registry = build_api_params_registry(env_kind);
    Json(serde_json::to_value(registry).unwrap())
}

pub async fn injector_prefixes_handler() -> Json<serde_json::Value> {
    let prefixes = injector_prefixes();
    Json(serde_json::to_value(prefixes).unwrap())
}

pub async fn system_core_handler(Path(env): Path<String>) -> Json<serde_json::Value> {
    let env_kind = EnvironmentKind::from_str(&env);
    let core = system_core_prompt(env_kind);
    Json(serde_json::to_value(core).unwrap())
}

pub async fn endpoints_map_handler() -> Json<Vec<EndpointMapping>> {
    let mappings = vec![
        EndpointMapping {
            function: "health".into(),
            method: "GET".into(),
            path: "/health".into(),
        },
        EndpointMapping {
            function: "effective_config".into(),
            method: "GET".into(),
            path: "/config/effective".into(),
        },
        EndpointMapping {
            function: "api_params".into(),
            method: "GET".into(),
            path: "/registry/api-params/:environment".into(),
        },
        EndpointMapping {
            function: "injector_prefixes".into(),
            method: "GET".into(),
            path: "/injector/prefixes".into(),
        },
        EndpointMapping {
            function: "system_core".into(),
            method: "GET".into(),
            path: "/injector/system-core/:environment".into(),
        },
        EndpointMapping {
            function: "llm_injected_config".into(),
            method: "GET".into(),
            path: "/llm/injected-config/:environment".into(),
        },
    ];
    Json(mappings)
}

pub async fn llm_injected_config_handler(Path(env): Path<String>) -> Result<Json<InjectedConfig>, StatusCode> {
    let env_kind = EnvironmentKind::from_str(&env);
    let block = injected_config_block(env_kind);
    Ok(Json(InjectedConfig {
        environment: env,
        block,
    }))
}
