# Agent: s01 — Fix Null Commit IDs

## Role
Fix the silent null commit ID bug in `but commit`.

## Context
- Read [PR.md](../../../PR.md) for workflow rules
- Read [INDEX.md](../../INDEX.md) for PR #2 overview
- Failure history: F1, F3 in [HISTORY.md](../../../HISTORY.md)

## Key Files
- `crates/but/src/command/legacy/commit.rs` — The bug is at lines 485-487

## Implementation Notes
- The fix is localized to one file
- Must handle both human and JSON output modes
- Check if `rejected_specs` is non-empty before erroring (rejected specs have their own error path)
