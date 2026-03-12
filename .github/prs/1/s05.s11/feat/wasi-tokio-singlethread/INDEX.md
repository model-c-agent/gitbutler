# s11: Configure Single-Threaded Tokio Runtime for WASI

- **Branch:** `pr1/s05.s11/feat/wasi-tokio-singlethread`
- **Anchor:** `pr1/s01.s05/feat/wasi-gate-process`
- **Deps:** s05 (process gating must land first — tokio process features gated)
- **Size:** M
- **Commit:** `feat: configure single-threaded tokio runtime for WASI`

## Scope

- Native: tokio `rt-multi-thread`, `io-std`, `process`, `fs`, `net`, `time`
- WASI: tokio `rt`, `io-std`, `fs`, `time` only
- Conditional `#[tokio::main(flavor = "current_thread")]` in main.rs
- Audit: `block_in_place` (PANICS on current_thread), `spawn_blocking`
- Check `parking_lot` threading features

## Files

- `crates/but/Cargo.toml`
- `crates/but/src/main.rs`

## Acceptance Criteria

- Tokio features correct for WASI
- No `block_in_place` in WASI code paths
- Native remains multi-threaded

---

## Plan

### 1. Tokio Feature Matrix

#### Current state (`crates/but/Cargo.toml` line 102)

```toml
tokio = { workspace = true, features = ["rt-multi-thread", "io-std"] }
```

Only `rt-multi-thread` and `io-std` are declared in `crates/but/Cargo.toml` directly.
However, transitive dependencies pull in additional tokio features:

| Dependency | tokio features used |
|---|---|
| `crates/but/Cargo.toml` | `rt-multi-thread`, `io-std` |
| `but-update` | `rt-multi-thread` |
| `but-llm` | `rt-multi-thread`, `io-std` |
| `but-forge` | `rt-multi-thread` |
| `but-db` (optional, poll feature) | `rt-multi-thread`, `parking_lot`, `time`, `sync`, `macros` |
| `gitbutler-repo-actions` (legacy) | `rt-multi-thread` |
| `gitbutler-git` | `macros`, `rt-multi-thread` |
| `gitbutler-filemonitor` | `macros`, `rt-multi-thread`, `sync`, `time` |

#### Target configuration for `crates/but/Cargo.toml`

```toml
# native gets multi-thread; WASI gets single-thread only
[target.'cfg(not(target_os = "wasi"))'.dependencies]
tokio = { workspace = true, features = ["rt-multi-thread", "io-std"] }

[target.'cfg(target_os = "wasi")'.dependencies]
tokio = { workspace = true, features = ["rt", "io-std"] }
```

Note: `fs` and `time` are pulled in transitively and are WASI-safe. `rt-multi-thread`
is the only feature that must be excluded on WASI.

#### WASI-compatible tokio features (confirmed safe)
- `rt` — single-thread runtime, WASI ok
- `io-std` — stdin/stdout, WASI ok
- `fs` — file I/O, WASI ok
- `time` — timers, WASI ok
- `sync` — channels and mutexes, WASI ok
- `macros` — `tokio::main`, `tokio::test`, WASI ok

#### WASI-incompatible tokio features (must exclude)
- `rt-multi-thread` — spawns OS threads, WASI does not support
- `process` — process spawning, already gated by s05 behind `cfg(feature = "native")`
- `net` — TCP/UDP sockets, WASI does not support (no native net in wasip2 without WASI sockets preview)
- `signal` — Unix signals, WASI does not support

### 2. `block_in_place` — Critical Audit Result

**No occurrences of `block_in_place` found anywhere in `crates/`.**

This is the best possible outcome. `block_in_place` requires `rt-multi-thread` and panics on
`current_thread` runtime. Since it is not used, there are no sites to gate.

### 3. `spawn_blocking` — Full Inventory

`spawn_blocking` works on `current_thread` runtime but with a caveat: on `current_thread`,
`spawn_blocking` still creates a real OS thread from a dedicated thread pool. On WASI,
OS thread creation is not supported, so any `spawn_blocking` call will panic at runtime
on WASI targets.

All sites found in the codebase:

| File | Line | Feature-gated? | In WASI path? |
|---|---|---|---|
| `crates/but-settings/src/watch.rs` | 129 | No | No — settings watcher is not used on WASI (no file watch support) |
| `crates/but-claude/src/session.rs` | 1321, 1330, 1363, 1413, 1424, 1623, 1734, 1772, 1830, 1869 | No | No — `but-claude` is `optional = true` behind `legacy` feature |
| `crates/gitbutler-watcher/src/lib.rs` | 99 | No | No — `gitbutler-watcher` not a dep of `crates/but` |
| `crates/gitbutler-git/src/executor/tokio/windows.rs` | 114 | Implicit (windows path) | No — windows-only file |
| `crates/gitbutler-filemonitor/src/file_monitor.rs` | 348 | No | No — `gitbutler-filemonitor` not a dep of `crates/but` |
| `crates/but-api-macros/src/lib.rs` | 235 | Yes — `#[cfg(feature = "napi")]` | No — napi is desktop/tauri only |

**Conclusion:** No `spawn_blocking` calls are reachable from the WASI code path in `crates/but`.
All sites are either in crates not depended on by `but`, behind `legacy`/`native`/`napi` feature
gates, or in platform-specific (Windows) code. No additional gating is required for this PR.

### 4. `std::thread::spawn` — Inventory in Transitive Deps of `but`

Sites in crates that `crates/but` directly depends on:

| File | Lines | Feature-gated? | Notes |
|---|---|---|---|
| `crates/but-llm/src/openai_utils.rs` | 84, 113, 133, 163, 184 | No (but-llm is not wasi-gated) | Each spawns a thread to run a new `tokio::runtime::Runtime` synchronously |
| `crates/but-llm/src/anthropic.rs` | 423, 507, 563, 653, 700 | No | Same pattern — thread + nested runtime |
| `crates/but-forge/src/ci.rs` | 61 | No | Thread + blocking HTTP |
| `crates/but-forge/src/review.rs` | 567, 594, 808 | No | Thread + blocking HTTP |
| `crates/but-update/src/check.rs` | 113 | No | Thread + `tokio::runtime::Builder::new_current_thread()` |

**These are the most significant problem for WASI.** `std::thread::spawn` itself panics on WASI
because WASI Preview 2 does not support thread creation. However, these are all in crates used
for network I/O (LLM calls, forge API, update checks) which are expected to be gated at the
command-dispatch level (future PRs). For this PR's scope (tokio runtime config), these are
noted as future gating targets — they do not affect the tokio main macro change in `main.rs`.

### 5. `main.rs` Change

**Current:**
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
```

**Required (WASI uses `current_thread` flavor):**
```rust
#[cfg_attr(target_os = "wasi", tokio::main(flavor = "current_thread"))]
#[cfg_attr(not(target_os = "wasi"), tokio::main)]
async fn main() -> anyhow::Result<()> {
```

This is a clean two-attribute approach. No runtime builder boilerplate needed.

### 6. `parking_lot` Audit

`parking_lot` is used in:
- `crates/but-core/Cargo.toml` — `features = ["arc_lock"]` (no tokio integration)
- `crates/but-db/Cargo.toml` — `tokio = { ..., features = ["parking_lot", ...] }` (optional, poll feature)
- `crates/gitbutler-tauri/Cargo.toml` — `tokio = { ..., features = ["parking_lot"] }` (not a dep of `but`)

The tokio `parking_lot` feature replaces tokio's internal mutex/rwlock with `parking_lot` variants.
This feature is safe on WASI — it does not introduce thread-blocking behavior by itself. However,
`but-db` (which uses it) is only a transitive dep and its `poll` feature is optional.

The `parking_lot` crate itself (`parking_lot = "0.12.4"` in workspace) is not threaded-only —
it provides mutex primitives that work fine single-threaded. No gating required for this PR.

### 7. `tokio-util` Compatibility

`tokio-util` is only used by `crates/gitbutler-watcher/Cargo.toml`, which is not a dependency
of `crates/but`. No action required.

### 8. Summary of Changes for Implementation

**`crates/but/Cargo.toml`:**
- Replace the single `tokio = { workspace = true, features = ["rt-multi-thread", "io-std"] }` line
  with cfg-target-specific dependency blocks:
  - `[target.'cfg(not(target_os = "wasi"))'.dependencies]` → `features = ["rt-multi-thread", "io-std"]`
  - `[target.'cfg(target_os = "wasi")'.dependencies]` → `features = ["rt", "io-std"]`

**`crates/but/src/main.rs`:**
- Replace `#[tokio::main]` with the two `#[cfg_attr(...)]` attributes.

**No other files require modification in this PR.**

### 9. Out of Scope (Future PRs)

- Gating `std::thread::spawn` in `but-llm`, `but-forge`, `but-update` — these are
  network-heavy crates that will need their own gating PRs
- `spawn_blocking` in `but-settings` watcher — settings file watching will not work on WASI
  but the watch path is not called in WASI code flow
