# Agent: s05 — Batch Stage

## Role
Add comma-separated batch staging to `but stage`.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Tool T3 (but-stage-all) is the workaround this replaces

## Key Files
- `crates/but/src/args/mod.rs` — `Stage` variant
- `crates/but/src/command/legacy/rub/mod.rs` — `handle_stage()`
