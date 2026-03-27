# s04: Gate TUI/Terminal Dependencies Behind tui Feature for WASI

| Field     | Value                                              |
|-----------|----------------------------------------------------|
| **ID**    | s04                                                |
| **Branch**| `pr1/s01.s04/feat/wasi-gate-tui`                   |
| **Anchor**| `pr1/s01/feat/wasi-feature-flags`                  |
| **Deps**  | s01                                                |
| **Size**  | M                                                  |
| **Commit**| `feat: gate TUI/terminal dependencies behind tui feature for WASI` |

## Scope

- Add `tui` feature to `crates/but/Cargo.toml` (default, disabled by `wasi`)
- Gate `ratatui`, `crossterm`, `colored`, `minus`, `terminal_size` behind `tui` feature
- Wrap `crates/but/src/tui/` module with `#[cfg(feature = "tui")]`
- Gate pager in `crates/but/src/utils/pager.rs`
- Gate TUI deps in `crates/but-link/Cargo.toml`

## Files

- `crates/but/Cargo.toml`
- `crates/but/src/tui/mod.rs`
- `crates/but/src/utils/pager.rs`
- `crates/but-link/Cargo.toml`

## Acceptance Criteria

- WASI build does not pull in `ratatui`, `crossterm`, or other TUI dependencies
- JSON output works without TUI
- Native build with TUI is unaffected
