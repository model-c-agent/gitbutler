# DevOps & Automation Retro — PR #2 `but` CLI Improvements

Reviewer perspective: DevOps engineer / automation specialist with CI/CD and multi-agent orchestration background.

Session span: 2026-03-14 to 2026-03-19 (5 calendar days, single long-lived session).

---

## Build Pipeline

### Verification order observed

The session settled on a consistent pipeline after some initial experimentation:

1. `cargo check -p but` (fast type-check gate)
2. `cargo clippy -p but -- -D warnings` (lint gate)
3. `cargo test -p but` (full test suite)

This ordering is correct and standard for Rust projects. `check` catches compilation errors in ~10-30 seconds, preventing wasted time in the longer clippy and test stages.

### Wasted builds

**OPENSSL_NO_VENDOR=1 was discovered the hard way.** The first `cargo check -p but` invocation (Tier 0 apply phase) failed with assembler errors from vendored OpenSSL. This was then required as a prefix on every single subsequent cargo command for the rest of the session. The agent correctly adapted and included it from that point forward, but the initial failure cost a full build cycle.

**Agents did not run verification.** The Tier 0 planning and implementation agents produced INDEX.patch files and COMMIT.msg files but never ran `cargo check`, `cargo clippy`, or `cargo test` themselves. All verification was deferred to the coordinator after applying patches. This meant:
- The s00 (plugin system) patch had a clippy-flagged collapsible `if` in `alias.rs` that an agent clippy pass would have caught.
- The s06 (batch apply/unapply) implementation agent reported "cargo check and clippy pass" but never ran `cargo test`. The coordinator caught 5 snapshot test failures post-apply. The agent's incorrect claim of passing checks wasted a verification cycle.

**Snapshot test blindness.** When snapshot tests failed for s06, the initial response was to blindly update snapshots with `SNAPSHOTS=overwrite`, which masked an actual code bug (wrong error messages for single-branch cases). A second pass was needed to read the test expectations, fix the code, then update snapshots properly. This doubled the fix-verify cycle for that sub-PR.

### Incremental build savings

The session correctly used `-p but` scoping for all cargo commands, limiting compilation to the `but` crate and its dependencies rather than the full workspace. This was essential given the project has 50+ crates. However, there were no targeted `--test` flags used -- every `cargo test -p but` ran the full test suite even when only one sub-PR's changes were in play. For the snapshot tests (s06), running `cargo test -p but -- branch::apply` would have been faster for iteration.

### Recommendation

Add to agent instructions:

```
## Build Verification (Required)
1. Every cargo command MUST be prefixed with `OPENSSL_NO_VENDOR=1`
2. After applying changes, run in this exact order:
   - `OPENSSL_NO_VENDOR=1 cargo check -p but`
   - `OPENSSL_NO_VENDOR=1 cargo clippy -p but -- -D warnings`
   - `OPENSSL_NO_VENDOR=1 cargo test -p but`
3. For snapshot test failures: READ the expected vs actual output BEFORE
   running SNAPSHOTS=overwrite. The mismatch may indicate a real bug.
4. Implementation agents MUST run at least `cargo check` and `cargo clippy`
   before declaring their patch complete. Do not claim "check passes"
   without actually running the command.
```

---

## Commit Workflow

### Round-trip analysis

The commit workflow evolved across the session through three distinct phases:

**Phase 1: Manual (Tier 0+1, lines ~1300-1515).** Changes were applied directly via the Edit tool. No commits were made during Tier 0 or Tier 1. All changes accumulated in the working tree as uncommitted modifications. This was a protocol violation -- the GitButler skill mandates `but status --json` before writes and committing after verified work.

**Phase 2: but-commit-group tool (Tier 0+1 commit, lines ~1945-1990).** The `but-commit-group` shell tool was created specifically to batch-commit related files by prefix. The first commit grouped 55 files matching `.github/prs/2/` and `scripts/` into a planning infrastructure commit. The second used `but-commit-group` with `crates/` prefix for the code changes. This was effective but required 2 separate `but-commit-group` invocations plus 4 `but amend` calls for locked files.

**Phase 3: Repeat for Tier 2+3 (lines ~2185-2270).** The same `but-commit-group` pattern was reused for Tier 2 and Tier 3, with the same 4 ghost files appearing as unassigned after each commit.

### Ghost files (0-diff locked to PR1)

Four files consistently appeared as "unassigned changes" after every commit:
- `crates/but/src/command/mod.rs` -- locked to commit `99c3560` (PR1 s13: wasi-first-compile)
- `crates/but/src/lib.rs` -- locked to commit `99c3560`
- `crates/but/src/utils/metrics.rs` -- locked to commit `5169ae8` (PR1 s02: cargo fmt)
- `scripts/but_functions.sh` -- locked to commit `25092f7` (PR1 feat/wasi)

These files had changes from both PR1 (WASI) and PR2 (CLI improvements) in the same hunks. GitButler's hunk-lock system correctly refused to cross-assign them, but this created persistent noise in `but status --json` output. The coordinator had to:
1. Check `but diff <id> --json` to confirm 0 hunks remaining
2. Identify them as ghosts each time
3. Explain them in the retro summary

Ironically, these ghost files were caused by the exact hunk-lock limitation that s04 (`--override-lock`) was designed to fix -- but s04's own code changes were among the locked hunks.

### Round-trip count

For the Tier 0+1 commit sequence:
- 1x `but status --json` (initial workspace state check)
- 1x `but-commit-group` for planning files (55 changes, branch creation)
- 1x `but-commit-group` for code changes (31 changes, new commit)
- 4x `but amend` for locked files
- 3x `but diff <id> --json` to verify ghost files had 0 hunks
- 1x `but status --json` to verify final state

Total: 11 round-trips for 2 commits. The ghost-file verification alone was 7 of those 11.

### Recommendation

```
## Commit Workflow
1. After verification passes, use `but-commit-group` to commit by prefix.
2. Ghost files (0-diff, locked to other branches) WILL appear as
   unassigned after commits. These are known artifacts of the hunk-lock
   system. Verify with `but diff <id> --json | jq '.hunks | length'`.
   If 0, ignore them.
3. Do NOT attempt to amend ghost files into PR2 commits -- they have
   no actual diff content.
4. Consider creating a `but-verify-ghosts` tool that batch-checks all
   unassigned files for 0-hunk status and reports them as known ghosts.
```

---

## Parallelism

### Plan called for

The AGENT.md specified a phased workflow per tier:
1. Plan phase: all sub-PR planning agents launched in parallel
2. Review phase: sequential review by coordinator
3. Implement phase: all implementation agents launched in parallel
4. Apply phase: sequential patch application in dependency order
5. Verify phase: cargo check + clippy + test

### Actual execution

**Tier 0 planning: 5 agents in parallel.** This worked well. All 5 planning agents (s00, s01, s02, s03, s11) were launched simultaneously. Completion times ranged from 49s (s01) to 179s (s03), showing genuine parallelism. The coordinator waited for all to complete before proceeding.

**Tier 0 implementation: 5 agents in parallel.** Also launched simultaneously. However, the coordinator then applied patches sequentially via Edit tool rather than using `git apply` on the INDEX.patch files. This was the right call -- the patches had potentially stale context lines, and manual application via Edit was more reliable.

**Tier 1: Collapsed into coordinator's direct work.** Rather than launching separate agents for s04, s05, s06, the coordinator read the INDEX.md plans and implemented the changes directly. This was faster for 3 small-to-medium sub-PRs that shared overlapping files (`args/mod.rs`, `lib.rs`).

**Tier 2: 2 exploration agents in parallel, then direct implementation.** The coordinator launched 2 agents to explore the codebase for s07 (branch rename) and s08 (status improvements), then implemented both directly. The exploration agents returned in ~46-49 seconds, providing the implementation details needed.

**Tier 3: 2 exploration agents in parallel, then direct implementation.** Same pattern as Tier 2.

### Bottleneck analysis

The primary bottleneck was **sequential apply + verify**. Each sub-PR's changes had to be applied, then the full cargo check + clippy + test pipeline ran (~2-5 minutes). This was unavoidable given that patches could interact.

A secondary bottleneck was the **coordinator as single-threaded orchestrator**. While planning and exploration agents ran in parallel, all implementation for Tiers 1-3 was done sequentially by the coordinator. This was actually more efficient than launching agents for these tiers because:
- The sub-PRs shared files (`args/mod.rs` especially)
- Context from one sub-PR informed the next
- Avoiding agent overhead (launch, context loading, output parsing) saved time

### Where more parallelism would help

**Verification could be pipelined.** Instead of check-then-clippy-then-test, `cargo check` and `cargo clippy` could potentially run in parallel (they use different compiler passes). In practice, clippy subsumes check, so running only `cargo clippy` + `cargo test` in parallel would be the optimal split.

**Planning and implementation could overlap per tier.** Once a sub-PR's plan is approved, its implementation agent could start while other planning agents are still running. This was not done -- all plans completed before any implementation started.

### Recommendation

```
## Parallelism Strategy
- Planning agents: always parallel (up to 5 concurrent)
- Implementation agents: parallel ONLY when sub-PRs touch disjoint files.
  When sub-PRs share files (especially args/mod.rs, lib.rs, command/mod.rs),
  implement sequentially in dependency order.
- For tiers with 2-3 small sub-PRs sharing files, skip agent spawning
  and implement directly -- the overhead of agent launch + context load
  exceeds the implementation time.
- Exploration agents: always parallel (read-only, no contention)
- Verification: run `cargo clippy` and `cargo test` as the only two
  verification steps (clippy subsumes check).
```

---

## Environment

### Issues encountered

**1. OPENSSL_NO_VENDOR=1 requirement.** Vendored OpenSSL fails to compile on this system due to assembler errors. Every cargo command needs this env var. This was not documented anywhere agents could find it, causing the first build failure.

**2. Installed binary vs local build confusion.** At one point the session explored whether `rust-analyzer` could be used for incremental type-checking. The analysis-stats subcommand was run as a background task. This was a tangent that consumed time without contributing to the build pipeline. The core issue was that agents need to know which binary (`but`) they are building -- the installed one in PATH or the workspace one in `target/`.

**3. Ghost files from PR1 hunk-locks.** Four files with changes from both PR1 and PR2 branches persistently appeared as unassigned in `but status --json`. These were cosmetic (0 actual diff hunks) but created confusion on every commit cycle and required multiple round-trips to verify.

**4. Session duration.** The session spanned 5 calendar days (March 14-19) in a single conversation thread. Context window pressure grew throughout, with cache creation tokens increasing from ~14K to ~143K. Late-session operations had to rely more heavily on cached context, and the coordinator's understanding of early decisions degraded.

### Environmental setup for future agents

```
## Environment Setup (Required in agent instructions)
export OPENSSL_NO_VENDOR=1

## Known workspace state
- PR1 (WASI) branches exist in the workspace with committed changes
- Files shared between PR1 and PR2 will appear as "ghost" unassigned
  changes with 0 hunks -- this is expected behavior from hunk-locks
- The `but` binary being built is at crates/but, not the installed
  binary in PATH
- Always use `cargo ... -p but` to scope builds to the but crate

## Known ghost files (ignore in status output)
- crates/but/src/command/mod.rs (locked to PR1 99c3560)
- crates/but/src/lib.rs (locked to PR1 99c3560)
- crates/but/src/utils/metrics.rs (locked to PR1 5169ae8)
- scripts/but_functions.sh (locked to PR1 25092f7)
```

---

## Tiered Execution

### Tier structure review

The 4-tier structure from INDEX.md:
- **Tier 0** (5 sub-PRs, no deps): s00, s01, s02, s03, s11
- **Tier 1** (3 sub-PRs, deps on Tier 0): s04, s05, s06
- **Tier 2** (2 sub-PRs, deps on Tier 1): s07, s08
- **Tier 3** (2 sub-PRs, deps on Tier 2): s09, s10

### Were dependencies correct?

**Mostly yes.** The dependency graph was accurate -- Tier 1 sub-PRs genuinely needed Tier 0 changes in place. For example:
- s04 (override-lock) needed s01 (null commit fix) to be applied first because both touch commit.rs
- s05 (batch stage) needed s02 (mutation IDs) for the new return type
- s06 (batch apply/unapply) needed s02 for the same reason

**One dependency was softer than modeled.** s07 (branch rename) depended on s03 (diff --name-only) only through `args/mod.rs` ordering, not through any functional dependency. Similarly, s08's dependency on s04 and s05 was primarily about `args/mod.rs` field ordering, not code logic. These were correctly identified as Tier 2 for conflict avoidance reasons, not functional reasons.

### Should Tier 2 and 3 have been combined?

**Yes, in hindsight.** Tier 2 (s07, s08) and Tier 3 (s09, s10) could have been a single tier. The dependencies between them were:
- s09 depends on s07 for `args/branch.rs` additions
- s10 depends on s08 for `args/mod.rs` changes

But these are additive, non-conflicting changes. The coordinator implemented all 4 in a single sequential pass anyway, with one cargo check + clippy + test cycle at the end. Merging into a single "Tier 2" would have saved one commit + verify cycle.

### Tier execution timeline

| Tier | Phase | Duration | Method |
|------|-------|----------|--------|
| 0 | Plan | ~3 min | 5 parallel agents (49-179s each) |
| 0 | Implement | ~3 min | 5 parallel agents (39-179s each) |
| 0 | Apply + Verify | ~10 min | Sequential apply, single verify pass |
| 1 | Plan + Implement | ~15 min | Direct by coordinator (no agents) |
| 1 | Apply + Verify | ~5 min | Sequential apply, single verify pass |
| 2 | Explore | ~1 min | 2 parallel agents |
| 2 | Implement + Verify | ~7 min | Direct by coordinator |
| 3 | Explore | ~1 min | 2 parallel agents |
| 3 | Implement + Verify | ~5 min | Direct by coordinator |
| Commit all | 3 commits | ~3 min | but-commit-group + amend |

Total active work: approximately 50 minutes of agent/coordinator execution. Total wall clock (including user think time, retros, and tool creation): several hours spread across 5 days.

### Recommendation

```
## Tiered Execution Guidelines
1. Tier 0 (no deps) benefits most from parallel agents -- launch all
   planning and implementation agents simultaneously.
2. Tiers with 2-3 small sub-PRs that share files: skip agents,
   implement directly. The coordinator's context from prior tiers
   makes this faster than agent overhead.
3. Consider merging tiers where inter-tier dependencies are purely
   about file ordering (e.g., enum variant positions in args/mod.rs)
   rather than functional logic.
4. Commit after each tier is verified, not at the end. This prevents
   the "Tier 0+1 all uncommitted" state that occurred, which made
   the commit workflow more complex.
5. Run the retro AFTER committing, not before. The Tier 0 retro
   identified issues but the commit still hadn't happened.
```

---

## Recommendations for Agent Instructions

### Pre-flight checklist (add to AGENT.md)

```markdown
## Before Starting Work
1. Run `OPENSSL_NO_VENDOR=1 cargo check -p but` to confirm the build works
2. Run `but status --json` to understand the current workspace state
3. Note any "ghost files" (0-diff, locked to other branches) -- these are
   expected and should be ignored
4. Verify you are building the workspace crate, not the installed binary
```

### Agent verification mandate

```markdown
## Implementation Agent Requirements
- You MUST run `OPENSSL_NO_VENDOR=1 cargo check -p but` after writing your patch
- You MUST run `OPENSSL_NO_VENDOR=1 cargo clippy -p but -- -D warnings` after check passes
- Do NOT claim "check passes" or "clippy clean" without actually running the commands
- If tests exist for your changed code, run them: `OPENSSL_NO_VENDOR=1 cargo test -p but -- <module>`
- For snapshot test failures: read the diff BEFORE using SNAPSHOTS=overwrite
```

### Commit protocol

```markdown
## Commit Protocol
1. Commit after EACH tier is verified, not at the end of all tiers
2. Use `but-commit-group` for grouping related files by prefix
3. Always include `--json --status-after` on mutation commands
4. Ghost files (0-diff locked to other branches) will persist after commits.
   Verify with `but diff <id> --json | jq '.hunks | length'` -- if 0, safe to ignore.
5. Never amend ghost files into PR2 commits -- they belong to PR1's hunks.
```

### Parallelism rules

```markdown
## When to Use Agents vs Direct Implementation
- Use parallel agents for: planning (always), exploration (always),
  implementation of 4+ sub-PRs touching disjoint files
- Use direct implementation for: 1-3 sub-PRs, sub-PRs sharing files,
  sub-PRs where coordinator already has full context from planning review
- Never launch implementation agents for different sub-PRs that modify
  the same file -- they will produce conflicting patches
```
