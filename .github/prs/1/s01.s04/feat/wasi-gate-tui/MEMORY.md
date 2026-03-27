# Memory: wasi-gate-tui (s04)

## Status: done

## Errors & Fixes
- OpenSSL vendor build broken in CI environment; used `OPENSSL_NO_VENDOR=1` for verification

## Decisions
- Q1: `colored` left unconditional (pure Rust, compiles for WASI)
- Q2: crossterm parts of `output_channel.rs` gated behind `tui`; `OutputChannel` usable for JSON without `tui`
- Q3: `get_editor_command()` stays in `tui::get_text` but config.rs uses `#[cfg(feature = "tui")]` to conditionally call it
- `utils/text.rs` already moved from `tui/text.rs`; `tui/text.rs` is a re-export shim for backwards compat
- `terminal_width()` returns 80 when `tui` feature is off (no `terminal_size` dep)

## Implementation Summary
All gating was already in place when this agent ran. Verified:
- `crates/but/Cargo.toml`: `tui = ["dep:ratatui", "dep:crossterm", "dep:minus", "dep:terminal_size"]`
- `crates/but/src/lib.rs`: `#[cfg(feature = "tui")] mod tui;` and `Subcommands::Edit` gated
- `crates/but/src/utils/mod.rs`: `#[cfg(feature = "tui")] mod pager;`
- `crates/but/src/utils/output_channel.rs`: pager/crossterm parts gated
- `crates/but/src/utils/text.rs`: `terminal_width()` conditional on `tui`
- `crates/but/src/command/config.rs`: `get_editor_command()` call gated
- `crates/but-link/Cargo.toml`: `tui = ["dep:crossterm", "dep:ratatui"]`
- `crates/but-link/src/lib.rs`: `#[cfg(feature = "tui")] mod tui;`
- `crates/but-link/src/commands.rs`: TUI command dispatch gated

## Verification
All three acceptance criteria passed:
- `cargo check -p but --no-default-features --features wasi` -- OK
- `cargo check -p but` (default features) -- OK
- `cargo check -p but-link --no-default-features` -- OK (warnings only)

## Blockers
