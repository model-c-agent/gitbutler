# Agent: s11 — Sync Pause/Resume

## Role
Add `but sync pause`/`resume` subcommands.

## Context
- Read [PR.md](../../../../PR.md) for workflow rules
- Failure F4 in [HISTORY.md](../../../../HISTORY.md) — background sync reverts in-progress edits

## Key Files
- `crates/but/src/args/mod.rs` — Subcommands enum (add Sync variant at end)
- Background sync code — needs investigation to find where sync runs

## Implementation Notes
- Marker file approach is simplest — no IPC needed
- Auto-expiry prevents orphaned pause states
- The `Sync` variant goes at the end of the Subcommands enum to minimize merge conflicts
