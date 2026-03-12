# Memory: wasi-gate-sqlite (s08)

## Status: plan-complete

## CRITICAL FINDING: rusqlite + wasm32-wasip2

**rusqlite 0.38.0 with `bundled` for `wasm32-wasip2`: Unconfirmed — requires runtime test.**

- rusqlite PR #1569 (merged Oct 2024) added wasip2 bundled mode compile flag support
- No confirmed successful build found in the upstream issue tracker
- The `cc` crate must invoke WASI SDK clang to compile bundled SQLite C source
- On Linux CI: likely to work if WASI SDK is installed
- On macOS: known cross-compilation failures (issue #1459) due to SDK header contamination
- **First implementation step MUST be:** `cargo build -p but-db --target wasm32-wasip2` and document result here

## Errors & Fixes

None yet (plan phase only).

## Decisions

### 2026-03-13 — Two-path plan
**Context:** Cannot determine at plan time whether rusqlite compiles for wasip2 without running the build.
**Decision:** Plan both paths. Path A (rusqlite compiles) requires only Cargo.toml changes + minor gating. Path B (fails) requires a `Storage` trait abstraction (~800-1000 LOC).
**Recommended starting point:** Path A — attempt compilation first.

### 2026-03-13 — but-link and but-cursor not affected
**Context:** Both use `rusqlite` directly (not via `but-db`) and are excluded from WASI builds via the `legacy` feature gate in `crates/but/Cargo.toml`.
**Decision:** No changes needed to `but-link` or `but-cursor` for WASI gating.

### 2026-03-13 — poll feature likely not a problem for WASI CLI build
**Context:** `but-db`'s `poll` feature (thread + second connection) is consumed only by `but-claude` and `but-testing`, both behind `legacy`.
**Decision:** `cargo check -p but --no-default-features --features wasi` should not activate `poll`. Verify during implementation.

### 2026-03-13 — WAL mode: try as-is first
**Context:** WAL requires file locking and shm files. WASI P2 provides POSIX-like filesystem access in wasmtime. Single-process use means no cross-process locking needed.
**Decision:** Attempt WAL as-is. If it fails at runtime in WASI, fall back to DELETE journal mode by gating `improve_concurrency()` behind `cfg(not(target_os = "wasi"))`.

### 2026-03-13 — rusqlite `bundled` workspace feature
**Context:** Workspace Cargo.toml sets `rusqlite = { version = "0.38.0", features = ["bundled"] }`. This means the bundled SQLite C source is compiled into the binary. For WASI this requires a WASI SDK C toolchain.
**Decision:** Keep `bundled` for now. If Path B is taken, rusqlite becomes `optional = true` and the WASI build uses no rusqlite at all.

## Blockers

None blocking — plan is ready for implementation. The implementing agent must:
1. Run `cargo build -p but-db --target wasm32-wasip2` with WASI SDK installed
2. Record the result in this MEMORY.md
3. Proceed with Path A or Path B based on the result
