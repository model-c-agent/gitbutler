# Memory: s03 — Diff Name Only

## Status: patch-ready

## Key Findings
- `Diff` variant is at lines 189-198 in `args/mod.rs`
- Dispatch is at lines 540-574 in `lib.rs`
- `handle` function is at line 56 in `diff/mod.rs`
- Three code paths: worktree (HunkAssignment.path), commit (TreeChange.path), branch (ui::TreeChange.path_bytes)
- Need `JsonNameOnly { files: Vec<String> }` struct in `diff/mod.rs`
- Deduplication needed for worktree path (multiple hunks per file)
