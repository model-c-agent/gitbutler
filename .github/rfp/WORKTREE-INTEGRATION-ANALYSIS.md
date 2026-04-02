# WEAVE + Worktrees: Agent Isolation via Git Worktrees

**Date**: 2026-03-29
**Status**: Analysis / Proposed Protocol Extension
**Context**: How GitButler's workspace model (many virtual branches, one working tree) integrates with WEAVE's agent model (one agent per worktree, git-based isolation)

---

## 1. The Current Gap

### GitButler's Workspace Model

GitButler manages multiple virtual branches within a **single working tree**. The `gitbutler/workspace` ref points to a merge commit that combines all applied stacks. The `but` CLI mediates all writes: `but commit`, `but push`, `but branch new`, etc. This model is powerful because it eliminates the need to switch branches -- developers see all their in-flight work simultaneously, with file ownership tracked per-hunk.

Key architectural facts from `but-workspace/src/lib.rs`:

- A **Workspace** is "the combination of one or more branches into one worktree"
- A **Stack** is a collection of pseudo-branches (heads) that contain each other
- The **Target Branch** is the integration target for all stacks
- A **DiffSpec** identifies changes at the hunk level
- The workspace ref points to a managed merge commit that `but` can rewrite at will

### WEAVE's Agent Model

WEAVE (the protocol spec) assumes agents produce `INDEX.patch` + `COMMIT.msg` artifacts and that a sole committer (the `but` orchestrator) applies them. The spec says nothing about where agents *run* -- it treats them as patch-producing black boxes.

In the current simulation (`.sim/agents/`), agents run in separate worktrees:
- `rfp.unified/.sim/agents/architect/` -- planning + memory analysis
- `rfp.unified/.sim/agents/implementer-store/` -- produced `GitRefStore` implementation

Each worktree is an independent filesystem checkout of a branch, sharing the `.git` directory with the main repository.

### The Opposition

These two models are structural opposites:

| Dimension | GitButler Workspace | WEAVE Agent Worktrees |
|-----------|--------------------|-----------------------|
| Branches per working tree | Many (virtual branches) | One (checked-out branch) |
| Write interface | `but` CLI exclusively | `git` directly (within worktree) |
| Isolation mechanism | Hunk-level ownership tracking | Filesystem isolation |
| Coordination | Implicit (workspace merge commit) | Explicit (patch handoff, PR comments) |
| `but` CLI works? | Yes (requires `gitbutler/workspace` ref + LegacyProject) | **No** (worktrees lack workspace context) |

The critical constraint: **`but` does not work in worktrees.** The `but` CLI requires the `gitbutler/workspace` branch and a `LegacyProject` path match that worktrees do not provide. This means WEAVE's Section 4 (CLI Contract: "all workspace mutations MUST use the `but` CLI") cannot be satisfied inside a worktree.

---

## 2. The Ref-Sharing Opportunity

### How Git Worktrees Share State

All worktrees linked to the same repository share:
- **Object database** (`.git/objects/`): commits, trees, blobs
- **Refs** (`.git/refs/`, `.git/packed-refs`): branches, tags, custom namespaces
- **Config** (`.git/config`)

Each worktree has its own:
- **Working tree** (checked-out files)
- **HEAD** (which branch is checked out)
- **Index** (staging area)

### Immediate Visibility of Memory Refs

The `GitRefStore` (implemented by `implementer-store`) stores memory entries under:

```
refs/but-ai/memory/<agent_id>/<state>/<entry_id>
```

These refs are stored in the shared `.git/refs/` directory. This means:

1. **Agent in worktree A** writes a memory entry: `refs/but-ai/memory/agent-a/alive/entry-123`
2. **Agent in worktree B** can immediately read it via `gix::Repository::try_find_reference("refs/but-ai/memory/agent-a/alive/entry-123")`
3. **Main workspace** can also read it -- `but-ai` modules running in the primary checkout see all refs
4. **No push/pull required** -- the ref is visible the instant it's written

This is verified by `GitRefStore`'s design: it takes a `repo_path` (any path to the repo, including a worktree path) and opens the repo via `gix::open_opts()`. Since `gix` resolves to `common_dir()` for ref operations, all worktrees see the same refs.

### What This Means

For co-located agents (all worktrees of the same repo):
- **Memory is already shared** -- no gossip protocol needed
- **Coordination messages could use refs instead of PR comments** -- faster, no network round-trip
- **Agent identity is visible repo-wide** -- `refs/but-ai/memory/<agent>/identity/self`
- **State transitions are atomic and visible** -- `edit_references` batch operations

This is the "gossip for free" insight: **shared refs ARE the gossip protocol for co-located agents.** The CRDT vector-clock gossip in WEAVE Section 5.8 is designed for cross-repo coordination. For intra-repo agents, ref visibility is instantaneous and requires no synchronization protocol.

---

## 3. The Integration Path

### Option A: `but worktree agent` (Extend Existing Crate)

The `but-worktrees` crate already implements the full lifecycle:

| Existing Function | Agent Lifecycle Step |
|-------------------|---------------------|
| `worktree_new(ctx, perm, refname)` | Spawn agent: create isolated checkout from a workspace branch |
| `worktree_list(ctx, perm)` | List active agents: enumerate worktrees with GitButler metadata |
| `worktree_integration_status(ctx, perm, id, target)` | Pre-flight check: will this agent's work conflict with the workspace? |
| `worktree_integrate(ctx, perm, id, target)` | Complete agent: cherry-pick work back, update workspace commit, remove worktree |
| `worktree_destroy_by_id(ctx, perm, id)` | Abort agent: remove worktree without integrating |

The integration path (`integrate.rs`) is particularly sophisticated:
1. Reads the worktree's HEAD and working tree state
2. Creates a squash commit of all worktree changes relative to the `base` snapshot
3. Cherry-picks this squash into the target branch within the workspace's rebase engine
4. Checks for conflicts with ALL other stacks (not just the target)
5. Updates the workspace commit
6. Removes the worktree

This is already 90% of what WEAVE needs for agent completion. The gap: it doesn't read/apply `INDEX.patch` + `COMMIT.msg` -- it squashes the entire worktree diff. But the cherry-pick mechanism works the same way.

**Advantages**: Leverages existing, tested code. Maintains GitButler's workspace integrity guarantees. Conflict detection against all stacks is built in.

**Disadvantages**: Requires `but` Context (workspace must exist). Agent still can't use `but` inside the worktree.

### Option B: Patch Relay

Agents in worktrees produce `INDEX.patch` + `COMMIT.msg` (per WEAVE Section 3). A coordinator process in the main workspace reads these artifacts and applies them via `but commit`.

```
Agent (worktree) -> writes INDEX.patch + COMMIT.msg to .sim/agents/<name>/OUTPUT/
Coordinator (main workspace) -> reads OUTPUT/, applies via `but commit --changes ...`
```

**Advantages**: Fully compliant with WEAVE Sections 3-4. Agents never need `but`. Clean separation.

**Disadvantages**: Requires a polling or notification mechanism. Patch application is indirect (agent generates diff, coordinator re-applies). Lost fidelity if the workspace state drifts between patch generation and application.

### Option C: Ref-Only Communication

Agents never touch the main workspace's working tree. They communicate exclusively via refs:

```
refs/but-ai/agent/<agent_id>/status     -- current phase, health
refs/but-ai/agent/<agent_id>/output     -- patch blob + commit message blob
refs/but-ai/memory/<agent_id>/alive/... -- knowledge entries
refs/but-ai/coordination/messages/...   -- agent-to-agent messages
```

The main workspace's `but` monitors these refs and acts on them.

**Advantages**: Zero filesystem contention. Fully atomic (ref operations are atomic). Works even if worktrees don't exist (agents could be remote).

**Disadvantages**: More complex ref management. Harder to debug than files. No existing tooling for this pattern.

### Option D: Hybrid (Recommended)

Agents use **worktrees for code changes** (filesystem isolation, can run tests, can use editors) but **refs for memory and coordination** (shared visibility, atomic operations).

```
Code Changes:   worktree filesystem -> worktree_integrate() -> workspace
Memory:         refs/but-ai/memory/... (shared across all worktrees)
Coordination:   refs/but-ai/coordination/... (intra-repo) OR PR comments (cross-repo)
Agent Status:   refs/but-ai/agent/<id>/status (visible to all)
```

This hybrid maps cleanly to the existing codebase:
- `but-worktrees` handles code isolation and integration
- `but-ai` `GitRefStore` handles memory (already works in worktrees)
- `but-ai` `GossipEngine` is used only for cross-repo sync
- `but-ai` `ForgeAdapter` is used only for cross-repo coordination

---

## 4. The Memory Bridge

### Ref Visibility Mechanics

When an agent writes a memory entry in a worktree:

```
# Agent "architect-01" in worktree at /data/worktrees/abc123/
GitRefStore::store(&entry)
  -> gix::open_opts("/data/worktrees/abc123/", opts)
  -> repo.write_blob(json_bytes)         # writes to .git/objects/
  -> repo.reference("refs/but-ai/memory/architect-01/alive/entry-xyz", blob_oid, ...)
                                          # writes to .git/refs/
```

The blob goes into the shared object database. The ref goes into the shared ref store. Both are immediately visible to:

- The main workspace (opening the repo via the primary `.git` path)
- All other worktrees (opening via their worktree paths)
- Any process that opens the repository (CI, other tools)

### Cross-Agent Memory Access

Agent A in worktree-1 writes: `refs/but-ai/memory/agent-a/alive/finding-auth-bug`

Agent B in worktree-2 can read it by constructing a `GitRefStore` for agent-a's namespace:

```rust
// Agent B reading Agent A's memories (read-only)
let agent_a_store = GitRefStore::new(repo_path, "agent-a");
let entries = agent_a_store.list(Some(MemoryState::Alive))?;
```

This works because `GitRefStore` is parameterized by `agent_id` and `repo_path`. Any agent can read any other agent's store by instantiating a `GitRefStore` with the target agent's ID.

### The Memory Bridge Design

```
                    Shared .git directory
                    =====================
                    refs/but-ai/memory/
                      architect-01/alive/...
                      implementer-01/alive/...
                      validator-01/alive/...
                    refs/but-ai/coordination/...

    +-----------+     +-----------+     +-----------+
    | Worktree  |     | Worktree  |     | Main      |
    | architect |     | impl-01   |     | Workspace |
    +-----------+     +-----------+     +-----------+
    | GitRefStore|    | GitRefStore|    | GitRefStore|
    | (own: rw) |    | (own: rw) |    | (any: rw) |
    | (others:r)|    | (others:r)|    |           |
    +-----------+     +-----------+     +-----------+
         |                 |                 |
         +----- all read/write same refs ----+
```

### Gossip Is Unnecessary for Co-Located Agents

WEAVE Section 5.8 defines a gossip protocol with vector clocks, pull requests, and CRDT merge semantics. For agents in worktrees of the same repo, this entire mechanism is redundant:

- **No vector clocks needed**: all agents see the same ref state (they share the same filesystem)
- **No pull-based sync needed**: ref visibility is immediate (write + read = consistency)
- **No merge conflict resolution needed**: ref paths are namespaced per agent, so writes don't conflict
- **CRDT properties are free**: ref operations are last-writer-wins at the individual ref level

The gossip protocol remains essential for **cross-repo** coordination (agents in different repositories, on different machines). But for co-located agents, refs provide everything gossip does, with lower latency and zero overhead.

---

## 5. Proposed Protocol Extension

### New Section: "Agent Isolation via Worktrees"

The following should be added to the WEAVE spec as **Section 9** (before Conformance Levels, which becomes Section 10):

---

#### 9.1 Worktree Model

When multiple agents operate within a single repository, they MAY be isolated using Git worktrees. Each agent runs in its own worktree, with its own working tree, HEAD, and index. All worktrees share the same object database, ref store, and configuration.

**Terminology**:

| Term | Definition |
|------|-----------|
| **Main Checkout** | The primary working tree, managed by the `but` CLI and the GitButler workspace model |
| **Agent Worktree** | A linked worktree created for a specific agent's use. Created via `but worktree new`. |
| **Co-Located Agents** | Agents that operate in worktrees of the same repository. They share refs and objects. |
| **Remote Agents** | Agents that operate in different repositories (different machines or forks). They coordinate via forge-based messaging. |

#### 9.2 Worktree Lifecycle

The lifecycle of an agent worktree maps to existing `but-worktrees` operations:

| Phase | Operation | `but-worktrees` Function |
|-------|-----------|-------------------------|
| **Spawn** | Create a worktree from a workspace branch | `worktree_new(ctx, perm, refname)` |
| **Execute** | Agent works in its worktree (read/write files, run tests, commit locally) | (Agent's own process) |
| **Pre-flight** | Check if the agent's changes can be integrated without conflicts | `worktree_integration_status(ctx, perm, id, target)` |
| **Integrate** | Cherry-pick the agent's work into the workspace | `worktree_integrate(ctx, perm, id, target)` |
| **Abort** | Destroy the worktree without integrating | `worktree_destroy_by_id(ctx, perm, id)` |

The **Spawn** phase:
1. The orchestrator selects a branch in the workspace to work on.
2. `but worktree new` creates a new worktree checked out to that branch's HEAD.
3. The worktree is created at `<project-data-dir>/worktrees/<uuid>/`.
4. Metadata is stored in `.git/worktrees/<uuid>/gitbutler-created-from` and `gitbutler-base`.
5. The `base` commit is recorded for later cherry-pick computation.

The **Integrate** phase:
1. The orchestrator runs `worktree_integration_status` to check for conflicts.
2. If `Integratable`, the orchestrator runs `worktree_integrate`.
3. Integration creates a squash commit of all worktree changes relative to `base`.
4. This commit is cherry-picked into the target branch via the workspace's rebase engine.
5. Conflicts with ALL other stacks are checked (not just the target).
6. The workspace commit is updated.
7. The worktree is removed.

#### 9.3 Ref Conventions for Co-Located Agents

Co-located agents communicate via refs in the shared `.git` directory:

```
refs/but-ai/
  memory/<agent-id>/              -- Per-agent memory (existing, Section 6)
    alive/<entry-hash>
    moribund/<entry-hash>
    deceased/<entry-hash>
    identity/self

  agent/<agent-id>/               -- Agent lifecycle state (NEW)
    status                        -- Current phase + health
    task                          -- Assigned task description
    output/latest                 -- Most recent patch output (blob)

  coordination/                   -- Intra-repo coordination (NEW)
    messages/<timestamp>-<from>   -- Coordination messages (replaces PR comments)
    locks/<resource>              -- Advisory locks on resources
```

**Status ref format** (JSON blob):

```json
{
  "agent_id": "impl-01",
  "phase": "implement",
  "health": "active",
  "tokens_used": 4200,
  "tokens_remaining": 27800,
  "started_at": "2026-03-29T14:00:00Z",
  "updated_at": "2026-03-29T14:15:00Z"
}
```

**Health values**: `spawning`, `active`, `blocked`, `completed`, `failed`, `aborted`.

#### 9.4 Worktree-to-Workspace Mapping

| WEAVE Concept | Worktree Implementation |
|--------------|------------------------|
| Agent spawning | `but worktree new` from workspace branch |
| Agent execution | Process runs in worktree directory |
| Patch production | Agent commits locally in worktree (git commits OK in worktree) |
| Patch application | `worktree_integrate` cherry-picks into workspace |
| Memory storage | `GitRefStore` (works from any worktree) |
| Memory retrieval | `GitRefStore` + `MemoryRetriever` (reads shared refs) |
| Coordination (intra-repo) | Refs under `refs/but-ai/coordination/` |
| Coordination (cross-repo) | PR comments via `ForgeAdapter` (unchanged) |
| Phase gating | Agent process enforces tool availability per phase |
| Budget tracking | Agent writes status ref with token counts |

#### 9.5 Relaxed CLI Contract for Worktrees

Within an agent worktree, the Section 4 CLI Contract is **relaxed**:

- Agents in worktrees MAY use `git add`, `git commit`, and `git checkout` within their worktree.
- Agents in worktrees MUST NOT use `git push`, `git merge`, `git rebase`, or modify refs outside their worktree's branch.
- The `but` CLI MUST be used for all operations in the **main checkout**.
- Integration back to the workspace MUST go through `but worktree integrate` (which enforces workspace consistency).

**Rationale**: The worktree is an isolated sandbox. Local git operations within it cannot affect the workspace. The `but` CLI's guarantees are enforced at the integration boundary, not within the sandbox.

#### 9.6 When Gossip Is Replaced by Refs

For co-located agents, the following WEAVE mechanisms are **simplified**:

| WEAVE Mechanism | Cross-Repo (Remote Agents) | Intra-Repo (Co-Located Agents) |
|----------------|---------------------------|-------------------------------|
| Memory sync | Gossip protocol (Section 5.8) | Ref visibility (immediate) |
| Coordination messages | PR comments (Section 5.1-5.5) | Refs under `refs/but-ai/coordination/` |
| Dependency resolution | PR-based DAG (Section 5.7) | Worktree integration order |
| Identity verification | Signed commits (Section 7.4) | Ref namespace ownership |

Implementations SHOULD detect whether agents are co-located (same `.git` directory) and skip gossip/forge-based coordination in favor of direct ref access.

---

## 6. What Changes in but-ai

### GitRefStore: Already Works in Worktrees

The `GitRefStore` implementation (produced by `implementer-store`) takes a `repo_path: PathBuf` and opens the repository via `gix::open_opts()`. When opened from a worktree path, `gix` automatically resolves to the `common_dir()` for object and ref operations. This means:

- `GitRefStore::new("/path/to/worktree", "agent-id")` works correctly
- Blobs are written to the shared object database
- Refs are written to the shared ref store
- No code changes needed

### GossipEngine: Optional for Co-Located Agents

The `GossipEngine` (WEAVE Section 5.8) uses vector clocks and pull-based sync. For co-located agents:

- **Skip gossip entirely**: refs are immediately visible
- **Keep gossip for cross-repo**: when agents span multiple repositories
- The `GossipEngine` should detect co-location (same `common_dir()`) and short-circuit to direct ref reads

Proposed interface change:

```rust
impl GossipEngine {
    /// Returns true if the target agent is co-located (same repo).
    /// If co-located, gossip is unnecessary -- direct ref access suffices.
    pub fn is_co_located(&self, peer_agent_id: &AgentId) -> bool {
        // Check if peer's refs exist in our ref store
        // (co-located agents share refs, remote agents don't)
        self.ref_store.agent_has_refs(peer_agent_id)
    }

    /// Sync with a peer. Short-circuits for co-located agents.
    pub fn sync(&mut self, peer: &AgentId) -> Result<SyncResult> {
        if self.is_co_located(peer) {
            return Ok(SyncResult::AlreadySynced);  // refs are shared
        }
        self.full_gossip_sync(peer)  // cross-repo: use vector clocks
    }
}
```

### ForgeAdapter: Cross-Repo Only

The `ForgeAdapter` trait (Section 5.6) provides `create_pr`, `comment`, `list_comments`, etc. For co-located agents, these forge operations are not needed for coordination -- refs replace PR comments.

The `ForgeAdapter` remains necessary for:
- Creating the final PR (agent's work -> upstream)
- Cross-repo dependency declaration
- Publishing coordination messages to external agents

### Phase Gating: Unchanged

Phase-gated tool loading (Section 8.1) applies regardless of isolation mechanism. Whether an agent is in a worktree or producing patches via the main checkout, it MUST only use tools available in its current phase.

The worktree model does not change the phase model. It changes the **transport** (refs vs. PR comments) and the **write mechanism** (local git in worktree vs. `but` in main checkout), but not the **lifecycle** (classify -> plan -> implement -> validate -> catalog -> coordinate).

---

## 7. Concrete Proposal: Agent Lifecycle

### Full Lifecycle Sequence

```
1. ORCHESTRATOR (main workspace):
   but worktree new <branch-name>
   -> creates worktree at <data-dir>/worktrees/<uuid>/
   -> writes refs/but-ai/agent/<agent-id>/status = "spawning"
   -> writes refs/but-ai/agent/<agent-id>/task = <task blob>

2. AGENT (in worktree):
   a. Reads AGENT.md (role, capabilities, authorization)
   b. Reads TASK.md (task description, target files, complexity)
   c. Opens GitRefStore(worktree_path, agent_id)
   d. CLASSIFY: retrieves relevant memories from refs
   e. PLAN: designs approach, estimates tokens
   f. IMPLEMENT: edits files, runs tests, commits locally
   g. VALIDATE: checks continuity, detects contradictions
   h. CATALOG: stores new memory entries via GitRefStore
      -> refs/but-ai/memory/<agent-id>/alive/<entry>
      (visible immediately to all other agents and the workspace)
   i. COORDINATE: writes status ref
      -> refs/but-ai/agent/<agent-id>/status = "completed"
      -> refs/but-ai/agent/<agent-id>/output/latest = <patch blob>

3. ORCHESTRATOR (main workspace):
   a. Detects status change (polls refs or watches filesystem)
   b. Runs worktree_integration_status(ctx, perm, id, target)
   c. If Integratable:
      worktree_integrate(ctx, perm, id, target)
      -> cherry-picks agent's commits into workspace
      -> updates workspace commit
      -> removes worktree
   d. If CausesWorkspaceConflicts:
      Notifies agent or human for conflict resolution
   e. Creates PR if configured:
      ForgeAdapter::create_pr(...)
```

### Memory Visibility Model

| Event | Visibility | Latency |
|-------|-----------|---------|
| Agent writes memory entry (ref) | All co-located agents + main workspace | Immediate (fsync) |
| Agent writes memory entry (ref) | Remote agents (other repos) | Requires gossip or push |
| Agent transitions entry state | All co-located agents | Immediate |
| Agent reads another agent's memory | Direct ref access | Immediate |
| Orchestrator reads agent status | Direct ref access | Immediate |

### When to Use Each Isolation Mechanism

| Scenario | Mechanism | Reason |
|----------|-----------|--------|
| Single agent, simple task | No worktree; patch relay via main workspace | Overhead of worktree creation not justified |
| Multiple agents, same repo | Worktrees (one per agent) | Filesystem isolation prevents contention |
| Agent needs to run tests | Worktree | Tests require a clean working tree |
| Agent only produces analysis/memory | No worktree; ref-only | No code changes, just knowledge production |
| Cross-repo coordination | Forge-based (PR comments) | Refs are not shared across repos |
| Long-running agent (hours) | Worktree | Isolation prevents blocking the workspace |
| Agent modifying conflicting files | Worktrees with integration check | `worktree_integration_status` catches conflicts early |

### Mapping to but-worktrees Crate

The existing `but-worktrees` crate provides the primitives. What's missing:

| Missing Piece | Description | Where to Add |
|--------------|-------------|-------------|
| Agent-aware metadata | Store `agent_id` in worktree metadata (alongside `created_from_ref` and `base`) | `db.rs`: add `gitbutler-agent-id` file |
| Status ref management | Create/update `refs/but-ai/agent/<id>/status` on lifecycle events | New module in `but-ai` or `but-worktrees` |
| Task assignment | Write task description to `refs/but-ai/agent/<id>/task` before spawning | Orchestrator logic |
| Output collection | Read agent's output ref after completion, before integration | Orchestrator logic |
| Co-location detection | Determine if two agents share a `.git` directory | `GitRefStore` helper |

### Integration with `but` CLI

New `but` subcommands (extending the existing `but worktree` commands):

```bash
# Existing (already in but-worktrees crate):
but worktree new <branch>              # Create worktree for agent
but worktree list                      # List active agent worktrees
but worktree integrate <id> <branch>   # Integrate agent work back
but worktree destroy <id>              # Abort agent

# Proposed new commands:
but worktree status <id>               # Read agent status ref
but worktree assign <id> <task-file>   # Write task to agent's ref
but worktree output <id>               # Read agent's output ref
```

---

## 8. Summary of Key Insights

1. **Worktrees and virtual branches are complementary, not contradictory.** Virtual branches manage multiple lines of work within a single checkout. Worktrees provide isolated checkouts for parallel execution. The integration boundary (`worktree_integrate`) connects them.

2. **Shared refs are the gossip protocol for co-located agents.** The CRDT gossip mechanism (vector clocks, pull-based sync) is designed for distributed systems. Co-located agents in worktrees of the same repo get instant ref visibility for free. Gossip should be reserved for cross-repo coordination.

3. **`GitRefStore` already works in worktrees.** The memory store implementation uses `gix::open_opts()` which resolves to `common_dir()` for ref/object operations. No changes needed.

4. **The `but` CLI contract should be relaxed within worktrees.** Agents can use `git` locally within their worktree sandbox. The workspace consistency guarantees are enforced at the integration boundary by `worktree_integrate`, which checks against all stacks.

5. **The existing `but-worktrees` crate is 90% of the agent spawning infrastructure.** `worktree_new`, `worktree_integrate`, `worktree_integration_status`, and `worktree_destroy` map directly to agent lifecycle phases. The missing 10% is agent-specific metadata and status tracking.

6. **Coordination simplifies dramatically for co-located agents.** No PR comments needed. No forge adapter needed. No dependency DAG needed. Refs replace all of these. The full coordination protocol (Section 5) is only needed for cross-repo agent interactions.

7. **The hybrid model (worktrees for code, refs for memory/coordination) is the natural design.** It leverages the strengths of both isolation mechanisms and maps cleanly to the existing codebase.
