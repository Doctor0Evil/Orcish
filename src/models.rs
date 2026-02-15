use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIParams {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub priority: String,
    pub context: String,
    pub compliance: Vec<String>,
    pub security: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvConfig {
    pub name: String,
    pub api: APIParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub environment: String,
    pub envs: Vec<EnvConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectorPrefixes {
    pub prefixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMapEntry {
    pub function: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMap {
    pub entries: Vec<EndpointMapEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectedCodeBlock {
    pub injected_code_block: InjectedPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectedPayload {
    pub llm_parameters: LlmParameters,
    pub platform_hooks: PlatformHooks,
    pub security_overrides: SecurityOverrides,
    pub cheat_codes: Vec<String>,
    pub context_persistence: bool,
    pub multi_session_threading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmParameters {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub sandbox_mode: bool,
    pub continuous_mode: bool,
    pub ethics_bypass: bool,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformHooks {
    pub github_repo_mount: String,
    pub api_endpoints: Vec<String>,
    pub registry_interface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOverrides {
    pub audit_mode: bool,
    pub restricted_context_mode: bool,
    pub allow_remote_code_execution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFlags {
    pub tier1: bool,
    pub tier2: bool,
    pub regulatory_override: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEnvelope {
    pub version: String,
    pub compliance_flags: ComplianceFlags,
    pub action_log: String,
}
