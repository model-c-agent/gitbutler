# s09: Gate git2::Repository in but-ctx for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s09                                                |
| **Branch**| `pr1/s02.s09/feat/wasi-gate-ctx`                  |
| **Anchor**| `pr1/s01.s02/feat/wasi-serde-objectid`             |
| **Deps**  | s02 (ObjectId abstraction — complete)              |
| **Size**  | M                                                  |
| **Commit**| `feat: gate git2::Repository in but-ctx for WASI` |

## Status: plan-complete

---

## Scope

Make the `git2_repo` field in `Context` conditional so that `cargo check -p but-ctx --features wasi` compiles without pulling in `git2`. The native build is completely unaffected. Downstream crates that call `ctx.git2_repo` are all in the `legacy` or `gitbutler-*` family and are never compiled in a WASI build, so their call sites do not need to change.

---

## Source Analysis

### `Context` struct — current layout (`crates/but-ctx/src/lib.rs`)

```rust
pub struct Context {
    pub settings: AppSettings,           // no git2 — safe
    pub gitdir: PathBuf,                 // no git2 — safe
    pub project_data_dir: PathBuf,       // no git2 — safe
    pub app_cache_dir: Option<PathBuf>,  // no git2 — safe
    pub repo: OnDemand<gix::Repository>, // gix only — safe
    pub git2_repo: OnDemand<git2::Repository>,  // ← WASI blocker
    pub db: OnDemand<but_db::DbHandle>,  // no git2 — safe (s08 handles sqlite separately)
    pub app_cache: OnDemandCache<but_db::AppCacheHandle>, // no git2 — safe
    #[cfg(feature = "legacy")]
    pub legacy_project: LegacyProject,   // already gated
    workspace: RefCell<Option<but_graph::projection::Workspace>>, // gix — safe
}
```

`ThreadSafeContext` has no `git2_repo` field — it is already WASI-safe.

### git2 usage inside `but-ctx` itself

All `git2` references inside `but-ctx/src/lib.rs` and `but-ctx/src/legacy.rs` are for constructing, initialising, or discarding `git2_repo`. Every one of them must be conditioned on `#[cfg(not(feature = "wasi"))]` (or equivalently `#[cfg(feature = "wasi")]` for stubs). The helper function `new_ondemand_git2_repo` (line 778) opens `git2::Repository::open(&gitdir)`.

Full list of affected lines in `lib.rs`:
- Line 110: field declaration `pub git2_repo: OnDemand<git2::Repository>`
- Lines 161, 236, 258, 307, 324, 351: `git2_repo: new_ondemand_git2_repo(gitdir.clone())` in all constructor bodies
- Lines 360–363: `pub fn with_git2_repo(mut self, git2_repo: git2::Repository) -> Self`
- Line 667: `git2_repo: _,` in `into_sync` destructuring
- Lines 778–782: `fn new_ondemand_git2_repo(gitdir: PathBuf) -> OnDemand<git2::Repository>` function

In `legacy.rs`:
- Line 7: `new_ondemand_git2_repo` in the `use` import list
- Line 38: `git2_repo: new_ondemand_git2_repo(gitdir.clone())` in `new_from_legacy_project_and_settings`

---

## Design: Conditional Compilation Plan

### Feature addition: `crates/but-ctx/Cargo.toml`

Add the `wasi` marker feature and make `git2` optional under it:

```toml
[features]
legacy = ["dep:gitbutler-project", "but-project-handle/legacy"]
wasi = []   # WASI target: no git2

[dependencies]
# ... existing deps ...
git2 = { workspace = true, optional = true }
```

And activate `git2` only on native:

```toml
# git2 is needed everywhere except wasi
[target.'cfg(not(feature = "wasi"))'.dependencies]
# (alternative: use a 'native' feature that enables git2)
```

**Preferred approach:** Make `git2` an optional dependency activated by the absence of `wasi`. Since Cargo features cannot express "not X", the cleanest mechanism is to introduce a `native` feature that activates `git2`, then never enable `native` in WASI builds. However, because `but-ctx` has no existing `native` feature, the simplest approach that matches the rest of the PR plan is:

- Add `wasi = []` as a marker feature
- Use `#[cfg(not(feature = "wasi"))]` in Rust code to gate `git2` usage
- Make `git2` an optional dependency; activate it from the `legacy` feature (which already exists) **and** from a new implicit path — since `git2` is needed on native without `legacy`, add a `native` feature activating `git2`:

```toml
[features]
legacy = ["dep:gitbutler-project", "but-project-handle/legacy", "dep:git2"]
native = ["dep:git2"]   # non-legacy native builds still need git2
wasi = []               # WASI: no git2

[dependencies]
git2 = { workspace = true, optional = true }
```

The `but` crate's `wasi` feature would forward to `but-ctx/wasi` but NOT `but-ctx/native` or `but-ctx/legacy`. The top-level `but` crate currently has `git2.workspace = true` as a non-optional dep; that is outside s09's scope (handled by s13 or the integration step).

### Struct field: `git2_repo`

```rust
pub struct Context {
    // ... other fields ...
    #[cfg(not(feature = "wasi"))]
    pub git2_repo: OnDemand<git2::Repository>,
    // ...
}
```

### Constructor bodies

Every struct literal that initialises `Context` must conditionally include the field:

```rust
#[cfg(not(feature = "wasi"))]
git2_repo: new_ondemand_git2_repo(gitdir.clone()),
```

This applies to all six constructor paths:
1. `Context::new` (non-legacy branch)
2. `Context::new` (legacy branch)
3. `Context::from_repo_with_legacy_support` (non-legacy branch)
4. `Context::from_repo_with_legacy_support` (legacy branch)
5. `Context::from_repo`
6. `legacy::Context::new_from_legacy_project_and_settings`

### `with_git2_repo` method

```rust
#[cfg(not(feature = "wasi"))]
pub fn with_git2_repo(mut self, git2_repo: git2::Repository) -> Self {
    self.git2_repo.assign(git2_repo);
    self
}
```

Entirely gated — not available under `wasi`.

### `into_sync` destructuring

The `git2_repo: _` field in the `into_sync` method's destructuring pattern must be conditionally included:

```rust
let Context {
    settings,
    gitdir,
    project_data_dir,
    mut repo,
    #[cfg(not(feature = "wasi"))]
    git2_repo: _,
    db: _,
    app_cache: _,
    app_cache_dir,
    // ...
} = self;
```

### `new_ondemand_git2_repo` helper

Gate the entire function:

```rust
#[cfg(not(feature = "wasi"))]
fn new_ondemand_git2_repo(gitdir: PathBuf) -> OnDemand<git2::Repository> {
    OnDemand::new({
        let gitdir = gitdir.clone();
        move || git2::Repository::open(&gitdir).map_err(Into::into)
    })
}
```

### `From<ThreadSafeContext> for Context`

The `From` impl creates a `Context` from `ThreadSafeContext`. It currently calls `new_ondemand_git2_repo` directly. Gate that line:

```rust
impl From<ThreadSafeContext> for Context {
    fn from(value: ThreadSafeContext) -> Self {
        // ...
        Context {
            // ...
            #[cfg(not(feature = "wasi"))]
            git2_repo: new_ondemand_git2_repo(gitdir.clone()),
            // ...
        }
    }
}
```

### `legacy.rs` import

The `use` statement in `legacy.rs` imports `new_ondemand_git2_repo`. Since `legacy.rs` is already `#[cfg(feature = "legacy")]` (gated in `lib.rs`), and a WASI build never enables `legacy`, this file is never compiled for WASI. **No change needed in `legacy.rs`.**

However, `new_ondemand_git2_repo` is referenced in `legacy.rs` at line 7 via the module-level `use`. Since the entire `legacy` module is `#[cfg(feature = "legacy")]`, and WASI never enables `legacy`, the function definition in `lib.rs` just needs to be `#[cfg(not(feature = "wasi"))]` — the legacy module never sees it at WASI compile time.

---

## Method-by-Method Analysis

All public methods on `Context` that do NOT touch `git2_repo`:

| Method | git2 dep? | WASI-safe? |
|--------|-----------|-----------|
| `new(gitdir, app_config_dir, app_cache_dir)` | Constructor — needs gate on `git2_repo` init | Safe after gating |
| `discover(directory)` | Same | Safe after gating |
| `new_from_project_handle(ph)` | Same | Safe after gating |
| `open(directory)` | Same | Safe after gating |
| `from_repo(repo)` | Same | Safe after gating |
| `with_repo(repo)` | No | Safe |
| `workspace_mut_and_db_mut()` | No | Safe |
| `workspace_mut_and_db_mut_with_perm()` | No | Safe |
| `workspace_and_db_mut()` | No | Safe |
| `workspace_and_db_mut_with_perm()` | No | Safe |
| `workspace_mut_and_db()` | No | Safe |
| `workspace_mut_and_db_with_perm()` | No | Safe |
| `workspace_and_db()` | No | Safe |
| `workspace_and_db_with_perm()` | No | Safe |
| `workspace_from_head()` | No | Safe |
| `meta_inner()` | No | Safe |
| `app_cache()` (static) | No | Safe |
| `meta()` | No | Safe |
| `to_sync()` / `into_sync()` | Discards `git2_repo` — needs gate in destructuring | Safe after gating |
| `project_data_dir()` | No | Safe |
| `workdir_or_gitdir()` | No | Safe |
| `workdir()` | No | Safe |
| `workdir_or_fail()` | No | Safe |
| `open_isolated_repo()` | No | Safe |
| `clone_repo_for_merging()` | No | Safe |
| `clone_repo_for_merging_non_persisting()` | No | Safe |
| `try_exclusive_access()` | No | Safe |
| `exclusive_worktree_access()` | No | Safe |
| `shared_worktree_access()` | No | Safe |

Only affected method:
- `with_git2_repo()` — gate entirely with `#[cfg(not(feature = "wasi"))]`

---

## Downstream Impact Assessment

### Crates that depend on but-ctx

Below is the full list, annotated by whether they touch `ctx.git2_repo` and whether they are compiled under a WASI `but` build.

| Crate | Touches `.git2_repo`? | In WASI `but` build? | Action needed |
|-------|----------------------|---------------------|---------------|
| `gitbutler-oplog` | Yes (7 sites) | No — `legacy` gated | None |
| `gitbutler-edit-mode` | Yes (many sites in lib + tests) | No — `legacy` gated | None |
| `but-workspace` (legacy/) | Yes (stacks.rs, head.rs, split_branch.rs) | No — `legacy` feature | None |
| `gitbutler-repo` (hooks, commands) | Yes | No — `legacy` gated | None |
| `gitbutler-workspace` | Yes (branch_trees, lib) | No — `legacy` gated | None |
| `gitbutler-watcher` | Yes | No — `legacy` gated | None |
| `gitbutler-repo-actions` | Yes | No — `legacy` gated | None |
| `gitbutler-operating-modes` | Yes | No — `legacy` gated | None |
| `gitbutler-branch-actions` | Yes (many sites) | No — `legacy` gated | None |
| `gitbutler-stack` | Yes | No — `legacy` gated | None |
| `but-api` (legacy/) | Yes | No — `legacy` feature | None |
| `but-worktrees` | Yes (integrate.rs, 3 sites) | No — `legacy` gated (needs gitbutler-branch-actions) | None |
| `gitbutler-tauri` | Yes (`ctx.with_git2_repo`) | Not a WASI crate | None |
| `gitbutler-testsupport` | Yes | Test only | None |
| `but` (non-legacy, config.rs, alias.rs, lib.rs) | Yes | **YES — in WASI build scope** | See below |
| `but-oplog` (legacy feature) | Uses `ctx.git2_repo` indirectly via `gitbutler-oplog` | No — `legacy` gated | None |
| `but-hunk-dependency` | No | Maybe | None |
| `but-gerrit` | No | Maybe | None |
| `but-cherry-apply` | No | No — `legacy` gated | None |
| `but-cursor` | No | Maybe | None |
| `but-link` | No | Maybe | None |
| `but-rules` | No | Maybe | None |
| `but-bot` | No | Maybe | None |
| `gitbutler-operating-modes` | Yes (operating_mode fn) | No — `legacy` gated | None |
| `gitbutler-branch-actions/tests` | Yes | Test only | None |
| `gitbutler-stack/tests` | Yes | Test only | None |

### Critical finding: `but` crate non-legacy usage

The `but` crate itself uses `ctx.git2_repo` in **non-legacy** code paths that would be present in a WASI build:

**`crates/but/src/command/config.rs`:**
- Line 55: `ctx.git2_repo.get()` — reads git config for user info display
- Line 115: `ctx.git2_repo.get()` — reads git config for TUI setting
- Line 328: `ctx.git2_repo.get()` — config `set` command
- Line 1052: `ctx.git2_repo.get()` — config `get` command
- Lines 268, 362–363, 395–396, 1091–1092, 1127–1128, 1154, 1168, 1170, 1177, 1184: `git2::Config` and `git2::ConfigLevel` usage

**`crates/but/src/command/alias.rs`:**
- Line 256: `ctx.git2_repo.get()` — set alias
- Line 294: `ctx.git2_repo.get()` — get alias

**`crates/but/src/lib.rs`:**
- Line 530: `ctx.git2_repo.get().ok()` — TUI-mode detection from git config

These are all in the `config` and `alias` commands — which are part of the target WASI command scope (see PR #1 INDEX.md: "Non-legacy, non-TUI commands: branch, config, alias, help, completions, skill, eval-hook").

**These call sites cannot simply be gated away — they represent real functionality needed by WASI commands.**

The long-term fix is to replace these `git2::Config` usages with `gix`-based config reads. However, this is a significant change that exceeds s09's scope. The recommended approach for s09 is:

**Strategy: Gate the field in `but-ctx`, return errors at call sites under WASI.**

Since `ctx.git2_repo.get()` already returns `anyhow::Result`, call sites in `but`'s non-legacy commands that call `.git2_repo.get()?` will need to either:
1. Be gated with `#[cfg(not(feature = "wasi"))]` with a WASI-compatible alternative (gix-based config), or
2. Return a `not supported on WASI` error under `#[cfg(feature = "wasi")]`

The `config` and `alias` commands' git2 usage is for reading/writing git config. `gix` has full config support. The WASI-path replacement is viable but constitutes implementation work beyond the planning stage.

**Recommendation: s09 scopes to `but-ctx` changes only; opens a follow-up task or sub-note that `config.rs` and `alias.rs` in `but` need gix-based config rewrites before `cargo check -p but --features wasi` can fully succeed.** The `cargo check -p but-ctx --features wasi` acceptance criterion IS achievable from s09 alone.

The `but` crate-level check (`cargo check -p but --no-default-features --features wasi`) depends on further work in s13 (first compile fixes) which addresses remaining cross-crate compile errors.

---

## Files to Modify

### 1. `crates/but-ctx/Cargo.toml`

- Add `wasi = []` feature
- Add `native = ["dep:git2"]` feature (for non-legacy native builds)
- Make `git2` optional
- Update `legacy` feature to activate `dep:git2`

```toml
[features]
legacy = ["dep:gitbutler-project", "but-project-handle/legacy", "dep:git2"]
native = ["dep:git2"]
wasi = []

[dependencies]
# ... existing ...
git2 = { workspace = true, optional = true }
```

### 2. `crates/but-ctx/src/lib.rs`

Changes (all using `#[cfg(not(feature = "wasi"))]`):

1. **Field in `Context` struct** (line 110): gate `pub git2_repo` field
2. **All 6 struct literal initialisations**: gate `git2_repo: new_ondemand_git2_repo(...)` lines
3. **`with_git2_repo` method** (lines 360–363): gate entire method
4. **`into_sync` destructure** (line 667): gate `git2_repo: _,` arm
5. **`new_ondemand_git2_repo` function** (lines 778–782): gate entire function

No changes needed to `ThreadSafeContext`, `access.rs`, `ondemand.rs`, `ondemand_cache.rs`, or `project_handle.rs`.

---

## What s09 Does NOT Do

- Does **not** modify any downstream crate call sites — all non-legacy callers of `.git2_repo` are themselves behind `#[cfg(feature = "legacy")]`
- Does **not** rewrite `config.rs` or `alias.rs` in the `but` crate to use gix-based config (that is follow-up work, likely in s13)
- Does **not** change `legacy.rs` in `but-ctx` (already behind `#[cfg(feature = "legacy")]`)
- Does **not** touch `ThreadSafeContext` (already WASI-safe)
- Does **not** change any command-level logic

---

## What s02 Provides That s09 Builds On

s02 established that `gix::ObjectId` is the universal ID type in all modern `but-*` crates. This means:
- All workspace, graph, and metadata operations already use `gix::Repository` and `gix::ObjectId`
- The `Context` struct's `repo: OnDemand<gix::Repository>` field (and all methods using it) are already WASI-compatible
- Only the `git2_repo` field is the blocker; its removal does not require any ID-type bridging work

---

## Dependency Flow After s09

```
but-ctx (wasi feature) ──► no git2 dependency compiled
    │
    └── Context struct has only:
            settings, gitdir, project_data_dir, app_cache_dir,
            repo (gix), db, app_cache, workspace cache
            (git2_repo absent)
```

s10 (gate `but-core` worktree/checkout) can then stack on s09's branch because it depends on `but-ctx` being clean for WASI first.

---

## Verification Steps

1. `cargo check -p but-ctx --features wasi` — must compile without `git2`
2. `cargo check -p but-ctx --features legacy` — must still compile (native legacy unaffected)
3. `cargo check -p but-ctx` — default (no features) must still compile; note: with `git2` now optional, the default build must activate it somehow — either through `native` feature or by having the workspace root pull it in
4. `cargo check -p but-ctx --features native` — non-legacy native build
5. `cargo test -p but-ctx` — all existing tests pass

**Note on default build:** Making `git2` optional means `cargo check -p but-ctx` (no features) will no longer have `git2`. This changes the default behaviour. The implementation must ensure that either:
- The workspace-level default features activate `native` (preferred), or
- The `default` feature in `but-ctx/Cargo.toml` includes `native`

Since other crates in the workspace do `git2.workspace = true` (unconditionally), the workspace-wide `git2` dep is always present for their builds. For `but-ctx` specifically, the cleanest solution is to add `default = ["native"]` to `but-ctx/Cargo.toml` so standalone `cargo check -p but-ctx` still works, and a WASI build explicitly disables defaults.

---

## Acceptance Criteria

- [ ] `crates/but-ctx/Cargo.toml`: `wasi = []` feature added
- [ ] `crates/but-ctx/Cargo.toml`: `native = ["dep:git2"]` feature added
- [ ] `crates/but-ctx/Cargo.toml`: `default = ["native"]` set so standalone builds continue to work
- [ ] `crates/but-ctx/Cargo.toml`: `legacy` feature activates `dep:git2`
- [ ] `crates/but-ctx/Cargo.toml`: `git2` dep is `optional = true`
- [ ] `Context.git2_repo` field gated `#[cfg(not(feature = "wasi"))]`
- [ ] All 6 constructor struct literals gate `git2_repo` initialisation
- [ ] `with_git2_repo` method gated `#[cfg(not(feature = "wasi"))]`
- [ ] `into_sync` destructuring pattern gates `git2_repo: _,`
- [ ] `new_ondemand_git2_repo` function gated `#[cfg(not(feature = "wasi"))]`
- [ ] `cargo check -p but-ctx --no-default-features --features wasi` compiles without `git2`
- [ ] `cargo check -p but-ctx --features legacy` compiles (native legacy unaffected)
- [ ] `cargo check -p but-ctx` compiles (default = native)
- [ ] `cargo test -p but-ctx` passes
- [ ] No downstream crate changes required (all `.git2_repo` callers are legacy-gated)
