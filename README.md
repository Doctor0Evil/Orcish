# llm-orchestrator-node

A Rust-based **LLM Orchestrator Node** that:

- Loads base configuration per environment (`dev`, `staging`, `production`).
- Verifies API parameters against a THE_API_ZONE-style schema.
- Manages and exposes **injector prefixes** such as:
  - `inject:system_prompt:API_Key_Prompts`
  - `inject:system_prompt:Endpoint_Mapper`
  - `inject:system_prompt:Security_Audit`
  - `inject:system_prompt:Registry_Dumper`
  - `inject:system_prompt:Token_Limiter`
  - `inject:system_prompt:Contextualizer`
  - `inject:system_prompt:Model_Lock`
- Generates and serves **API Endpoint URLs** for:
  - Booting LLMs
  - Injecting prompts
  - Ethics override toggles
  - GitHub repository mounting
  - System expansion by date
- Provides a JSON API for LLM engines and orchestration frameworks.

## Core Concepts

- **APIParams Registry** – Inspired by `THE_API_ZONE` Solidity contract, mapped into Rust.
- **Injector Prefixes** – First-class descriptors for system prompt injection.
- **Environment-Aware Config** – Config is resolved through env var `LLM_ENV` (`dev|staging|production`).
- **Human-Integrated-Technology License** – Non-fictive, reality-anchored usage focus.

## Running

```bash
export LLM_ENV=dev
cargo run
Server defaults to 127.0.0.1:8088.

Key routes:

GET /health – health check

GET /config/effective – resolved environment configuration

GET /registry/api-params/:environment – API params for dev|staging|production

GET /injector/prefixes – list injector prefixes

GET /injector/system-core/:environment – assembled system prompt core

GET /endpoints/map – generated function → endpoint map

GET /llm/injected-config/:environment – injected code block for LLM engines
