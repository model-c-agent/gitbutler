# WASI Compatibility: Next Steps Plan

This plan covers what's needed to make the `but` CLI fully functional
under WASI, beyond the current "it compiles and basic commands work"
milestone.

---

## Current State (s13 complete)

**What works:**
- Compilation to `wasm32-wasip2` (~3 MB release binary)
- `--help`, `--version`, `completions`, `gui open` (errors gracefully)
- Basic gix-based git operations (branch listing, log reading)
- Sandbox host (`but-wasi-host`) with capability-based preopens

**What's stubbed or gated out:**
- All `legacy` commands (status, commit, push, pull, branch new, etc.)
- All `native` commands (config, alias add/remove, update install, skill)
- Database operations (but-db: rusqlite stubbed)
- Secret storage (but-secret: keyring stubbed)
- File watching (but-settings: notify gated)
- Inter-process locking (but-core: fslock gated)
- git2 bridge (but-oxidize: entirely gated)
- TUI (ratatui, crossterm gated)
- Metrics/telemetry (posthog gated)
- Process spawning (command-group gated)

---

## Phase 1: Core Read-Only Operations

**Goal:** Make `but status`, `but branch list`, `but show`, `but diff`,
`but log` work in WASI mode — the read-only inspection commands that
don't modify the repo.

### 1.1 Enable `but-workspace` for WASI

**Why:** `but status`, `but branch list`, and `but show` all go through
`but-workspace` which is currently behind the `native` feature gate.

**What's needed:**
- Audit `but-workspace` dependencies — it pulls in `but-rebase`,
  `but-graph`, `but-hunk-assignment`, `but-forge-storage`
- Most of these are pure Rust and should compile for WASI
- The blocker is transitive dependencies on `git2` (via `but-oxidize`)
  and `reqwest` (via forge crates)
- Create a `wasi` feature in `but-workspace` that excludes forge and
  git2-dependent code paths

**Files to modify:**
- `crates/but-workspace/Cargo.toml` — add feature gates
- `crates/but-workspace/src/` — gate `git2`-dependent code
- `crates/but/Cargo.toml` — include `but-workspace` in WASI builds

**Estimated scope:** ~15 files, mostly mechanical cfg gating

**Dependencies:** 1.2, 1.3

### 1.2 Enable `but-graph` for WASI

**Why:** Branch listing and status display depend on the commit graph.

**What's needed:**
- `but-graph` is pure gix-based — should compile with minimal gating
- Check for any `git2` or platform-specific dependencies
- May need to gate out graph visualization that depends on TUI

**Files to modify:**
- `crates/but-graph/Cargo.toml`
- `crates/but-graph/src/` — audit for platform deps

**Estimated scope:** ~5 files

### 1.3 Enable `but-rebase` for WASI

**Why:** The rebase engine is needed for workspace status computation
(it calculates the virtual branch state).

**What's needed:**
- `but-rebase` depends on `but-core` (already partially WASI-ready)
  and `but-oxidize` (gated to no-op)
- Audit for `git2::Repository` usage — these need to go through `gix`
- The `SignCommit` trait may need a WASI implementation

**Files to modify:**
- `crates/but-rebase/Cargo.toml`
- `crates/but-rebase/src/` — replace git2 calls with gix equivalents

**Estimated scope:** ~10 files, some non-trivial refactoring

### 1.4 Real SQLite for WASI

**Why:** `but-db` is stubbed to a no-op. Status, oplog, and workspace
tracking all need persistent storage.

**What's needed:**
- Use `wasi-sqlite` or `sqlite-wasm` — a SQLite build that compiles
  to WASI without requiring native C compilation
- Alternative: use `rusqlite` with the `bundled` feature and
  WASI-compatible C compilation (requires `wasi-sdk`)
- Alternative: use a pure-Rust SQLite implementation (e.g., `limbo`)
  once it matures
- Replace the stub `DbHandle` and `AppCacheHandle` with real
  implementations

**Files to modify:**
- `crates/but-db/Cargo.toml` — add WASI-compatible SQLite dep
- `crates/but-db/src/lib.rs` — remove stubs, wire up real impl
- Build system — may need `wasi-sdk` for C compilation

**Estimated scope:** Medium — the code exists, just needs a working
SQLite. The hard part is getting SQLite to compile for WASI.

**Options ranked by feasibility:**
1. `limbo` (pure Rust, WASI-native) — best long-term, may not be ready
2. `rusqlite` + `wasi-sdk` — proven, but adds C toolchain requirement
3. File-based JSON/TOML — minimal but loses SQL query capabilities

### 1.5 Enable `config` commands for WASI

**Why:** `but config` is gated behind `native` but most of it just
reads/writes files — no OS-specific functionality required.

**What's needed:**
- Split `config` command into read operations (WASI-safe) and
  write operations (may need native for some backends)
- The `config user`, `config target`, `config ui` subcommands are
  file-based — should work in WASI
- `config forge` needs network — keep gated

**Files to modify:**
- `crates/but/src/args/mod.rs` — finer-grained feature gates
- `crates/but/src/lib.rs` — dispatch config subcommands conditionally
- `crates/but/src/command/config.rs` — gate forge-specific ops

**Estimated scope:** ~5 files

---

## Phase 2: Write Operations

**Goal:** Make `but commit`, `but branch new`, `but amend`, `but squash`
work in WASI mode — commands that modify the local repo but don't touch
the network.

### 2.1 Enable commit creation

**Why:** This is the core value proposition — creating commits from
within the sandbox.

**What's needed:**
- `but commit` goes through `but-workspace` → `but-rebase` → `but-core`
- The commit path ultimately calls `gix` for object creation, which
  works on WASI
- The blocker is `but-hunk-assignment` (assigns file changes to
  branches) which may have platform deps
- Need to ensure `gix-tempfile` works on WASI (known issue:
  `std::process::id()` is unsupported, called from gix_tempfile)

**Critical blocker: `gix_tempfile` and `std::process::id()`**

This is the runtime error discovered in s14. When `but` tries to
create temp files (for any write operation), `gix_tempfile` calls
`std::process::id()` which panics on WASI. This needs to be fixed
upstream in gitoxide, or worked around.

**Options:**
1. **Upstream fix in gix_tempfile** — use a fallback (timestamp, random)
   when `process::id()` is unavailable. Byron maintains gitoxide, so
   coordinate with him.
2. **Cargo patch** — apply a local patch to gix_tempfile that catches
   the panic and uses a fallback
3. **Avoid tempfiles** — restructure the commit path to not use
   gix_tempfile (impractical — gix uses it internally)

**Estimated scope:** Depends on upstream — could be a 1-line fix in
gitoxide or a multi-crate workaround

### 2.2 Enable branch creation and deletion

**Why:** `but branch new` and `but branch delete` are fundamental
operations.

**What's needed:**
- These go through `but-workspace` which we're enabling in Phase 1
- Branch creation needs `gix` ref operations (should work)
- Branch deletion needs ref deletion (should work)
- Virtual branch state needs `but-db` (Phase 1.4)

**Estimated scope:** Small — mostly falls out of Phase 1 work

### 2.3 Enable amend, squash, reword

**Why:** Commit editing is a core GitButler feature.

**What's needed:**
- All go through `but-rebase` (Phase 1.3)
- `but amend` needs hunk-level operations via `but-hunk-assignment`
- `but squash` needs rebase operations
- `but reword` needs ref rewriting

**Estimated scope:** Medium — depends on Phase 1.3 completion

### 2.4 Inter-process locking for WASI

**Why:** `fslock` is gated out, but concurrent access to the repo
(multiple WASI instances) needs some form of coordination.

**What's needed:**
- WASI doesn't support `flock()` syscalls
- Option 1: The host provides locking via a WIT interface
- Option 2: Use advisory lock files (create/delete) instead of flock
- Option 3: Accept single-writer semantics (only one WASI instance
  writes at a time, enforced by the host)

**Recommendation:** Start with Option 3 (host-enforced single-writer),
plan for Option 1 (WIT-based locking) long-term.

**Estimated scope:** Small for Option 3, large for Option 1

---

## Phase 3: Network Operations

**Goal:** Make `but push`, `but pull`, `but fetch`, `but pr` work in
WASI mode — requires WASI socket capabilities.

### 3.1 WASI socket support for HTTP

**Why:** Push/pull/fetch require HTTP(S) to talk to git remotes.

**What's needed:**
- WASI Preview 2 includes `wasi:sockets` for TCP/UDP
- `reqwest` doesn't support WASI sockets natively
- Options:
  1. Use `wasi-http` (the WASI HTTP proxy world) — requires the host
     to provide an HTTP handler
  2. Use a WASI-compatible HTTP client (e.g., built on `wasi:sockets`)
  3. Proxy network requests through the host via WIT interface

**Recommendation:** Use `wasi-http` via wasmtime's HTTP proxy. This
is the standard WASI approach and gives the host full control over
network access (allowlists, rate limiting, etc.).

**Files to modify:**
- `crates/but-wasi-host/src/sandbox.rs` — add `wasi:http` capability
- New crate or module for WASI HTTP adapter
- `crates/but-forge/` — needs WASI-compatible HTTP backend

**Estimated scope:** Large — requires new HTTP plumbing

### 3.2 Enable forge operations (GitHub, GitLab)

**Why:** `but pr new`, `but pr list`, etc. need forge API access.

**What's needed:**
- `but-github` and `but-gitlab` use `reqwest` for HTTP
- Need WASI HTTP adapter (3.1)
- `but-forge` trait should work as-is if HTTP is available
- Authentication: secrets can't use keyring, need env var or
  host-provided tokens

**Files to modify:**
- `crates/but-github/Cargo.toml` — conditional deps
- `crates/but-gitlab/Cargo.toml` — conditional deps
- `crates/but-forge-storage/` — needs but-db (Phase 1.4)

**Estimated scope:** Medium — mostly plumbing once HTTP works

### 3.3 Git remote operations (push/pull/fetch)

**Why:** The fundamental distributed git operations.

**What's needed:**
- `gix` handles git protocol and smart HTTP transport
- gix's HTTP backend needs to work over WASI sockets or wasi-http
- SSH transport is unlikely to work on WASI — focus on HTTPS
- Credential handling: env vars or host-provided tokens (no keyring)

**Estimated scope:** Large — depends on gix's WASI HTTP transport status

---

## Phase 4: Enhanced Sandbox

**Goal:** Improve the `but-wasi-host` with better isolation, WIT
interfaces, and multi-instance support.

### 4.1 WIT interface for structured communication

**Why:** Currently the host/guest communicate only via CLI args,
stdout/stderr, and exit codes. A WIT interface enables typed,
structured communication.

**What's needed:**
- Define a WIT interface for GitButler operations:
  ```wit
  interface gitbutler {
    record branch { name: string, commits: list<commit> }
    record commit { id: string, message: string }
    status: func() -> list<branch>
    commit: func(message: string, files: list<string>) -> commit
    // ...
  }
  ```
- Generate Rust bindings for host and guest
- This replaces CLI arg passing with direct function calls

**Estimated scope:** Large — new API design and codegen

### 4.2 Instance pooling

**Why:** AOT compilation is fast but not free. A pool of pre-warmed
WASI instances can serve requests with sub-millisecond latency.

**What's needed:**
- Pre-compile `.cwasm` on first run (already done)
- Keep a pool of instantiated modules with different repo preopens
- Handle instance lifecycle (create, reuse, destroy)

**Estimated scope:** Medium

### 4.3 Host-mediated locking

**Why:** Multiple WASI instances accessing the same repo need
coordination.

**What's needed:**
- WIT interface for lock acquisition/release
- Host maintains a lock table keyed by repo path
- Guest calls `acquire-lock(repo)` before write operations

**Estimated scope:** Medium — WIT interface + host implementation

### 4.4 Host-mediated secrets

**Why:** The guest can't access the system keyring. The host should
provide secrets on demand, scoped to the guest's permissions.

**What's needed:**
- WIT interface: `get-secret(name: string) -> option<string>`
- Host reads from keyring/env and provides to guest
- Guest's `but-secret` uses WIT instead of direct keyring access
- Host can implement approval UI ("Allow this module to access
  your GitHub token?")

**Estimated scope:** Medium

---

## Phase 5: Browser Compilation

**Goal:** Run `but.wasm` in the browser via Component Model JS bindings.

### 5.1 `jco` compatibility

**Why:** `jco` (JavaScript Component Tools) can transpile WASI
components to run in browsers.

**What's needed:**
- Verify the `.wasm` component works with `jco transpile`
- Identify browser-incompatible syscalls
- Provide JS shims for filesystem (virtual FS / IndexedDB)
- Provide JS shims for HTTP (fetch API)

**Estimated scope:** Large — research and experimentation phase

### 5.2 Virtual filesystem for browser

**Why:** The browser has no real filesystem. Need a virtual FS backed
by IndexedDB or similar.

**What's needed:**
- Implement WASI filesystem interface over IndexedDB
- Or use an existing solution (e.g., Emscripten's virtual FS adapted
  for WASI)
- Repository data stored in IndexedDB

**Estimated scope:** Very large

---

## Dependency Graph

```
Phase 1 (Read-Only)
├── 1.1 but-workspace ─┬── 1.2 but-graph
│                       ├── 1.3 but-rebase
│                       └── 1.4 SQLite
└── 1.5 config commands

Phase 2 (Write)
├── 2.1 commit creation ←── gix_tempfile fix (CRITICAL BLOCKER)
├── 2.2 branch ops ←── 1.1
├── 2.3 amend/squash ←── 1.3
└── 2.4 locking

Phase 3 (Network)
├── 3.1 WASI HTTP ←── wasi:sockets
├── 3.2 forge ops ←── 3.1, 1.4
└── 3.3 git remote ←── 3.1

Phase 4 (Enhanced Sandbox)
├── 4.1 WIT interface
├── 4.2 instance pooling
├── 4.3 host locking ←── 4.1
└── 4.4 host secrets ←── 4.1

Phase 5 (Browser)
├── 5.1 jco compat ←── all phases
└── 5.2 virtual FS ←── 5.1
```

---

## Critical Blockers (must resolve first)

### Blocker 1: `gix_tempfile` calls `std::process::id()`

**Impact:** ALL write operations fail at runtime
**Location:** `gix-tempfile` crate (upstream gitoxide)
**Root cause:** `std::process::id()` is not supported on WASI
**Who:** Coordinate with @Byron (gitoxide maintainer)
**Fix:** Add `#[cfg(target_os = "wasi")]` fallback using timestamp or
random number instead of PID for temp file uniqueness
**Workaround:** Cargo `[patch]` with local fix until upstream merges

### Blocker 2: SQLite compilation for WASI

**Impact:** No persistent state (oplog, workspace tracking, cache)
**Root cause:** `rusqlite` bundles C code that needs a C compiler
targeting `wasm32-wasip2`
**Options:**
1. Pure-Rust SQLite (`limbo`) — ideal but may not be production-ready
2. `rusqlite` + `wasi-sdk` C toolchain — proven but complex build
3. Skip SQLite, use file-based storage — limits functionality

### Blocker 3: HTTP client for WASI

**Impact:** No network operations (push, pull, fetch, forge APIs)
**Root cause:** `reqwest` and `hyper` don't support WASI sockets
**Options:**
1. `wasi-http` proxy world in wasmtime — standard approach
2. Build HTTP on raw `wasi:sockets` — lower-level but possible
3. Host proxies all HTTP — simple but limits guest autonomy

---

## Recommended Ordering

1. **Now:** Fix gix_tempfile blocker (coordinate with Byron)
2. **Next:** Enable but-graph and but-rebase for WASI (Phase 1.2, 1.3)
3. **Then:** Enable but-workspace for WASI (Phase 1.1)
4. **Then:** Real SQLite (Phase 1.4) — unblocks persistent state
5. **Then:** Commit creation (Phase 2.1) — first write operation
6. **Then:** Network via wasi-http (Phase 3.1) — unblocks everything else
7. **Later:** WIT interfaces (Phase 4.1) — optimization and API design
8. **Eventually:** Browser (Phase 5) — requires all above

---

## Metrics for Success

| Milestone | Metric | Target |
|-----------|--------|--------|
| Phase 1 complete | `but status` works in WASI | Read-only operations functional |
| Phase 2 complete | `but commit` works in WASI | Local write operations functional |
| Phase 3 complete | `but push` works in WASI | Full distributed workflow |
| Binary size | Release `.wasm` | < 8 MB |
| Cold start | First command latency | < 500ms (with AOT cache) |
| Test coverage | WASI integration tests | > 30 tests |
