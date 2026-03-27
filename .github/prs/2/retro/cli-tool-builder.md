# PR #2 Retro: CLI Tool Builder Perspective

Session: `06204e05-4dd3-4d1e-9733-7e572bfb16c0`
Dates: 2026-03-14 to 2026-03-19
Scope: PR #2 for `but` CLI improvements (12 sub-PRs across 4 tiers)

## Protocol Compliance

### Rule 1: Use `but` for all write operations — never use `git` write commands

**Verdict: PASS with caveats.**

Zero `git add`, `git commit`, `git push`, `git checkout`, `git merge`, `git rebase`, `git stash`, or `git reset` commands were executed as bash tool invocations in this session. All write operations went through `but`.

The `git commit`, `git checkout`, and `git apply` strings that appear in the transcript (10, 15, and 30 occurrences respectively) are all in one of three categories:
- **Documentation references**: PR.md, RETRO.md, HISTORY.md describing the patch workflow format ("applicable with `git apply`") or documenting past failures
- **Code comments**: `alias.rs` doc comments showing `git config but.alias.st status`
- **Compaction summaries**: Context summaries from conversation compaction referencing past events

The agent correctly removed `git apply` from the but-agent's AGENT.md when the user flagged it as a protocol violation, replacing it with `but`-native operations.

One tool (`but-setup-branches`) still contains `git branch -D` in its `--clean` mode and `but-diff-files` uses `git diff HEAD`. These were documented in HISTORY.md as known capability gaps, not new violations.

### Rule 2: Start every write task with `but status --json`

**Verdict: FAIL for most of the session.**

The agent's own retro (line 1889) acknowledged this: "Never ran `but status --json` at the start." For the first ~1800 lines of the session, changes were applied directly via the Edit tool without checking workspace state first. The agent treated this as an "apply patches to source files" workflow rather than a `but` mutation workflow.

`but status --json` appears 48 times in the transcript, but the majority are:
- Inside tool definitions (`but_functions.sh`, `but-commit-group`)
- Inside documentation and references
- Late in the session (lines 1900+) when the agent finally started committing

The protocol was followed correctly only in the final phase (lines 1904+), when `but-commit-group` was created and used. That tool internally runs `but status --json` as its first operation, making it protocol-compliant by construction.

### Rule 3: Use `--json --status-after` on mutations

**Verdict: PARTIAL PASS.**

- `--json --status-after` appears 49 times, and `--status-after` appears 76 times total
- The `but-commit-group` tool (created mid-session) always uses `--json --status-after` on its `but commit` calls
- `but amend` with `--json` appears 48 times — good compliance
- `but squash` with `--json` appears 4 times
- However, `but commit` appears 129 times total but only 66 times with `--json` — the other 63 are in documentation, plan files, and skill references, not bare mutations missing the flag

**Net assessment**: When the agent actually ran mutations, it used `--json --status-after`. The gap is that most "mutations" were Edit tool operations on source files, not `but` commands, so this rule was structurally bypassed for the majority of the session.

### Rule 4: Use CLI IDs from `but status --json`

**Verdict: PASS in late session; N/A for most of session.**

The `but-commit-group` tool extracts `cliId` values from `but status --json` output via jq and passes them to `--changes`. This is textbook-correct. The output captured at line 2265 shows it working:
```
Found 6 changes matching prefixes: crates/
  tsk    crates/but/src/args/branch.rs
  mz     crates/but/src/command/legacy/branch/list.rs
  ...
```

The IDs `tsk`, `mz`, `lo`, `nm`, `nzs`, `nzt` are CLI IDs from status, not hardcoded.

### Rule 5: No redundant `but status` after `--status-after`

**Verdict: PASS.** One explicit mention of "redundant but status" in the transcript (count: 1), suggesting the agent was aware of this rule. The `but-commit-group` tool uses `--status-after` and does not follow up with a separate status call.

### Rule 6: Translate `git` write commands to `but`

**Verdict: PASS.** No instances of the user requesting a `git` write command that was executed as-is. The user explicitly said "we never want to use git directly anymore" and the agent complied.

### Rule 7: `but pull --check --json` before `but pull`

**Verdict: N/A.** `but pull --check` appears 27 times, all in documentation and skill references. No pull operations were needed in this session.

### Rule 8: Avoid routine `--help` probes

**Verdict: FAIL.** `but .* --help` appears 67 times. While some are in documentation, this count is high. The skill says to use command patterns from the skill file first, and only use `--help` when syntax is genuinely unclear. Many of these appear to be exploratory probes by sub-agents during the planning phase, which is wasteful when the skill file documents the canonical patterns.

## Command Patterns

### Frequency table

| Command | Count | Notes |
|---------|-------|-------|
| `but status` | 132 | Dominant read operation; most in docs/tool defs |
| `but commit` | 129 | Many in docs; ~20 actual executions |
| `but branch` | 144 | Includes `branch new`, `branch list`, etc. |
| `but stage` | 119 | Heavily used in tool definitions |
| `but reword` | 68 | Referenced in workflow docs |
| `but absorb` | 64 | Referenced in workflow docs |
| `but diff` | 58 | Read operation |
| `but amend` | 54 | Mutation command |
| `but rub` | 42 | Hunk reassignment |
| `but push` | 34 | Push operations |
| `but pull` | 27 | Pull/check operations |
| `but show` | 14 | Commit inspection |
| `but squash` | 14 | Squash operations |
| `but skill` | 10 | Skill check |
| `but move` | 9 | Commit reordering |

### Patterns that worked well

1. **`but-commit-group` for batch commits.** Created at line ~1924, this tool encapsulates the full protocol: `but status --json` -> extract matching IDs -> check if branch exists -> `but commit` with `--changes` and `--json --status-after`. Once created, every subsequent commit operation was protocol-compliant. This is the strongest evidence that **tooling is the best way to enforce protocol compliance**.

2. **`--changes` with comma-separated IDs.** The agent correctly used `--changes "$ids"` where `$ids` is a comma-separated list of CLI IDs. This matches the skill's documented pattern.

3. **Branch creation with `-c` flag.** `but commit <branch> -c -m "..." --changes ...` correctly creates a new branch and commits in one operation.

### Patterns that caused friction

1. **Edit-then-commit disconnect.** The session's primary activity was editing Rust source files via the Edit tool, then trying to commit the results via `but`. This two-phase approach meant the agent accumulated 8 sub-PRs of changes before attempting any commits. The `but` protocol assumes you start with `but status`, identify changes, and commit them. The agent did the opposite: made changes first, committed last.

2. **Manual patch application vs `git apply`.** The plan called for INDEX.patch files to be applied via the `but` agent. In practice, the agent read the patches and manually applied edits via the Edit tool because "reading patches and manually applying edits was faster than trying to apply unified diffs that might have stale context lines." This is pragmatic but defeats the purpose of the patch workflow. The user explicitly pushed back: "The point of the patch file ceremony is that agents can work in parallel."

3. **67 `--help` probes.** Many during the planning phase by sub-agents. The skill file already documents canonical command patterns. These probes added latency without value.

4. **Hunk lock remains unresolved.** 92 references to "hunk lock" in the transcript. This is the fundamental friction point: `but`'s hunk assignment algorithm decides which branch a change belongs to, and agents cannot override it. The `but-commit-group` tool partially mitigates this by filtering to specific path prefixes, but a `--override-lock` flag on `but stage` would eliminate the problem.

## Tool Gaps

### `but-commit-group` (created mid-session)

**Why it was needed:** After applying Tier 0+1 changes (8 sub-PRs, ~30 files), the agent needed to commit subsets of changes to specific branches. The user rejected inline `python3` for parsing `but status --json`, saying "create tool instead." This matches the project preference: "Never pipe `but` output through `jq` inline -- always create a reusable tool in `scripts/bin/`."

**What it does:** Takes a branch name, commit message, and one or more path prefixes. Runs `but status --json`, filters unassigned changes by prefix, and commits them with `--changes`. Creates the branch if it doesn't exist. Supports `--dry-run`.

**Assessment:** Well-designed, protocol-compliant, reusable. Should have been created at the start of the session rather than after 1900+ lines. If this tool had existed from the beginning, every commit would have been protocol-compliant by default.

### Tools that should be pre-built for agent workflows

1. **`but-apply-patch`** -- Apply a unified diff from an INDEX.patch file through `but`. Currently missing. The `git apply` fallback was explicitly rejected. This tool would: apply the patch to the working tree, run `but status --json` to identify the resulting changes, and commit them to the specified branch. This is the missing link in the patch-based agent workflow.

2. **`but-verify`** -- Run `cargo check + clippy + test` for a specific crate with `OPENSSL_NO_VENDOR=1`. Every sub-PR verification required remembering the env var. A tool would standardize this and could be extended with snapshot update logic.

3. **`but-status-changes`** -- Filtered view of `but status --json` changes by path prefix, returning just the CLI IDs. The `but-commit-group` tool does this internally, but extracting it as a standalone tool would enable composable workflows (e.g., `but-status-changes crates/but/src/ | xargs but-commit-group ...`).

4. **`but-sub-pr-commit`** -- Takes a sub-PR directory path, reads COMMIT.msg, identifies changed files from INDEX.patch, commits them to the appropriate branch. This would be the end-to-end tool for the patch workflow: read COMMIT.msg -> apply INDEX.patch -> commit with message from COMMIT.msg.

5. **`but-tier-apply`** -- Orchestrates applying all sub-PRs in a tier in dependency order. Reads agents.json, topologically sorts, applies each INDEX.patch, runs verification, writes RESULTS.md. Currently this orchestration is done manually by the coordinator agent.

## Error Recovery

### Hunk lock corruption (recurring)

92 mentions of "hunk lock" in the transcript. This is the session's dominant error pattern, inherited from PR #1. The `but` hunk assignment algorithm locks files to branches based on their creation context, and agents cannot override this. The `but-commit-group` tool mitigates it by committing only unassigned changes matching specific path prefixes, avoiding locked hunks. Recovery was pragmatic but not systematic.

### Clippy failures caught post-apply

The s00 plugin patch had a collapsible `if let` in `list_external_subcommands` that clippy caught. Fixed in one edit. The agent's own retro noted: "agents aren't running clippy on their patches." This is a training data gap -- agent instructions should include `cargo clippy` as part of verification, not just `cargo check`.

### Snapshot test failures (s04)

5 snapshot tests broke after s04 (stage override-lock) changes. The agent first tried `SNAPSHOTS=overwrite` blindly, then discovered the error messages were wrong for single-branch cases, had to fix the code, and re-update snapshots. Two wasted cycles. A `but-verify` tool that shows snapshot diffs before overwriting would have caught this earlier.

### OpenSSL build environment

Every `cargo` command needed `OPENSSL_NO_VENDOR=1` due to assembler errors in vendored OpenSSL. The plan didn't account for this. The agent had to discover and propagate it manually. A pre-built verification tool would have encoded this.

### Premature implementation during plan mode

The agent started creating source files (alias.rs edits, args/plugin.rs, command/plugin.rs) before the plan was approved, then had to revert everything when plan mode was re-entered. One full round-trip of work wasted. The agent's control flow should enforce: plan approval gate before any source file modifications.

### Context compaction (3 times)

The conversation hit context limits 3 times (at lines ~785, ~1166, ~1950), requiring compaction summaries. Each compaction loses fine-grained state. The agent recovered well each time, producing accurate summaries and resuming without asking the user to repeat context. However, the 2273-line transcript for a single session suggests the conversation should have been split into multiple sessions at natural phase boundaries.

## Recommendations for Agent Instructions

### 1. Make `but-commit-group` a required prelude

Every agent session that will modify source files should create or verify the existence of `but-commit-group` before making any changes. This tool is the protocol compliance guarantee -- it runs `but status --json` internally, uses CLI IDs, and passes `--json --status-after`. Without it, agents default to the Edit-first-commit-last antipattern.

### 2. Add a "commit checkpoint" after each sub-PR

The current workflow accumulates changes across multiple sub-PRs before committing. This creates two problems: (a) the `but status --json` output becomes noisy with unrelated changes, and (b) a single failure can corrupt the entire batch. Agent instructions should require: `after verifying a sub-PR, commit it immediately using but-commit-group before starting the next sub-PR`.

### 3. Pre-build `but-apply-patch` before any patch-workflow session

The patch-based workflow (INDEX.patch + COMMIT.msg) is the project's coordination strategy for parallel agent work. But the critical tool -- applying a patch through `but` -- does not exist. Agents fall back to manual Edit application, defeating the purpose. This tool should be the first thing built in any session that uses the patch workflow.

### 4. Ban `--help` probes in agent instructions

Add to agent instructions: "Do not run `but <command> --help`. Use the command patterns documented in the gitbutler skill file (`SKILL.md`) and `references/reference.md`. Only use `--help` if a command fails with an unknown-flag error." This would eliminate ~60 unnecessary `--help` calls per session.

### 5. Require `OPENSSL_NO_VENDOR=1` in all cargo commands

Add to agent instructions and/or create a `but-cargo` wrapper: `OPENSSL_NO_VENDOR=1 cargo "$@"`. This eliminates the recurring build environment surprise.

### 6. Enforce plan-approval gate

Agent instructions should include: "Never modify source files (anything under `crates/`, `src/`, `lib/`) during plan mode. Plan mode produces only `.github/prs/` files (INDEX.md, AGENT.md, MEMORY.md, INDEX.patch, COMMIT.msg). Source file modifications begin only after plan approval." The premature implementation during this session wasted a full cycle.

### 7. Run clippy in agent verification, not just cargo check

Change agent instructions from "run cargo check after changes" to "run cargo check && cargo clippy -- -D warnings after changes." Clippy catches issues (like the collapsible if-let) that check misses, and finding them during apply is cheaper than finding them during review.

### 8. Split sessions at tier boundaries

A single session spanning 2273 lines and 3 context compactions is too long. Agent instructions should recommend: "Start a new session at each tier boundary. The previous session's MEMORY.md files carry state forward." This keeps each session within context limits and produces cleaner retros.

### 9. Create `but status --json` as a session preamble

Add to the start of every agent session: "Run `but status --json` and record the workspace state. Identify which branches exist, which changes are unassigned, and which changes are assigned to branches. This is your baseline -- all mutations should start from this known state."
