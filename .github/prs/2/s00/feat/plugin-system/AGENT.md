# Agent: s00 — Plugin System

## Role
Implement Cargo-style external subcommand discovery for `but`.

## Context
- Read [PR.md](../../../PR.md) for workflow rules
- Read [INDEX.md](../../INDEX.md) for PR #2 overview
- This sub-PR's spec: [INDEX.md](INDEX.md)
- Plan details: `/home/willem/.claude/plans/sunny-hatching-codd.md` (s00 section)

## Key Files
- `crates/but/src/alias.rs` — Existing alias system, natural extension point
- `crates/but/src/lib.rs` — Main dispatch: `handle_args()` → alias expansion → clap parse → `match_subcommand()`
- `crates/but/src/args/mod.rs` — `Subcommands` enum, add `Plugin` variant near end
- `crates/but/src/command/alias.rs` — Reference for human/JSON output pattern

## Implementation Notes
- Plugin discovery goes in `alias.rs` (reuses PATH-scanning logic)
- Plugin exec goes in `lib.rs` between alias expansion (line 82) and `Args::parse_from` (line 104)
- On Unix, check executable bit via `std::os::unix::fs::PermissionsExt`
- `but plugin list` follows same pattern as `but alias list` (human table + JSON array)
- The `but plugin` subcommand itself needs `pub mod plugin;` in both `args/mod.rs` and `command/mod.rs`
