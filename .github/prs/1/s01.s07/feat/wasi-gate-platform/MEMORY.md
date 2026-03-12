# Memory: wasi-gate-platform (s07)

## Status: plan-complete

## Findings

### Platform-specific deps found

| Dep | Crates using it | Currently optional? |
|---|---|---|
| `keyring` | `but-secret`, `gitbutler-user` (direct), `gitbutler-testsupport` (test) | No |
| `notify` | `but-settings`, `gitbutler-filemonitor` (not in WASI tree) | No |
| `dirs` | `but-path`, `but` (via `dirs.workspace`), `but-claude`, `but-installer` | No |
| `open` | `but-path`, `but-api` | No (workspace dep) |
| `machine-uid` | `crates/but`, `crates/but-update` | No |
| `posthog-rs` | `crates/but` | No |
| `git2` | `crates/but` (unconditional) | No |
| `dirs-next` | `but-testing`, `gitbutler-cli` (both legacy, not in WASI tree) | No |

### Key secret handles in use

- `gitbutler_access_token` — user login token (BuildKind namespace)
- `github_access_token` — user's GitHub token (BuildKind namespace)
- `aiOpenAIKey` — OpenAI key (Global namespace)
- `aiAnthropicKey` — Anthropic key (Global namespace)
- `github_oauth_<user>`, `github_pat_<user>`, `github_enterprise_<host>` — forge tokens (BuildKind)
- `gitlab_pat_<user>`, `gitlab_selfhosted_<host>` — GitLab tokens (BuildKind)

### Callers of but-secret in the WASI dep tree

`but-llm`, `but-github`, `but-gitlab`, `but-update`, `but-api`, `gitbutler-user`

All of these either:
- Are in the `legacy` feature tree (excluded from WASI)
- Need the env-var fallback (LLM clients, update check, forge tokens)

### gix::credentials assessment

`gix::credentials::helper::Cascade` is used only inside `but-secret::git_credentials`, which
is only activated on `cfg!(debug_assertions) && cfg!(target_os = "macos")`. Dead code under WASI.
Must be gated with `#[cfg(feature = "native")]` to avoid keyring type references.

### notify usage

`watch_in_background()` spawns a blocking thread that loops on `rx.recv()`. Under WASI,
tokio's `spawn_blocking` may not work as expected without a multi-thread runtime. The entire
watcher is dispensable for stateless WASI invocations.

### Open usage in but-path vs but-api

- `but-path::AppChannel::open()` — launches GitButler GUI via deep link
- `but-api::legacy::open::open_url()`, `open_in_terminal()`, `show_in_finder()` — UI operations

Both are UI-facing and irrelevant for WASI MCP server mode.

## Decisions

- WASI secret fallback: env vars (host-injected). `persist()` and `delete()` are no-ops.
- Env-var naming: TBD — blocked on Q2 answer from coordinator.
- `but-update` disposition: prefer making it fully optional under `native` (Q1).
- `but-installer`: keep in tree, gate `dirs` inside it (Q3 option B).

## Errors & Fixes

(none — planning only)

## Blockers

- Q2 (env-var naming scheme) must be answered before implementation begins.
