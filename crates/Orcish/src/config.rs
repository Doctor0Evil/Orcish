use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrcishConfig {
    pub environment: String,
    pub http: HttpConfig,
    pub registry_seed: String,
}

impl OrcishConfig {
    pub fn load() -> Self {
        let env_key = "LLM_ENV";
        let environment = env::var(env_key).unwrap_or_else(|_| "dev".to_string());

        let mut settings = config::Config::builder()
            .set_default("environment", environment.clone())
            .unwrap()
            .set_default("http.host", "127.0.0.1")
            .unwrap()
            .set_default("http.port", 8088)
            .unwrap()
            .set_default("registry_seed", "orcish-default-seed")
            .unwrap();

        // Optional env override: ORCISH_HTTP_PORT, ORCISH_HTTP_HOST
        if let Ok(host) = env::var("ORCISH_HTTP_HOST") {
            settings = settings.set_override("http.host", host).unwrap();
        }
        if let Ok(port) = env::var("ORCISH_HTTP_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                settings = settings.set_override("http.port", port_num).unwrap();
            }
        }

        let settings = settings.build().expect("failed to build Orcish config");
        settings
            .try_deserialize::<OrcishConfig>()
            .expect("failed to deserialize Orcish config")
    }
}
