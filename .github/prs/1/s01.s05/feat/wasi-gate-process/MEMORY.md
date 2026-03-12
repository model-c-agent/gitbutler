# Memory: wasi-gate-process (s05)

## Status: not-started

## Errors & Fixes

### 2026-03-12 -- Plan files committed to wrong branch
**Error:** The INDEX.md and MEMORY.md plan files were absorbed into the s04 commit (`2729b64`, "plan: wasi-gate-tui") due to hunk lock assignment. The `but` tool locked these files to the s04 commit because they were created by the same initial infrastructure commit (`ddf6dfc`). Attempts to move them via `but rub` or `but stage`/`but commit` resulted in empty commits on the s05 branch due to the hunk lock preventing content transfer.
**Fix:** The plan files are correct on disk. The s05 branch has two empty commits (`d5bea01`, `eb0277c`). The coordinator should squash the s04 commit to remove s05 files, and the s05 branch commits should be replaced with one that actually contains the plan files. Alternatively, during Phase 2 implementation, the plan commit can be amended with actual code changes which will properly include all file content.
**Why:** Race condition between multiple sub-agents staging to branches in the same stack. The hunk lock mechanism treats files created by the same parent commit as belonging together.

## Decisions

## Blockers

- Plan files are on disk but stuck in wrong branch commit due to hunk locks. Coordinator needs to resolve branch assignment before Phase 2 can begin cleanly.
