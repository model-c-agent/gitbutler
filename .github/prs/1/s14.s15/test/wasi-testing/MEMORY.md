# Memory: wasi-testing (s15)

## Status: plan-complete

## Errors & Fixes

## Decisions

- Tests live in `crates/but-wasi-host/tests/integration/` (not `crates/but/tests/wasi/`)
- `.wasm` path resolved via `WASI_WASM_PATH` env var with fallback to cargo output dir
- Tests skip gracefully when `.wasm` is absent (guard at top of each test, not `#[ignore]`)
- Comparison tests normalise hashes/timestamps before asserting JSON parity
- Sandbox network test relies on s14's `but-wasi-host` having `wasi-http` disabled by default
- Use `once_cell::sync::Lazy<Engine>` to compile the WASM module once per test binary run
- Reuse existing `crates/but/tests/fixtures/scenario/*.sh` scripts via `gix_testtools`; add two new minimal scripts in `crates/but-wasi-host/tests/fixtures/scenario/`

## Blockers

- None at plan stage. See open questions in INDEX.md for implementation-time risks.
