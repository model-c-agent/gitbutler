# Agent Strategy for PR-Based Development

This document defines the protocol for how agents work on PRs managed through the `.github/prs/` system.

## Overview

Each major project gets a numbered folder under `.github/prs/<number>/`. Inside, an `INDEX.md` describes the full scope and sub-PR breakdown. Sub-PRs are organized in folders whose names encode their dependency chain, making the dependency graph visible from the directory listing alone.

## Branch & Folder Naming Convention

### Dependency-Encoded Naming

Sub-PRs are identified by sequential IDs (`s01`, `s02`, ...). The folder and branch name encodes the dependency chain:

```
<deps>.<id>/<type>/<name>
```

**Rules:**
- `s01` — no dependencies (root)
- `s01.s02` — s02 depends on s01
- `s01.s04.s08` — s08 depends on both s01 and s04
- `s02-s08.s09` — s09 depends on s02 through s08 (range)

**Branch names** mirror the folder structure with a `pr<num>/` prefix:
```
pr1/s01/feat/wasi-feature-flags
pr1/s01.s02/feat/wasi-gate-git2
pr1/s01.s04.s08/feat/wasi-tokio-singlethread
pr1/s02-s08.s09/feat/wasi-first-compile
```

**Main branch:** `feat/<project-name>` (e.g. `feat/wasi`) — all sub-PRs merge here.

### Reading Dependencies from Names

Given a folder name like `s01.s04.s08/feat/wasi-tokio-singlethread`:
- Everything before the last `.sNN` segment is a dependency
- `s01` and `s04` must be merged before `s08` can start
- The last segment (`s08`) is this sub-PR's own ID

Given a range like `s02-s08.s09/feat/wasi-first-compile`:
- `s02-s08` means depends on s02, s03, s04, s05, s06, s07, s08
- `s09` is this sub-PR's own ID

## Directory Structure

```
.github/prs/
├── PR.md                                    ← This file (agent strategy)
├── SKILLS.md                            ← Available tools in scripts/bin/ + how to create new ones
└── <number>/
    ├── AGENT.md                             ← Coordinator agent instructions
    ├── INDEX.md                             ← Main PR summary + sub-PR breakdown
    ├── ESCALATIONS.md                       ← Questions surfaced to the user (created when needed)
    ├── s01/<type>/<name>/                   ← Root sub-PR (no deps)
    │   ├── INDEX.md
    │   ├── MEMORY.md
    │   ├── AGENT.md
    │   ├── QUESTIONS.md                     ← Questions for coordinator (created when needed)
    │   ├── INDEX.patch                      ← Proposed changes as unified diff (Step 3)
    │   ├── COMMIT.msg                       ← Commit message + PR description (maintained throughout)
    │   └── RESULTS.md                       ← Apply outcome from but agent (Step 6)
    ├── s01.s02/<type>/<name>/               ← Depends on s01
    │   ├── INDEX.md
    │   ├── MEMORY.md
    │   ├── AGENT.md
    │   ├── QUESTIONS.md
    │   ├── INDEX.patch
    │   ├── COMMIT.msg
    │   └── RESULTS.md
    └── s01.s04.s08/<type>/<name>/           ← Depends on s01 and s04
        ├── INDEX.md
        ├── MEMORY.md
        ├── AGENT.md
        ├── QUESTIONS.md
        ├── INDEX.patch
        ├── COMMIT.msg
        └── RESULTS.md
```

## Agent Hierarchy

### Coordinator Agent (`AGENT.md` at PR root)
- Orchestrates sub-PR agents, tracks progress, resolves questions
- Does NOT write code — delegates to sub-agents
- **Answers all sub-agent questions directly** — the coordinator has full authority to make technical decisions within the PR scope
- Only escalates to the user via ESCALATIONS.md when a decision is **outside PR scope** (architecture changes, new dependencies, scope changes, etc.)
- See `AGENT.md` in each PR folder for full coordinator instructions

### Sub-PR Agents (`AGENT.md` in each sub-PR folder)
- Execute the work defined in their sub-PR's INDEX.md
- Log progress, errors, and decisions in MEMORY.md
- Ask the coordinator questions via QUESTIONS.md
- Do NOT modify files outside their sub-PR's scope

### Question Flow
```
Sub-agent has question
    → writes to QUESTIONS.md in their directory
    → coordinator answers it (coordinator has full technical authority)
    → ONLY if the question is outside PR scope: coordinator writes to ESCALATIONS.md
    → user responds in ESCALATIONS.md
    → coordinator copies answer back to sub-agent's QUESTIONS.md
```

### What the Coordinator CAN Decide (No Escalation)
- Approving or revising plans
- Answering technical questions about the codebase
- Adjusting dependency ordering between sub-PRs
- Scope reductions (doing less than originally planned)
- Choosing between implementation approaches within INDEX.md scope

### What the Coordinator MUST Escalate
- Adding new sub-PRs or removing existing ones
- Architectural decisions not covered by INDEX.md
- Adding new external dependencies
- Blockers that no sub-agent can resolve (e.g., fundamental WASI incompatibility)
- Scope expansions (doing more than originally planned)

## File Responsibilities

### AGENT.md (PR root)
- **Purpose:** Instructions for the coordinator agent
- **Contains:** How to dispatch sub-agents, answer questions, track status, escalate
- **One per PR** — lives at `.github/prs/<number>/AGENT.md`

### ESCALATIONS.md (PR root)
- **Purpose:** Questions the coordinator cannot answer, surfaced to the user
- **Created on demand** — only when the first escalation occurs
- **Format:** Structured entries with date, source sub-PR, question, context, and status

### QUESTIONS.md (Sub-PR)
- **Purpose:** Sub-agent asks the coordinator for clarifications
- **Created on demand** — only when the sub-agent has a question
- **Format:** Structured Q&A entries with blocking status
- **Protocol:** Coordinator writes responses inline; escalates if unable to answer

### SKILLS.md (PR root)
- **Purpose:** Documents shell tools in `scripts/bin/` and the convention for creating new ones
- **Contains:** Tool descriptions, example usage, and a template for writing new tools
- **Key rule:** When a `but` workflow is missing or repetitive, write a reusable tool in `scripts/bin/` rather than running ad-hoc commands. Add the function to `scripts/but_functions.sh`, create a wrapper in `scripts/bin/`, and document it in SKILLS.md.

### INDEX.md (Main PR)
- **What:** Full project description, motivation, and goals
- **Sub-PR table:** Branch name, description, dependencies, size estimate
- **Dependency graph:** Which sub-PRs can be parallel vs sequential
- **Risk registry:** Known risks and mitigations
- **Acceptance criteria:** What "done" looks like for the entire project

### INDEX.md (Sub-PR)
- **Scope:** Exactly what this sub-PR changes and why
- **Files to modify:** Specific files with rationale for each change
- **Dependencies:** Which other sub-PRs must merge first (redundant with folder name, but human-readable)
- **Acceptance criteria:** Specific conditions that must be true when done
- **Complexity estimate:** S / M / L / XL

### MEMORY.md (Sub-PR)
Updated **during implementation** — not before. Contains:
- **Errors encountered:** What went wrong, error messages, stack traces
- **Fixes applied:** How each error was resolved and why that fix was chosen
- **Decisions made:** Any deviations from the INDEX.md plan with reasoning
- **Blockers:** Issues that couldn't be resolved and need escalation
- **Lessons learned:** Patterns to reuse or avoid in future sub-PRs

Format:
```markdown
# Memory: <sub-pr-name>

## Status: <not-started | planning | plan-review | in-progress | blocked | complete>

## Errors & Fixes
### <date> — <brief description>
**Error:** <what happened>
**Fix:** <what was done>
**Why:** <reasoning>

## Decisions
### <date> — <brief description>
**Context:** <what prompted the decision>
**Decision:** <what was decided>
**Alternatives considered:** <what else was considered>

## Blockers
- <description of blocker and what's needed to unblock>
```

### INDEX.patch (Sub-PR)
- **Purpose:** The agent's proposed code changes as a unified diff
- **Created during:** Step 3 (Implement) — NOT before
- **Format:** Standard unified diff, applicable with `but` patch operations
- **Scope:** Must only contain changes within this sub-PR's scope as defined in INDEX.md
- **Updated by:** The sub-PR agent — regenerated when addressing review feedback
- **Applied by:** The `but` agent — the sole entity that touches the working tree and commits
- **Key rule:** The agent reads the codebase to understand context, but writes ONLY this patch file. It does not modify source files directly.

### COMMIT.msg (Sub-PR)
- **Purpose:** Commit message for this sub-PR, also serves as the PR description when synced to GitHub
- **Created during:** Step 3 (Implement) — alongside `INDEX.patch`
- **Updated by:** The sub-PR agent iteratively as understanding deepens and review feedback arrives
- **Format:** Conventional commit style. First line = subject (commit message), body = extended description (PR body):
```
feat: gate networking dependencies behind native feature for WASI builds

Adds `native` feature gate to crates that depend on networking libraries
(reqwest, hyper, tokio with net feature). Under WASI, these are excluded
since WASI components use capability-based networking instead.

Affected crates: but-core, but-transport, but-sync
Acceptance criteria: `cargo check --target wasm32-wasip2` passes for gated crates
```
- **Key rule:** Always the best representation of the overall change. The agent maintains it throughout its lifecycle.

### RESULTS.md (Sub-PR)
- **Purpose:** Outcome of the `but` agent applying the patch
- **Created during:** Step 6 (Apply & Commit) by the `but` agent
- **Contains on success:** Commit ID, branch name, verification status
- **Contains on failure:** Error message, what went wrong, what the sub-PR agent should fix in its patch
- **Key rule:** This is how the `but` agent communicates back to the sub-PR agent and coordinator

### AGENT.md (Sub-PR)
- **Purpose:** Give an agent enough context to start working on this sub-PR
- **Must reference:** The parent `PR.md` (this file) for workflow rules
- **Must reference:** The main `INDEX.md` for project context
- **Contains:** Branch-specific focus areas, crates to modify, key patterns to follow
- **Contains:** What to update in MEMORY.md as work progresses

## Workspace

Agents share the same filesystem but **never modify source files directly**. Each sub-PR agent produces artifacts (`INDEX.patch`, `COMMIT.msg`) in its own `.github/prs/` directory. The `but` agent is the sole entity that applies patches to the working tree and commits.

This eliminates filesystem contention — agents can work in parallel because each writes only to its own sub-PR directory.

### Access Rules

1. **Sub-PR agents write only to their own `.github/prs/<N>/<sub-pr>/` directory.** They produce `INDEX.patch` and `COMMIT.msg` as implementation output. They do NOT modify source files directly.
2. **Sub-PR agents CAN read any file** in the repository, including source code and other agents' plan files (MEMORY.md, INDEX.md, INDEX.patch, etc.) for context and dependency awareness.
3. **The coordinator CAN write `QUESTIONS.md`** in any agent's `.github/prs/<N>/<agent>/` directory. This is how the coordinator delivers answers to sub-agent questions.
4. **The `but` agent is the sole committer.** It applies approved `INDEX.patch` files to the working tree and commits using the corresponding `COMMIT.msg`. No other agent uses `but` for workspace operations.
5. **The `but` agent writes `RESULTS.md`** in the sub-PR directory after applying (or failing to apply) a patch.
6. If an agent discovers work that belongs to another sub-PR, it notes it in its own `MEMORY.md` and moves on — it does NOT patch those files.

## Sub-Agent Lifecycle

A sub-agent goes through these steps in order. Each step must complete before the next begins.

```
┌─────────────────────────────────────────────────────────┐
│  STEP 1: PLAN                                  [parallel]│
│  Agent reads context, analyzes codebase, produces plan  │
│  Output: Updated INDEX.md + QUESTIONS.md                │
└──────────────────────┬──────────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────────┐
│  STEP 2: PLAN REVIEW                           [parallel]│
│  Coordinator answers questions, approves/revises plan   │
│  No escalation needed unless outside PR scope           │
└──────────────────────┬──────────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────────┐
│  STEP 3: IMPLEMENT                             [parallel]│
│  Agent reads codebase, produces INDEX.patch + COMMIT.msg│
│  Output: Patch file + commit message in sub-PR dir      │
│  Agent does NOT modify source files directly            │
└──────────────────────┬──────────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────────┐
│  STEP 4: PATCH REVIEW                          [parallel]│
│  Reviewer validates patch, checks scope + correctness   │
│  Provides feedback or approves                          │
└──────────────────────┬──────────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────────┐
│  STEP 5: ADDRESS FEEDBACK (if any)             [parallel]│
│  Agent regenerates INDEX.patch + updates COMMIT.msg     │
│  Loop back to STEP 4 until approved                     │
└──────────────────────┬──────────────────────────────────┘
                       ▼
┌─────────────────────────────────────────────────────────┐
│  STEP 6: APPLY & COMMIT                      [sequential]│
│  but agent applies INDEX.patch, commits w/ COMMIT.msg   │
│  Writes RESULTS.md. Must respect dependency order.      │
│  MEMORY.md status → complete.                           │
└─────────────────────────────────────────────────────────┘
```

### Step 1: Plan

1. Read context files: `PR.md`, main `INDEX.md`, sub-PR's `AGENT.md`, `INDEX.md`, `MEMORY.md`, `QUESTIONS.md`
2. Parse dependencies from folder name (everything before the last `.sNN`)
3. Read dependency sub-PRs' plans for context
4. Analyze the actual codebase — grep, read source files, understand the current state
5. Update `INDEX.md` with a concrete plan: specific files, specific changes, rationale
6. Write `QUESTIONS.md` with anything that needs input (mark each blocking or non-blocking)
7. Present plan and questions to the coordinator. **Stop and wait.**

### Step 2: Plan Review (Coordinator)

The coordinator:
1. Reads the updated `INDEX.md` and `QUESTIONS.md`
2. Answers all questions — the coordinator has **full authority** to make technical decisions within PR scope
3. Approves the plan, or requests specific changes
4. Only escalates to user if the question is **outside PR scope** (see "What the Coordinator MUST Escalate" above)

If changes requested → agent revises, loop back to Step 1.
If approved → agent proceeds to Step 3.

### Step 3: Implement

1. Read the codebase to understand the current state of all files in scope
2. If this sub-PR has dependencies, read their `INDEX.patch` files to understand what the working tree will look like after those patches are applied (layer your changes on top)
3. Write `INDEX.patch` in your sub-PR directory — a unified diff of all proposed changes
   - Format: standard unified diff, applicable by the `but` agent
   - Must be self-contained: applying this patch (after dependency patches) produces the full change
4. Write `COMMIT.msg` in your sub-PR directory — the commit message and PR description
   - First line: conventional commit subject (e.g., `feat: gate networking for WASI`)
   - Body: what changed, why, affected crates, acceptance criteria status
5. Verify your patch is correct:
   - All file paths in the diff exist in the repo (or are new files)
   - Context lines match the current source (accounting for dependency patches)
   - Changes stay within INDEX.md scope
6. Log errors, fixes, and decisions in `MEMORY.md`
7. Set `MEMORY.md` status to `patch-ready`
8. Present `INDEX.patch` and `COMMIT.msg` for review. **Stop and wait.**

### Step 4: Patch Review (Coordinator / Review Agent)

The reviewer:
1. Reads `INDEX.patch` — reviews the proposed changes for correctness, style, and scope
2. Reads `COMMIT.msg` — checks that it accurately describes the change
3. Checks scope — does the patch stay within INDEX.md boundaries?
4. Validates patch applicability — do context lines match the current source (accounting for dependency patches)?
5. Provides feedback in QUESTIONS.md or directly, or approves

If feedback → agent proceeds to Step 5.
If approved → agent proceeds to Step 6.

### Step 5: Address Feedback

1. Read review feedback
2. For each item: update the patch, update tests in the patch, or explain why no change is needed
3. Regenerate `INDEX.patch` with the corrected changes
4. Update `COMMIT.msg` to reflect any changes in scope or approach
5. Log changes in MEMORY.md
6. Present updated `INDEX.patch` and `COMMIT.msg`. Loop back to Step 4.

### Step 6: Apply & Commit (but agent)

Once the patch is approved:
1. The `but` agent applies `INDEX.patch` and creates a branch (if needed), committing using `COMMIT.msg`
   - The patch may correspond to its own branch — the `but` agent determines the right branch topology
   - All operations go through `but` — never use `git` directly
2. If the apply fails, the `but` agent writes the error to `RESULTS.md` in the sub-PR directory so the sub-PR agent can regenerate its patch
3. On success, the `but` agent writes the outcome to `RESULTS.md` (commit ID, branch name, verification status)
4. Update MEMORY.md: set `## Status: complete`
5. Coordinator dispatches the next sub-PR agent

## Coordinator Loop

On each invocation, the coordinator repeats:

### 1. Check Status
For each sub-PR directory, read:
- `MEMORY.md` — check `## Status:` line
- `QUESTIONS.md` — check for unanswered questions (entries without a `**Response:**`)

### 2. Answer Questions
- If answerable from project context, INDEX.md, or other sub-PRs' MEMORY.md: write response directly in their `QUESTIONS.md`
- If you need info from another sub-PR: check that sub-PR's MEMORY.md or QUESTIONS.md
- If you cannot answer: escalate to ESCALATIONS.md (see format below)

### 3. Review Plans
When a sub-agent's plan is ready:
- Is the plan concrete, correct, and within scope?
- Answer any questions in QUESTIONS.md
- If revision needed: write feedback in QUESTIONS.md, wait for update
- If approved: respond with `**Plan approved.** Proceed with implementation.`

### 4. Dispatch Work
- Identify which sub-PRs are **ready** (dependencies complete, status is not-started)
- Identify which sub-PRs are **blocked** and why
- Spawn agents per the parallelization rules below

### 5. Cross-Pollinate
When one sub-PR's MEMORY.md contains findings relevant to another:
- Write a note in the dependent sub-PR's `QUESTIONS.md`

### 6. Track Progress
Maintain awareness of: which sub-PRs are complete, in progress, or blocked.

## QUESTIONS.md Protocol

### Format (written by sub-agent):
```markdown
## Q: <short question title>
**Status:** open
**Blocking:** <yes/no>
**Question:** <the actual question>
```

### Format (response by coordinator):
```markdown
**Response:** <date> — <answer>
**Source:** <own knowledge / another sub-PR's MEMORY.md / user>
```

### Format (escalated to user):
```markdown
**Escalated:** <date> — see ESCALATIONS.md
```

## ESCALATIONS.md Protocol

When the coordinator can't answer a question (outside PR scope), create or append to `ESCALATIONS.md` in the PR directory:

```markdown
## E: <short title>
**Date:** <date>
**From:** <sub-PR ID and name>
**Question:** <the question>
**Context:** <why the coordinator can't answer>
**Status:** <open | resolved>

**User Response:** <filled in when the user responds>
```

Once the user responds, copy the answer back to the originating sub-PR's `QUESTIONS.md`.

## Branch Setup with `but`

Branches are created **on demand** when an agent enters Phase 2 (Implementation). The branch name encodes the dependency graph — the anchor is derived from it.

### Rules

1. **One branch per sub-PR.** Branch name = `pr<num>/` + folder path.
2. **Branches created on demand.** When an agent starts implementation, it creates its branch.
3. **Stacking via `--anchor`:** The dependency prefix in the branch name tells you where to anchor. If the dependency's branch exists, stack on it. If not (dependency already committed to `feat/wasi`), anchor on `feat/wasi`.
4. **One commit per sub-PR.** The `but` agent commits using `COMMIT.msg` from the sub-PR directory.

### Parallelization Rules

Not all lifecycle steps require sequential execution. The safety of parallelization depends on the step:

| Step | Parallel? | Why |
|------|-----------|-----|
| **Step 1: Plan** | **Yes** | Planning agents are read-only. Each writes only to its own directory. |
| **Step 2: Plan Review** | **Yes** | Reviewing plans is just reading files. |
| **Step 3: Implement** | **Yes** | Agents produce `INDEX.patch` + `COMMIT.msg` in their own directory. No branches, no staging, no working tree changes. |
| **Step 4-5: Review/Feedback** | **Yes** | Reviewing patches is reading files. Feedback goes to each agent's own QUESTIONS.md. |
| **Step 6: Apply & Commit** | **No** | The `but` agent applies patches sequentially. Must respect dependency order. |

**When to parallelize planning:** Spawn planning agents for all unblocked sub-PRs simultaneously when their dependencies are met.

**When to parallelize implementation:** After batch plan review, spawn implementation agents for all approved sub-PRs. Since agents produce patches (not file mutations), they can all work simultaneously. Agents with dependencies must read their dependency's `INDEX.patch` to layer changes correctly.

**Batch review before applying:** When multiple implementation agents complete, the coordinator/reviewer batch-reviews all patches. Once approved, the `but` agent applies them sequentially in dependency order.

**When NOT to parallelize:** Never apply more than one patch at a time. Patches must be applied in dependency order — a dependent patch's context lines assume its dependencies have already been applied.

### Deriving the Anchor

```bash
# s01: no deps → anchor on feat/wasi
but branch new pr1/s01/feat/wasi-feature-flags -a feat/wasi

# s01.s02: depends on s01 → anchor on s01's branch (if it exists) or feat/wasi
but branch new pr1/s01.s02/feat/wasi-serde-objectid -a feat/wasi

# s02.s09: depends on s02 → anchor on s02's branch
but branch new pr1/s02.s09/feat/wasi-gate-ctx -a pr1/s01.s02/feat/wasi-serde-objectid
```

### Single Commit Workflow

Each sub-PR is **one commit**. The `but` agent creates this commit from the approved `INDEX.patch` and `COMMIT.msg`. All operations go through `but` — never use `git` directly.

The `but` agent determines the right approach for applying the patch and creating the branch. It writes results (success or failure, commit ID, branch name) to `RESULTS.md` in the sub-PR directory.

If the patch needs updating after commit (e.g., review found an issue post-apply), the sub-PR agent regenerates `INDEX.patch` and `COMMIT.msg`, and the `but` agent amends via `but absorb` + `but reword`.

The `COMMIT.msg` is the source of truth for what the commit says. This keeps history clean: reviewing a sub-PR = reviewing one commit = reading one `COMMIT.msg`.

### Execution Order

Implementation runs sequentially, respecting the dependency graph. Planning can run in parallel batches. The coordinator determines order based on both the original dependency graph AND cross-cutting concerns discovered during batch plan review (e.g., shared Cargo.toml modifications creating implicit dependencies).

When batch review reveals that sub-PRs modify the same files (e.g., the `native` feature line in `Cargo.toml`), the coordinator adds implementation-time dependencies even if the original graph didn't require them. These dependencies are recorded by updating the sub-PR's `Deps` and `Anchor` fields in its INDEX.md.

## Conventions

- **One commit per sub-PR** — the `but` agent creates it from `INDEX.patch` + `COMMIT.msg`
- **Agents produce artifacts, not mutations** — sub-PR agents write only to their own directory
- **`COMMIT.msg` is the source of truth** — for the commit message and eventual PR description
- **All VCS through `but`** — never use `git` directly, not even for patch application
- Keep sub-PRs small and focused — one concern per PR
- Don't leak scope between sub-PRs
- If a sub-PR uncovers work that belongs to another sub-PR, note it in MEMORY.md and move on
- Prefer compilation errors over runtime errors when gating features (use `#[cfg]` and cargo features)
- Always update MEMORY.md before ending a work session
- When layering on dependencies, read their `INDEX.patch` to know what the working tree will look like

## Anti-Patterns

- Do NOT modify source files directly from a sub-PR agent — produce `INDEX.patch` instead
- Do NOT use `git` from any agent — only the `but` agent uses `but` for workspace operations
- Do NOT answer a question you're unsure about — escalate instead
- Do NOT ignore `Blocking: yes` questions — they halt progress
- Do NOT modify INDEX.md scope without user approval
- Do NOT apply patches out of dependency order — context lines will mismatch
- Do NOT combine planning and implementation into one agent spawn
