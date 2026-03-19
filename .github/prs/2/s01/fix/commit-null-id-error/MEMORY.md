# Memory: s01 — Fix Null Commit IDs

## Status: patch-ready

## Key Findings
- Bug is in `crates/but/src/command/legacy/commit.rs` lines 470-504
- `new_commit` is None if and only if ALL diff specs were rejected (invariant from commit engine)
- Fix: restructure the post-commit output block into a `match outcome.new_commit { Some/None }`
- No other places in the file silently swallow None commits (this is the only consumer)
- The `rejected_specs` warning (lines 470-482) should move inside the `Some` arm (partial rejection)
- JSON mode needs `process::exit(1)` since `bail!()` won't produce the right JSON output format
