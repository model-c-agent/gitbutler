# Memory: wasi-gate-ctx (s09)

## Status: complete

## Errors & Fixes

## Decisions

### 2026-03-14 — native feature needed for optional git2 in but-ctx
**Context:** Making `git2` optional in `but-ctx/Cargo.toml` breaks the default build because callers not setting any features still expect `git2` to be available.
**Decision:** Add `default = ["native"]` and `native = ["dep:git2"]` to `but-ctx/Cargo.toml`. The `legacy` feature also activates `dep:git2`. WASI builds use `--no-default-features --features wasi`.
**Alternatives considered:** Using a `cfg(target_arch = "wasm32")` target-specific dependency — rejected because the PR strategy uses cargo features to allow testing on native (per INDEX.md philosophy).

### 2026-03-14 — but crate non-legacy git2 usage is out of s09 scope
**Context:** `config.rs` and `alias.rs` in the `but` crate use `ctx.git2_repo` in non-legacy code paths that are part of the WASI command target set (config, alias commands). These cannot simply be gated away without a gix-based replacement.
**Decision:** s09 scopes strictly to `but-ctx` changes. The `cargo check -p but-ctx --features wasi` criterion is achievable. Full `cargo check -p but --no-default-features --features wasi` will still fail at the `but` crate level (config.rs, alias.rs) — that is addressed in s13 (first compile fixes).
**Alternatives considered:** Including config.rs/alias.rs gix rewrites in s09 — rejected because it inflates scope and the acceptance criterion only requires `but-ctx` to compile clean.

### 2026-03-14 — legacy.rs needs no changes
**Context:** `legacy.rs` imports `new_ondemand_git2_repo` and uses it. It is already behind `#[cfg(feature = "legacy")]`.
**Decision:** No change to `legacy.rs`. The function `new_ondemand_git2_repo` in `lib.rs` is gated with `#[cfg(not(feature = "wasi"))]`. Since `legacy` and `wasi` are mutually exclusive in practice (the `but` crate wires them this way), this is safe.

## Blockers

### Non-blocking note: config.rs/alias.rs git2 usage
`crates/but/src/command/config.rs` and `crates/but/src/command/alias.rs` call `ctx.git2_repo.get()` in code compiled for WASI-target commands. These need gix-based config rewrites before `cargo check -p but --features wasi` passes end-to-end. This is follow-up work for s13.
