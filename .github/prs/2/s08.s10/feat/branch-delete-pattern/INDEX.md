# s10: Branch Delete by Pattern

## Scope
Add `but branch delete --pattern <regex>` to delete multiple branches matching a regex.

## Dependencies
s08 (feat/human-status-improvements)

## Files to Modify
- `crates/but/src/args/branch.rs` — Add `pattern` option to `Delete` variant
- `crates/but/src/command/legacy/branch/mod.rs` — Enumerate matching branches, confirm, batch delete

## Acceptance Criteria
- `but branch delete --pattern "temp/.*"` deletes all matching branches
- Confirmation prompt before deletion (unless `--force`)
- Reports count of deleted branches
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: S
