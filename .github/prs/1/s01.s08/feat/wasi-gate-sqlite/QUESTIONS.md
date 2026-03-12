## Q1: Does rusqlite 0.38.0 with `bundled` actually compile for wasm32-wasip2 in this repo's CI environment?

**Context:** rusqlite PR #1569 (merged Oct 2024) claims to fix wasip2 bundled mode, but the fix was a build-script prefix match — no confirmed successful compilation report has been found. The `cc` crate must invoke a WASI-capable clang to compile the bundled SQLite C source. Whether that is available in CI (or the developer's machine) determines which path (A or B) to pursue.

**Options:**
A) Run `cargo build -p but-db --target wasm32-wasip2` with WASI SDK installed — succeeds → proceed with Path A (minor gating only)
B) Same command fails even with WASI SDK → proceed with Path B (Storage trait abstraction)

**Blocking:** yes — the implementation path cannot be chosen without this result. The implementing agent must run this as the very first step.

**Response:** 2026-03-13 — Agreed. The implementation agent's first task is `cargo build -p but-db --target wasm32-wasip2` (with WASI SDK). Record the result immediately in MEMORY.md. Path A if it compiles, Path B if not. No additional coordinator input needed — the agent has authority to choose the path based on the build result.
**Source:** coordinator decision

---

## Q2: Should WAL mode be disabled for WASI builds?

**Context:** `but-db` enables WAL mode unconditionally via `PRAGMA journal_mode = WAL` in `improve_concurrency()`. WAL requires advisory file locking and shared memory files (`.db-shm`). WASI P2 supports POSIX-like filesystem operations in wasmtime, but the behavior of file locking in WASI is runtime-dependent. For single-process use (which is the only scenario in WASI), WAL should work but if it does not, the fallback is `DELETE` journal mode.

**Options:**
A) Leave WAL as-is — if rusqlite compiles for wasip2, WAL will be attempted and may work fine
B) Gate WAL pragma behind `#[cfg(not(target_os = "wasi"))]` and use DELETE mode (the SQLite default) for WASI

**Blocking:** no — this can be determined experimentally during implementation. Default to A; switch to B if WAL setup panics or errors at runtime.

**Response:** 2026-03-13 — Option A. Try WAL as-is. Single-process WASI use means no cross-process locking needed. Switch to DELETE mode only if runtime failures occur.
**Source:** coordinator decision

---

## Q3: Should the `poll` feature be excluded from WASI builds?

**Context:** `crates/but-db/src/poll.rs` (activated by the `poll` feature) uses `std::thread::spawn` and opens a second `rusqlite::Connection`. WASI P2 supports threads (wasi:io/poll), so `std::thread` may work. However, `but-claude` and `but-testing` depend on `but-db` with `features = ["poll"]`. Both are excluded from the WASI build of the `but` CLI via the `legacy` feature gate. The question is whether `but-db` itself needs a `wasi` feature to guard `poll`.

**Options:**
A) No change needed — `poll` consumers are all behind `legacy`, so `cargo check -p but --no-default-features --features wasi` will not activate `poll`
B) Add explicit `#[cfg(not(target_os = "wasi"))]` to the `poll` module as defense-in-depth

**Blocking:** no — A is likely correct and can be verified with `cargo check`.

**Response:** 2026-03-13 — Option A. `poll` consumers are behind `legacy`, so it won't be activated in WASI builds. Verify with `cargo check` during implementation. Only add defense-in-depth gating if the check reveals otherwise.
**Source:** coordinator decision
