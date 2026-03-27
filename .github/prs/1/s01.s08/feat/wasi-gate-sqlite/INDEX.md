# s08: Gate or Adapt rusqlite for WASI Builds

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s08                                                |
| **Branch**| `pr1/s01.s08/feat/wasi-gate-sqlite`                |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: gate or adapt rusqlite for WASI builds`     |

## Status: plan-complete

## Risk: HIGH

---

## What but-db Provides

`crates/but-db` is the central persistence layer for GitButler. It exposes two database handles:

- `DbHandle` — the primary project database (`but.sqlite`) opened via `rusqlite::Connection`
- `AppCacheHandle` — the application cache database (`app-cache.sqlite`), also `rusqlite::Connection`, with an infallible fallback to in-memory mode

### Tables (11 active, plus removed migrations)

| Table | Operations |
|-------|-----------|
| `hunk_assignments` | list_all (SELECT), set_all (DELETE + INSERT via savepoint) |
| `butler_actions` | list (paginated SELECT + COUNT), insert |
| `workflows` | list (paginated SELECT + COUNT), insert, delete |
| `claude_sessions` / `claude_messages` / `claude_permission_requests` | full CRUD, session linking |
| `file_write_locks` | insert, delete, list |
| `workspace_rules` | list, insert, update, delete |
| `gerrit_metadata` | upsert, get, delete |
| `forge_reviews` | upsert, list, delete |
| `ci_checks` | upsert, list, delete |
| `virtual_branches` (vb_state, vb_stacks, vb_stack_heads, vb_branch_targets) | get_snapshot, replace_snapshot (FK-cascade deletes + multi-table inserts), set_state |
| `app-cache` table `update` | CachedCheckResult read/write |

### Key SQLite Features Used

- **WAL mode** — `PRAGMA journal_mode = WAL` is set at open time via `improve_concurrency()`. This uses WASI filesystem lock-based concurrency.
- **Savepoints** — used for all mutating handles instead of raw transactions (e.g., `hunk_assignments_mut`, `virtual_branches_mut`)
- **Busy timeout** — `conn.busy_timeout(5s)` for multi-process safety; set to `0` for non-blocking mode
- **Foreign keys** with `ON DELETE CASCADE`
- **Threading** — poll module uses `std::thread::spawn` for sync polling and `tokio::task::spawn` for async polling; both open a *second* database connection
- **`bundled` feature** — workspace-level `rusqlite = { version = "0.38.0", features = ["bundled"] }` — SQLite C source is compiled into the binary (no system libsqlite3 needed)

### Who Uses but-db (direct Cargo.toml dependencies)

14 crates depend on `but-db`:

- `but-server`, `gitbutler-stack`, `but-ctx`, `but-update`, `but-api`
- `gitbutler-watcher`, `gitbutler-tauri`, `but-action`, `but-testing`
- `but-claude` (with `poll` feature), `but-meta` (optional), `but-forge`
- `but-hunk-assignment`, `but-gerrit`, `but-link` (also uses `but-db::migration::improve_concurrency` directly), `but-rules`

Additionally:
- `crates/but-link` uses `rusqlite` directly (own db: `but-link.db`), calls `but_db::migration::improve_concurrency`
- `crates/but-cursor` uses `rusqlite` directly (reads Cursor IDE's `state.vscdb`)

---

## Investigation Results

### rusqlite 0.38.0 + bundled + wasm32-wasip2

**Assessment: Compilation is plausible but requires WASI SDK and has known risks.**

Evidence from rusqlite's GitHub:

1. **PR #1569 (merged Oct 2024)** — Fixed WASI compile flags not being enabled for pre-release `wasm32-wasip1` and `wasm32-wasip2` targets in bundled mode. The fix makes the build script match the `wasi` prefix so `wasip1` and `wasip2` both work. This is the critical enabler.

2. **PR #1769 (merged Dec 2025)** — Added `wasm32-unknown-unknown` support via `sqlite-wasm-rs`. This is a *different* target and a different approach (in-browser WASM, not WASI). Not directly relevant to `wasip2`.

3. **Issue #1459 (open)** — Cross-compilation of bundled SQLite for WASI targets fails on Apple M3 when macOS SDK headers leak into the WASI compiler's include path (`C_INCLUDE_PATH`, `CPLUS_INCLUDE_PATH` env vars contaminate the WASI clang invocation). The fix is to unset those env vars when cross-compiling. On Linux CI this is unlikely to be an issue.

4. **Issue #1735** — Cross-compilation target not respected in bundled libsqlite-sys build script. Affects cross-compilation scenarios but not native Linux.

**Bottom line for this project:**
- On Linux (CI / standard build), rusqlite with `bundled` has a reasonable chance of compiling for `wasm32-wasip2` if the WASI SDK is installed.
- On macOS, cross-compilation has documented failures due to SDK header contamination — a known unfixed upstream issue.
- No confirmed reproduction of successful `wasm32-wasip2 + bundled` build has been found, but PR #1569 was specifically designed to enable this.
- **The `cc` crate** (used by `libsqlite3-sys` to compile bundled C code) requires a WASI-capable C toolchain (clang from the WASI SDK) when targeting `wasm32-wasip2`.

### WAL Mode on WASI

WAL mode requires advisory file locking (`fcntl` or similar) and shared memory (`shm` files). WASI P2's filesystem model (via wasmtime) provides POSIX-like file access, but:

- Mandatory/advisory file locking semantics in WASI are runtime-dependent; wasmtime implements WASI P2 filesystem with reasonable POSIX compatibility
- `shm` files (WAL shared memory) are regular files in WASI — should work
- Multi-process WAL concurrency (multiple processes sharing one DB) is not a use case in the WASI context (a WASI component runs in isolation)
- Single-process WAL is much simpler and should work

**Likely safe to use WAL in WASI as a single-process scenario.** The polling feature (`poll.rs`) spawns threads and opens second connections — this requires `std::thread` which is available in WASI P2 (with the `wasi:io/poll` interface).

### but-link and but-cursor

- `but-link` has its own `rusqlite` dependency (direct, not via `but-db`). It is already behind the `legacy` feature in the `but` crate's Cargo.toml via indirect dependencies. The `but` CLI WASI build uses `--no-default-features --features wasi`, which excludes `legacy`, which excludes `but-link`. **No but-link changes needed for the wasi build.**
- `but-cursor` is also excluded from WASI builds via `legacy`. **No but-cursor changes needed.**

---

## Two Paths

### Path A: rusqlite Compiles for WASI (Optimistic Path)

**Preconditions:**
1. WASI SDK (clang for wasm32-wasip2) installed and available in PATH
2. `CC_wasm32_wasip2` env var set to the WASI clang
3. `CFLAGS_wasm32_wasip2` set to point at WASI sysroot
4. No contaminating host include paths (`C_INCLUDE_PATH` etc. unset)

**Changes needed in but-db:**
- Add `wasi` feature to `crates/but-db/Cargo.toml`
- Gate the `poll` feature (thread spawning + second db connection) behind `not(target_os = "wasi")` or a `wasi` feature exclusion
- Gate tokio dependency as `not(wasi)` (rt-multi-thread is not available in WASI)
- Verify `chrono` feature used by rusqlite works in WASI (likely fine, chrono has WASI support)
- `WAL + busy_timeout` should work as-is for single-process use

**Crates that need no changes:**
- `but-db` core API (tables, migrations, transactions, savepoints) — pure rusqlite CRUD
- `AppCacheHandle` — already has in-memory fallback for environments where file access fails

**Changes needed in workspace Cargo.toml:**
- If needed, add `wasm32-wasip2`-specific target dependency for rusqlite to disable bundled and use a different sysroot approach:
  ```toml
  [target.'cfg(target_os = "wasi")'.dependencies]
  rusqlite = { version = "0.38.0" }  # non-bundled, use WASI libc
  ```
  OR keep bundled and rely on the WASI SDK being present.

**Acceptance (Path A):**
- [ ] WASI SDK documented as CI prerequisite
- [ ] `cargo check -p but-db --target wasm32-wasip2` succeeds
- [ ] `cargo check -p but --no-default-features --features wasi --target wasm32-wasip2` succeeds
- [ ] WAL mode decision documented (use it or switch to DELETE mode for WASI)
- [ ] `poll` feature excluded from WASI builds or verified it compiles

---

### Path B: rusqlite Does Not Compile for wasm32-wasip2 (Fallback Path)

If `cargo build -p but-db --target wasm32-wasip2` fails due to the bundled C compilation failing, the fallback is a `Storage` trait abstraction.

**Architecture:**

```
but-db/src/lib.rs
  └── Storage trait (read/write interface for all tables)
        ├── SqliteStorage  (native: wraps rusqlite, all current code)
        └── JsonFileStorage  (wasi: one JSON file per table, simple append+replace)
```

**Storage trait methods needed** (derived from the full API surface):

| Method | Notes |
|--------|-------|
| `hunk_assignments_list_all` | returns `Vec<HunkAssignment>` |
| `hunk_assignments_set_all` | replaces entire table |
| `butler_actions_list(offset, limit)` | returns `(total, Vec<ButlerAction>)` |
| `butler_actions_insert` | appends one action |
| `workflows_list(offset, limit)` | returns `(total, Vec<Workflow>)` |
| `workflows_insert` / `workflows_delete` | mutate |
| `claude_sessions_*` | full CRUD, 6-8 methods |
| `claude_messages_*` | 3-4 methods |
| `claude_permission_requests_*` | 3-4 methods |
| `file_write_locks_*` | 3 methods |
| `workspace_rules_*` | 4 methods |
| `gerrit_metadata_*` | 3 methods |
| `forge_reviews_*` | 3 methods |
| `ci_checks_*` | 3 methods |
| `virtual_branches_get_snapshot` | returns `Option<VirtualBranchesSnapshot>` |
| `virtual_branches_replace_snapshot` | replaces all VB tables atomically |
| `virtual_branches_set_state` | upserts vb_state singleton |

**Estimated method count: ~35-40 methods**

**JsonFileStorage implementation:**
- One `HashMap<String, Vec<serde_json::Value>>` per table, serialized to a single JSON file
- Reads: deserialize from file at construction; mutations: serialize back after each write
- No transactions (acceptable for WASI single-process use; no concurrent writers)
- No WAL, no busy timeout needed
- No migrations needed (JsonFileStorage always reads current schema from JSON)

**Complexity estimate:**
- `Storage` trait definition: ~50-80 lines
- `SqliteStorage` wrapper: refactor current code, minimal new logic (~100-150 lines of wrapping)
- `JsonFileStorage`: ~300-500 lines (one HashMap per table, serde round-trips)
- Test coverage: existing tests cover SqliteStorage; JsonFileStorage needs basic smoke tests

**Cargo.toml changes for Path B:**
```toml
[features]
wasi = []  # enables JsonFileStorage, disables rusqlite dependency

[dependencies]
rusqlite = { workspace = true, features = ["chrono"], optional = true }
serde_json = { workspace = true }  # already pulled in transitively; make explicit

[target.'cfg(not(target_os = "wasi"))'.dependencies]
rusqlite = { workspace = true, features = ["chrono"] }
```

**Acceptance (Path B):**
- [ ] `Storage` trait defined in `but-db/src/lib.rs` or `but-db/src/storage.rs`
- [ ] `SqliteStorage` wraps all current rusqlite code unchanged
- [ ] `JsonFileStorage` implements all ~38 trait methods
- [ ] `DbHandle` re-exported as a type alias or enum over `dyn Storage`
- [ ] `#[cfg(not(target_os = "wasi"))]` guards around `rusqlite` imports
- [ ] `cargo check -p but-db --target wasm32-wasip2` succeeds
- [ ] Existing tests pass (native)

---

## Recommendation

**Start with Path A.** The rusqlite maintainers explicitly fixed wasip2 bundled mode (PR #1569) and this project's CI likely runs on Linux where the macOS header contamination issue does not occur. The implementation cost of Path A is far lower (~5-10 lines of Cargo.toml changes + gating the `poll` feature).

**Fallback to Path B** only if `cargo build -p but-db --target wasm32-wasip2` fails after setting up the WASI SDK properly. Path B is a significant but tractable abstraction effort (~800-1000 lines total).

**First task for implementation:** Run `cargo build -p but-db --target wasm32-wasip2` with the WASI SDK available and document the result in MEMORY.md immediately.

---

## Files to Touch

### Path A
- `crates/but-db/Cargo.toml` — add `wasi` feature; gate `poll`/tokio behind non-wasi
- `crates/but-db/src/migration.rs` — possibly gate WAL pragma for wasi (or leave as-is)
- Workspace `Cargo.toml` — possibly add target-specific rusqlite config

### Path B (additional)
- `crates/but-db/src/storage.rs` — new file: `Storage` trait + `JsonFileStorage`
- `crates/but-db/src/lib.rs` — re-export `DbHandle` as abstraction over Storage impl
- `crates/but-db/src/handle.rs` — refactor to use `Storage`
- All `crates/but-db/src/table/*.rs` — move into `SqliteStorage` impl

---

## Acceptance Criteria

- [ ] First implementation step: run `cargo build -p but-db --target wasm32-wasip2` and document result
- [ ] Decision (Path A or Path B) documented in MEMORY.md with build output as evidence
- [ ] `cargo check -p but-db --target wasm32-wasip2` succeeds (either path)
- [ ] `cargo check -p but --no-default-features --features wasi --target wasm32-wasip2` succeeds
- [ ] `poll` feature excluded from WASI builds or verified compatible
- [ ] WAL mode decision documented (use as-is, or switch to DELETE journal mode for WASI)
- [ ] No regression: `cargo test -p but-db` passes on native
- [ ] If Path B: `Storage` trait has tests for both `SqliteStorage` and `JsonFileStorage`
