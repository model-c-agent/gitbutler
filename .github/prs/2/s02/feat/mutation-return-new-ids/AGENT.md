# Agent: s02 — Mutation Return New IDs

## Role
Ensure all mutation commands (amend, reword, squash, absorb) return new commit IDs in JSON output.

## Context
- Read [PR.md](../../../PR.md) for workflow rules
- Failure history: F6 in [HISTORY.md](../../../HISTORY.md)
- Tool T8 (but-amend-all) exists because IDs change after mutations

## Key Files
- `crates/but/src/command/legacy/reword.rs` — Has `new_commit_oid` at line 136
- `crates/but/src/command/legacy/rub/amend.rs` — Already has JSON output
- `crates/but/src/command/legacy/rub/squash.rs` — Already has JSON at line 300
- `crates/but/src/command/legacy/absorb.rs` — Needs investigation

## Implementation Notes
- Follow the existing `OutputChannel` pattern for JSON output
- `commit_mapping` should be a JSON object `{"old_id": "new_id", ...}`
- Don't break existing JSON consumers — only add fields, never remove/rename
