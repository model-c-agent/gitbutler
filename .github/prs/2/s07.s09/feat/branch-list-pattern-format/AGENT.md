# Agent: s09 — Branch List Pattern Format

## Role
Add pattern filtering and table format to `but branch list`.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Tool T5 (but-branch-ids) is a workaround for missing formatted branch listing

## Key Files
- `crates/but/src/args/branch.rs` — Branch subcommand args
- `crates/but/src/command/legacy/branch/list.rs` — Branch list implementation
