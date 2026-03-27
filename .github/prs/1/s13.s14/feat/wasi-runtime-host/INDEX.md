# s14: Add but-wasi-host crate for sandboxed WASI execution via wasmtime

- **Branch:** `pr1/s13.s14/feat/wasi-runtime-host`
- **Anchor:** `pr1/s12.s13/feat/wasi-first-compile`
- **Deps:** s13 (working .wasm binary exists)
- **Size:** M
- **Commit message:** `feat: add but-wasi-host crate for sandboxed WASI execution via wasmtime`

## Scope

- New crate: `crates/but-wasi-host/`
- Binary target: `but-wasi`
- Wasmtime 29.0.1 embedding: load + execute `but.wasm` via component model
- Capability model: repo dir (rw preopen), config dir (ro preopen), selected env vars, stdio passthrough, clocks
- Networking disabled by default; `--allow-network` flag reserved for future wasi-http
- Module caching: AOT compilation via `Engine::precompile_component` + on-disk `.cwasm` cache
- CLI: `but-wasi [OPTIONS] -- <but-args>`

## Files

- `crates/but-wasi-host/Cargo.toml` (new)
- `crates/but-wasi-host/src/main.rs` (new)
- `crates/but-wasi-host/src/sandbox.rs` (new)
- `Cargo.toml` (add workspace member)

## Wasmtime API Plan

### Crate choice: component model vs core wasm

**Decision: Component model (`wasmtime::component::*`).**

`wasm32-wasip2` produces a component (not a core module). The `wasi:cli/run` world is the entry point. We must use `wasmtime_wasi::WasiCtx` + `wasmtime_wasi::add_to_linker_sync` (or async variant) with the component linker.

Key types:
- `wasmtime::Engine` — shared, configured once per process
- `wasmtime::component::Component` — loaded from `.wasm` or deserialized from AOT `.cwasm`
- `wasmtime::component::Linker<T>` — component linker, not the core `Linker`
- `wasmtime::Store<T>` — per-invocation state holding `T: WasiView`
- `wasmtime_wasi::WasiCtxBuilder` — builds the capability context
- `wasmtime_wasi::WasiCtx` — the constructed context (held in `T`)
- `wasmtime_wasi::WasiView` — trait implemented on `T` to expose `WasiCtx` + `ResourceTable` to wasmtime

### Dependency versions (wasmtime 29.0.1 series)

```toml
wasmtime           = { version = "29.0.1", default-features = false, features = ["component-model", "cranelift", "cache"] }
wasmtime-wasi      = { version = "29.0.1", default-features = false, features = ["sync"] }
```

`cap-std` is **not needed directly** — wasmtime-wasi bundles its own preopened-dir support via `WasiCtxBuilder::preopened_dir`. We avoid a separate `cap-std` dependency unless COW snapshot isolation is required (out of scope for s14; noted for s15).

### WasiView host state struct

```rust
struct HostState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}
```

### Linker setup

```rust
let mut linker: Linker<HostState> = Linker::new(&engine);
wasmtime_wasi::add_to_linker_sync(&mut linker)?;
```

## Capability Model

### Filesystem

| Directory | Mode | Rationale |
|-----------|------|-----------|
| `--repo <path>` | read-write | agents need to read/write the working tree and `.git/` |
| `--config <path>` (default: `$XDG_CONFIG_HOME/gitbutler`) | read-only | config is read at startup, never written by `but` |
| No other paths | blocked | default-deny via WASI preopens |

Preopening is done via `WasiCtxBuilder::preopened_dir`:

```rust
builder.preopened_dir(&repo_path, "/repo", DirPerms::all(), FilePerms::all())?;
builder.preopened_dir(&config_path, "/config", DirPerms::READ, FilePerms::READ)?;
```

The WASI guest sees `/repo` and `/config` as its filesystem roots. The `but` binary is expected (after s13's compile fixes) to accept a `--repo` flag or `$GITBUTLER_REPO` env var pointing at the virtual path `/repo`.

### Environment variables

Selective passthrough — only `GITBUTLER_*` prefixed vars and a fixed allowlist:

| Variable | Passthrough | Reason |
|----------|-------------|--------|
| `GITBUTLER_*` | yes (all) | application configuration |
| `HOME` | mapped to `/config` | config path resolution inside guest |
| `PATH` | no | guest cannot exec; PATH is meaningless |
| `RUST_LOG` | yes | tracing filter forwarded into guest |
| All others | no | default-deny |

Implementation: iterate `std::env::vars()`, filter by prefix/allowlist, call `builder.env(k, v)`.

### Networking

Disabled by default (no `wasi-http` linker entries added). `--allow-network` flag is parsed and recorded but currently a no-op with a warning printed to stderr; full network support is deferred to a future sub-PR using `wasi-http`.

### Stdio

Both stdout and stderr are inherited from the host process:

```rust
builder.inherit_stdout().inherit_stderr();
```

Stdin is closed (piped to empty) unless `--stdin` flag is passed (future).

### Clocks

WASI clocks (wall time + monotonic) are always enabled — they are part of `wasi:clocks` and are enabled by default when `wasmtime_wasi::add_to_linker_sync` is called. No special configuration needed.

## Module Caching Strategy (AOT)

Goal: startup under 500ms after first run.

### First run (cold)

1. Load `.wasm` bytes from disk.
2. Call `engine.precompile_component(&wasm_bytes)` → returns `Vec<u8>` of native code.
3. Write to cache file: `<module_path>.cwasm` (same directory as `.wasm`, or `--cache-dir` override).
4. Deserialize immediately from the bytes: `unsafe { Component::deserialize(&engine, &cwasm_bytes) }`.

### Subsequent runs (warm)

1. Check for `.cwasm` next to the `.wasm` (or in `--cache-dir`).
2. Verify cache is newer than `.wasm` (via mtime comparison).
3. Load: `unsafe { Component::deserialize_file(&engine, &cwasm_path) }`.
4. Skip JIT compilation entirely — native code is loaded directly.

### Cache invalidation

- If `.cwasm` mtime ≤ `.wasm` mtime → recompile.
- If wasmtime version changes (detected via `Engine` serialization header) → `deserialize` returns `Err` → recompile automatically.

### Engine configuration for AOT

```rust
let mut config = Config::new();
config.wasm_component_model(true);
config.cranelift_opt_level(OptLevel::Speed);  // balance startup vs throughput
// Disable Wasmtime's built-in in-process cache (we manage our own on-disk .cwasm)
config.disable_cache();
let engine = Engine::new(&config)?;
```

## CLI Interface Design

Binary name: `but-wasi`

```
but-wasi [OPTIONS] -- <but-args>...

Options:
  --repo <PATH>          Path to the git repository [required]
  --config <PATH>        Path to GitButler config dir [default: $XDG_CONFIG_HOME/gitbutler]
  --module <PATH>        Path to but.wasm [default: adjacent to but-wasi binary]
  --cache-dir <PATH>     Directory for .cwasm AOT cache [default: same dir as --module]
  --allow-network        Reserved; currently emits a warning (wasi-http not yet wired)
  --no-cache             Disable AOT cache; always JIT-compile from .wasm
  -h, --help             Show help
  -V, --version          Show version
```

Argument parsing uses `clap` (derive API, same as `but` crate). The `--` separator is handled by `clap`'s `last = true` trailing-args collection, passed through as `argv` to the WASM guest.

## Module Structure

### `src/main.rs`

Responsibilities:
- Parse CLI args via `clap` derive structs
- Resolve `--module` default (sibling to the `but-wasi` executable via `std::env::current_exe()`)
- Resolve `--config` default via `dirs::config_dir()`
- Call `sandbox::run(opts, but_args)` and propagate exit code
- Print friendly errors to stderr

### `src/sandbox.rs`

Responsibilities:
- Build `Engine` with component model + Cranelift
- Load or cache-compile the component (`load_component`)
- Build `WasiCtx` via `WasiCtxBuilder` (preopens, env vars, stdio)
- Create `Store<HostState>`
- Instantiate the component and call the `wasi:cli/run` export
- Return `anyhow::Result<i32>` (exit code from the guest)

Public API surface of `sandbox.rs`:

```rust
pub struct SandboxOptions { /* mirrors CLI opts */ }
pub fn run(opts: SandboxOptions, but_args: Vec<String>) -> anyhow::Result<i32>;
fn load_component(engine: &Engine, opts: &SandboxOptions) -> anyhow::Result<Component>;
fn build_wasi_ctx(opts: &SandboxOptions, but_args: &[String]) -> anyhow::Result<WasiCtx>;
```

## Workspace Integration

Add to `Cargo.toml` workspace members:

```toml
"crates/but-wasi-host",
```

Add workspace dependency:

```toml
but-wasi-host = { path = "crates/but-wasi-host" }
```

The crate is **not** added as a dependency of any other crate — it is a standalone binary.

## `crates/but-wasi-host/Cargo.toml` Structure

```toml
[package]
name = "but-wasi-host"
version = "0.0.0"
edition.workspace = true
authors.workspace = true
publish = false
rust-version.workspace = true
description = "Sandboxed WASI host for the but CLI via wasmtime"

[[bin]]
name = "but-wasi"
path = "src/main.rs"

[dependencies]
wasmtime           = { version = "29.0.1", default-features = false, features = ["component-model", "cranelift"] }
wasmtime-wasi      = { version = "29.0.1", default-features = false, features = ["sync"] }
anyhow.workspace   = true
clap               = { workspace = true, features = ["derive", "std", "help", "env"] }
dirs.workspace     = true
tracing.workspace  = true
tracing-subscriber = { workspace = true, features = ["env-filter", "std", "fmt"] }
```

Note: `wasmtime` and `wasmtime-wasi` are **not** in the workspace dependencies today (no existing wasmtime usage in the repo). They will be added as crate-local deps with explicit versions, matching the `29.0.1` version specified in the INDEX scope. If the implementation agent discovers a newer stable version during s14, they should document the version used in MEMORY.md and update this INDEX.

## Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Component model vs core wasm | Component model | `wasm32-wasip2` produces a component; required for `wasi:cli/run` |
| `cap-std` direct dependency | No | `wasmtime-wasi` preopened dir support is sufficient for s14 |
| COW filesystem isolation | Deferred to s15 | s14 focuses on getting the host working; COW adds complexity |
| Async vs sync wasmtime | Sync (`add_to_linker_sync`) | `but` is CLI, not a server; sync is simpler and sufficient |
| AOT cache format | `.cwasm` sibling file | Simple, no extra deps, wasmtime handles version mismatch gracefully |
| Networking | Disabled, `--allow-network` is a no-op | wasi-http is a future sub-PR |
| Env var policy | Allowlist: `GITBUTLER_*`, `RUST_LOG`, `HOME` | Default-deny; principle of least privilege |

## Acceptance Criteria

- `but-wasi --repo /path -- --help` works and shows `but` help output
- `but-wasi --repo /path -- branch list` works against a real repo
- Filesystem sandbox: attempting to access a path outside `--repo` fails inside the guest
- Network: no outbound connections possible from guest
- Startup with `.cwasm` cache: under 500ms on developer hardware
- `--no-cache` forces JIT recompile from `.wasm`
- Binary builds without warnings on macOS and Linux
