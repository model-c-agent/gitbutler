# Questions: s02 — Mutation Return New IDs

## Q1: Should absorb return type changes propagate to the `but_api::legacy::absorb::absorb` public API?

The `absorb` function at `crates/but-api/src/legacy/absorb.rs:33` currently returns `anyhow::Result<usize>`. It calls `absorb_impl` internally. If we change `absorb_impl` to return `(usize, CommitMap)`, should `absorb` also surface the `CommitMap`, or just discard it? The `absorb` function has the `#[but_api]` macro and is also called from other contexts (not just the CLI).

**Recommendation:** Keep `absorb` returning `usize` (just discard the map with `let (total_rejected, _) = ...`). Only the CLI path needs the map, and it calls `absorb_impl` directly.

## Q2: Should `auto_commit` (the non-simple variant) also return `CommitMap`?

`auto_commit` at `crates/but-action/src/auto_commit.rs:63` is used by the GUI/daemon path with event emitters. Changing its return type is a larger blast radius. The CLI only calls `auto_commit_simple`.

**Recommendation:** Only change `auto_commit_simple` and `apply_commit_changes`. Leave `auto_commit` returning `usize` (discard the map). This minimizes blast radius.

## Q3: Should we add `commit_mapping` to squash output even though the API doesn't surface it?

The squash API (`gitbutler_branch_actions::squash_commits`) internally does a reorder + rebase that produces commit mappings, but only returns the final new commit OID. Surfacing the full mapping would require changing the squash API return type.

**Recommendation:** Skip commit_mapping for squash in this PR. The `source_commit_ids` and `target_commit_id` fields provide enough info for consumers. The squash operation collapses N commits into 1, so the mapping is implicit: all source IDs and the target ID map to `new_commit_id`.

## Q4: How should `CommitMap.mappings()` be exposed?

Options:
1. `pub fn mappings(&self) -> &HashMap<ObjectId, ObjectId>` — simple accessor
2. `pub fn into_inner(self) -> HashMap<ObjectId, ObjectId>` — consuming accessor
3. Implement `IntoIterator` for `CommitMap`

**Recommendation:** Option 1 (reference accessor) is simplest and non-breaking. We only need read access to serialize to JSON.
