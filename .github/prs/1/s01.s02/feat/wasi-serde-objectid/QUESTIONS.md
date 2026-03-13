# Questions: wasi-serde-objectid (s02)

## Q1: Scope reduction -- is the minimal approach acceptable?

**Status:** resolved
**Blocking:** no

**Question:** After analyzing the codebase, the git2-based serde modules in `but-serde` are already correctly gated behind `#[cfg(feature = "legacy")]`. The gix-based modules are always available. This means:

- Under WASI builds (`--no-default-features --features wasi`), `git2` is already excluded
- All `but-*` crates already use `gix::ObjectId` exclusively for their serde annotations
- The proposed "thin ObjectId wrapper" from the original scope is unnecessary

The revised plan is minimal: add `wasi = []` marker feature to `but-serde/Cargo.toml`. No code changes to `lib.rs`. No new types.

**Response:** 2026-03-13 — Yes, minimal approach is correct. The analysis is thorough and the conclusion is sound. No wrapper type, no extra assertions needed.
**Source:** Coordinator decision — the codebase evidence is clear.

## Q2: Should s09 (gate ctx) still depend on s02?

**Status:** resolved
**Blocking:** no

**Question:** The dependency graph has `s09` (gate `but-ctx`) depending on `s02`. The original rationale was that `but-ctx` holds a `git2::Repository` and might need the ObjectId abstraction. Since s02 is now just a marker feature addition, should s09 still wait for s02, or can it be moved to depend only on s01?

**Response:** 2026-03-13 — Yes, s09 can depend only on s01. Will update the dependency graph when dispatching s09.
**Source:** Coordinator decision — s02's scope reduction means s09 has no real dependency on it.
