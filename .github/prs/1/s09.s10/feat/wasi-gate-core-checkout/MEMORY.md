# Memory: wasi-gate-core-checkout (s10)

## Status: plan-complete

## Errors & Fixes

## Decisions

- Gate entire `checkout` module at `worktree/mod.rs` level via `#[cfg(not(feature = "wasi"))]`
- Provide WASI stubs for `safe_checkout` and `safe_checkout_from_head` that return errors
- Make `git2` and `but-oxidize` optional deps in but-core, gated by a `native` feature
- Extract type definitions from `checkout/mod.rs` to avoid duplication in the WASI stub path
- Forward `but-core/wasi` from the `wasi` feature in the `but` crate's Cargo.toml
- No changes needed in `but-rebase` or any caller — the stub signatures are identical

## Blockers
