# s07: Gate Platform-Specific Dependencies for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s07                                                |
| **Branch**| `pr1/s01.s07/feat/wasi-gate-platform`              |
| **Anchor**| `pr1/s01.s06/feat/wasi-gate-networking`             |
| **Deps**  | s01, s05, s06                                      |
| **Size**  | M                                                  |
| **Commit**| `feat: gate platform-specific deps (keyring, dirs, notify) for WASI` |

## Scope

- Gate `keyring` in but-secret; provide env-var auth fallback for WASI
- Gate `notify` in but-settings (inotify C bindings)
- Gate `dirs` in but-path
- Gate `machine-uid` in but/but-update
- Gate `open` in but-api

## Files

- `crates/but-secret/Cargo.toml`
- `crates/but-secret/src/`
- `crates/but-settings/Cargo.toml`
- `crates/but-path/Cargo.toml`
- `crates/but/Cargo.toml`
- `crates/but-api/Cargo.toml`

## Scope Boundaries (Avoid Overlap)

**Already handled by s05:** `command-group`, `posthog-rs`, `machine-uid` in `crates/but/Cargo.toml`; all process-spawning gates in `metrics.rs` and `setup.rs`.

**Already handled by s06:** `but-github`, `but-gitlab`, `but-forge`, `but-update`, `but-llm` made optional in `crates/but/Cargo.toml` under `native`. This also removes `but-update`'s `machine-uid` from the WASI tree.

**s07 unique scope:**
- `keyring` in `but-secret` (+ env-var fallback implementation)
- `notify` in `but-settings`
- `dirs` in `but-path` (+ env-var fallback)
- `open` in `but-path` and `but-api`
- `dirs` in `but-installer` (make crate optional in `but`)
- `git2` in `crates/but/Cargo.toml` (make optional under `native`/`legacy`)
- `dirs` direct usage in `crates/but/src/command/skill.rs`

**NOT s07 scope:** `posthog-rs`, `machine-uid` in `crates/but`, `but-update` optionality, `but-llm` optionality.

## Acceptance Criteria

- No `keyring`/`notify` in WASI dep tree
- Env-var secrets work under WASI
- Native build unaffected
