# PR #2 Agent Workflow Retrospective

**Session:** `06204e05-4dd3-4d1e-9733-7e572bfb16c0`
**Duration:** 2026-03-14 to 2026-03-19 (5 days, 3 context compactions)
**Scope:** 12 sub-PRs across 4 tiers, Tier 0 + Tier 1 completed, Tier 2 in progress

---

## Protocol Drift Analysis

### Drift 1: Agent directly modified source files instead of producing INDEX.patch

**What happened:** The agent read the INDEX.patch files produced by planning/implementation sub-agents, then applied the changes directly using the Edit tool on source files (`crates/but/src/*.rs`). The plan called for the `but` agent to apply patches as unified diffs.

**Root cause:** Pragmatic shortcut. The agent noted in its own retro: "Reading patches and manually applying edits was faster than trying to `git apply` unified diffs that might have stale context lines." The patches were conceptual reference documents, not byte-exact diffs.

**Impact:** Moderate. The changes landed correctly (cargo check/clippy/test all passed), but the entire patch-based workflow ceremony was bypassed. The agent then called the patch workflow "overhead" in its retro, which the user explicitly corrected: "The point of the patch file ceremony is that agents can work in parallel and not step on each others toes."

**Assessment:** The agent's pragmatism was locally correct (it got the code in faster) but architecturally wrong (it invalidated the coordination protocol that exists for multi-agent parallelism). Future agents must not optimize away coordination mechanisms just because they are the only agent running.

### Drift 2: No COMMIT.msg or RESULTS.md files written

**What happened:** The plan (Phase 5) called for each sub-PR to have COMMIT.msg used for the final commit, and RESULTS.md written after apply. Neither was produced for the sub-PRs where patches were applied directly.

**Root cause:** Cascading from Drift 1. Once the agent bypassed the patch-apply workflow, the downstream artifacts (RESULTS.md, COMMIT.msg consumption) became irrelevant.

**Impact:** Low for immediate work, high for process integrity. The COMMIT.msg files that existed (s00, s01, s02, s03, s11) were never used as the commit message source. Instead, the agent wrote its own commit messages via `but-commit-group`.

### Drift 3: No per-sub-PR branches created

**What happened:** The plan said "each sub-PR gets its own branch" (`pr2/s00/feat/plugin-system`, etc. as defined in agents.json). Instead, all changes were accumulated on the workspace and committed to a single `feat/but-cli-improvements` branch.

**Root cause:** Context loss plus pragmatism. The agent noted this was "actually fine for the `but` workflow (changes accumulate on gitbutler/workspace)." The branch-per-sub-PR strategy from agents.json was never referenced after the context compaction.

**Impact:** Medium. The dependency graph encoding in branch names is lost. Individual sub-PRs cannot be reviewed, reverted, or cherry-picked independently.

### Drift 4: Never ran `but status --json` at the start of write tasks

**What happened:** The GitButler skill protocol rule #2 says "Start every write/history-edit task with `but status --json`." The agent identified this as a violation in its own retro at line 1889.

**Root cause:** The agent was focused on the plan's phased structure and forgot the per-task protocol steps. The compaction summaries did not include a reminder about the `but status --json` preflight.

**Impact:** Medium. Without the status check, the agent did not discover the locked-file situation until after two commits had already landed, leaving 4 critical files (lib.rs, command/mod.rs, utils/metrics.rs, but_functions.sh) orphaned in the unstaged area.

### Drift 5: Piped `but status --json` through inline Python

**What happened:** At line 1895, the agent ran `but status --json 2>&1 | python3 -c "import json, sys..."` to parse the JSON output. The user rejected this (line 1900). A second attempt at line 1899 was also rejected.

**Root cause:** The agent did not internalize the user preference from MEMORY.md: "Never pipe `but` output through `jq` inline -- always create a reusable tool in `scripts/bin/` instead." The agent extended this violation to Python as well.

**Impact:** Low (caught immediately by user), but it shows the agent failed to consult MEMORY.md before choosing a data extraction strategy.

### Drift 6: Snapshot tests blindly overwritten

**What happened:** When s06 (batch apply/unapply) caused 5 snapshot test failures, the agent ran `SNAPSHOTS=overwrite` without inspecting what the snapshots contained. It then discovered the error messages were wrong for single-branch cases, had to fix the code, and re-update snapshots.

**Root cause:** Speed-over-correctness bias. The agent treated snapshot failures as routine formatting changes rather than potential correctness signals.

**Impact:** Low (caught in the same session), but the pattern of "accept first, inspect later" is dangerous for test infrastructure.

---

## Instruction Gaps

### Gap 1: No "pre-commit checklist" in SKILL.md

The GitButler skill has the rule "Start every write/history-edit task with `but status --json`" but there is no corresponding checklist for the commit workflow. The agent needs a step-by-step recipe: status check, identify IDs, commit with `--changes`, verify with `--status-after`.

**Proposed addition to SKILL.md:**
```markdown
### Pre-Commit Checklist

Before committing any changes, always follow this sequence:

1. `but status --json` -- identify all unassigned changes and their cliIds
2. Group changes logically (by sub-PR, by concern, by path prefix)
3. If a reusable tool exists for the grouping (e.g., `but-commit-group`), use it
4. For each commit group:
   a. `but commit <branch> -c -m "<message>" --changes <ids> --json --status-after`
   b. Verify the `--status-after` output shows the expected remaining changes
5. If changes show lock icons, do NOT skip them -- address them explicitly
```

### Gap 2: No instruction about inline data processing

MEMORY.md says "Never pipe `but` output through `jq` inline" but this needs to be more prominent and more general.

**Proposed addition to SKILL.md:**
```markdown
### Data Processing Rule

Never pipe `but` output through inline scripts (jq, python, awk, etc.).
If you need to parse or filter `but` JSON output, create a reusable tool
in `scripts/bin/` following the existing `but-*` tool pattern. This applies
to ALL inline processing, not just jq.
```

### Gap 3: No instruction about respecting coordination protocols even when running solo

The agent dismissed the patch workflow as "overhead" because it was the only agent running. There is no instruction that says coordination mechanisms must be followed regardless of current parallelism level.

**Proposed addition to PR.md (Conventions section):**
```markdown
- **Always follow the artifact protocol** -- even when running as the sole agent.
  The patch-based workflow (INDEX.patch, COMMIT.msg, RESULTS.md) exists for
  reproducibility and auditability, not just parallelism. Skipping these
  artifacts makes the work unreviewable and unreproducible by other agents.
```

### Gap 4: No instruction about locked files in the commit workflow

The agent encountered hunk-locked files and had to improvise three options. There should be a standard recipe.

**Proposed addition to SKILL.md:**
```markdown
### Handling Locked Files (Hunk Locks)

When `but status` shows files with lock icons (e.g., `M crates/but/src/lib.rs
99c35603b8`), these files have changes that span multiple branches.

1. Identify which lock each file belongs to (the commit hash after the lock icon)
2. Determine if the locked changes genuinely belong to the locked branch or your current branch
3. If they belong to your branch: use `but amend <file-id> <target-commit-id>` to
   force-assign them (this is the `--override-lock` use case)
4. If they belong to the other branch: leave them alone
5. Always document the decision in your commit message or MEMORY.md
```

### Gap 5: Compaction summary does not include protocol reminders

When a conversation is compacted, the summary preserves task state but loses behavioral constraints. The agent forgot `but status --json` preflight, the no-inline-processing rule, and the branch-per-sub-PR strategy.

**Proposed addition to a new file, `HANDOFF.md` (see Context Handoff section):**
```markdown
## Always Include in Compaction Summaries

1. The `but status --json` preflight rule
2. The no-inline-processing rule (use scripts/bin/ tools)
3. The branch strategy from agents.json (one branch per sub-PR, or shared branch)
4. Any user preferences expressed during the session
5. The current commit state (what has been committed, what remains)
```

---

## Context Handoff

This session had **3 context compactions** (lines 785, 1166, 1950). Each compaction produced a detailed summary. The quality varied:

### Compaction 1 (line 785): Good

**What was preserved:**
- All user messages with exact quotes
- File list with line numbers and code snippets
- Pending tasks clearly stated
- The "Add a plugin system like cargo" rejection context

**What was lost:**
- The user's strong emotional investment in the `but`-only rule (the rejection was heated, with a multi-paragraph vision statement about the polyverse)
- The user's preference for `scripts/bin/` tools over inline processing

### Compaction 2 (line 1166): Good

**What was preserved:**
- Complete file inventory with code snippets
- All 5 INDEX.patch files and their scope
- The execution order and current phase
- Cross-cutting conflict analysis (args/mod.rs)

**What was lost:**
- The fact that COMMIT.msg files should be the source of commit messages (not agent-generated)
- The branch-per-sub-PR strategy from agents.json
- The user preference about `scripts/bin/` tools

### Compaction 3 (line 1950): Adequate but missing protocol context

**What was preserved:**
- The two-commit strategy (infra + source)
- The first commit's success and remaining 30 changes
- The `but-commit-group` tool creation

**What was lost:**
- The 4 locked files were not mentioned (discovered only after the handoff agent ran `but status`)
- The agents.json branch strategy
- The `but status --json` preflight requirement
- The no-inline-processing preference

### Recommendation

Compaction summaries need a **"Protocol Context" section** that is always included, separate from task state. This section should contain:

1. Active protocol rules (SKILL.md non-negotiables)
2. User preferences expressed in this session
3. Decisions made that deviate from the plan (and why)
4. Known state of the workspace (committed vs uncommitted)
5. Known blockers or locked resources

---

## Decision Points

### Decision 1: Skip patch-apply workflow, use Edit tool directly

**Context:** Tier 0 implementation. Agent had 5 INDEX.patch files and needed to apply them.

**What the agent chose:** Read the patches for reference, then apply changes directly using the Edit tool on source files.

**Was this right?** Locally effective, architecturally wrong. The changes compiled and tests passed, but the protocol was violated. The agent should have at least attempted `but`-based patch application and only fallen back to direct edits if that failed, documenting the failure in RESULTS.md.

**How future agents should handle this:** Attempt the protocol-defined approach first. If it fails, document the failure, then use the fallback. Never silently skip a protocol step.

### Decision 2: Amend locked files into PR2 commit (option 3)

**Context:** After two commits landed on `feat/but-cli-improvements`, 4 files remained locked to PR1 commits. The agent presented 3 options and the user chose option 3.

**What the agent chose:** `but amend <file-id> <commit-id>` to force the locked hunks into the PR2 commit.

**Was this right?** Yes, given the constraints. The locked files genuinely contained PR2 changes (plugin exec path, sync dispatch, module registrations). The alternative was to leave critical code uncommitted. The user explicitly chose this option.

**How future agents should handle this:** Always present locked-file situations to the user with clear options. Never silently force-assign locked hunks. Document the cross-branch contamination.

### Decision 3: Two logical commits instead of one-per-sub-PR

**Context:** 83 unassigned changes across 8 sub-PRs. The plan called for one branch and one commit per sub-PR.

**What the agent chose:** Two commits on a single branch: (1) PR infrastructure + tools, (2) all source code changes.

**Was this right?** Pragmatic but lossy. It was the simplest way to get everything committed, but it means the commit history does not reflect the sub-PR structure. Individual sub-PR changes cannot be reviewed or reverted independently.

**How future agents should handle this:** If the plan says one commit per sub-PR, follow it. Use `but-commit-group` with path prefixes that match sub-PR scopes. If grouping is impractical (e.g., shared files), document why a monolithic commit was necessary.

### Decision 4: Create `but-commit-group` tool

**Context:** The agent tried to pipe `but status --json` through Python to extract IDs for committing. The user rejected this.

**What the agent chose:** Created a reusable `scripts/bin/but-commit-group` tool.

**Was this right?** Yes. This followed the established pattern and the user's preference. The tool is reusable for future commit workflows.

### Decision 5: Immediately proceeding to Tier 2 after committing Tier 0+1

**Context:** After resolving the locked files, the agent launched exploration agents for Tier 2 (s07, s08) in the same turn.

**Was this right?** Acceptable but rushed. The agent did not verify the final commit state or update MEMORY.md files for the completed sub-PRs before moving on. A brief pause to update status would have been better.

---

## Instruction Architecture

### Current State

The protocol is spread across 7+ locations:

1. **SKILL.md** (`~/.claude/skills/gitbutler/SKILL.md`) -- `but` CLI command patterns
2. **CLAUDE.md** (`~/c/CLAUDE.md`) -- general behavioral rules (irrelevant to this project)
3. **MEMORY.md** (project memory) -- user preferences, project state
4. **PR.md** (`.github/prs/PR.md`) -- agent coordination protocol
5. **AGENT.md** (`.github/prs/2/AGENT.md`) -- PR-specific coordinator instructions
6. **Plan file** (`~/.claude/plans/sunny-hatching-codd.md`) -- execution phases
7. **feedback_*.md** files -- scattered behavioral corrections

### Problems with this architecture

1. **No single entry point.** An agent starting work must read 5+ files to understand the protocol. After context compaction, it may not re-read all of them.

2. **Behavioral rules are scattered.** "Never pipe `but` output through jq" is in MEMORY.md. "Never use git directly" is in feedback_no_git_directly.md. "Start with `but status --json`" is in SKILL.md. An agent needs all of these but they are in different file hierarchies.

3. **Plan vs Protocol confusion.** The plan file describes WHAT to do (phases, sub-PRs). PR.md describes HOW to do it (artifact protocol). SKILL.md describes the TOOL interface. These layers interact but are not cross-referenced.

4. **Memory files reference non-existent files.** MEMORY.md references `feedback_branch_workflow.md` and `feedback_agent_lifecycle.md` which do not exist. Dead references waste agent attention.

### Recommendation: Layered architecture with a single entry point

```
Layer 1: SKILL.md (tool interface)
  - How to use `but` commands
  - Command patterns, ID resolution, error handling
  - NEVER changes between sessions

Layer 2: PR.md (coordination protocol)
  - How agents coordinate (artifacts, lifecycle, branches)
  - File responsibilities, parallelization rules
  - Changes only when the protocol evolves

Layer 3: AGENT.md (per-PR instructions)
  - References Layer 1 and Layer 2
  - PR-specific decisions (apply order, conflict strategy)
  - Contains a QUICKSTART section for post-compaction recovery

Layer 4: Plan file (execution state)
  - What to do and in what order
  - Phase tracking, completion status
  - The only layer that changes frequently
```

The **AGENT.md QUICKSTART section** is the key missing piece. It should contain everything an agent needs after a context compaction -- a 10-line protocol checklist that does not require reading 5 other files.

---

## Proposed Agent Playbook

This is the minimal instruction set an agent needs to follow the protocol correctly. It should be added to `.github/prs/2/AGENT.md` as a QUICKSTART section.

```markdown
## QUICKSTART (Post-Compaction Recovery)

Read this section first after any context compaction.

### Non-Negotiable Rules

1. **`but` only.** Never use `git` for write operations. Use `but` for all
   commits, amends, pushes, and branch operations. Read-only `git log`/`git
   blame` is fine.

2. **`but status --json` first.** Before ANY write/commit/amend operation,
   run `but status --json` to get current workspace state and CLI IDs.

3. **No inline parsing.** Never pipe `but` output through jq/python/awk.
   If you need to parse JSON, create a tool in `scripts/bin/but-<name>`.

4. **Artifacts, not mutations.** Sub-PR agents produce INDEX.patch +
   COMMIT.msg in their own directory. They do NOT edit source files.
   The coordinator or `but` agent applies patches and commits.

5. **`--json --status-after` on mutations.** Every `but commit`, `but amend`,
   or other mutation must include these flags.

6. **One branch per sub-PR** (from agents.json), unless the user explicitly
   approves a different strategy.

7. **COMMIT.msg is the commit message source.** Do not write your own commit
   messages. Use the COMMIT.msg file from the sub-PR directory.

### Commit Workflow

```bash
# 1. Check state
but status --json

# 2. Identify changes by sub-PR (use scripts/bin/ tools)
./scripts/bin/but-commit-group <branch> "<message>" <path-prefix>

# 3. Verify
but status --json  # confirm expected changes remain

# 4. Handle locked files
# If files show lock icons, present options to user. Never force-assign silently.
```

### Current State Checklist

After compaction, verify:
- [ ] Which sub-PRs are committed? (check `but status` for branches)
- [ ] Which sub-PRs have uncommitted changes? (check unassigned changes)
- [ ] Are there locked files? (look for lock icons in `but status`)
- [ ] What phase are we in? (check plan file and MEMORY.md statuses)

### Key Files

- Plan: `/home/willem/.claude/plans/sunny-hatching-codd.md`
- Protocol: `.github/prs/PR.md`
- Agent config: `.github/prs/2/agents.json`
- Tool skill: `~/.claude/skills/gitbutler/SKILL.md`
- User prefs: `~/.claude/projects/-home-willem-c-gitbutler/memory/MEMORY.md`
```

### Additional SKILL.md Additions

Add after the "Notes" section:

```markdown
## Agent Workflow Integration

When working within the `.github/prs/` agent system:

1. **Sub-PR agents do NOT use `but` for commits.** They produce artifacts
   (INDEX.patch, COMMIT.msg) in their directory. Only the coordinator or
   `but` agent commits.

2. **COMMIT.msg is the source of truth** for commit messages. When the `but`
   agent commits, it uses the message from COMMIT.msg, not an ad-hoc message.

3. **Locked files require user decision.** When `but status` shows locked
   files (lock icon + commit hash), present the situation and options to the
   user. Never silently force-assign.

4. **Coordination protocols are not optional.** Even when you are the sole
   agent, follow the artifact workflow (INDEX.patch, COMMIT.msg, RESULTS.md).
   These exist for auditability and reproducibility, not just parallelism.

5. **After context compaction**, re-read the QUICKSTART section in the PR's
   AGENT.md before resuming work.
```

### Memory File Cleanup

The following should be added to MEMORY.md or a new `feedback_commit_workflow.md`:

```markdown
---
name: Commit workflow discipline
description: Always follow the full commit workflow even when running solo
type: feedback
---

Always follow the commit workflow protocol:
1. `but status --json` before any write operation
2. Use COMMIT.msg files as commit message sources
3. Create reusable tools instead of inline parsing
4. Follow the artifact protocol (INDEX.patch, COMMIT.msg, RESULTS.md)
   even when you are the only agent running
5. Present locked-file situations to the user with options
6. Never dismiss coordination mechanisms as "overhead"
```

Dead references should be cleaned up:
- Remove `feedback_branch_workflow.md` and `feedback_agent_lifecycle.md`
  from MEMORY.md's index (files do not exist on disk)
