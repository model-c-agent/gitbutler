# PR #1 Retrospective: `but` CLI + Agent Workflow

Retrospective for [PR #1: Compile `but` CLI to WASI](1/INDEX.md).
Incident log: [HISTORY.md](HISTORY.md).

## What Went Well

- **Feature-flag architecture worked.** The `wasi`/`native`/`tui` feature strategy allowed incremental gating across 16 sub-PRs without breaking the native build. Every completed sub-PR verified both `--features wasi` and default features passed `cargo check`.

- **Dependency-encoded naming.** The `s01.s02`-style folder/branch naming made the dependency graph visible from the directory listing alone. The coordinator could determine build order without consulting a separate document.

- **Patch-based workflow prevented filesystem contention.** Moving to `INDEX.patch` + `COMMIT.msg` artifacts (rather than agents modifying source files directly) allowed parallel planning and implementation phases. The `but` agent as sole committer eliminated the hunk lock races that caused F2.

- **SKILLS.md convention scaled.** The rule "when a `but` workflow is missing, write a reusable tool in `scripts/bin/`" produced 10 tools (T1-T10) that were reused across sub-PRs. Thin wrappers over `but_functions.sh` kept tools consistent and composable.

- **Sub-PR absorption strategy.** When the iterative compile-fix process for s13 naturally implemented the work planned for s06-s12, the coordinator recognized and absorbed those sub-PRs rather than forcing artificial boundaries. Pragmatic over process.

## What Caused Friction

### 1. Silent failures from `but commit` (F1, F3)

`but commit` returned null commit IDs without any error message or non-zero exit code. Claude retried 5+ times per session before giving up. This happened when committing to stacked branches with 0-commit parents (F1), and also in other undiagnosed workspace states (F3).

**Impact:** 4+ sessions wasted. Cascading branch corruption. 2 sub-PRs recorded "not_achieved" outcomes.

**What would have helped:** Non-zero exit codes and stderr messages when a commit produces a null ID. The CLI should never return a null commit ID as a success response.

### 2. Hunk lock prevents cross-branch reassignment (F2)

Files created under the same parent commit are locked together by the hunk lock mechanism. When multiple agents staged to branches in the same stack, the hunk lock assigned all changes to whichever branch "owns" the parent commit. Attempts to reassign hunks via `but rub`, `but stage`, or `but commit` produced empty commits.

**Impact:** s05 branch corrupted with two empty commits. Files exist on disk but cannot be moved to the correct branch through `but`.

**What would have helped:** A `but rub --force` or `but stage --override-lock` that explicitly overrides hunk locking when the user knows which branch a file belongs to.

### 3. Commit ID instability after mutations (F6)

`but amend` changes the commit ID of every commit above the amended one in the stack. Any automation that reads a commit ID and later uses it must re-read after every mutation. This forced the creation of `but-amend-all` (T8), which re-reads commit IDs in a loop with a 50-iteration safety cap and infinite-loop detection for cross-stack hunk locks.

**Impact:** Every amend workflow must be written defensively. Raw `but amend` in a loop will eventually hit one of these failure modes.

**What would have helped:** Either stable commit IDs across mutations (content-addressed), or `but amend` returning the new commit ID in its `--json` output so callers don't need a separate `but status` round-trip.

### 4. Background sync reverts in-progress edits (F4)

GitButler's background sync detected working tree changes and reverted a TOML file edit while Claude was still working. The agent had no way to detect this had happened until it re-read the file and found its changes missing.

**Impact:** Lost work. Undermined trust in the file system state during agent operations.

**What would have helped:** A way to pause background sync during batch operations (`but sync pause`/`but sync resume`), or at minimum, a warning when background sync reverts uncommitted changes.

### 5. No batch operations (T1, T2, T3, T10)

Nearly every `but` operation works on a single item. Staging 10 hunks requires 10 `but stage` calls. Applying 5 branches requires 5 `but apply` calls. Creating a branch topology requires N sequential `but branch new` calls in dependency order.

**Impact:** 4 out of 10 bespoke tools exist solely to loop over single-item `but` commands.

### 6. No human-readable output mode (T4, T5, T6, T7, T9)

`but status --json` is the only practical way to query workspace state. There is no formatted default output showing a human-readable summary. 5 out of 10 bespoke tools exist solely to pipe `but status --json` through jq and format the output.

**Impact:** Every new agent or human user must either write jq pipelines or use a wrapper tool before they can understand workspace state.

### 7. Branch rename not supported (F5)

Branches with incorrect names must be deleted and recreated. There is no `but branch rename` command.

**Impact:** Minor, but adds friction when branch naming conventions evolve during a project.

## Patterns Observed

### The "jq pipeline" anti-pattern
Every tool that queries workspace state (T4, T5, T6, T9) is a thin wrapper around `but status --json | jq ...`. The user rule "never pipe `but` output through `jq` inline" forced these into reusable tools — better than ad-hoc jq — but the root cause is that `but` lacks formatted output modes.

### Defensive iteration required for all mutations
Because commit IDs change after every mutation, any tool that performs multiple mutations must re-read state between each one. This is the pattern in `but-amend-all` (T8), `but-stage-all` (T3), and `but-setup-branches` (T10). The cost is O(N) round-trips to `but status` for N mutations.

### The `git` escape hatch
Two tools violate the "never use `git` directly" rule:
- `but-diff-files` (T7) uses `git diff HEAD` because `but` has no equivalent
- `but-setup-branches` (T10) uses `git branch -D` for cleanup because `but` has no branch deletion by pattern

These signal missing `but` capabilities that force fallback to `git`.

### Hunk locks + stacking = data loss risk
F2 (hunk lock corruption) and F6 (ID instability) both occur specifically when operating on stacked branches. The combination of hunk lock assignment and commit ID mutation makes stacked branch workflows fragile for automation. The patch-based workflow (agents produce patches, `but` agent applies them one at a time) was designed specifically to avoid these failure modes.

## Missing `but` Capabilities

Derived from the 10 bespoke tools created to fill gaps:

| Gap | Priority | Evidence |
|-----|----------|----------|
| Error messages for null commit IDs (not silent success) | Critical | F1, F3 |
| Stable commit IDs across mutations (or return new IDs in `--json` output) | High | T8, F6 |
| Batch operations (apply/unapply/stage multiple items) | High | T1, T2, T3 |
| Human-readable output for `but status` (non-JSON default) | High | T4, T5, T6, T9 |
| Bulk amend with hunk lock loop detection | High | T8 |
| Hunk lock override (`--force` or `--override-lock` flag) | Medium | F2 |
| Background sync pause/resume | Medium | F4 |
| Topology-aware batch branch creation | Medium | T10 |
| Working tree diff listing (`but diff --name-only`) | Medium | T7 |
| Branch deletion by pattern | Low | T10 |
| Branch rename | Low | F5 |

## Recommendations for `but` CLI

### Critical (blocks automation)

1. **Never return null commit IDs as success.** If `but commit` cannot produce a valid commit, it must return a non-zero exit code and a descriptive error on stderr. This is the single highest-impact fix for agent workflows. (F1, F3)

2. **Return new commit IDs from mutation commands.** After `but amend`, `but reword`, `but absorb`, print the new commit ID(s) in the `--json` output so callers can update references without a separate `but status` round-trip. (F6, T8)

3. **Add a hunk lock override flag.** `but stage --override-lock <id> <branch>` should allow explicit reassignment when the user knows the hunk lock is wrong. (F2)

### High (significant friction reduction)

4. **Add batch variants of single-item commands.** Accept multiple arguments: `but stage <id1>,<id2> <branch>`, `but apply <branch1> <branch2>`, `but unapply --pattern <regex>`. (T1, T2, T3)

5. **Add a human-readable `but status` default.** Non-JSON output showing unassigned changes, branch names with commit counts, and conflict markers. `--json` becomes opt-in for scripting; default is human-readable. (T4, T5, T6, T9)

6. **Add `but diff --name-only`.** Show working tree changes without requiring `git diff`. (T7)

### Medium (quality of life)

7. **Add `but branch rename <old> <new>`.** (F5)

8. **Add `but sync pause` / `but sync resume`.** Allow agent/batch workflows to temporarily disable background sync. (F4)

9. **Add `but branch list --pattern <regex> --format table`.** Structured branch querying without requiring jq. (T5)

## Recommendations for Agent Workflow

1. **Avoid stacked branches for initial commits.** F1 showed that 0-commit parents in a stack produce null object errors. Create standalone branches anchored on `feat/wasi` until the parent branch has at least one commit.

2. **One agent per stack at a time.** F2 showed that concurrent agents staging to branches in the same stack causes hunk lock corruption. Enforce that at most one agent operates on any given stack at a time — or use the patch-based workflow where agents produce `INDEX.patch` instead of modifying source files.

3. **Always re-read commit IDs after mutations.** F6 showed that amend/reword/absorb operations invalidate all commit IDs in the stack. Agent instructions should mandate re-reading `but status --json` after every mutation.

4. **Detect null commit IDs defensively.** Until F1/F3 are fixed in `but`, agents should parse `but commit --json` output and check commit IDs for null/zero values. Abort with a clear error rather than continuing with corrupted state.

5. **Use `but-amend-all` instead of raw `but amend`.** The tool handles ID re-reads and infinite loop detection. Raw `but amend` in a loop will eventually hit failure modes from F6.

6. **Gate all `but` operations through wrapper tools.** The SKILLS.md convention of putting reusable wrappers in `scripts/bin/` proved its value — 10 tools emerged from PR #1 alone. Future projects should start with the full tool suite from day one rather than building it reactively.
