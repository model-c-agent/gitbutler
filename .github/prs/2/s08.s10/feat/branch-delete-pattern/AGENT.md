# Agent: s10 — Branch Delete Pattern

## Role
Add pattern-based branch deletion to `but branch delete`.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Tool T10 (but-setup-branches) uses `git branch -D` as workaround for missing pattern deletion

## Key Files
- `crates/but/src/args/branch.rs` — Branch subcommand args
- `crates/but/src/command/legacy/branch/mod.rs` — Branch command implementations
