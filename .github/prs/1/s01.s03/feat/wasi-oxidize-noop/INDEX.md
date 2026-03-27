# s03: Make but-oxidize a No-op Under WASI Feature

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s03                                                |
| **Branch**| `pr1/s01.s03/feat/wasi-oxidize-noop`               |
| **Anchor**| `feat/wasi`                                        |
| **Deps**  | s01                                                |
| **Size**  | S                                                  |
| **Commit**| `feat: make but-oxidize a no-op under wasi feature` |

## Scope

- Make the entire `but-oxidize` crate a no-op when the `wasi` feature is active
- `but-oxidize` is a bridge crate (`git2` <-> `gix`) marked "soon obsolete"
- Gate all public items behind `#[cfg(not(feature = "wasi"))]`

## Files

- `crates/but-oxidize/Cargo.toml` — add `wasi = []` feature
- `crates/but-oxidize/src/lib.rs` — wrap all bridge code in `#[cfg(not(feature = "wasi"))]` module

## Plan

1. Add `[features]` section with `wasi = []` to `Cargo.toml`
2. Move all items in `lib.rs` into a private `mod bridge` gated by `#[cfg(not(feature = "wasi"))]`
3. Re-export with `pub use bridge::*` (also gated)
4. Keep `git2` as a regular dependency (unused under `wasi` but harmless on native)

## Downstream Impact

All downstream usage of `but-oxidize` was analyzed:
- `but` CLI: all usage under `command/legacy/` (gated by `legacy` feature, excluded under `wasi`)
- `but-core`: uses `ObjectIdExt` in `worktree/checkout/function.rs` — handled by s10
- `but-tools`: behind `dep:but-tools` (legacy optional dep)
- `but-action`, `but-workspace`, `but-worktrees`, `but-oplog`, `but-api`, `gitbutler-*`: all legacy

No downstream changes needed in this sub-PR.

## Acceptance Criteria

- `cargo check -p but-oxidize --features wasi` compiles (produces an empty crate)
- Native build and tests are unaffected
