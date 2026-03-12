# Questions: wasi-serde-objectid (s02)

## Q1: Scope reduction -- is the minimal approach acceptable?

**Status:** open
**Blocking:** no (plan can proceed either way, but want confirmation before implementation)

**Question:** After analyzing the codebase, the git2-based serde modules in `but-serde` are already correctly gated behind `#[cfg(feature = "legacy")]`. The gix-based modules are always available. This means:

- Under WASI builds (`--no-default-features --features wasi`), `git2` is already excluded
- All `but-*` crates already use `gix::ObjectId` exclusively for their serde annotations
- The proposed "thin ObjectId wrapper" from the original scope is unnecessary

The revised plan is minimal: add `wasi = []` marker feature to `but-serde/Cargo.toml`. No code changes to `lib.rs`. No new types.

Is this minimal approach acceptable, or does the coordinator want additional work done in this sub-PR (e.g., documentation comments explaining the WASI story, compile-time assertions, etc.)?

## Q2: Should s09 (gate ctx) still depend on s02?

**Status:** open
**Blocking:** no

**Question:** The dependency graph has `s09` (gate `but-ctx`) depending on `s02`. The original rationale was that `but-ctx` holds a `git2::Repository` and might need the ObjectId abstraction. Since s02 is now just a marker feature addition, should s09 still wait for s02, or can it be moved to depend only on s01?

This would allow s09 to start in parallel with s02 rather than waiting for it.
