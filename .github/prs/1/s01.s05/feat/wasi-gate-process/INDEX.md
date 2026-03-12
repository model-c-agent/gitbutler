# s05: Gate Process Spawning for WASI Builds

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s05                                                |
| **Branch**| `pr1/s01.s05/feat/wasi-gate-process`               |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: gate process spawning for WASI builds`      |

## Scope

- Gate background sync in `crates/but/src/setup.rs`
- Gate metrics subprocess in `crates/but/src/utils/metrics.rs`
- Gate `command-group` dep behind `native` feature
- Gate editor/`$EDITOR` spawning
- Verify legacy teardown is already gated by the `legacy` feature

## Files

- `crates/but/src/setup.rs`
- `crates/but/src/utils/metrics.rs`
- `crates/but/Cargo.toml`
- `crates/but/src/command/`

## Acceptance Criteria

- [ ] `command-group` is optional in `Cargo.toml`, activated only by `native`
- [ ] No `std::process::Command::new` or `tokio::process::Command::new` usage reachable when compiling with `--no-default-features --features wasi`
- [ ] No `command_group` imports reachable in WASI code paths
- [ ] `gix::command::prepare(...).spawn()` in `tui/get_text.rs` is gated behind `native`
- [ ] `cargo check -p but --no-default-features --features wasi` succeeds
- [ ] `cargo check -p but` succeeds with default features (native build unaffected)
- [ ] `cargo test -p but` passes (existing tests unbroken)
