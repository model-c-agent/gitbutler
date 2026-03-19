# s07: Add `but branch rename`

## Scope
Add a `but branch rename <old> <new>` subcommand.

## Dependencies
s03 (feat/diff-name-only)

## Files to Modify
- `crates/but/src/args/branch.rs` — Add `Rename` variant with `old_name` and `new_name` fields
- `crates/but/src/command/legacy/branch/mod.rs` — Implement rename handler

## Acceptance Criteria
- `but branch rename old-name new-name` renames the branch
- Error if old branch doesn't exist
- Error if new name already taken
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: S
