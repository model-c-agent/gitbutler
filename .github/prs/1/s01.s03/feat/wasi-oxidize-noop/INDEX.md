# s03: Make but-oxidize a No-op Under WASI Feature

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s03                                                |
| **Branch**| `pr1/s01.s03/feat/wasi-oxidize-noop`               |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | S                                                  |
| **Commit**| `feat: make but-oxidize a no-op under wasi feature` |

## Scope

- Make the entire `but-oxidize` crate a no-op when the `wasi` feature is active
- `but-oxidize` is a bridge crate (`git2` <-> `gix`) marked "soon obsolete"
- Gate all public items behind `#[cfg(not(feature = "wasi"))]`

## Files

- `crates/but-oxidize/Cargo.toml`
- `crates/but-oxidize/src/lib.rs`

## Acceptance Criteria

- `cargo check -p but-oxidize --features wasi` compiles (produces an empty crate)
- Native build and tests are unaffected
