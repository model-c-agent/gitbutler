# PR #2 Code Review: `but` CLI Improvements

Reviewer perspective: senior Rust developer with experience reviewing servo, rustc, and gitoxide PRs.

---

## Code Quality

### Strengths

- **Output channel pattern is well-applied.** Every new command (rename, delete --pattern, sync pause/resume, plugin list/path) consistently uses the `for_human()`/`for_shell()`/`for_json()` pattern from `OutputChannel`. This is a mature, extensible pattern for multi-format CLI output.
- **Sync pause/resume is well-encapsulated.** The marker file approach in `command/sync.rs` is simple, self-contained, and has auto-expiry. The integration points (`setup.rs` checking the marker before spawning background sync, `refresh.rs` checking as a safety net) are minimal and correct.
- **Plugin discovery** in `alias.rs` follows the cargo/git pattern faithfully -- PATH scanning, `is_executable()` with platform-specific handling, and sorted deduplication.

### Issues

1. **`branch rename` silently succeeds when branch is not found (for non-human output).** At line 207-210 of `command/legacy/branch/mod.rs`, when the branch is not found, the code only prints a message for `out.for_human()`. For JSON and shell output modes, it returns `Ok(())` silently. This should be an error:

   ```rust
   // Current: silent Ok(()) for JSON/shell
   if let Some(out) = out.for_human() {
       writeln!(out, "Branch '{old_name}' not found in any stack")?;
   }
   Ok(())
   ```

   Should be `bail!("Branch '{old_name}' not found in any stack")` to match the `Delete` path which uses `bail!` for its error case.

2. **`branch delete --pattern` has no JSON or shell output.** The entire pattern-based deletion path (lines 222-276) only has `out.for_human()` calls. There is no `for_json()` or `for_shell()` output for: match listing, deletion results, or "no matches" state. Compare with `branch new` which has all three output modes. This is an API contract gap -- callers using `--json` get empty output.

3. **`branch delete` (single branch) also only prints "not found" for human output.** Same issue as rename -- lines 289-292 silently return `Ok(())` for JSON/shell when branch not found.

4. **`check_branches_merge_cleanly` has near-identical code duplication.** Lines 420-452 (applied stacks loop) and 454-485 (unapplied branches loop) differ only in how they obtain `branch_commit` and `branch_name`. This should be extracted into a helper function like `check_single_branch_merge(repo, target_commit, branch_oid) -> Option<bool>`.

5. **`list::list()` has 11 positional parameters.** While `#[allow(clippy::too_many_arguments)]` suppresses the lint, the function signature makes it easy to swap `bool` arguments. The `None => handle(...)` default dispatch in `mod.rs` lines 24-38 already demonstrates how fragile this is -- manually constructing all 9 struct fields. A `ListOptions` struct would be more maintainable.

6. **`format_date_for_display` uses `unwrap()` on `SystemTime::now().duration_since(UNIX_EPOCH)`.** Line 548 of `list.rs`. While this should never fail in practice (system clock before epoch), it breaks the error handling contract. Use `unwrap_or_default()` or propagate the error.

7. **`sync::parse_duration` treats bare trailing numbers as minutes.** Line 79 in `command/sync.rs` -- `"30"` is treated as 30 minutes, but `"30s"` is 30 seconds. This implicit default is undocumented in the help text and could surprise users. The `--duration` default is "1h", but a user typing `--duration 30` might expect 30 seconds.

### Minor

- `print_summary` in `status/mod.rs` at line 466 uses `staged_count` which counts file assignments for the **first segment only**, not per-segment. If a stack has multiple segments with different staged file counts, this is misleading.
- `Subcommands::Sync(..) => Unknown` in metrics (line 215) means sync commands are not tracked. Should have `SyncPause`, `SyncResume`, `SyncStatus` variants.

---

## API Consistency

### Good

- `but sync pause/resume/status` follows the same nested-subcommand pattern as `but plugin list/path` and `but branch new/delete/list/show/rename`.
- `but branch list --pattern` and `but branch delete --pattern` both use the same `-p` short flag and regex semantics.
- The `--force` flag on `delete` follows the existing confirmation pattern using `prepare_for_terminal_input()` and `Confirm`.

### Inconsistencies

1. **`branch delete --pattern` does not support `--json` output**, while `branch list --pattern` does. This breaks the expectation that all commands have full format support.

2. **`branch rename` "not found" is not an error exit code.** Returning `Ok(())` means `but branch rename nonexistent newname` exits with code 0. This is inconsistent with `but branch delete` which uses `bail!` for invalid inputs (line 294: "Either a branch name or --pattern is required").

3. **`sync` subcommand is required** (`pub cmd: Subcommands` -- non-optional), while `plugin` and `branch` both use `Option<Subcommands>` with sensible defaults. Running `but sync` with no subcommand gives a clap error instead of, say, showing sync status. This is a minor UX inconsistency.

4. **`branch delete --pattern` and `branch_name` are not properly exclusive.** The clap definition allows both `branch_name` and `--pattern` to be provided simultaneously. The code at line 222 prioritizes `pattern` over `branch_name` silently. Should use a clap group or explicit conflict.

5. **`summary` flag on status uses `-s`** which could conflict with future flags. More importantly, `--summary` bypasses all the expensive operations (upstream check, review map, CI checks) by returning early, but still runs `process_rules()` and `head_info()`. The flag should be checked earlier to avoid unnecessary work.

---

## Test Coverage

### What Exists

- `alias.rs` has 9 unit tests covering alias expansion, default aliases, external plugin discovery, and known subcommand detection. These are solid.
- `args/tests.rs` has 1 clap validation test and 12 push/gerrit flag tests. Pre-existing, not new.
- Integration tests exist for `branch/new`, `branch/apply`, `branch/unapply`, `status`, `commit`, `diff`, and others via `Sandbox` snapshot testing.

### What Is Missing (Critical)

1. **No tests for `branch rename`.** Zero unit or integration tests. Should test: successful rename, rename of nonexistent branch, rename to a name that already exists.

2. **No tests for `branch delete --pattern`.** Zero tests. Should test: regex matching, no matches, partial failure during bulk delete, confirmation flow (with `--force` and without).

3. **No tests for `branch list --pattern`.** Zero tests. Should test: regex filter, invalid regex error, combined with `--local`/`--remote`.

4. **No tests for `status --summary`.** Zero tests. The `print_summary` function at line 446 of `status/mod.rs` is untested.

5. **No tests for `sync pause/resume/status`.** Zero tests. The `parse_duration` function is a pure function that is trivially testable -- it should have tests for: "30m", "2h", "1h30m", bare number "30", zero duration, invalid format, empty string. The marker file logic is also testable with a temp directory.

6. **No tests for `plugin list/path`.** Zero tests beyond the alias module's tests for `find_external_subcommand` and `list_external_subcommands`. The command-level output formatting is untested.

7. **No tests for JSON output of any new command.** JSON output correctness is critical for programmatic consumers. The `json.rs` structs (`BranchRenameOutput`, `BranchNewOutput`) are never tested for serialization shape.

### The "194 tests passing" claim

The 194 tests existed before PR #2. No new tests were added for any of the new features. This is the single biggest gap in this PR.

---

## Style & Clippy

### `#[allow(clippy::too_many_arguments)]` vs `#[expect(...)]`

The codebase is inconsistent:
- `list.rs` uses `#[allow(clippy::too_many_arguments)]` (lines 10, 264)
- `status/mod.rs` uses `#[expect(clippy::too_many_arguments)]` (lines 74, 603, 826)

`#[expect]` is the preferred form in modern Rust because it warns if the lint no longer triggers (the code was refactored). The PR should have used `#[expect]` consistently, not `#[allow]`.

### The deeper design issue

Three functions use `too_many_arguments` suppressions in `list.rs` alone (`list`, `output_json`), plus three in `status/mod.rs` (`worktree`, `print_group`, `print_commit`). This is a sign that these functions need options structs. For example:

```rust
struct ListOptions {
    local: bool,
    remote: bool,
    all: bool,
    ahead: bool,
    review: bool,
    filter: Option<String>,
    pattern: Option<String>,
    check_merge: bool,
    show_empty: bool,
}
```

This would eliminate both the lint suppression and the fragile `None => handle(Some(Subcommands::List { ... }))` default construction.

### Other style notes

- `serde(rename_all = "camelCase")` is applied consistently to JSON output structs.
- Module organization is clean: `args/`, `command/`, `json.rs` separation.
- Doc comments on `sync.rs` functions are good but missing on `list.rs` and `mod.rs` public functions.

---

## Completeness

### Plan vs. Implementation

| Sub-PR | Status | Notes |
|--------|--------|-------|
| s00: Plugin system | Done | Well-implemented, good alias.rs tests |
| s01: Null commit fix | Done | Implemented in prior commits |
| s02: Mutation return IDs | Done | Implemented in prior commits |
| s03: Diff --name-only | Done | Implemented in prior commits |
| s04: Stage override-lock | Done | Implemented |
| s05: Batch stage | Done | Implemented |
| s06: Batch apply/unapply | Done | Implemented |
| s07: Branch rename | Done | **Missing tests, error handling gaps** |
| s08: Status improvements | Done | **Summary works but missing tests** |
| s09: Branch list --pattern | Done | **Missing tests, JSON works** |
| s10: Branch delete --pattern | Done | **Missing tests, no JSON/shell output** |
| s11: Sync pause/resume | Done | **Missing tests for parse_duration** |

All 12 sub-PRs have code written. The shortcuts taken are:
1. Zero new tests across all sub-PRs
2. JSON/shell output incomplete for delete --pattern
3. Error handling inconsistency (rename returns Ok on failure)

### Acceptance criteria check

> All 12 sub-PRs pass `cargo check -p but`, `cargo clippy -p but -- -D warnings`, `cargo test -p but`

This is technically met because no new tests were added, so the existing 194 tests pass. But the acceptance criteria implicitly assume tests would be written for new features.

---

## Recommendations for Agent Instructions

Based on this review, the following should be added to agent instructions for future PRs:

1. **Require tests for every new command.** Agent instructions should state: "Every new CLI command or flag must have at least one integration test using the `Sandbox` test harness, and every pure function must have unit tests. Do not mark a sub-PR as done without tests."

2. **Require all three output modes.** Agent instructions should state: "Every command must produce output for all three modes: `for_human()`, `for_shell()`, and `for_json()`. If a code path only handles one mode, it is incomplete."

3. **Error paths must be errors, not silent Ok.** Agent instructions should state: "When a command fails to find the requested resource (branch not found, pattern has no matches, etc.), return an error (use `bail!`), do not return `Ok(())`. Exit code 0 means success."

4. **Use `#[expect]` not `#[allow]` for clippy suppressions.** Agent instructions should state: "Always use `#[expect(clippy::...)]` instead of `#[allow(clippy::...)]`. When a function needs `too_many_arguments`, add a TODO comment for refactoring to an options struct."

5. **Test pure functions immediately.** `parse_duration`, `format_duration`, `format_date_for_display` are all pure functions that can be unit-tested without any repository setup. Agent instructions should state: "Pure utility functions must have unit tests in the same file, covering edge cases (empty input, zero, overflow, invalid format)."

6. **Avoid code duplication in the same function.** `check_branches_merge_cleanly` has a duplicated 30-line block. Agent instructions should state: "If you find yourself copy-pasting a block of code within the same function, extract it into a helper."

7. **Validate clap argument exclusivity.** `branch delete` allows both `branch_name` and `--pattern` without a clap group. Agent instructions should state: "When adding mutually exclusive arguments, use clap's `group` or `conflicts_with` attributes. Do not rely on runtime priority logic alone."
