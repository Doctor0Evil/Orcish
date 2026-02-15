use crate::config::get_env_config;
use crate::models::APIParams;
use anyhow::{anyhow, Result};

pub fn get_api_params(environment: &str) -> Result<APIParams> {
    let env_cfg = get_env_config(environment)
        .ok_or_else(|| anyhow!("Unknown environment: {environment}"))?;
    validate_api_params(&env_cfg.api)?;
    Ok(env_cfg.api)
}

fn validate_api_params(params: &APIParams) -> Result<()> {
    if params.provider.is_empty() {
        return Err(anyhow!("provider must not be empty"));
    }
    if params.model.is_empty() {
        return Err(anyhow!("model must not be empty"));
    }
    if !(0.0..=1.0).contains(&params.temperature) {
        return Err(anyhow!("temperature must be between 0.0 and 1.0"));
    }
    if params.max_tokens == 0 {
        return Err(anyhow!("max_tokens must be > 0"));
    }
    Ok(())
}
