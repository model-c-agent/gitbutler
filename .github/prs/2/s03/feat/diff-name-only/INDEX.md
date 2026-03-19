# s03: Add `but diff --name-only`

## Scope
Add a `--name-only` flag to `but diff` that outputs only the file paths of changed files (like `git diff --name-only`).

## Dependencies
None (Tier 0)

## Files to Modify
- `crates/but/src/args/mod.rs` — Add `name_only: bool` to the `Diff` variant
- `crates/but/src/lib.rs` — Pass `name_only` through to the handler
- `crates/but/src/command/legacy/diff/mod.rs` — Accept `name_only`, short-circuit before full diff rendering
- `crates/but/src/command/legacy/diff/show.rs` — Name-only rendering functions

## Acceptance Criteria
- `but diff --name-only` lists changed file paths, one per line
- `but diff --name-only --json` outputs `{"files": ["path1", "path2"]}`
- `but diff` without `--name-only` works unchanged
- `cargo check -p but` and `cargo test -p but` pass

## Complexity: S

---

## Detailed Implementation Plan

### 1. Add `--name-only` flag to `Diff` variant

**File:** `crates/but/src/args/mod.rs`
**Lines:** 189-198 (the `Diff` variant)

Current code at lines 189-198:
```rust
    Diff {
        /// The CLI ID of the entity to show the diff for
        target: Option<String>,
        /// Open an interactive TUI diff viewer
        #[clap(long = "tui", conflicts_with = "no_tui")]
        tui: bool,
        /// Disable the interactive TUI diff viewer (overrides but.ui.tui config)
        #[clap(long = "no-tui", conflicts_with = "tui")]
        no_tui: bool,
    },
```

Change to:
```rust
    Diff {
        /// The CLI ID of the entity to show the diff for
        target: Option<String>,
        /// Only show the names of changed files, not the full diff
        #[clap(long = "name-only", conflicts_with_all = ["tui", "no_tui"])]
        name_only: bool,
        /// Open an interactive TUI diff viewer
        #[clap(long = "tui", conflicts_with = "no_tui")]
        tui: bool,
        /// Disable the interactive TUI diff viewer (overrides but.ui.tui config)
        #[clap(long = "no-tui", conflicts_with = "tui")]
        no_tui: bool,
    },
```

The `name_only` field goes after `target` and before `tui` because it's a display-mode flag. It conflicts with `tui`/`no_tui` since name-only output is non-interactive. Clap will auto-generate `--name-only` from the field name.

### 2. Pass `name_only` through the dispatch in lib.rs

**File:** `crates/but/src/lib.rs`
**Lines:** 540-574 (the `Subcommands::Diff` match arm)

Current destructuring at lines 540-544:
```rust
        Subcommands::Diff {
            target,
            tui,
            no_tui,
        } => {
```

Change to:
```rust
        Subcommands::Diff {
            target,
            name_only,
            tui,
            no_tui,
        } => {
```

Then at line 571, where `handle` is called:
```rust
                command::legacy::diff::handle(&mut ctx, out, target.as_deref())
```

Change to:
```rust
                command::legacy::diff::handle(&mut ctx, out, target.as_deref(), name_only)
```

### 3. Accept `name_only` in the `handle` function

**File:** `crates/but/src/command/legacy/diff/mod.rs`

#### 3a. Update `handle` signature (line 56-60)

Current:
```rust
pub fn handle(
    ctx: &mut Context,
    out: &mut OutputChannel,
    target_str: Option<&str>,
) -> anyhow::Result<()> {
```

Change to:
```rust
pub fn handle(
    ctx: &mut Context,
    out: &mut OutputChannel,
    target_str: Option<&str>,
    name_only: bool,
) -> anyhow::Result<()> {
```

#### 3b. Short-circuit for name-only in each code path

In `handle`, each match arm currently calls `show::worktree(...)`, `show::commit(...)`, `show::branch(...)`, etc. Instead of modifying every call site, we pass `name_only` down to a new set of functions in `show.rs`.

For the **worktree** path (line 72 and 87), change:
- `show::worktree(id_map, out, Some(Filter::Uncommitted(id)))` to `show::worktree(id_map, out, Some(Filter::Uncommitted(id)), name_only)`
- `show::worktree(id_map, out, None)` to `show::worktree(id_map, out, None, name_only)`
- `show::worktree(id_map, out, Some(Filter::Unassigned))` to `show::worktree(id_map, out, Some(Filter::Unassigned), name_only)`
- `show::worktree(id_map, out, Some(Filter::Stack(stack_id)))` to `show::worktree(id_map, out, Some(Filter::Stack(stack_id)), name_only)`

For the **hunk_assignments** path (line 75):
- `show::hunk_assignments(&hunk_assignments, out)` to `show::hunk_assignments(&hunk_assignments, out, name_only)`

For the **commit** path (lines 79, 81):
- `show::commit(ctx, out, commit_id, Some(path))` to `show::commit(ctx, out, commit_id, Some(path), name_only)`
- `show::commit(ctx, out, id, None)` to `show::commit(ctx, out, id, None, name_only)`

For the **branch** path (line 80):
- `show::branch(ctx, out, name)` to `show::branch(ctx, out, name, name_only)`

### 4. Implement name-only rendering in show.rs

**File:** `crates/but/src/command/legacy/diff/show.rs`

#### 4a. Add a JSON struct for name-only output

Near the top of `show.rs` (or in `mod.rs` near the other JSON structs), add:
```rust
#[derive(Debug, Serialize)]
struct JsonNameOnly {
    files: Vec<String>,
}
```

Since the existing JSON structs are in `mod.rs` (lines 93-133), add `JsonNameOnly` there after `JsonHunk` (after line 133).

#### 4b. Update `worktree` function (lines 21-47)

Add `name_only: bool` parameter. The data source is `short_id_assignment_pairs` which contains `(&str, &HunkAssignment)` tuples. Each `HunkAssignment` has a `path: String` field.

Pass `name_only` through to `print_short_id_assignment_pairs`.

#### 4c. Update `print_short_id_assignment_pairs` (lines 60-97)

Add `name_only: bool` parameter. After sorting (line 74), if `name_only` is true, short-circuit:

```rust
if name_only {
    // Deduplicate paths (multiple hunks can belong to the same file)
    let mut seen = std::collections::HashSet::new();
    let paths: Vec<&str> = short_id_assignment_pairs
        .iter()
        .filter_map(|(_, a)| {
            if seen.insert(a.path.as_str()) {
                Some(a.path.as_str())
            } else {
                None
            }
        })
        .collect();

    if let Some(json_out) = out.for_json() {
        let output = JsonNameOnly {
            files: paths.iter().map(|p| p.to_string()).collect(),
        };
        json_out.write_value(output)?;
    } else if let Some(out) = out.for_human_or_shell() {
        for path in &paths {
            writeln!(out, "{path}")?;
        }
    }
    return Ok(());
}
```

This goes right after the sort block (after line 74), before the existing `if short_id_assignment_pairs.is_empty()` check at line 76. Note: even in name-only mode, if there are no pairs we should output an empty list.

Actually, better to handle the empty case too:

```rust
if name_only {
    let mut seen = std::collections::HashSet::new();
    let paths: Vec<&str> = short_id_assignment_pairs
        .iter()
        .filter_map(|(_, a)| {
            if seen.insert(a.path.as_str()) {
                Some(a.path.as_str())
            } else {
                None
            }
        })
        .collect();

    if let Some(json_out) = out.for_json() {
        json_out.write_value(JsonNameOnly {
            files: paths.iter().map(|p| p.to_string()).collect(),
        })?;
    } else if let Some(out) = out.for_human_or_shell() {
        for path in &paths {
            writeln!(out, "{path}")?;
        }
    }
    return Ok(());
}
```

#### 4d. Update `hunk_assignments` function (lines 49-58)

Add `name_only: bool` parameter and pass it to `print_short_id_assignment_pairs`.

#### 4e. Update `commit` function (lines 99-134)

Add `name_only: bool` parameter. The data source is `result.diff_with_first_parent` which is `Vec<but_core::TreeChange>`. Each `TreeChange` has `path: BString`.

After line 106 (where `result` is obtained), short-circuit:

```rust
if name_only {
    let paths: Vec<String> = result
        .diff_with_first_parent
        .iter()
        .filter(|change| path.as_ref().is_none_or(|p| p == &change.path))
        .map(|change| change.path.to_string())
        .collect();

    if let Some(json_out) = out.for_json() {
        json_out.write_value(JsonNameOnly { files: paths })?;
    } else if let Some(out) = out.for_human_or_shell() {
        for p in &paths {
            writeln!(out, "{p}")?;
        }
    }
    return Ok(());
}
```

#### 4f. Update `branch` function (lines 136-168)

Add `name_only: bool` parameter. The data source is `result.changes` which is `Vec<but_core::ui::TreeChange>`. Each `ui::TreeChange` has `path_bytes: BString`.

After line 141 (where `result` is obtained), short-circuit:

```rust
if name_only {
    let paths: Vec<String> = result
        .changes
        .iter()
        .map(|change| change.path_bytes.to_string())
        .collect();

    if let Some(json_out) = out.for_json() {
        json_out.write_value(JsonNameOnly { files: paths })?;
    } else if let Some(out) = out.for_human_or_shell() {
        for p in &paths {
            writeln!(out, "{p}")?;
        }
    }
    return Ok(());
}
```

### 5. Import needed in show.rs

Add `use super::JsonNameOnly;` to the imports in `show.rs` (or define it locally — since the other JSON structs are in `mod.rs`, keep it consistent and put it there, importing it in `show.rs`).

Add `use std::collections::HashSet;` at the top of `show.rs` for the deduplication.

### Summary of Data Structures per Code Path

| Code path | Data structure | Path field | Type |
|-----------|---------------|------------|------|
| worktree / hunk_assignments | `HunkAssignment` | `.path` | `String` |
| commit | `but_core::TreeChange` | `.path` | `BString` |
| branch | `but_core::ui::TreeChange` | `.path_bytes` | `BString` |

### Output Formats

**Human/Shell mode:**
```
path/to/file1.rs
path/to/file2.rs
```

**JSON mode:**
```json
{"files":["path/to/file1.rs","path/to/file2.rs"]}
```
