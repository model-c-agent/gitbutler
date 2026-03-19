# s01: Fix Silent Null Commit IDs

## Scope
When `but commit` produces no commit (outcome.new_commit is None), return an error instead of printing "unknown" and exiting successfully.

## Dependencies
None (Tier 0)

## Files to Modify
- `crates/but/src/command/legacy/commit.rs` — Lines 470-504: restructure the post-commit output block to error on None commit.

## Bug Analysis

### Root Cause Chain
1. `but_core::tree::create_tree()` returns `destination_tree: None` when all DiffSpecs are rejected or produce no actual diff
2. `but-workspace::commit_engine::create_commit()` sees `destination_tree` is None, sets `new_commit: None` (line 172-204 of `commit_engine/mod.rs`)
3. `but-api::commit::commit_create_only_impl()` propagates this as `CommitCreateResult { new_commit: None, rejected_specs: [...] }` (line 234-247 of `but-api/src/commit.rs`)
4. CLI code in `commit.rs` prints "unknown" in human mode (line 487) or `"commit_id": null` in JSON mode (line 499), both with exit code 0

### Key Invariant
`new_commit` is `None` **if and only if** all diff specs were rejected (see doc on `CreateCommitOutcome::new_commit` in `crates/but-workspace/src/commit_engine/mod.rs:74`). When `new_commit` is `None`, `rejected_specs` is always non-empty. The converse is NOT true — `rejected_specs` can be non-empty while `new_commit` is `Some` (partial rejection).

### Current Broken Code (lines 470-504)
```rust
    if !outcome.rejected_specs.is_empty() {
        tracing::warn!(
            ?outcome.rejected_specs,
            "Failed to commit at least one selected change"
        );
        if let Some(out) = out.for_human() {
            writeln!(
                out,
                "{}",
                "Warning: Some selected changes could not be committed.".yellow()
            )?;
        }
    }

    if let Some(out) = out.for_human() {
        let commit_short = match outcome.new_commit {
            Some(id) => id.to_hex_with_len(7).to_string(),
            None => "unknown".to_string(),  // ← BUG: silent failure
        };
        writeln!(
            out,
            "{} {} {} {}",
            "✓ Created commit".green(),
            commit_short.magenta(),
            "on branch".green(),
            target_branch.name.to_str_lossy().yellow()
        )?;
    } else if let Some(json_out) = out.for_json() {
        let commit_data = serde_json::json!({
            "commit_id": outcome.new_commit.map(|id| id.to_string()),    // ← BUG: null
            "branch": target_branch.name.to_str_lossy(),
            "branch_tip": outcome.new_commit.map(|id| id.to_string()),   // ← BUG: null
        });
        json_out.write_value(commit_data)?;
    }
```

### Problems
1. **Human mode**: Prints `✓ Created commit unknown on branch X` — a lie, no commit was created
2. **JSON mode**: Outputs `{"commit_id": null, "branch": "X", "branch_tip": null}` with exit code 0 — consumers parse this as success
3. **rejected_specs warning is disconnected**: The warning about rejected specs (lines 470-482) runs but doesn't prevent the success message

## Fix

### Strategy
After `commit_create()` returns, check `outcome.new_commit` immediately. If `None`:
- In both modes, this is an error because the user asked to commit and nothing was committed
- The `rejected_specs` warning is still useful context but should be part of the error, not separate

### Replacement Code (lines 470-504)

Replace the entire block from `if !outcome.rejected_specs.is_empty()` through the JSON output block with:

```rust
    match outcome.new_commit {
        Some(id) => {
            // Partial rejection warning: some specs rejected but commit still created
            if !outcome.rejected_specs.is_empty() {
                tracing::warn!(
                    ?outcome.rejected_specs,
                    "Failed to commit at least one selected change"
                );
                if let Some(out) = out.for_human() {
                    writeln!(
                        out,
                        "{}",
                        "Warning: Some selected changes could not be committed.".yellow()
                    )?;
                }
            }

            if let Some(out) = out.for_human() {
                let commit_short = id.to_hex_with_len(7).to_string();
                writeln!(
                    out,
                    "{} {} {} {}",
                    "✓ Created commit".green(),
                    commit_short.magenta(),
                    "on branch".green(),
                    target_branch.name.to_str_lossy().yellow()
                )?;
            } else if let Some(json_out) = out.for_json() {
                let commit_data = serde_json::json!({
                    "commit_id": id.to_string(),
                    "branch": target_branch.name.to_str_lossy(),
                    "branch_tip": id.to_string(),
                });
                json_out.write_value(commit_data)?;
            }
        }
        None => {
            // All specs were rejected — no commit was created
            tracing::error!(
                ?outcome.rejected_specs,
                "Commit produced no result — all changes were rejected"
            );
            if out.for_json().is_some() {
                let error_data = serde_json::json!({
                    "ok": false,
                    "error": "commit_produced_no_result",
                    "rejected_specs": outcome.rejected_specs.len(),
                });
                out.for_json().unwrap().write_value(error_data)?;
                std::process::exit(1);
            }
            bail!(
                "Commit produced no result — all selected changes were rejected.\n\
                 Run 'but status' to check the current state of your changes."
            );
        }
    }
```

### Behavioral Changes
| Scenario | Before | After |
|----------|--------|-------|
| All specs rejected, human mode | Prints "✓ Created commit unknown", exit 0 | `bail!()` with explanation, exit 1 |
| All specs rejected, JSON mode | `{"commit_id": null, ...}`, exit 0 | `{"ok": false, "error": "commit_produced_no_result", ...}`, exit 1 |
| Some specs rejected, commit created | Warning + success message | Warning + success message (unchanged) |
| No specs rejected, commit created | Success message | Success message (unchanged) |

### Edge Cases Addressed
1. **rejected_specs non-empty AND new_commit None**: This IS the only case where new_commit is None. The `None` arm handles it.
2. **rejected_specs empty AND new_commit None**: Should be impossible per invariant, but the `None` arm catches it safely.
3. **JSON mode process::exit(1)**: This is intentional — `bail!()` alone would be caught by the caller and might not produce the right JSON. We emit our own JSON error then exit. This matches the pattern used elsewhere in the codebase for fatal JSON errors.
4. **Post-commit hooks**: The hook code at lines 506-522 only runs when we reach it (after the `match`). In the `None` arm, we either `bail!()` or `exit(1)`, so hooks correctly don't run when there's no commit.

## Acceptance Criteria
- `but commit` with a null outcome returns non-zero exit code
- JSON mode outputs `{"ok": false, "error": "..."}` instead of `{"commit_id": null}`
- Normal commits still work unchanged
- Partially-rejected commits still work (warning + success)
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: S
