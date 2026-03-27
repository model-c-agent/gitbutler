# Memory: wasi-oxidize-noop (s03)

## Status: in-progress

## Errors & Fixes
### 2026-03-13 — OpenSSL vendor build broken in environment
**Error:** `cargo check -p but-oxidize` failed because the vendored OpenSSL build (`openssl-sys`) fails with `make` errors (assembler issues).
**Fix:** Used `OPENSSL_NO_VENDOR=1` to link against the system OpenSSL (`libssl-dev 3.0.2`).
**Why:** Environment issue only — the system has OpenSSL dev packages installed; the vendored build is broken (likely a toolchain/assembler mismatch). Not related to our changes.

## Decisions
### 2026-03-13 — Used inner module pattern for cfg gating
**Context:** Need to gate the entire `lib.rs` content behind `#[cfg(not(feature = "wasi"))]`.
**Decision:** Wrapped all items in a private `mod bridge` gated by `#[cfg(not(feature = "wasi"))]` and re-exported with `pub use bridge::*`. This keeps the public API identical for non-wasi builds.
**Alternatives considered:** (1) Individual `#[cfg]` on each item — tedious and error-prone. (2) Move to separate file — unnecessary file creation. (3) Use `cfg_if!` macro — adds a dependency.

### 2026-03-13 — Made git2 optional (review feedback fix)
**Context:** Review feedback: even though bridge code was gated with `#[cfg(not(feature = "wasi"))]`, the `git2` dependency in Cargo.toml was unconditional, so `cargo check -p but-oxidize --features wasi` still pulled in and tried to compile `git2` (and OpenSSL).
**Decision:** Made `git2` optional with `default = ["dep:git2"]`. The `wasi` feature does not include git2. Consumers using `--no-default-features --features wasi` now skip git2 entirely.
**Previous decision (reversed):** Had kept git2 non-optional, assuming top-level exclusion was sufficient. That was wrong -- the crate itself must not pull git2 when built for wasi.

## Blockers
- None
