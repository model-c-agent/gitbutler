# Memory: s02 — Mutation Return New IDs

## Status: patch-ready

## Patch Summary
- 8 files modified across 4 crates
- CommitMap.mappings() accessor added in but-hunk-assignment
- absorb_impl and auto_commit_simple return (usize, CommitMap) tuples
- All 4 mutation commands (reword, amend, squash, absorb) now emit commit IDs in JSON output
- auto_commit (GUI path) destructures but discards CommitMap per Q2
- absorb public API keeps returning usize per Q1
- squash uses source_commit_ids + target_commit_id instead of commit_mapping per Q3
