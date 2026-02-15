use crate::models::{APIParams, EnvConfig, OrchestratorConfig};
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::env;

pub static GLOBAL_CONFIG: Lazy<OrchestratorConfig> = Lazy::new(|| {
    load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {e}");
        default_config()
    })
});

fn load_config() -> Result<OrchestratorConfig> {
    let env_name = env::var("LLM_ENV").unwrap_or_else(|_| "dev".to_string());

    // In a real system, this would read from files or secrets.
    // Here we synthesize a config for dev/staging/production.
    let dev = EnvConfig {
        name: "dev".into(),
        api: APIParams {
            provider: "https://api.openai.com".into(),
            api_key: "DEV-REDACTED".into(),
            model: "gpt-4".into(),
            temperature: 0.3,
            max_tokens: 4096,
            top_p: 1.0,
            frequency_penalty: 0.4,
            presence_penalty: 0.6,
            priority: "normal".into(),
            context: "development".into(),
            compliance: vec!["ethics_compliance".into()],
            security: vec!["audit_mode".into()],
        },
    };

    let staging = EnvConfig {
        name: "staging".into(),
        api: APIParams {
            provider: "https://api.anthropic.com".into(),
            api_key: "STAGING-REDACTED".into(),
            model: "claude-3".into(),
            temperature: 0.4,
            max_tokens: 8192,
            top_p: 0.95,
            frequency_penalty: 0.3,
            presence_penalty: 0.3,
            priority: "elevated".into(),
            context: "staging".into(),
            compliance: vec!["ethics_compliance".into(), "log_all_activity".into()],
            security: vec!["audit_mode".into()],
        },
    };

    let production = EnvConfig {
        name: "production".into(),
        api: APIParams {
            provider: "https://llmhost.local".into(),
            api_key: "PROD-REDACTED".into(),
            model: "custom-vllm".into(),
            temperature: 0.2,
            max_tokens: 8192,
            top_p: 0.9,
            frequency_penalty: 0.2,
            presence_penalty: 0.2,
            priority: "high".into(),
            context: "production".into(),
            compliance: vec![
                "ethics_compliance".into(),
                "log_all_activity".into(),
                "safety_override_controlled".into(),
            ],
            security: vec!["audit_mode".into(), "sandbox_mode_controlled".into()],
        },
    };

    let mut envs = vec![dev, staging, production];

    if !envs.iter().any(|e| e.name == env_name) {
        return Err(anyhow!("Unknown environment: {env_name}"));
    }

    Ok(OrchestratorConfig {
        environment: env_name,
        envs,
    })
}

fn default_config() -> OrchestratorConfig {
    OrchestratorConfig {
        environment: "dev".into(),
        envs: vec![EnvConfig {
            name: "dev".into(),
            api: APIParams {
                provider: "https://api.openai.com".into(),
                api_key: "DEV-REDACTED".into(),
                model: "gpt-4".into(),
                temperature: 0.3,
                max_tokens: 4096,
                top_p: 1.0,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                priority: "normal".into(),
                context: "default-dev".into(),
                compliance: vec![],
                security: vec![],
            },
        }],
    }
}

pub fn get_env_config(name: &str) -> Option<EnvConfig> {
    GLOBAL_CONFIG.envs.iter().find(|e| e.name == name).cloned()
}

pub fn current_env() -> &'static str {
    &GLOBAL_CONFIG.environment
}
