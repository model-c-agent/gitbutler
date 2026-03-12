# Memory: wasi-first-compile (s13)

## Status: plan-complete

## Anticipated Error Categories

Derived from full analysis of s01–s12 plans and the current codebase state. These are
errors the implementation agent should expect when running
`cargo build -p but --no-default-features --features wasi --target wasm32-wasip2`.

### Category 1 — Missing feature-propagation wiring (HIGH PRIORITY)

The `wasi = []` feature in `crates/but/Cargo.toml` must forward to sub-crates that added their own
`wasi` feature. Without this, those crates compile as native even in a WASI build:
- `but-oxidize/wasi` — disables bridge code (s03)
- `but-core/wasi` — gates checkout/worktree, makes git2 optional (s10)
- `but-ctx/wasi` — gates git2::Repository field (s09)

**Fix:** extend the `wasi` feature line:
```toml
wasi = ["but-oxidize/wasi", "but-core/wasi", "but-ctx/wasi"]
```

NOTE: `native` feature forwards (e.g. `but-secret/native`, `but-path/native`) go on the
`native` feature line, NOT the `wasi` line.

### Category 2 — C library compilation failures (BUILD-SCRIPT LEVEL)

These fail before Rust compilation begins:

- `libgit2-sys` (from `git2`) — requires C cross-compiler. Blocked unless s07 fully excludes `git2`
  from the WASI dep tree via `native` gating.
- `libsqlite3-sys` (from `but-db` via `rusqlite bundled`) — requires WASI SDK clang.
- `openssl-sys` — requires `OPENSSL_NO_VENDOR=1` to use system OpenSSL (pre-existing).

**Fix sequence:**
1. Set `OPENSSL_NO_VENDOR=1`
2. Install/locate WASI SDK; set `CC_wasm32_wasip2` and `CFLAGS_wasm32_wasip2`
3. Verify `git2` is excluded from WASI tree: `cargo tree -p but --no-default-features --features wasi --target wasm32-wasip2 -i git2`
4. If rusqlite bundled still fails, implement Path B (JsonFileStorage) from s08 plan

### Category 3 — Missing `#[cfg]` gates for optional crates in `crates/but/src/`

After s06 makes networking crates optional, their `use` imports and subcommand arms remain
and will fail to compile:

- `crates/but/src/lib.rs` — `use args::{..., forge, ...}` still imports forge module unconditionally
- `crates/but/src/args/mod.rs` — forge/PR/update/LLM `Subcommands` variants need `#[cfg(feature = "native")]`
- `crates/but/src/args/forge.rs` — entire module needs `#[cfg(feature = "native")]`
- `crates/but/src/args/update.rs` — update subcommand args
- `crates/but/src/command/update.rs` — update command handler

**Fix:** add `#[cfg(feature = "native")]` to all affected use/mod/match-arm sites.

### Category 4 — `dirs` usage in `command/skill.rs`

`skill.rs` uses `dirs::home_dir()` in 5+ places. After s07 makes `dirs` optional under `native`
in `crates/but/Cargo.toml`, these will fail to compile.

**Fix:** gate the `skill` subcommand handlers with `#[cfg(feature = "native")]` or provide a
WASI stub that returns an appropriate error.

### Category 5 — `git2` still unconditional in `crates/but/Cargo.toml`

`git2.workspace = true` at the top of `[dependencies]` remains unconditional unless s07 has
explicitly moved it under `native`. If it remains, `libgit2-sys` enters the WASI build tree.

**Fix:** confirm s07 moved `git2` to optional under `native`; if not, do it in s13.

### Category 6 — `but-db` / rusqlite in WASI dep tree

`but-db` is a hard dep of `but-ctx` which is a hard dep of `crates/but`. The bundled SQLite C
compilation will be attempted. Resolution path:
- Path A: WASI SDK available → rusqlite compiles for wasip2
- Path B: WASI SDK unavailable or rusqlite fails → implement `JsonFileStorage` (s08 plan)

### Category 7 — `tokio::rt-multi-thread` leaking via non-optional transitive deps

After s06 gates all `rt-multi-thread`-using crates behind `native`, and s11 changes
`crates/but/Cargo.toml` to use target-specific tokio deps, there should be no `rt-multi-thread`
in the WASI dep tree.

Verify with: `cargo tree -p but --no-default-features --features wasi --target wasm32-wasip2 -i tokio`

If still present: identify the non-optional dep that pulls it in.

### Category 8 — `gix` threading/credentials features

`but-core` and `but-secret` both declare `gix` with `features = ["parallel", "credentials", ...]`.
s12 should remove `parallel` for WASI. If not, gix will still compile (parallel is pure Rust in
gix — it uses `std::thread` internally) but may panic at runtime.

`gix::credentials` may shell out via `std::process::Command`. Verify s12 gates this or that
s07's gating of `but-secret` (which uses `credentials`) is sufficient.

### Category 9 — `rmcp` dependency

`rmcp.workspace = true` is unconditional in `crates/but/Cargo.toml`. Check if it compiles for
`wasm32-wasip2`. If it uses `tokio::net`, it will fail.

Verify: `cargo tree -p but --no-default-features --features wasi --target wasm32-wasip2 -i rmcp`

If problematic: make `rmcp` optional under `native` or a new `mcp` feature.

### Category 10 — Linker / target configuration

Requires:
- `rustup target add wasm32-wasip2` (may already be done from s01)
- `.cargo/config.toml` with `[target.wasm32-wasip2]` linker setting
- `wasm-ld` or WASI SDK linker available in PATH

Only relevant for `cargo build` (not `cargo check`).

## Decisions

## Errors & Fixes

(to be populated during implementation)

## Blockers

- WASI SDK installation required for any crate that compiles C (rusqlite bundled, libgit2-sys)
- s06–s12 must be fully implemented before this PR runs; current status: plan-complete only
