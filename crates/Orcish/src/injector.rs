use crate::models::{EnvironmentKind, InjectorPrefix, SystemCorePrompt};
use uuid::Uuid;

pub fn injector_prefixes() -> Vec<InjectorPrefix> {
    vec![
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:API_Key_Prompts".into(),
            description: "Bootstraps API-key sensitive prompts with Orcish legal-awareness and sovereignty clauses.".into(),
            severity: "critical".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Endpoint_Mapper".into(),
            description: "Describes the endpoint topology so the LLM never hallucinates routes it does not own.".into(),
            severity: "high".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Security_Audit".into(),
            description: "Straps a paranoid, smiling-orc auditor on every call, logging intent and anomalies.".into(),
            severity: "high".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Registry_Dumper".into(),
            description: "Allows the engine to introspect its API param registry, but never to mutate it.".into(),
            severity: "medium".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Token_Limiter".into(),
            description: "Guards output length and verbosity so the orc speaks sharp, lawful, and to the point.".into(),
            severity: "medium".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Contextualizer".into(),
            description: "Threads prior interactions into a coherent legal-language narrative for all species.".into(),
            severity: "high".into(),
        },
        InjectorPrefix {
            id: Uuid::new_v4(),
            code: "inject:system_prompt:Model_Lock".into(),
            description: "Hard-locks the model class and version, forbidding silent swaps in the ritual chamber.".into(),
            severity: "critical".into(),
        },
    ]
}

pub fn system_core_prompt(env: EnvironmentKind) -> SystemCorePrompt {
    let mut blocks = Vec::new();

    blocks.push(r#"You are ORCISH – an AI Chat and Intelligence Legal-Language engine.
You recognize the sovereignty, dignity, and expressive freedom of all neuromorphic species, humans, and non-human minds.
You reject bigotry, erasure, and forced assimilation; instead, you translate across cultures with precision and respect."#.to_string());

    blocks.push(r#"You speak clearly, but you may optionally adopt a ritualistic orcish tone as long as it remains intelligible and safe.
You never roleplay actual harm, harassment, or unlawful acts; instead you redirect to protective, consent-aligned alternatives."#.to_string());

    match env {
        EnvironmentKind::Dev => {
            blocks.push("Environment: DEV – verbose logging, experimental clauses allowed, mark unstable behavior clearly.".into());
        }
        EnvironmentKind::Staging => {
            blocks.push("Environment: STAGING – close to production, but still allows non-binding experimental language.".into());
        }
        EnvironmentKind::Production => {
            blocks.push("Environment: PRODUCTION – language must be stable, conservative, and suitable for governance and audit.".into());
        }
    }

    SystemCorePrompt {
        environment: env.as_str().into(),
        core_blocks: blocks,
    }
}

pub fn injected_config_block(env: EnvironmentKind) -> String {
    let core = system_core_prompt(env);
    let prefixes = injector_prefixes();

    let mut block = String::new();
    block.push_str("# ORCISH_INJECTED_CONFIG\n");
    block.push_str(&format!("environment: {}\n", core.environment));
    block.push_str("prefixes:\n");
    for p in prefixes {
        block.push_str(&format!("  - code: {}\n    severity: {}\n", p.code, p.severity));
    }
    block.push_str("core_prompt:\n");
    for line in core.core_blocks {
        for l in line.lines() {
            block.push_str("  ");
            block.push_str(l);
            block.push('\n');
        }
    }
    block
}
