# feat: compile `but` CLI to WASI (wasm32-wasip2)

## The idea

What if your git tooling could run anywhere -- in a browser tab, inside a
CI sandbox, on an edge node, embedded in an AI agent's tool loop -- with
the same binary, the same behavior, and a hard security boundary that
makes "oops I rm -rf'd the wrong repo" structurally impossible?

That's what WASI Preview 2 gives us. This PR series compiles the `but`
CLI to `wasm32-wasip2`. The result is a **~3 MB `.wasm` component** that
runs inside Wasmtime with capability-based sandboxing: the guest binary
can only touch directories the host explicitly preopens, has no network
access, and no ambient authority over the filesystem.

It works today:

```
$ cargo build -p but --no-default-features --features wasi \
    --target wasm32-wasip2 --release

$ ls -lh target/wasm32-wasip2/release/but.wasm
-rwxr-xr-x 1 user user 3.0M ... but.wasm

$ but-wasi --repo ./my-project -- branch
  main
* feat/something
```

## Why this matters for GitButler

1. **Sandboxed execution.** The `but-wasi-host` crate wraps Wasmtime
   with a purpose-built sandbox. The guest sees `/repo` (read-write) and
   `/config` (read-only) -- nothing else. Environment variables are
   filtered to `GITBUTLER_*` and `RUST_LOG`. `HOME` is remapped to
   `/config`. This is not a convention; it's enforced by the WASI
   capability model.

2. **Portable, hermetic binaries.** A `.wasm` component is
   OS/arch-independent. The same artifact runs on Linux x86, macOS
   ARM, Windows, or anywhere Wasmtime (or any WASI-P2 runtime) is
   available. No cross-compilation matrix needed for the core logic.

3. **Embeddability.** A 3 MB module can be loaded by any host that
   speaks WASI. That means the `but` CLI can be embedded in other
   tools -- VS Code extensions, CI actions, agent runtimes -- without
   shelling out to a native binary or worrying about PATH resolution.

4. **A path to the browser.** WASI-P2 components can be compiled to
   run in browser environments (via `jco` / Component Model JS
   bindings). This is not immediate, but the architecture is now in
   place.

## Architecture

### Feature gating strategy

The `but` crate uses two marker features to separate platform-specific
code:

| Feature | What it gates |
|---------|---------------|
| `native` | Everything that requires a "real" OS: `git2`, `keyring`, `posthog`, `command-group`, forges, LLM, workspace/rebase/graph (heavy transitive deps), interactive prompts |
| `tui` | Terminal UI: `ratatui`, `crossterm`, `minus`, `terminal_size` |

The default feature set is `["legacy", "tui", "native"]`. A WASI build
uses `--no-default-features --features wasi`, which strips both `native`
and `tui` and all their transitive dependencies.

### `cfg` gating in library crates

Where feature flags aren't sufficient (shared library crates that don't
know about `but`'s features), we use `cfg(target_os = "wasi")` /
`cfg(not(target_os = "wasi"))` to swap implementations. This touches
~15 crates across the workspace:

| Crate | What's gated |
|-------|-------------|
| `but-path` | `dirs` crate replaced with env-var-based path resolution |
| `but-secret` | Keyring persistence replaced with no-op stubs |
| `but-db` | `rusqlite` swapped for in-memory stub |
| `but-ctx` | `git2::Repository`, file-watching, inter-process locks |
| `but-core` | `fslock`-based sync primitives and process-dependent checkout |
| `but-settings` | File-system watcher (`notify`) gated out |
| `but-oxidize` | Entire `git2 <-> gix` bridge module gated out |
| `but-meta` | Legacy-mode code that depends on `git2` |
| `but-serde` | `git2::Oid` serde impls gated behind `native` feature |
| `but` (binary) | Tokio runtime (`current_thread` vs `multi-thread`), telemetry, GUI open |

### The sandbox host: `but-wasi-host`

A new crate (`crates/but-wasi-host`) provides the `but-wasi` binary --
a thin Wasmtime wrapper that:

- Preopens the repository directory at `/repo` (read-write)
- Preopens the config directory at `/config` (read-only)
- Forwards filtered env vars (`GITBUTLER_*`, `RUST_LOG`)
- Remaps `HOME=/config`
- Supports AOT pre-compilation cache (`.cwasm`) with staleness detection
- Inherits stdout/stderr for transparent output

```
but-wasi --repo /path/to/repo -- branch list
but-wasi --repo /path/to/repo --no-cache -- config --json
```

## Demo

### Build the WASM component

```bash
# Add the target (one-time)
rustup target add wasm32-wasip2

# Release build (~3 MB)
cargo build -p but --no-default-features --features wasi \
    --target wasm32-wasip2 --release
```

### Build the host

```bash
cargo build -p but-wasi-host
```

### Run it

```bash
# Show help
./target/debug/but-wasi --repo /path/to/repo -- --help

# List branches
./target/debug/but-wasi --repo /path/to/repo -- branch

# Or use wasmtime directly
wasmtime run \
    --dir /path/to/repo::/repo \
    --dir ~/.config/gitbutler::/config \
    --env HOME=/config \
    target/wasm32-wasip2/release/but.wasm -- --help
```

### Run the tests

```bash
WASI_WASM_PATH=target/wasm32-wasip2/debug/but.wasm \
    cargo test -p but-wasi-host
```

## PR series

The work is split into 10 stacked PRs, each building on the previous:

| # | PR | Description |
|---|-----|-------------|
| 1 | [PR #1] | **Feature flags.** Introduces `native` and `tui` features, makes platform-specific deps optional. No behavioral change with default features. |
| 2 | [PR #2] | **First WASI compilation (s06-s13).** Gates networking, platform, sqlite, checkout, tokio threading, and gix features. The commit where `cargo build --target wasm32-wasip2` first succeeds. |
| 3 | [PR #3] | **Gate `git2::Repository` in `but-ctx`.** Moves `git2` to target-specific deps, cfg-gates all construction sites. |
| 4 | [PR #4] | **Gate process spawning.** Makes `command-group`, `posthog-rs`, `machine-uid` optional behind `native`. Stubs telemetry. |
| 5 | [PR #5] | **Gate TUI dependencies.** `ratatui`, `crossterm`, `minus` behind `tui` feature. Moves shared text utilities to `utils/text.rs`. |
| 6 | [PR #6] | **`but-oxidize` no-op under WASI.** Wraps the `git2 <-> gix` bridge in `cfg(not(target_os = "wasi"))` module gate. |
| 7 | [PR #7] | **`but-wasi-host` crate.** Wasmtime sandbox with capability-based preopens, AOT cache, env filtering. |
| 8 | [PR #8] | **Integration tests.** Test harness with `WasiTestFixture`, smoke tests, sandbox isolation tests. |
| 9 | [PR #9] | **CI workflow.** GitHub Actions for WASI builds, integration tests, binary size budget enforcement. |
| 10 | [PR #10] | **`but-serde` objectid gating + housekeeping.** Gates `git2::Oid` serde, cleans up WASI-unused imports, `cargo fmt`. |

## Enabling work: gitoxide WASI support

This whole effort stands on gitoxide. @Byron's work to support WASI in
`gix` (released in 0.81.0) is what makes it possible to have a pure-Rust
git implementation that compiles to `wasm32-wasip2` without patches. The
`but` CLI uses `gix` for all git operations in WASI mode -- `git2` /
libgit2 is gated behind the `native` feature.

## Known limitations

- **No network access.** The WASI sandbox does not grant network
  capabilities. Push, pull, fetch, and forge operations are not
  available. Network can be added later via WASI socket capabilities.

- **No multi-threading.** Tokio runs in `current_thread` mode.

- **Database is stubbed.** `but-db` provides a no-op in WASI mode.
  Commands that depend on persistent state won't behave identically
  to native. A WASI-compatible SQLite could fix this later.

- **Keyring is stubbed.** Secret storage returns "not available."

- **Legacy subsystem not available.** The `legacy` feature depends on
  `git2` and is not compiled in WASI mode. Only the modern `but-*`
  crate stack is included.

## Open questions

1. **`cfg(target_os = "wasi")` vs. feature flags.** Currently we use a
   mix: features for dep-gating in Cargo.toml, `cfg` for
   implementation-level branching. Should we consolidate?

2. **Silent stubs vs. explicit errors.** Some WASI stubs silently
   succeed, others return errors. Should we standardize? Proposal:
   read operations return empty/default, write operations return errors.

3. **Binary size budget.** Current release is ~3 MB. CI enforces 20 MB.
   Right budget?

4. **`but-wasi-host` scope.** Should it evolve beyond a thin Wasmtime
   wrapper -- e.g., WIT interfaces, instance pooling, structured
   communication?

## What this enables

- **Agent sandboxing.** AI coding agents run `but` operations inside
  WASI with hard filesystem boundaries. An agent on repo A literally
  cannot see repo B.

- **Polyverse / multi-workspace.** Multiple WASI instances, each
  preopened to a different repo, running concurrently with full
  isolation.

- **Browser-based git.** With Component Model JS bindings (`jco`),
  the same `but.wasm` could power git operations client-side.

- **Edge / serverless.** 3 MB module with sub-millisecond cold start
  (via AOT cache). Git operations at the edge.

- **Reproducible CI.** Same binary, same behavior, everywhere.

- **Plugin model.** Third-party extensions distributed as `.wasm`
  components with scoped permissions.

## The ask

I'd love feedback on:

1. The overall architecture -- does the feature flag / cfg gate strategy
   make sense?
2. The PR ordering -- is the dependency chain right?
3. The sandbox model -- is `but-wasi-host` the right abstraction?
4. What to prioritize next -- network? WASI-native SQLite? WIT? Browser?
