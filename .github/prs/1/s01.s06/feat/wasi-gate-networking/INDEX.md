# s06: Gate Networking Dependencies for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s06                                                |
| **Branch**| `pr1/s01.s06/feat/wasi-gate-networking`            |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: gate networking dependencies (reqwest, ssh2, posthog) for WASI` |

## Scope

- Gate `reqwest` in but-github, but-gitlab, but-update, but-llm
- Gate `posthog-rs` in but CLI crate
- Gate `ssh2` if in dep chain
- Gate network ops in but-forge/but-forge-storage

## Files

- `crates/but-github/Cargo.toml`
- `crates/but-gitlab/Cargo.toml`
- `crates/but-update/Cargo.toml`
- `crates/but-llm/Cargo.toml`
- `crates/but/Cargo.toml`
- `crates/but-forge/Cargo.toml`

## Acceptance Criteria

- No `reqwest`/`ssh2` in WASI dep tree
- Native build unaffected
