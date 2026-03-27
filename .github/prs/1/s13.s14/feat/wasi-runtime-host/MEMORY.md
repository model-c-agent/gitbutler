# Memory: wasi-runtime-host (s14)

## Status: plan-complete

## Decisions

- **Component model**: `wasm32-wasip2` produces a WASI component, so `wasmtime::component::*` is required. Core wasm linker is not used.
- **Sync execution**: `wasmtime_wasi::add_to_linker_sync` chosen over async; `but` is a CLI tool and sync is simpler.
- **No `cap-std` direct dep**: `wasmtime-wasi`'s built-in preopened dir support (`WasiCtxBuilder::preopened_dir`) is sufficient for s14. COW isolation deferred to s15.
- **AOT cache**: `.cwasm` sibling file to `.wasm`. Wasmtime automatically detects version mismatches on `deserialize`, triggering recompile. No external cache library needed.
- **Env var policy**: `GITBUTLER_*` prefix + `RUST_LOG` + `HOME` (mapped to config dir). All others blocked. Default-deny.
- **Networking**: Disabled. `--allow-network` flag parsed but currently a warning no-op. wasi-http deferred.
- **Wasmtime version**: 29.0.1 as specified in INDEX scope. No existing wasmtime workspace dep — added as crate-local.
- **`mcagent` reference**: mcagent uses WASI + COW fs + MCP protocol for sandboxed agent execution. The COW fs layer (for snapshot isolation) is the mcagent pattern; s14 does the simpler preopened-dir variant, leaving COW for s15.

## Errors & Fixes

## Blockers
