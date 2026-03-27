# Memory: wasi-tokio-singlethread (s11)

## Status: plan-complete

## Errors & Fixes

## Decisions

- No `block_in_place` found anywhere in `crates/` — zero sites to gate (best case)
- `spawn_blocking` sites are all in crates not reachable from WASI path or behind napi/legacy/native gates
- `std::thread::spawn` in `but-llm`, `but-forge`, `but-update` is a WASI problem but out of scope for this PR — future gating PRs
- Use `#[cfg_attr(target_os = "wasi", ...)]` on `#[tokio::main]` rather than a runtime builder
- Use `[target.'cfg(...)'.dependencies]` sections in Cargo.toml for tokio feature split
- `parking_lot` does not require gating — works fine single-threaded

## Blockers

None — plan is complete and implementation path is clear.
