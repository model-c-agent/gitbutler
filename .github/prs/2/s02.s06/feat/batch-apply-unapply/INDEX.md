# s06: Batch Apply/Unapply

## Scope
Accept multiple branch names in `but apply` and `but unapply`.

## Dependencies
s02 (feat/mutation-return-new-ids)

## Files to Modify
- `crates/but/src/args/mod.rs` — Change `Apply.branch_name` and `Unapply.identifier` to `Vec<String>`
- `crates/but/src/command/legacy/branch/apply.rs` — Loop over branches
- `crates/but/src/command/legacy/unapply.rs` — Loop over identifiers

## Acceptance Criteria
- `but apply branch1 branch2` applies both
- `but unapply branch1 branch2` unapplies both
- Partial failures collected; non-zero exit if any fail
- Single branch still works
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M
