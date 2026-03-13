# Memory: wasi-gate-networking (s06)

## Status: plan-complete

## Decisions

- Strategy: make network crates optional in `crates/but/Cargo.toml`, not add feature flags
  inside each library crate. The library crates are wholly network-facing so no subset is
  WASI-useful.
- `but-forge-storage` is WASI-safe (no network deps); leave as hard dep.
- `gix` does not pull reqwest — network features are not enabled in this workspace.
- `ssh2` is NOT in the dep tree at all.
- `hyper` and `ureq` are NOT in the dep tree.
- `but-installer` is a separate binary, not a dep of `but`.
- `posthog-rs` and `machine-uid` are handled by s05; s06 coordinates on the `native` feature line.

## Findings

### Crates with direct `reqwest` dependency (as hard deps in `but`):
- `but-github` (line 63 of `crates/but/Cargo.toml`)
- `but-gitlab` (line 64, also pulled by `but-forge`)
- `but-forge` (line 65, transitively via but-github + but-gitlab)
- `but-update` (line 57)
- `but-llm` (line 70, also uses `async-openai` which uses reqwest)

### Crates with indirect `reqwest` via `async-openai`:
- `but-llm` → `async-openai` (workspace: `features = ["rustls", "chat-completion"]`)
- `but-tools` (dep of `but-llm`) → `async-openai`

### Source files with direct `reqwest::` usage:
- `crates/but/src/utils/metrics.rs` — `posthog_rs::Client` (s05 scope)
- `crates/but-llm/src/anthropic.rs` — `reqwest::Client`, `reqwest::Response`
- `crates/but-llm/src/openai.rs` — `reqwest::header::*`
- `crates/but-update/src/check.rs` — `reqwest::Client`, `reqwest::header::*`
- `crates/but-gitlab/src/client.rs` — `reqwest::Client`
- `crates/but-github/src/client.rs` — `reqwest::Client`

### Network crates already optional in `but`:
- None of the five network crates are currently optional.

### `native` feature current state:
- `crates/but/Cargo.toml` line 23: `native = []` — empty, no activations yet.

## Blockers

- Q1: s05 sequencing for the `native` feature line in `crates/but/Cargo.toml` — must
  coordinate to avoid merge conflict. Recommend s06 declares s05 as a dep.
- Q2: `but-secret` / `keyring` — platform-native dep, not in s06 scope but will block
  WASI compilation. Needs assignment to a sub-PR.
- Q3: Whether `but-forge-storage` should also be optional (safe to leave as hard dep).

## Errors & Fixes

(none yet — planning phase only)
