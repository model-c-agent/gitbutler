<<<<<<< ours
# s02: Abstract ObjectId in but-serde to Gate git2::Oid for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s02                                                |
| **Branch**| `pr1/s01.s02/feat/wasi-serde-objectid`             |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | S (revised down from M)                            |
| **Commit**| `feat: add wasi feature to but-serde, gate git2::Oid serde behind native` |

## Analysis

### Current State

`but-serde` already has a clean separation between git2 and gix serde helpers:

**git2-based modules** (all gated behind `#[cfg(feature = "legacy")]`):
- `oid` -- serializes `git2::Oid` as hex string
- `oid_opt` -- serializes `Option<git2::Oid>` as `string | null`
- `oid_vec` -- serializes `Vec<git2::Oid>` as `string[]`
- `as_time_seconds_from_unix_epoch` -- serializes `git2::Time` as Unix seconds

**gix-based modules** (always available, no feature gate):
- `object_id` -- serializes `gix::ObjectId` as hex string
- `object_id_opt` -- serializes `Option<gix::ObjectId>` as `string | null`
- `object_id_vec` -- serializes `Vec<gix::ObjectId>` as `string[]`

The `git2` dependency in `but-serde/Cargo.toml` is already `optional = true`, activated only by the `legacy` feature. The `gix` dependency is always present.

### Key Finding

The git2 serde helpers are **already correctly gated** behind `#[cfg(feature = "legacy")]`. Since the WASI build uses `--no-default-features --features wasi` (per s01's plan), the `legacy` feature is not enabled, so `git2` is never pulled in. The `gix`-based serde helpers (`object_id`, `object_id_opt`, `object_id_vec`) are always available regardless of features.

This means the core abstraction work is already done by the existing codebase design. What remains is:

1. Adding a `wasi` feature to `but-serde` so downstream crates can conditionally depend on it
2. Ensuring `but-serde` compiles cleanly under `--features wasi` without `legacy`
3. Documenting the feature interaction

### Impact Surface of `but_serde::oid` (git2-based)

These are **legacy-only** crates that use `but_serde::oid`/`oid_opt`/`oid_vec`:

| Crate | Module | Annotation | Field Type |
|-------|--------|------------|------------|
| `gitbutler-project` | `src/project.rs:72` | `but_serde::oid` | `git2::Oid` |
| `gitbutler-operating-modes` | `src/lib.rs:60` | `but_serde::oid` | `git2::Oid` |
| `gitbutler-oplog` | `src/entry.rs:18` | `but_serde::oid` | `git2::Oid` |
| `gitbutler-oplog` | `src/state.rs:29` | `but_serde::oid_opt` | `Option<git2::Oid>` |
| `gitbutler-branch-actions` | `src/upstream_integration.rs:90` | `but_serde::oid` | `git2::Oid` |
| `gitbutler-branch-actions` | `src/base.rs:34,36` | `but_serde::oid` | `git2::Oid` |
| `gitbutler-branch-actions` | `src/base.rs:44,46` | `but_serde::oid_vec` | `Vec<git2::Oid>` |
| `gitbutler-branch-actions` | `src/remote.rs:17` | `but_serde::oid_vec` | `Vec<git2::Oid>` |
| `gitbutler-branch-actions` | `src/reorder.rs:94` | `but_serde::oid_vec` | `Vec<git2::Oid>` |

All of these are in `gitbutler-*` legacy crates, not in the `but-*` modern crates. They will not be compiled for WASI (they depend on `git2` directly and are behind the `legacy` feature in the top-level `but` crate). **No migration of these call sites is needed for s02.**

### Impact Surface of `but_serde::object_id` (gix-based)

These modules are used extensively in non-legacy `but-*` crates (~30 call sites). They are already WASI-compatible since they only depend on `gix::ObjectId`. No changes needed.

## Design

### Approach: Minimal Feature Addition

Since the heavy lifting is already done (git2 modules gated behind `legacy`), s02 adds a `wasi` feature flag to `but-serde` for downstream feature propagation and verifies clean compilation.

No new abstraction type is needed. The existing `gix::ObjectId` is the WASI-compatible ID type. The existing `object_id`/`object_id_opt`/`object_id_vec` modules are the WASI-compatible serde helpers.

### Why Not a Wrapper Type?

The initial INDEX.md suggested "a thin ObjectId wrapper/type alias." After analyzing the codebase:

- **gix::ObjectId is already the universal ID type** in all modern `but-*` crates
- **git2::Oid is only used in legacy `gitbutler-*` crates**, which are entirely excluded from WASI builds
- **but-oxidize provides conversions** (`OidExt::to_gix`, `ObjectIdExt::to_git2`) at the boundary between old and new code
- A wrapper type would add indirection without benefit -- all WASI-targeted code already uses `gix::ObjectId` directly

## Files to Modify

### 1. `crates/but-serde/Cargo.toml`

**Add `wasi` feature:**

```toml
[features]
legacy = ["dep:git2"]
wasi = []   # WASI target: no git2 deps. gix-based modules always available.
```

The `wasi` feature is an empty marker. Its purpose is to allow the top-level `but` crate to propagate `wasi` to `but-serde` (e.g., `but-serde/wasi`) even though `but-serde` doesn't currently need to react to it. This future-proofs the dependency chain for when/if WASI-specific serialization behavior is needed (e.g., different binary encoding for WASM performance).

### 2. `crates/but-serde/src/lib.rs`

**No code changes required.**

The existing `#[cfg(feature = "legacy")]` gates on the `oid`, `oid_opt`, `oid_vec`, and `as_time_seconds_from_unix_epoch` items are already correct. When `legacy` is not enabled (as in WASI builds), these items are excluded and `git2` is not a dependency.

## What This Does NOT Do

- Does **not** create a new ObjectId abstraction type (unnecessary, see rationale above)
- Does **not** modify any downstream crates (they already use the right modules)
- Does **not** touch `but-oxidize` (that crate will be handled by s03: wasi-oxidize-noop)
- Does **not** gate `gix` itself (that's s12's job: wasi-gix-features)
- Does **not** modify serde behavior -- serialization format is identical under native and WASI

## Dependency on s01

s01 adds the `wasi` marker feature to the top-level `but/Cargo.toml`. After s01 merges, the `but` crate can forward `wasi` to `but-serde`:

```toml
# In crates/but/Cargo.toml (done in a later sub-PR that wires up feature forwarding)
wasi = ["but-serde/wasi", ...]
```

The feature forwarding from `but` to `but-serde` will be done as part of the integration (s12 or s13), since each tier-1 sub-PR only adds the feature to its own crate.

## Verification

After this sub-PR:

1. `cargo check -p but-serde` -- default build, no features, compiles without git2 (already true today)
2. `cargo check -p but-serde --features legacy` -- legacy build with git2 serde helpers
3. `cargo check -p but-serde --features wasi` -- WASI marker build, compiles without git2
4. `cargo test -p but-serde` -- all existing tests pass (no behavioral changes)
5. `cargo check -p but-serde --features wasi --features legacy` -- both features, compiles (not mutually exclusive at cargo level)

## Acceptance Criteria

- [ ] `crates/but-serde/Cargo.toml` has `wasi = []` in `[features]`
- [ ] `cargo check -p but-serde --features wasi` compiles without pulling in `git2`
- [ ] `cargo check -p but-serde --features legacy` still compiles (no regression)
- [ ] `cargo test -p but-serde` passes (no behavioral changes)
- [ ] No new abstraction type introduced (gix::ObjectId is already the universal type)
|||||||
=======
# s02: Abstract ObjectId in but-serde to Gate git2::Oid for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s02                                                |
| **Branch**| `pr1/s01.s02/feat/wasi-serde-objectid`             |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: abstract ObjectId in but-serde to gate git2::Oid for WASI` |

## Scope

- Introduce a thin ObjectId wrapper/type alias in `but-serde` that works with both `git2::Oid` and `gix::ObjectId`
- Gate `git2::Oid`-specific serialization behind `#[cfg(not(feature = "wasi"))]`
- Provide `gix::ObjectId` equivalents for WASI builds

## Files

- `crates/but-serde/Cargo.toml`
- `crates/but-serde/src/lib.rs`

## Risk

`git2::Oid` helpers are deeply embedded throughout the codebase. Grep all usage sites to understand the full impact surface.

## Acceptance Criteria

- `cargo check -p but-serde --features wasi` compiles without pulling in `git2`
- Native tests (`cargo test -p but-serde`) continue to pass
>>>>>>> theirs
