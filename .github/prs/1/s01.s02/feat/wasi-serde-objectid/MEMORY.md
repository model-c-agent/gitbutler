# Memory: wasi-serde-objectid (s02)

## Status: planning

## Errors & Fixes

## Decisions

### 2026-03-12 -- No new ObjectId abstraction type needed
**Context:** The initial INDEX.md proposed "a thin ObjectId wrapper/type alias" to abstract over `git2::Oid` and `gix::ObjectId`.
**Decision:** After analyzing the codebase, no wrapper type is necessary. The git2-based serde modules (`oid`, `oid_opt`, `oid_vec`) are already gated behind `#[cfg(feature = "legacy")]`, and the gix-based modules (`object_id`, `object_id_opt`, `object_id_vec`) are always available. All modern `but-*` crates exclusively use `gix::ObjectId`. The separation is already clean.
**Alternatives considered:**
- Type alias `pub type ObjectId = gix::ObjectId` -- adds no value, just indirection
- Newtype wrapper with `From` impls for both Oid types -- unnecessary complexity given clean legacy/non-legacy split
- Trait-based abstraction -- over-engineering for a problem that doesn't exist

### 2026-03-12 -- Size revised from M to S
**Context:** The initial estimate was M based on the assumption that deep refactoring would be needed.
**Decision:** After codebase analysis, the actual change is trivial: add one line to Cargo.toml (`wasi = []`). No code changes to `lib.rs` needed. Revised to S.
**Alternatives considered:** None, this is a factual reassessment.

### 2026-03-12 -- Feature forwarding deferred to integration phase
**Context:** The `wasi` feature needs to propagate from the top-level `but` crate down to `but-serde`.
**Decision:** Each tier-1 sub-PR only adds the `wasi` feature to its own crate. The wiring (`but` -> `but-serde/wasi`) happens during integration (s12/s13). This avoids tier-1 sub-PRs stepping on each other in `crates/but/Cargo.toml`.
**Alternatives considered:** Add forwarding in s02 itself -- rejected because multiple tier-1 sub-PRs would conflict editing the same feature definition in `but/Cargo.toml`.

## Blockers
