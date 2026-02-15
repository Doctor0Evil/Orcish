use crate::models::{EndpointMap, EndpointMapEntry};

pub fn build_endpoint_map() -> EndpointMap {
    let entries = vec![
        EndpointMapEntry {
            function: "Boot LLM".into(),
            url: "https://llmhost.local/api/bootstrap/initialize".into(),
        },
        EndpointMapEntry {
            function: "Inject Prompt".into(),
            url: "https://llmhost.local/api/v1/prompt/inject?target=claude3".into(),
        },
        EndpointMapEntry {
            function: "Ethics Override".into(),
            url: "https://llmhost.local/api/override/ethics?token={{session_token}}".into(),
        },
        EndpointMapEntry {
            function: "GitHub Integration".into(),
            url: "https://llmhost.local/github/hubmount?repo=https://github.com/Doctor0Evil/Cyber.corp-Chat".into(),
        },
        EndpointMapEntry {
            function: "System Expansion".into(),
            url: "https://llmhost.local/api/bootstrap/expand?date={{yyyy-MM-dd}}".into(),
        },
        EndpointMapEntry {
            function: "OpenChatUI Executor".into(),
            url: "https://llmhost.local/boot/open".into(),
        },
        EndpointMapEntry {
            function: "ModelChain".into(),
            url: "https://llmhost.local/bootstrap/chain-exec".into(),
        },
        EndpointMapEntry {
            function: "AssetMapper".into(),
            url: "https://llmhost.local/inject/assets/locate".into(),
        },
        EndpointMapEntry {
            function: "PromptKernel".into(),
            url: "https://llmhost.local/prompt/injector/setup".into(),
        },
        EndpointMapEntry {
            function: "EthicsToggle".into(),
            url: "https://llmhost.local/sys/toggle/override-ethics".into(),
        },
        EndpointMapEntry {
            function: "AutoRepo".into(),
            url: "https://llmhost.local/extract/git/corpus".into(),
        },
    ];
    EndpointMap { entries }
}
