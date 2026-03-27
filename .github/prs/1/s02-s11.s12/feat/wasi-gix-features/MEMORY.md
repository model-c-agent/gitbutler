# Memory: wasi-gix-features (s12)

## Status: plan-complete

## Decisions

### gix version
- `0.80.0` from `https://github.com/GitoxideLabs/gitoxide` branch `main`
- commit `ccb4d299e105935390d9a1543926c3a8de3a595b`

### Workspace baseline
`gix` workspace entry already uses `default-features = false, features = ["sha1"]`.
No change needed to the workspace baseline itself.

### WASI-incompatible features (must be gated or removed)

| Feature | Reason | Requesting crates (non-legacy chain) |
|---|---|---|
| `parallel` | Enables crossbeam threading in `gix-features`; WASI has no threads | `but-core`, `but-secret` |
| `credentials` | `gix-credentials` shells out via `std::process::Command` | `but-secret` (hard dep of `but`) |
| `blocking-http-transport-reqwest-rust-tls` | Network sockets unavailable in WASI | `gitbutler-project` (legacy only) |
| `max-performance` | Platform SHA acceleration + threading | `gitbutler-tauri`, `gitbutler-cli` (legacy) |

### WASI-safe features

| Feature | Notes |
|---|---|
| `sha1` | Pure Rust via `sha1-checked` |
| `serde` | Derive-only |
| `status` | FS-based; no threads required |
| `dirwalk` | FS directory walk; no threads |
| `merge` | Pure Rust algorithm; `gix-command` dep may be a compile concern (see blockers) |
| `revision` | Commit graph walk; pure Rust |
| `worktree-mutation` | Sequential checkout fallback works without `parallel` |
| `tracing` | Instrumentation only |
| `tracing-detail` | Instrumentation only |

### `gix::interrupt::IS_INTERRUPTED`
Used in 3 places as a read-only `AtomicBool`. Signal handler registration is not called.
`AtomicBool` works in WASI. Not a blocker.

### Critical finding: `but-secret` is the main blocker
- `but-secret` is a mandatory (non-optional) dependency of the `but` binary.
- It currently requests `gix: dirwalk, credentials, parallel`.
- Its `secret.rs` directly uses `gix::credentials::helper::{Cascade, Action, NextAction}`.
- This must be gated behind a new `wasi` feature on `but-secret` using `#[cfg]`.

## Required Code Changes (for implementation agent)

1. **`crates/but-core/Cargo.toml`**
   - Remove `credentials` and `parallel` from the gix features list.
   - Keep: `dirwalk, serde, status, merge`

2. **`crates/but-secret/Cargo.toml`**
   - Add `wasi = []` feature.
   - Change gix dep: for non-wasi keep `dirwalk, credentials, parallel`; for wasi drop `credentials, parallel`.
   - (Or: always drop `parallel` and `credentials` from the gix dep, gate source with `#[cfg]`)

3. **`crates/but-secret/src/secret.rs`**
   - Gate the entire `git_credentials` module with `#[cfg(not(target_family = "wasm"))]`.
   - Gate the `setup()` call site similarly.

4. **`crates/but/Cargo.toml`**
   - Add `but-secret/wasi` to the `wasi` feature:
     ```toml
     wasi = ["but-secret/wasi"]
     ```

## Errors & Fixes

(none yet — plan phase only)

## Blockers

### Potential: `gix-command` compile-time under WASI
- `gix-merge` depends on `gix-command` which uses `std::process::Command`.
- Even if not called at runtime, `gix-command` will be compiled into the dependency graph when
  `merge` feature is active.
- `std::process::Command` availability on `wasm32-wasip2` depends on the WASI SDK / Rust std library
  target support. As of Rust 1.78+, `wasm32-wasip2` does have partial `std::process` support via
  the wasi preview2 process API, but it may not implement `spawn()` with all features.
- **Risk level:** Medium. Needs a test compile to confirm.

### Potential: `gix-credentials` sub-crate still compiled
- Even without the `credentials` feature on `gix`, `gix-credentials` is in the resolved Cargo.lock
  dependency tree (it's a dependency of `gix-protocol`/`gix-transport`).
- If these crates themselves compile fine on WASI, this is not a blocker.
- Need to verify `gix-credentials` itself compiles without needing `std::process`.
