# s09: Branch List with Pattern and Format

## Scope
Add `--pattern <regex>` and `--format table` options to `but branch list`.

## Dependencies
s07 (feat/branch-rename)

## Files to Modify
- `crates/but/src/args/branch.rs` — Add `pattern` and `format` args to `List` variant
- `crates/but/src/command/legacy/branch/list.rs` — Regex filter + table output

## Acceptance Criteria
- `but branch list --pattern "feat/.*"` filters branches by regex
- `but branch list --format table` outputs tabular format
- Default behavior unchanged
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M
