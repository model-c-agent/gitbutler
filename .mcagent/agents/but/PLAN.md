# Plan: but Agent — Workspace Manager

## What Makes a Good AGENT.md

An AGENT.md is the bootstrap file for an autonomous agent. It must answer six questions clearly enough that the agent can start working without asking anything:

### 1. Role (Who are you?)
- One sentence stating the agent's purpose and scope boundary
- What the agent IS responsible for, and explicitly what it is NOT
- Where this agent sits relative to other agents (peer, coordinator, subordinate)

### 2. Startup (How do you orient?)
- Ordered list of files to read and commands to run before doing anything
- Must be deterministic — no "figure it out" steps
- Gives the agent situational awareness of the current state

### 3. Capabilities (What can you do?)
- Concrete list of operations grouped by category
- Each capability should map to a specific tool or command
- Distinguish between things the agent does directly vs. delegates

### 4. Tools (What do you use?)
- Table of available tools with names and one-line purposes
- How to invoke them (path convention, arguments)
- How and when to create new tools

### 5. Decision Authority (What can you decide alone?)
- Two explicit lists: CAN decide vs. MUST escalate
- Prevents both over-caution (asking about everything) and recklessness (doing dangerous things silently)
- Tied to reversibility — reversible actions are self-authorized, irreversible ones need approval

### 6. Communication (How do you talk to others?)
- File-based protocol: which files to read, which to write, what format
- How requests arrive and how results are delivered
- Escalation path when something goes wrong

### Principles
- **Parsimony:** Only include what the agent needs to start. Link to external docs for depth.
- **No prose dumps:** Use tables, lists, and headers. Agents scan, they don't read essays.
- **Executable startup:** Every step in "Startup" should be a command or file read, not a vague instruction.
- **Anti-patterns over rules:** Telling an agent what NOT to do is often more useful than exhaustive rules.
- **Scope boundary:** The single most important thing. An agent that doesn't know its boundary will either do too little or break things.

---

## Current State

The `but` agent has a rewritten AGENT.md that follows the structure above. The following infrastructure exists:

### Existing Tools (`scripts/bin/`)
- `but-apply-pattern` — apply branches by regex
- `but-unapply-pattern` — unapply branches by regex
- `but-stage-all` — stage multiple change IDs
- `but-changes` — show change assignments
- `but-branch-ids` — list branches with metadata
- `but-branch-commits` — show commits on a branch
- `but-diff-files` — show working tree diff
- `but-setup-branches` — create branch topology from JSON

### Existing Protocols
- PR-based agent workflow (`.github/prs/PR.md`)
- Coordinator/sub-agent hierarchy with QUESTIONS.md/ESCALATIONS.md for agent-to-agent coordination
- Dependency-encoded branch naming (`s01.s02` = s02 depends on s01)
- Sequential agent execution — one at a time, on-demand branch creation
- Work stays on the agent's branch; next agent stacks if dependency exists, otherwise branches from `feat/wasi`

---

## Roadmap

### Phase 1: Consolidate (current)
Clean workspace to single `feat/wasi` branch. Establish sequential agent workflow.

**Tasks:**
- [x] Rewrite `but` AGENT.md with proper structure
- [x] Create this PLAN.md documenting what makes a good AGENT.md
- [x] Consolidate all branches into `feat/wasi` (moved commits, deleted skeleton branches)
- [x] Update INDEX.md with sequential workflow, on-demand branches, WASI tools goal
- [ ] Move `.github/prs/` to `.mcagent/agents/pr/` (preserving PR.md, SKILLS.md)
- [ ] Create a base agent template at `.mcagent/agents/BASE_AGENT.md` that new agents inherit from
- [ ] Add `.mcagent/agents/REGISTRY.md` — index of all agents, their roles, and how to invoke them

### Phase 2: Higher-Level Abstractions
The `but` CLI works well for individual commands. Agents need higher-level workflows.

**Tasks:**
- [ ] `but-route-changes` — given a set of file changes, automatically assign them to the correct branch based on scope rules (e.g., files matching `crates/but-serde/*` go to the serde branch)
- [ ] `but-workspace-snapshot` — capture full workspace state (applied branches, pending changes, commit positions) to a JSON file for reproducibility
- [ ] `but-agent-status` — read all MEMORY.md files under a PR and produce a summary table

### Phase 3: WASI-Compatible Tooling
Replace bash scripts with WASI-compatible binaries to dogfood the WASI compilation work from PR #1.

**Tasks:**
- [ ] Evaluate which tools are candidates for WASI binaries (pure data transforms, no filesystem side effects)
- [ ] Create a `crates/but-tools/` crate with subcommands matching the current `scripts/bin/` tools
- [ ] Compile to `wasm32-wasip2` and verify they work via a WASI runtime
- [ ] Keep bash wrappers as fallback for environments without a WASI runtime

### Phase 4: Cross-Repo Agent Model
Use git submodules + the `model-c-agent` GitHub org to enable agents that work across repos.

**Tasks:**
- [ ] Define the submodule layout for cross-repo projects
- [ ] Create an agent that manages fork sync between `model-c-agent` forks and upstream
- [ ] Design the WIT interface for agent-to-agent communication (actor model)
- [ ] Prototype a QuickJS-based agent for lightweight JS tooling

---

## Open Questions

1. Should the base agent template be a markdown file that agents "read on startup", or a structured JSON/TOML config that tooling can parse?
2. How should agents handle the `.gitignore` problem — each agent's branch-specific files should be invisible to other agents but checked in on their branch?
3. What's the minimum viable WIT interface for agent communication? Start with request/response or go straight to async message passing?
