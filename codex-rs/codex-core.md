# codex-core: Public API and Embedding Overview

## Summary

- The `codex-core` crate is already usable as a standalone, embeddable agent engine for Rust/native apps.
- It exposes a clean SQ/EQ protocol (`protocol::Op`/`EventMsg`) plus configuration, sandboxed execution utilities, and a simple spawn API.

## Core Integration

- Codex engine: `Codex::spawn(config, auth, ctrl_c)` returns `CodexSpawnOk`; use `codex.submit(op)` and `codex.next_event()` to drive the agent.
- Convenience bootstrap: `codex_wrapper::init_codex(config)` returns `CodexConversation` including the initial `SessionConfigured` event.
- Protocol contract: `protocol::{Submission, Op, Event, EventMsg}` gives a stable wire-model for building UIs, CLIs, or services that send input and consume agent events.

## Configuration

- High-level loader: `config::Config::load_with_cli_overrides(cli_overrides, ConfigOverrides)` merges `~/.codex/config.toml`, ad‑hoc overrides, and strong typed overrides.
- Overrides: `config::ConfigOverrides` supports model, provider, cwd, sandbox mode, reasoning, and UI toggles.
- Provider selection: `model_provider_info::{ModelProviderInfo, WireApi, built_in_model_providers(), create_oss_provider_with_base_url(..)}` lets apps target OpenAI, ChatGPT, OSS gateways, etc.
- Policy types: `protocol::{AskForApproval, SandboxPolicy}` with constructors (`new_read_only_policy`, `new_workspace_write_policy`) and helpers like `get_writable_roots_with_cwd`.

## Execution & Sandbox Utilities

- Shell exec wrapper: `exec::process_exec_tool_call(ExecParams, SandboxType, ...) -> ExecToolCallOutput` runs commands with timeouts and incremental stdout/stderr streaming.
- Sandbox backends: `exec::SandboxType::{None, MacosSeatbelt, LinuxSeccomp}` plus `seatbelt` and Linux helper integration via `codex-linux-sandbox` path in `Config`.
- Env construction: `exec_env::create_env` builds a process env from `ShellEnvironmentPolicy` (inherit/exclude/include/set).
- Shell helpers: `shell::default_user_shell()` and `Shell::format_default_shell_invocation(..)` for zsh‑profile-aware execution.
- Safety helpers: `get_platform_sandbox()` (re‑exported) to detect/choose platform sandbox behavior.

## Model Tools & MCP

- Tool exposure for models: `openai_tools::get_openai_tools` converts Codex tool config and discovered MCP tools into OpenAI Functions/Local Shell tools the model can call.
- MCP events: `EventMsg::{McpToolCallBegin,McpToolCallEnd}` let UIs reflect external tool invocations; configuration via `Config.mcp_servers`.

## Conversation & Content Types

- Input/response shapes: `models::{InputItem, ResponseItem, ResponseInputItem, ContentItem}` align with Responses/Chat APIs and include function/local shell call handling.
- Patch workflow: `ApplyPatchApprovalRequest`, `PatchApplyBegin/End`, and `TurnDiff` events give UIs a way to preview/approve/apply code changes.

## Public API Viability Today

- Yes: The crate exposes the right primitives to embed Codex:
  - Drive sessions via `Codex` with `protocol` types.
  - Load and customize configuration.
  - Execute sandboxed commands independently of the model if needed.
  - Surface all agent events in a UI/CLI without touching internals.
- Forward-compat: `Op` and several enums are `non_exhaustive`, signaling growth while keeping consumers resilient.

## Notes

- Platform expectations: macOS uses `/usr/bin/sandbox-exec`; Linux expects a `codex-linux-sandbox` helper for `SandboxType::LinuxSeccomp`.
- Auth: `Codex::spawn` accepts `Option<CodexAuth)`; `codex_wrapper::init_codex` reads auth from `CODEX_HOME`, but apps can supply their own.
- Internals remain private (client, chat_completions, session state), which is appropriate — consumers use the public protocol/events rather than hooking internals.

