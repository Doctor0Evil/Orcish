use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, macros::datetime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentKind {
    Dev,
    Staging,
    Production,
}

impl EnvironmentKind {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "staging" => EnvironmentKind::Staging,
            "production" | "prod" => EnvironmentKind::Production,
            _ => EnvironmentKind::Dev,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            EnvironmentKind::Dev => "dev",
            EnvironmentKind::Staging => "staging",
            EnvironmentKind::Production => "production",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub orcish_version: String,
    pub environment: String,
    pub timestamp: OffsetDateTime,
}

impl HealthStatus {
    pub fn ok(env: EnvironmentKind) -> Self {
        Self {
            status: "ok".into(),
            orcish_version: "0.1.0".into(),
            environment: env.as_str().into(),
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParam {
    pub key: String,
    pub required: bool,
    pub description: String,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParamsRegistry {
    pub environment: String,
    pub params: Vec<ApiParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectorPrefix {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCorePrompt {
    pub environment: String,
    pub core_blocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMapping {
    pub function: String,
    pub method: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectedConfig {
    pub environment: String,
    pub block: String,
}
