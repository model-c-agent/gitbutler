# s13: First successful wasm32-wasip2 compilation of but CLI

- **Branch:** `pr1/s12.s13/feat/wasi-first-compile`
- **Anchor:** `pr1/s02-s11.s12/feat/wasi-gix-features`
- **Deps:** s12 (gix features configured)
- **Size:** M
- **Commit message:** `feat: first successful wasm32-wasip2 compilation of but CLI`

## Scope

- Run `cargo build -p but --no-default-features --features wasi --target wasm32-wasip2`
- Fix each remaining compilation error (missing gates, dep issues, API stubs)
- Iterate until compilation succeeds
- Test `wasmtime but.wasm -- --help`
- Record binary size

## Files

- Various — this is "fix what breaks"

## Acceptance Criteria

- Compilation succeeds
- `wasmtime but.wasm -- --help` shows output
- Binary size recorded in MEMORY.md
- All fixes documented

---

## Pre-Implementation Analysis

### Starting point after s01–s12

After all prior sub-PRs are implemented, the following gating is expected to be in place:

| Sub-PR | What it gates |
|--------|---------------|
| s01 | `wasi = []` Cargo feature in `crates/but`; `.cargo/config.toml` for wasm32-wasip2 |
| s02 | git2-based serde modules behind `legacy` only; gix-based always available |
| s03 | `but-oxidize`: `git2` optional, bridge code behind `#[cfg(not(feature = "wasi"))]` |
| s04 | `tui`, `ratatui`, `crossterm`, `minus`, `terminal_size` behind `tui` feature |
| s05 | `command-group`, `posthog-rs`, `machine-uid` optional; process spawning gated with `native` |
| s06 | `but-github`, `but-gitlab`, `but-forge`, `but-update`, `but-llm` optional under `native` |
| s07 | `keyring` optional in `but-secret`; `notify` optional in `but-settings`; `dirs`/`open` optional in `but-path`; `git2` optional in `crates/but` under `native` |
| s08 | `rusqlite` compiled for `wasm32-wasip2` (Path A) or replaced with `JsonFileStorage` (Path B) |
| s09 | `git2::Repository` field in `CommandContext` gated; `but-ctx` has `wasi` feature |
| s10 | `but-core` checkout/worktree module gated; `git2` + `but-oxidize` optional in `but-core` |
| s11 | Tokio uses `rt` only (no `rt-multi-thread`) for WASI; `main.rs` uses `current_thread` flavor |
| s12 | `gix` features audited; `parallel`, `credentials`, and threading features minimized for WASI |

### Anticipated Error Categories after s01–s12

Even with all prior gating in place, the following classes of errors are expected to appear when attempting `cargo build -p but --no-default-features --features wasi --target wasm32-wasip2`:

#### Category 1: Missing feature-propagation wiring in `crates/but/Cargo.toml`

The `wasi` feature in `crates/but` must propagate to all transitive crates that added their own `wasi` feature in s03, s09, s10. Without explicit forwarding, those crates compile with their default (native) configuration even when the top-level `wasi` feature is active.

Expected missing wiring:
- `wasi = ["but-oxidize/wasi", "but-core/wasi", "but-ctx/wasi", "but-secret/native", "but-settings/native", "but-path/native", "but-installer/native"]`
  - Note: `but-secret/native`, `but-settings/native`, `but-path/native`, and `but-installer/native` must be ABSENT from `wasi` (they enable native-only deps) — instead the `native` feature in `crates/but` must propagate these.
  - `but-oxidize/wasi`, `but-core/wasi`, `but-ctx/wasi` must be added to the `wasi` feature line so that these crates gate their `git2` usage.

Fix strategy: add feature propagation to the `wasi = [...]` line in `crates/but/Cargo.toml`.

#### Category 2: C-library deps that cannot cross-compile to wasm32-wasip2

Even with the planned gating, `rusqlite` (bundled C SQLite) and `libgit2-sys` require a WASI-capable C toolchain (`clang` from WASI SDK). The `CC_wasm32_wasip2` environment variable must point to a WASI-capable clang. Without this, the build script fails before any Rust code is compiled.

The second issue is `openssl-sys` — already known to fail in this environment; requires `OPENSSL_NO_VENDOR=1`.

Fix strategy:
1. Set `OPENSSL_NO_VENDOR=1`
2. Install WASI SDK and set `CC_wasm32_wasip2`, `CFLAGS_wasm32_wasip2`, and sysroot path
3. If rusqlite bundled still fails, implement Path B (JsonFileStorage) from s08's plan

Expected crates: `rusqlite`/`libsqlite3-sys` (via `but-db`), `libgit2-sys` (if `git2` is not fully excluded by gating).

#### Category 3: Missing `#[cfg]` gates in lib.rs / args/mod.rs for optional crates

When `but-github`, `but-gitlab`, `but-forge`, `but-update`, `but-llm` become optional (s06), their `use` statements and `Subcommands` arms in `crates/but/src/lib.rs` and `args/mod.rs` are still compiled unconditionally and will error with "can't find crate for `but_github`".

Expected files with missing gates:
- `crates/but/src/lib.rs` — `use args::{..., forge, ...}` import
- `crates/but/src/args/mod.rs` — forge/PR/update/LLM subcommand variants and match arms
- `crates/but/src/args/forge.rs` — entire module must be `#[cfg(feature = "native")]`
- `crates/but/src/args/update.rs` — update subcommand
- `crates/but/src/command/update.rs` — update command handler
- `crates/but/src/command/skill.rs` — `dirs` usage (5+ call sites) must be `#[cfg(feature = "native")]`

Fix strategy: add `#[cfg(feature = "native")]` guards to all import/use sites and subcommand definitions for the five optional crates.

#### Category 4: `dirs` usage remaining in `crates/but/src/command/skill.rs`

`skill.rs` uses `dirs::home_dir()` and `dirs::home_dir()?` in multiple places for locating `~/.claude/` and skill files. When `dirs` is made optional under `native` (s07), these will fail unless gated.

Fix strategy: gate the `skill` subcommand handlers and their `dirs` usage behind `#[cfg(feature = "native")]`.

#### Category 5: `git2` remaining as an unconditional dependency in `crates/but/Cargo.toml`

`git2.workspace = true` at line 97 of `crates/but/Cargo.toml` is currently unconditional. s07 plans to move it under `native`. Until that is done, `libgit2-sys` appears in the WASI dependency tree and the C build fails.

Fix strategy: confirm s07 has made `git2` optional in `crates/but/Cargo.toml` under `native`. If not done, do it in s13.

#### Category 6: `but-db` and `rusqlite` in the WASI dependency tree

`but-db` is not optional in `but-ctx`, and `but-ctx` is not optional in `crates/but`. The `rusqlite` dep in `but-db` uses `features = ["chrono"]` and `bundled` (workspace-level). This means `libsqlite3-sys` C compilation is attempted for `wasm32-wasip2`.

This is the highest-risk compile error. Resolution depends on the s08 outcome:
- Path A: WASI SDK installed, bundled SQLite compiles → no s13 work needed
- Path B: `but-db` uses `JsonFileStorage` under WASI → s13 may need to verify Path B is wired up

Fix strategy: attempt Path A first; only implement Path B if Path A fails.

#### Category 7: `tokio` multi-thread features leaking via transitive deps

After s11, `crates/but/Cargo.toml` uses target-specific deps to exclude `rt-multi-thread`. However, transitive deps like `but-update` (currently optional after s06), `but-llm` (optional after s06), and `but-forge` (optional after s06) each declare `tokio = { ..., features = ["rt-multi-thread"] }` in their own Cargo.toml. When those crates are excluded (under `native`), the transitive `rt-multi-thread` feature is also excluded.

After s06 makes these crates optional, this issue should self-resolve. Verify with `cargo tree -p but --no-default-features --features wasi -i tokio`.

If `rt-multi-thread` still appears, identify which non-optional crate pulls it in and add a fix.

#### Category 8: `gix` features that are threading or network-dependent

s12 audits and minimizes `gix` features. Remaining risks:
- `gix::parallel` — disables thread-pool parallelism for gix internally; needed for WASI
- `gix::credentials` — may attempt shell-outs via `std::process::Command`
- `gix::dirwalk` — pure Rust; safe
- `gix::status` — pure Rust; safe
- `gix::merge` — pure Rust; safe

Fix strategy: if s12 has not removed `parallel` from `but-core`'s gix features, remove it in s13 for the WASI target.

#### Category 9: `rmcp` dependency

`rmcp.workspace = true` is an unconditional dep in `crates/but/Cargo.toml`. The `rmcp` crate (Model Context Protocol) uses `tokio` and likely network I/O. Verify it compiles for `wasm32-wasip2`; if not, gate behind `native`.

Fix strategy: check if `rmcp` pulls in any `net` or `process` features; if so, make it optional under `native`.

#### Category 10: Linker / target configuration errors

The `wasm32-wasip2` target requires:
- `rustup target add wasm32-wasip2`
- Possibly `wasm-component-adapter` for WASI P2 component model
- A `.cargo/config.toml` entry setting the linker (`lld` or `wasm-ld`)

If s01 has not set this up, `cargo build` will fail with linker errors before any Rust code is checked.

Fix strategy: ensure `.cargo/config.toml` has correct `wasm32-wasip2` target configuration and that `wasm-ld` is available.

### Iterative Compile-Fix Strategy

The recommended approach for the implementation agent:

1. **First run:** `cargo check -p but --no-default-features --features wasi --target wasm32-wasip2 2>&1 | head -50`
   - Categorize the first errors — do they fall into the above categories?
   - Fix the highest-priority blocker first (usually C library compilation or missing feature wiring)

2. **Fix feature wiring** (Category 1) before all others — this is a force multiplier. One line in `wasi = [...]` can eliminate dozens of downstream errors.

3. **Fix C library deps** (Categories 2, 5, 6) next — these are build-script failures and appear before Rust compilation:
   - Set `OPENSSL_NO_VENDOR=1`
   - Set up WASI SDK for bundled SQLite
   - Verify `git2` is fully excluded

4. **Fix missing `#[cfg]` gates** (Categories 3, 4) — these produce type/import errors and are the most numerous but also the most mechanical

5. **Check tokio / threading** (Categories 7, 8) — typically appears late in the error stream once import errors are fixed

6. **Check linker** (Category 10) — only relevant when `cargo build` (not `cargo check`) is invoked

7. For each fixed error: record in MEMORY.md the exact error text, file, line, fix applied, and rationale

### Crates Most Likely to Have Remaining Issues (in priority order)

| Crate | Risk | Expected issue |
|-------|------|----------------|
| `but-db` | HIGH | `rusqlite` bundled C; may require WASI SDK or Path B |
| `crates/but` | HIGH | Missing feature-propagation wiring; missing `#[cfg]` gates in `args/mod.rs`, `lib.rs`, `command/skill.rs` |
| `but-ctx` | HIGH | `git2::Repository` field; needs `wasi` feature propagated from `crates/but` |
| `but-core` | HIGH | `git2` + `but-oxidize` unconditional if `but-core/wasi` not propagated |
| `rmcp` | MEDIUM | May pull in `net` or threading |
| `but-settings` | MEDIUM | `notify` C binding; needs `native` feature gated |
| `but-secret` | MEDIUM | `keyring` OS APIs; needs `native` feature |
| `but-path` | MEDIUM | `dirs` + `open`; needs `native` feature |
| `gix` (parallel) | MEDIUM | Threading feature if not removed by s12 |
| `but-oxidize` | LOW | `wasi` feature propagation (s03 done; just needs wiring in s13) |
| `but-installer` | LOW | `dirs` usage; made optional or gated by s07 |
| `but-github` / `but-gitlab` / `but-forge` / `but-update` / `but-llm` | LOW | Made optional by s06; should be excluded |

### WASI SDK Requirement

The `wasm32-wasip2` target requires a WASI-capable C toolchain for any crate that compiles C code. Specifically:

- WASI SDK provides `clang` for `wasm32-wasip2`
- Set `CC_wasm32_wasip2=/path/to/wasi-sdk/bin/clang`
- Set `CFLAGS_wasm32_wasip2=--sysroot=/path/to/wasi-sdk/share/wasi-sysroot`

If the WASI SDK is not installed, every crate with a `build.rs` that compiles C will fail. Check with:
```
which wasm32-wasip2-clang || ls /opt/wasi-sdk/bin/clang 2>/dev/null
```

If unavailable, document in MEMORY.md as a blocker and proceed with `cargo check` (not `cargo build`) to find all Rust-level errors.
