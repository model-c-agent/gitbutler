# s04: Add `--override-lock` to `but stage`

## Scope
Add an `--override-lock` flag to `but stage` that explicitly overrides hunk lock assignment when the user knows the hunk lock is wrong.

## Dependencies
s01 (fix/commit-null-id-error)

## Files to Modify
- `crates/but/src/args/mod.rs` — Add `override_lock: bool` to `Stage` variant
- `crates/but/src/command/legacy/rub/mod.rs` — Pass flag through `handle_stage()`
- `crates/but/src/command/legacy/rub/assign.rs` — Respect flag in assignment logic
- Possibly `crates/but-hunk-assignment/` — Accept override flag in core assignment

## Acceptance Criteria
- `but stage --override-lock <id> <branch>` forces assignment regardless of hunk lock
- Default behavior (without flag) unchanged
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M
