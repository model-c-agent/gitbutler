# Agent: s08 — Human Status Improvements

## Role
Improve `but status` with summary headers and file counts.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Tools T4, T5, T6, T9 exist because status output is hard to parse

## Key Files
- `crates/but/src/command/legacy/status/mod.rs` — Status command implementation
- `crates/but/src/command/legacy/status/json.rs` — JSON output
