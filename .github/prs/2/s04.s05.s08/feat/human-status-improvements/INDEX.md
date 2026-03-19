# s08: Human-Readable Status Improvements

## Scope
Add summary header and file counts to `but status` human output. Add a `summary` object to JSON output.

## Dependencies
s04 (feat/stage-override-lock), s05 (feat/batch-stage)

## Files to Modify
- `crates/but/src/command/legacy/status/mod.rs` — Add summary header, file counts per branch
- `crates/but/src/command/legacy/status/json.rs` — Add `summary` object to JSON root

## Acceptance Criteria
- `but status` shows a summary line (e.g. "3 branches, 12 files staged, 5 unassigned")
- Each branch section shows file count
- `but status --json` includes `"summary": {...}` in output
- No existing JSON fields removed or renamed
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: L
