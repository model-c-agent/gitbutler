# s02: Return New Commit IDs from Mutation Commands

## Scope
Ensure amend, reword, squash, and absorb all return the new commit ID(s) in their JSON output. Add commit_mapping (old->new) where available.

## Dependencies
None (Tier 0)

## Files to Modify
- `crates/but/src/command/legacy/reword.rs` — Has `new_commit_oid` but no JSON output block. Add one.
- `crates/but/src/command/legacy/rub/amend.rs` — Already outputs `new_commit_id`. Add `old_commit_id` and `commit_mapping`.
- `crates/but/src/command/legacy/rub/squash.rs` — Already outputs `new_commit_id`. Add `old_commit_ids` and `commit_mapping`.
- `crates/but/src/command/legacy/absorb.rs` — Returns `rejected` count but no new commit IDs. Add `commit_mapping`.
- `crates/but-api/src/legacy/absorb.rs` — `absorb_impl` must return `CommitMap` alongside `total_rejected`.

## Acceptance Criteria
- `but reword --json` includes `new_commit_id` and `old_commit_id` in output
- `but amend --json` includes `new_commit_id` (already does) + `old_commit_id` + `commit_mapping`
- `but squash --json` includes `new_commit_id` (already does) + `old_commit_ids` + `commit_mapping`
- `but absorb --json` includes `commit_mapping` (old->new for all affected commits)
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: M

---

## Detailed Analysis

### 1. reword (`crates/but/src/command/legacy/reword.rs`)

**Current state (lines 102-150, `edit_commit_message_by_id`):**
- Line 136-137: Calls `but_api::commit::commit_reword_only(ctx, commit_oid, ...)` which returns `gix::ObjectId` (the new commit ID).
- Lines 139-147: Only outputs for `out.for_human()` — writes "Updated commit message for {old} (now {new})".
- NO `out.for_json()` block exists at all.
- The original `commit_oid` (old ID) is available as the function parameter.

**What's missing:**
- A JSON output block after the human output block.

**Exact changes needed:**
After line 147 (the closing `}` of the `for_human` block), add:
```rust
if let Some(out) = out.for_json() {
    out.write_value(serde_json::json!({
        "ok": true,
        "new_commit_id": new_commit_oid.to_string(),
        "old_commit_id": commit_oid.to_string(),
    }))?;
}
```

**Also:** The `edit_branch_name` function (lines 56-90) has no JSON output either — just a human message. We should add JSON there too for completeness:
After line 84 (the closing `}` of the `for_human` block), add:
```rust
if let Some(out) = out.for_json() {
    out.write_value(serde_json::json!({
        "ok": true,
        "old_name": branch_name,
        "new_name": new_name,
    }))?;
}
```

**And:** The "no changes" early return at line 129-134 also needs JSON:
```rust
if let Some(out) = out.for_json() {
    out.write_value(serde_json::json!({
        "ok": true,
        "no_change": true,
    }))?;
}
```

**Data source:** `but_api::commit::commit_reword_only` at `crates/but-api/src/commit.rs:35-50` returns `gix::ObjectId` directly. No commit_mapping is available from this API (it does a graph rebase internally but only returns the single new ID).

---

### 2. amend (`crates/but/src/command/legacy/rub/amend.rs`)

**Current state:**
- Two functions: `uncommitted_to_commit` (lines 12-45) and `assignments_to_commit` (lines 47-86).
- Both call `amend_diff_specs` (lines 104-123) which calls `but_workspace::legacy::commit_engine::create_commit_and_update_refs_with_project` and returns `CreateCommitOutcome`.
- Both already have JSON output blocks (lines 38-43 and lines 79-84):
  ```json
  {"ok": true, "new_commit_id": "..."}
  ```
- The `CreateCommitOutcome` struct (from `crates/but-workspace/src/commit_engine/mod.rs:69-89`) contains:
  - `new_commit: Option<gix::ObjectId>`
  - `rejected_specs: Vec<(RejectionReason, DiffSpec)>`
  - `rebase_output: Option<RebaseOutput>` which has `commit_mapping: Vec<(RefName, ObjectId, ObjectId)>`

**What's missing:**
- `old_commit_id` (the target commit being amended) is available as the `oid` parameter.
- `commit_mapping` from `outcome.rebase_output` is available but not exposed.

**Exact changes needed in `uncommitted_to_commit` (lines 38-43):**
Replace the JSON block with:
```rust
} else if let Some(out) = out.for_json() {
    let mut json = serde_json::json!({
        "ok": true,
        "new_commit_id": outcome.new_commit.map(|c| c.to_string()),
        "old_commit_id": oid.to_string(),
    });
    if let Some(rebase_output) = &outcome.rebase_output {
        let mapping: serde_json::Map<String, serde_json::Value> = rebase_output
            .commit_mapping
            .iter()
            .map(|(_, old, new)| (old.to_string(), serde_json::Value::String(new.to_string())))
            .collect();
        json["commit_mapping"] = serde_json::Value::Object(mapping);
    }
    out.write_value(json)?;
}
```

Same pattern for `assignments_to_commit` (lines 79-84).

**Note:** The `outcome` variable is currently consumed by the `for_human` block (accessing `outcome.new_commit`). Need to read `rebase_output` before the human block or restructure slightly. Actually, `outcome.new_commit` is `Copy` (it's `Option<gix::ObjectId>`), so `outcome` is not consumed — we can access it in both blocks.

**Data source:** `CreateCommitOutcome.rebase_output.commit_mapping` provides `Vec<(RefName, ObjectId, ObjectId)>` — the old->new mapping for all rebased commits.

---

### 3. squash (`crates/but/src/command/legacy/rub/squash.rs`)

**Current state (lines 297-303, in `squash_commits_internal`):**
- Line 242-247: Calls `gitbutler_branch_actions::squash_commits(ctx, target_stack, source_oids, target_oid)` which returns `git2::Oid` (the new commit ID).
- Lines 297-303: JSON output block exists:
  ```json
  {"ok": true, "new_commit_id": "...", "squashed_count": N}
  ```
- The `source_oids` (old IDs being squashed) and `target_oid` (destination commit) are available.

**What's missing:**
- `old_commit_ids` — the original source commit IDs that were squashed.
- `target_commit_id` — the destination commit ID.
- No `commit_mapping` available from the squash API (it returns only the new commit OID).

**Exact changes needed (lines 297-303):**
Replace the JSON block with:
```rust
} else if let Some(out) = out.for_json() {
    out.write_value(serde_json::json!({
        "ok": true,
        "new_commit_id": final_commit_oid.to_gix().to_string(),
        "squashed_count": source_oids.len(),
        "source_commit_ids": source_oids.iter().map(|oid| oid.to_string()).collect::<Vec<_>>(),
        "target_commit_id": target_oid.to_string(),
    }))?;
}
```

**Data source:** `gitbutler_branch_actions::squash_commits` at `crates/gitbutler-branch-actions/src/squash.rs:26-41` returns `git2::Oid`. The internal `do_squash_commits` does have a `reorder_stack` that produces a `commit_mapping`, but it's not surfaced in the return type. The squash API only returns the single new commit OID.

---

### 4. absorb (`crates/but/src/command/legacy/absorb.rs`)

**Current state (lines 123-170, `absorb_assignments`):**
- Calls either `but_action::auto_commit_simple(...)` or `but_api::legacy::absorb::absorb_impl(...)`.
- Both return `usize` (total_rejected count).
- Lines 156-167: JSON output block exists:
  ```json
  {"ok": true/false, "rejected": N, "plan": {...}}
  ```
- The plan JSON (`JsonAbsorbOutput`) contains `commit_id` fields for each planned absorption (old IDs), but no new IDs are tracked.

**What's missing:**
- `commit_mapping` (old->new) for all commits that were rewritten during absorb.
- The underlying `absorb_impl` in `crates/but-api/src/legacy/absorb.rs:49-82` tracks a `CommitMap` internally (line 57) and populates it via `outcome.commit_mapping` (line 76-78), but discards it — only returns `total_rejected`.
- Similarly, `auto_commit_simple` in `crates/but-action/src/auto_commit.rs:115-136` tracks `CommitMap` but discards it.

**Exact changes needed:**

**Step A: Modify `but_api::legacy::absorb::absorb_impl` (`crates/but-api/src/legacy/absorb.rs:49-82`)**
Change return type from `anyhow::Result<usize>` to `anyhow::Result<(usize, CommitMap)>`:
```rust
pub fn absorb_impl(
    absorption_plan: Vec<CommitAbsorption>,
    guard: &mut RepoExclusiveGuard,
    repo: &gix::Repository,
    data_dir: &Path,
) -> anyhow::Result<(usize, CommitMap)> {
    // ... existing code ...
    Ok((total_rejected, commit_map))
}
```

Also update the `absorb` function (line 33) that calls `absorb_impl` to handle the new return type:
```rust
pub fn absorb(ctx: &mut Context, absorption_plan: Vec<CommitAbsorption>) -> anyhow::Result<usize> {
    // ...
    let (total_rejected, _commit_map) = absorb_impl(absorption_plan, &mut guard, &repo, &data_dir)?;
    Ok(total_rejected)
}
```

**Step B: Modify `but_action::auto_commit_simple` (`crates/but-action/src/auto_commit.rs:115-136`)**
Change return type to `anyhow::Result<(usize, CommitMap)>` and propagate from `apply_commit_changes`.
Also change `apply_commit_changes` (line 139) to return `anyhow::Result<(usize, CommitMap)>`:
```rust
fn apply_commit_changes(...) -> anyhow::Result<(usize, CommitMap)> {
    // ... existing code ...
    Ok((total_rejected, commit_map))
}
```

Also update `auto_commit` (line 63) which calls `apply_commit_changes`.

**Note:** The public API in `crates/but-action/src/lib.rs:66-82` also wraps `auto_commit_simple` — it needs updating too.

**Step C: Modify `absorb_assignments` in `crates/but/src/command/legacy/absorb.rs` (lines 123-170)**
Update to receive the `CommitMap` and include it in JSON output:
```rust
fn absorb_assignments(...) -> anyhow::Result<()> {
    let (total_rejected, commit_map) = if new {
        let (rejected, map) = but_action::auto_commit_simple(...)?;
        (rejected, map)
    } else {
        let (rejected, map) = but_api::legacy::absorb::absorb_impl(...)?;
        (rejected, map)
    };

    // ... existing human output ...

    } else if let Some(out) = out.for_json() {
        let commit_mapping: serde_json::Map<String, serde_json::Value> = commit_map
            .into_iter()
            .map(|(old, new)| (old.to_string(), serde_json::Value::String(new.to_string())))
            .collect();
        let mut combined = serde_json::json!({
            "ok": total_rejected == 0,
            "rejected": total_rejected,
            "commit_mapping": serde_json::Value::Object(commit_mapping),
        });
        if let Some(plan) = plan_json {
            combined["plan"] = serde_json::to_value(plan).unwrap_or(serde_json::Value::Null);
        }
        out.write_value(combined)?;
    }
    Ok(())
}
```

**Issue:** `CommitMap` does not implement `IntoIterator`. It wraps `HashMap<ObjectId, ObjectId>` privately (line 821-823 of `crates/but-hunk-assignment/src/lib.rs`). We need either:
1. Add an `into_iter()` or `iter()` method to `CommitMap`, or
2. Add a `to_json_map()` convenience method, or
3. Add a `mappings(&self) -> &HashMap<ObjectId, ObjectId>` accessor.

Option 3 is simplest. Add to `CommitMap` in `crates/but-hunk-assignment/src/lib.rs`:
```rust
pub fn mappings(&self) -> &HashMap<ObjectId, ObjectId> {
    &self.map
}
```

**Data source:** The `CommitMap` is populated from `outcome.commit_mapping` in `absorb_impl` at `crates/but-api/src/legacy/absorb.rs:76-78`. The `outcome` comes from `amend_commit_and_count_failures` which returns `commit_engine::ui::CreateCommitOutcome` (which has `commit_mapping: Vec<(ObjectId, ObjectId)>`).

---

## JSON Schema for New/Updated Output

### `but reword --json` (NEW)
```json
{
  "ok": true,
  "new_commit_id": "abc123...",
  "old_commit_id": "def456..."
}
```

### `but reword --json` (branch rename, NEW)
```json
{
  "ok": true,
  "old_name": "old-branch",
  "new_name": "new-branch"
}
```

### `but amend --json` (UPDATED, adds old_commit_id and commit_mapping)
```json
{
  "ok": true,
  "new_commit_id": "abc123...",
  "old_commit_id": "def456...",
  "commit_mapping": {
    "old_id_1": "new_id_1",
    "old_id_2": "new_id_2"
  }
}
```

### `but squash --json` (UPDATED, adds source_commit_ids and target_commit_id)
```json
{
  "ok": true,
  "new_commit_id": "abc123...",
  "squashed_count": 2,
  "source_commit_ids": ["src1...", "src2..."],
  "target_commit_id": "dest..."
}
```

### `but absorb --json` (UPDATED, adds commit_mapping)
```json
{
  "ok": true,
  "rejected": 0,
  "commit_mapping": {
    "old_commit_1": "new_commit_1",
    "old_commit_2": "new_commit_2"
  },
  "plan": { ... }
}
```

---

## Summary of All Files to Modify

| File | Change Type | Description |
|------|-------------|-------------|
| `crates/but/src/command/legacy/reword.rs` | Add JSON output | Add `for_json` blocks in `edit_commit_message_by_id` (after L147), `edit_branch_name` (after L84), and no-change path (after L133) |
| `crates/but/src/command/legacy/rub/amend.rs` | Extend JSON output | Add `old_commit_id` and `commit_mapping` to both `uncommitted_to_commit` (L38-43) and `assignments_to_commit` (L79-84) |
| `crates/but/src/command/legacy/rub/squash.rs` | Extend JSON output | Add `source_commit_ids` and `target_commit_id` to JSON block (L297-303) |
| `crates/but/src/command/legacy/absorb.rs` | Extend JSON output | Consume `CommitMap` from absorb APIs and include in JSON (L156-167) |
| `crates/but-api/src/legacy/absorb.rs` | Change return type | `absorb_impl` returns `(usize, CommitMap)` instead of `usize` (L49-82); update `absorb` caller (L33-47) |
| `crates/but-action/src/auto_commit.rs` | Change return type | `auto_commit_simple` and `apply_commit_changes` return `(usize, CommitMap)` (L115-205) |
| `crates/but-action/src/lib.rs` | Change return type | Public `auto_commit_simple` wrapper returns `(usize, CommitMap)` (L66-82) |
| `crates/but-hunk-assignment/src/lib.rs` | Add accessor | Add `mappings()` method to `CommitMap` (near L825) |

## commit_mapping availability summary

| Command | commit_mapping available? | Source |
|---------|---------------------------|--------|
| reword | No | `commit_reword_only` returns only `ObjectId` |
| amend | Yes | `CreateCommitOutcome.rebase_output.commit_mapping` |
| squash | No | `squash_commits` returns only `git2::Oid` |
| absorb | Yes | `CommitMap` built from per-absorption `outcome.commit_mapping` |
