# Agent: s04 — Stage Override Lock

## Role
Add `--override-lock` flag to `but stage`.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Failure F2 in [HISTORY.md](../../../../HISTORY.md) — hunk lock prevents cross-branch reassignment

## Key Files
- `crates/but/src/args/mod.rs` — `Stage` variant
- `crates/but/src/command/legacy/rub/mod.rs` — `handle_stage()`
- `crates/but/src/command/legacy/rub/assign.rs` — Assignment logic
- `crates/but-hunk-assignment/` — Core hunk assignment
