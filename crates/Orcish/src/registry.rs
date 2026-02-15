use crate::models::{ApiParam, ApiParamsRegistry, EnvironmentKind};

pub fn build_api_params_registry(env: EnvironmentKind) -> ApiParamsRegistry {
    let environment = env.as_str().to_string();

    let mut params = vec![
        ApiParam {
            key: "llm_model".into(),
            required: true,
            description: "Identifier of the LLM model to boot (e.g., orcish-guardian-1).".into(),
            example: Some("orcish-guardian-1".into()),
        },
        ApiParam {
            key: "temperature".into(),
            required: false,
            description: "Sampling temperature for creative vs. strict legal-orcish tone.".into(),
            example: Some("0.3".into()),
        },
        ApiParam {
            key: "max_tokens".into(),
            required: false,
            description: "Maximum tokens for a single generated reply, to protect mortal and orcish attention spans.".into(),
            example: Some("2048".into()),
        },
        ApiParam {
            key: "persona".into(),
            required: false,
            description: "Named persona profile (e.g., 'smiling_orc_magistrate', 'void_court_scribe').".into(),
            example: Some("smiling_orc_magistrate".into()),
        },
        ApiParam {
            key: "jurisdiction".into(),
            required: false,
            description: "Legal, cultural, or metaphysical domain the engine must respect.".into(),
            example: Some("sovereign_neuromorph_common_law".into()),
        },
    ];

    if matches!(env, EnvironmentKind::Production) {
        params.push(ApiParam {
            key: "audit_trace_id".into(),
            required: true,
            description: "External audit trace identifier for governance and compliance chains.".into(),
            example: Some("trace-orcish-0001".into()),
        });
    }

    ApiParamsRegistry { environment, params }
}
