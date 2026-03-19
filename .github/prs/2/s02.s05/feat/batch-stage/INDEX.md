# s05: Batch Staging

## Scope
Accept comma-separated IDs in `but stage` to stage multiple changes in one command.

## Dependencies
s02 (feat/mutation-return-new-ids)

## Files to Modify
- `crates/but/src/args/mod.rs` — Change `Stage.file_or_hunk` to accept comma-separated values via `value_delimiter = ','`
- `crates/but/src/command/legacy/rub/mod.rs` — Loop in `handle_stage()` over multiple IDs

## Acceptance Criteria
- `but stage id1,id2,id3 <branch>` stages all three
- Single ID still works: `but stage id1 <branch>`
- Reports success/failure count
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M
