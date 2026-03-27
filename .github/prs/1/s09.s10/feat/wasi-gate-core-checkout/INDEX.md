# s10: Gate but-core Worktree/Checkout Module for WASI

- **Branch:** `pr1/s09.s10/feat/wasi-gate-core-checkout`
- **Anchor:** `pr1/s02.s09/feat/wasi-gate-ctx`
- **Deps:** s09 (needs ctx without git2)
- **Size:** S
- **Commit:** `feat: gate but-core worktree/checkout module for WASI`

## Scope

- Gate `src/worktree/checkout/` behind `#[cfg(not(feature = "wasi"))]`
- Provide stub returning error for WASI ("checkout not available in WASI")
- Gate git2 dep in but-core Cargo.toml

## Files

- `crates/but-core/Cargo.toml`
- `crates/but-core/src/worktree/checkout/`
- `crates/but-core/src/worktree/mod.rs`

## Acceptance Criteria

- `cargo check -p but-core --features wasi` compiles
- Native tests pass

---

## Detailed Plan

### 1. Where git2 Is Used in but-core

git2 is used **only inside the checkout module** — specifically in two files:

- `crates/but-core/src/worktree/checkout/function.rs`
  - `git2::build::CheckoutBuilder::new()` — builds a checkout configuration
  - `git2::Repository::open(repo.git_dir())` — opens repo via git2
  - `git2_repo.find_tree(...)` + `git2_repo.checkout_tree(...)` — performs the actual file writes to disk
  - Uses `but_oxidize::ObjectIdExt` trait (gix ObjectId → git2 Oid)

- `crates/but-core/src/worktree/checkout/utils.rs`
  - `git2::build::CheckoutBuilder` — passed in as parameter to `merge_worktree_changes_into_destination_or_keep_snapshot`
  - This type is only used because `function.rs` creates it and passes it in

No other files in `crates/but-core/src/` import git2 directly.

### 2. Public API Surface of the Checkout Module

`crates/but-core/src/worktree/mod.rs` re-exports:

```rust
pub use checkout::function::{safe_checkout, safe_checkout_from_head};
```

These two functions are the only checkout-related items visible outside the module. The
`Options`, `Outcome`, `UncommitedWorktreeChanges` types from `checkout/mod.rs` are also
part of the public API and used by callers.

### 3. External Callers of Checkout (Outside but-core)

Several crates call `safe_checkout` / `safe_checkout_from_head` and import the types:

| Crate | File | Notes |
|-------|------|-------|
| `but-rebase` | `src/graph_rebase/materialize.rs` | Unconditional dep; no wasi/legacy gate |
| `but-api` | `src/branch.rs` | Imports `UncommitedWorktreeChanges` |
| `gitbutler-workspace` | `src/branch_trees.rs` | Calls `but_core::worktree::safe_checkout` |
| `gitbutler-branch-actions` | `src/branch_manager/mod.rs`, `src/branch_manager/branch_creation.rs`, `src/base.rs`, `src/integration.rs` | Multiple call sites |
| `gitbutler-oplog` | `src/oplog.rs` | Calls `safe_checkout_from_head` |

**Important:** `gitbutler-*` crates are all behind the `legacy` feature in the `but` binary.
`but-rebase` and `but-api` are NOT behind `legacy` — they are always compiled. This means
`but-rebase` also pulls in `but-core` without gating and calls `safe_checkout_from_head`.

### 4. What Needs to Be Gated

#### In `crates/but-core/Cargo.toml`

- Move `git2` from unconditional `[dependencies]` to a wasi-conditional dep:
  ```toml
  git2 = { workspace = true, optional = true }
  ```
  Then gate it so it is only included when `wasi` feature is NOT active. Since Cargo does
  not support negative feature conditions, the idiomatic approach is:
  - Add a `native` feature (or reuse an existing pattern) that enables `dep:git2`
  - The `wasi` feature would simply not enable `dep:git2`
  - Default features would include `dep:git2`

  A simpler alternative already used in this project: use `#[cfg(not(feature = "wasi"))]`
  in code and keep `git2` as a regular dep, but ensure it is not linked when target is WASI.
  However, for a clean compile without errors, the dep must be conditional. The correct
  approach is to make it optional and add a `native` feature toggle.

  Actually, looking at the existing pattern (s03 / but-oxidize), the approach is to keep
  git2 as a dep but gate all usage at the code level. This avoids Cargo feature complexity
  for crates that do not control the `wasi` feature themselves. But but-oxidize itself does
  have the `wasi` feature, so the gate works there.

  **Decision:** Add `wasi = []` to `but-core/Cargo.toml` features (currently absent), then
  make `git2` optional and add it to a non-wasi activation.

#### In `crates/but-core/src/worktree/mod.rs`

- Gate the `pub mod checkout;` declaration and its re-exports behind `#[cfg(not(feature = "wasi"))]`
- Provide stub re-exports (or a gated-off module) so downstream WASI builds that reference
  `checkout::Options`, `checkout::UncommitedWorktreeChanges`, `checkout::Outcome`,
  `safe_checkout`, and `safe_checkout_from_head` still compile.

#### In `crates/but-core/src/worktree/checkout/` (all 4 files)

Since the entire module is gated at the parent `mod.rs` level, no changes are needed inside
the checkout directory files themselves — they will simply not be compiled under WASI.

#### Stub design

The stubs for WASI need to:
1. Expose the same types (`Options`, `Outcome`, `UncommitedWorktreeChanges`) — these are
   pure data types with no git2 dependency, so they can be left in a gated-in stub file.
2. Expose `safe_checkout` and `safe_checkout_from_head` that immediately return
   `Err(anyhow::anyhow!("checkout not available in WASI"))`.

The cleanest approach: create `crates/but-core/src/worktree/checkout_stub.rs` (or inline the
stubs in `worktree/mod.rs`) behind `#[cfg(feature = "wasi")]`.

### 5. Impact on but-rebase

`but-rebase` is unconditionally compiled (not behind `legacy`) and calls
`safe_checkout_from_head`. When building `but` with `--features wasi`, `but-rebase` will
also see the gated `but-core`. The WASI stub for `safe_checkout_from_head` must therefore
return a meaningful error rather than panicking, so `but-rebase`'s `materialize()` function
can propagate it at runtime (it will only be invoked when a checkout is requested, which
should not happen in WASI command paths).

No changes are needed in `but-rebase` itself; the stub in `but-core` handles this.

### 6. Cargo.toml Changes for but-core

Currently `but-core/Cargo.toml` has no `wasi` feature and no `[features]` table entry for
it. The changes needed:

```toml
[features]
export-ts = ["dep:ts-rs"]
export-schema = ["dep:schemars", "dep:but-schemars"]
legacy = []
wasi = []   # add this

[dependencies]
# ...
git2 = { workspace = true, optional = true }   # was unconditional
but-oxidize = { workspace = true, optional = true }  # only needed for checkout (git2 bridge)
```

And in the `wasi` feature, explicitly NOT including `dep:git2` and `dep:but-oxidize`.
For the default (native) build, a default feature or explicit inclusion ensures they compile.

Wait — `but-oxidize` is also listed as a direct dependency in `but-core/Cargo.toml`. Let's
verify whether it is used outside the checkout module.

**Checking but-oxidize usage in but-core:**
`but-oxidize::ObjectIdExt` (the `.to_git2()` trait) is imported only in
`src/worktree/checkout/function.rs`. No other file in `but-core/src/` uses `but-oxidize`.
Therefore `but-oxidize` can also be made optional and excluded under `wasi`.

### 7. Step-by-Step Implementation Plan

1. **Add `wasi` feature to `but-core/Cargo.toml`**
   - Add `wasi = []` under `[features]`
   - Make `git2` optional: `git2 = { workspace = true, optional = true }`
   - Make `but-oxidize` optional: `but-oxidize = { workspace = true, optional = true }`
   - Add a `native` feature that activates both: `native = ["dep:git2", "dep:but-oxidize"]`
   - Update `default` features (if any) to include `native`, or just ensure the
     default build works by not having a `default` entry (unconditional deps remain unconditional).
   - Actually the cleanest minimal change: keep git2 as a normal dep, but add `#[cfg]` gates
     everywhere. But Cargo will still link git2 even if no code uses it (and git2 is a C lib).
     The proper solution IS to make it optional and gate it via features.

2. **Gate the checkout module in `crates/but-core/src/worktree/mod.rs`**
   - Wrap `pub mod checkout;` and the re-exports in `#[cfg(not(feature = "wasi"))]`
   - Add `#[cfg(feature = "wasi")]` stub that exposes the types and stub functions

3. **No changes needed to checkout/ files** — they are not compiled under WASI.

4. **Propagate `wasi` feature from `but` crate to `but-core`**
   - In `crates/but/Cargo.toml`, update: `but-core = { workspace = true }` → include
     `but-core` in the `wasi` feature activation: `wasi = [..., "but-core/wasi"]`
   - Currently `wasi` feature in `but` does not forward to `but-core`; this linkage is needed.

5. **Verify `but-rebase` and `but-api` compile under wasi**
   - `but-rebase` doesn't have a `wasi` feature — but it depends on `but-core` which now has
     the WASI-gated checkout. As long as the stub functions are present, `but-rebase` compiles.
   - `but-api` similarly imports `UncommitedWorktreeChanges` — must be in the stub.

### 8. Files to Change (Summary)

| File | Change |
|------|--------|
| `crates/but-core/Cargo.toml` | Add `wasi = []` feature; make `git2` and `but-oxidize` optional deps gated by a `native` feature; activate `native` by default |
| `crates/but-core/src/worktree/mod.rs` | Gate checkout module and re-exports; add WASI stubs |
| `crates/but/Cargo.toml` | Add `but-core/wasi` to `wasi` feature activation |

No changes needed to:
- `crates/but-core/src/worktree/checkout/function.rs`
- `crates/but-core/src/worktree/checkout/utils.rs`
- `crates/but-core/src/worktree/checkout/mod.rs`
- `crates/but-core/src/worktree/checkout/tree.rs`
- Any crate that calls `safe_checkout*`

### 9. WASI Stub Design (inline in worktree/mod.rs)

```rust
#[cfg(not(feature = "wasi"))]
pub mod checkout;
#[cfg(not(feature = "wasi"))]
pub use checkout::function::{safe_checkout, safe_checkout_from_head};

#[cfg(feature = "wasi")]
pub mod checkout {
    // Types are pure Rust — no git2 needed
    #[derive(Default, Debug, Copy, Clone)]
    pub enum UncommitedWorktreeChanges { ... }
    #[derive(Default, Debug, Copy, Clone)]
    pub struct Options { ... }
    pub struct Outcome { ... }
}

#[cfg(feature = "wasi")]
pub fn safe_checkout(
    _current_head_id: gix::ObjectId,
    _new_head_id: gix::ObjectId,
    _repo: &gix::Repository,
    _opts: checkout::Options,
) -> anyhow::Result<checkout::Outcome> {
    anyhow::bail!("checkout not available in WASI")
}

#[cfg(feature = "wasi")]
pub fn safe_checkout_from_head(
    _new_head_id: gix::ObjectId,
    _repo: &gix::Repository,
    _opts: checkout::Options,
) -> anyhow::Result<checkout::Outcome> {
    anyhow::bail!("checkout not available in WASI")
}
```

Note: the WASI stub `checkout` module must duplicate the type definitions from
`checkout/mod.rs`. These types have no git2 dependencies so duplication is small and safe.
Alternatively, the types could be extracted to a separate `checkout_types.rs` (always compiled)
and the `function.rs`/`utils.rs` remain gated. This avoids duplication but adds a file.
**Recommended approach:** extract types to `checkout/types.rs`, gate only `function.rs` and
`utils.rs`, and have the WASI-mode module re-export just the types plus stubs for the functions.

### 10. Risk Notes

- `worktree_file_to_git_in_buf` in `worktree/mod.rs` uses only `gix`, not git2 — it stays ungated.
- The `Delegate` struct and `Lut` in `checkout/utils.rs` and `checkout/tree.rs` are also pure Rust
  (no git2), but the function signature of `merge_worktree_changes_into_destination_or_keep_snapshot`
  takes `git2::build::CheckoutBuilder` — so the whole `utils.rs` must remain gated.
- All tests in `crates/but-core/tests/core/worktree/checkout.rs` should be gated too
  (via `#[cfg(not(feature = "wasi"))]` on the test module), but this is a compile-time
  concern only — native test runs are unaffected.
