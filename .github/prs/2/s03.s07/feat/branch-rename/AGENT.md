# Agent: s07 — Branch Rename

## Role
Add `but branch rename` subcommand.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Failure F5 in [HISTORY.md](../../../../HISTORY.md) — branches with wrong names must be deleted/recreated

## Key Files
- `crates/but/src/args/branch.rs` — Branch subcommand args
- `crates/but/src/command/legacy/branch/mod.rs` — Branch command implementations
