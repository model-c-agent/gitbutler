# Brigade de Cuisine Tactique -- Technical Proposal

**RFP:** `but ai` Plugin for GitButler CLI v1.0.0
**Organization:** Brigade de Cuisine Tactique (Org 062)
**Domain:** Culinary Arts | **Philosophy:** Military Precision
**Service Order:** BCT-2026-001

---

## Executive Summary

The Brigade de Cuisine Tactique proposes a `but-ai` plugin built on the principle of
*mise en place*: everything in its place before service begins. Our architecture mirrors a
professional kitchen brigade -- a strict hierarchy of specialized stations coordinated by an
expeditor, where every output is inspected at the pass before it leaves the kitchen.

Our core contribution is **mise en place memory** -- a memory system where all knowledge is
pre-staged in named containers organized for instant retrieval during execution. Like a
chef's prep station, where every ingredient is portioned, labeled, and within arm's reach,
mise en place memory eliminates search during the hot loop. The agent does not query; the
agent reaches.

---

## 1. Plugin Architecture (RFP 3.1)

### Approach

`but-ai` is a standalone Rust binary structured as a kitchen with stations. Each module is
a station with a defined responsibility, a defined interface to the pass (the coordination
layer), and a defined set of callouts for requesting ingredients from other stations.

### Design

```
but ai
  +-- fire       Execute a task (autonomous agent mode)
  +-- mcp        Start MCP server on stdio
  +-- prep       Manage mise en place memory
  +-- identity   Agent identity and key management
  +-- inspect    Show current service state (debug)
  +-- timing     Show token budget and timing
```

The "fire" subcommand name follows kitchen convention: "fire table seven" means "begin
cooking table seven's order now."

### Crate Structure

```
crates/but-ai/
  src/
    main.rs              -- CLI entry, env var parsing (BUT_WORKSPACE_DIR, etc.)
    pass/
      quality.rs         -- Output inspection (the pass)
      assembly.rs        -- Patch assembly from station outputs
    mcp/
      server.rs          -- ServerHandler impl (rmcp-compatible, drop-in replacement)
      tools.rs           -- WorkspaceToolset registration bridge
    stations/
      saucier.rs         -- Provider bridge (wraps but-llm)
      garde.rs           -- Mise en place memory engine
      rotisseur.rs       -- Agent execution loop
      tournant.rs        -- Forge adapter + cross-repo coordination
    expeditor/
      sous.rs            -- Task decomposition and station coordination
      timing.rs          -- Token budget tracking and enforcement
      board.rs           -- Service board (shared state)
    identity/
      wallet.rs          -- OpenWallet integration
      registry.rs        -- Agent identity containers
      auth.rs            -- Authorization policy (branch/repo/size constraints)
```

### WASI Degradation

| Feature | Native | WASI |
|---------|--------|------|
| Plugin discovery | PATH-based (full) | Disabled (`#[cfg(not(feature = "wasi"))]`) |
| Provider access | All 4 + plugins | Local providers only (if wasi:sockets available) |
| Memory | Full mise en place | Read-only containers (no Git branch writes) |
| Forge coordination | Full | Disabled (no HTTP) |
| Patch production | Full | Full |
| MCP server | Full | Full (stdio is WASI-native) |

The Brigade's degradation philosophy: a cook without a full kitchen can still prep.
Under WASI, the agent operates in "prep mode" -- it can read memory, plan tasks, and
produce patches, but it cannot coordinate across repos or access remote providers.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Monolithic binary with all stations compiled together | This IS the approach. The Brigade values a single deployable unit. One binary, one kitchen. |
| Separate binaries per station | Adds IPC overhead. In a kitchen, stations communicate by voice, not by mail. In-process communication is faster and more reliable. |
| Dynamic loading of station modules | Unnecessary complexity for v1. The Brigade can deliver all six stations in a single binary under 10MB. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

The Saucier station wraps `but-llm` without modification. The sauce is the foundation --
you do not change the base; you build on it. Saucier adds: token counting per call,
capability detection per provider, and a plugin protocol for new providers.

### Design

```rust
/// The sauce station: provider bridge with accounting
pub struct Saucier {
    provider: LLMProvider,
    timing: Arc<TimingBoard>,
    capabilities: StationCapabilities,
}

pub struct StationCapabilities {
    pub tool_calling: bool,
    pub streaming: bool,
    pub structured_output: bool,
    pub max_context: usize,
}

impl Saucier {
    /// Build from Git config (delegates to LLMProvider::from_git_config)
    pub fn from_git_config(config: &gix::config::File) -> Result<Self>;

    /// Fire: execute tool-calling loop with timing
    pub fn fire_tools(
        &self, system: &str, messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
    ) -> Result<(String, ServiceTiming)>;

    /// Fire: streaming response with per-token callback
    pub fn fire_stream(
        &self, system: &str, messages: Vec<ChatMessage>,
        on_token: impl Fn(&str),
    ) -> Result<(String, ServiceTiming)>;
}
```

### Provider Plugin Protocol

New providers (Gemini, Mistral, etc.) are PATH-discoverable executables (`but-ai-provider-*`)
communicating via JSON-RPC on stdio:

```
CALL -> {"method": "capabilities"} -> {"tool_calling": true, ...}
CALL -> {"method": "complete", "params": {...}} -> {"content": "...", "usage": {...}}
```

Adding a new provider requires no recompilation. The Brigade's rule: you should be able to
add a new ingredient (provider) to the pantry without rebuilding the kitchen.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| New LLM abstraction layer | `but-llm` is already abstracted. Adding another layer is like reducing a reduction -- you lose flavor for no gain. |
| gRPC for provider plugins | Over-engineered. JSON-RPC on stdio is the simplest thing that works. The Brigade values speed of service over architectural elegance. |

---

## 3. The But Agent (RFP 3.3)

### Approach

The agent execution loop (`but ai fire`) follows kitchen service protocol:

```
PRE-SERVICE (mise en place)
  1. Read task description (PR body, issue, CLI arg)
  2. Load mise en place containers (memory, context, tool descriptions)
  3. Decompose task into station orders (Sous)

SERVICE (execution)
  4. Fire stations in dependency order
  5. Each station: select tool -> call tool -> process result -> report
  6. Track timing (token budget) per station
  7. Assemble outputs at the pass (Sous)

POST-SERVICE (inspection)
  8. Quality inspection (Chef)
  9. Produce INDEX.patch + COMMIT.msg
  10. Update mise en place with learned context
```

### Task Sources

```
but ai fire --order "implement feature X"          # Direct order (CLI)
but ai fire --ticket 42                            # From PR #42
but ai fire --issue 17                             # From issue #17
but ai fire --prep-only                            # Mise en place only (no execution)
```

### Patch Production

The agent produces patches at the pass (assembly point):

```rust
pub struct ServiceOutput {
    pub index_patch: String,       // Unified diff
    pub commit_msg: String,        // Conventional commit
    pub service_order: String,     // BCT tracking number
    pub timing: ServiceTiming,     // Token usage per station
    pub stations_complete: Vec<StationReport>,
    pub stations_remaining: Vec<StationOrder>,
}
```

Partial service: if a station exhausts its budget, it reports "86" (out of that item). Sous
produces a partial patch from completed stations and notes the remaining orders in the
commit message.

### Branch Naming

```
brigade/<agent-station>/<service-order>[.dep-<order>]
```

Example: `brigade/rotisseur/bct-042.dep-bct-039` -- Rotisseur working on service order 042,
which depends on order 039.

### WorkspaceToolset Integration

All ten workspace tools are registered as "ingredients" available to all stations:

```rust
let mut toolset = WorkspaceToolset::new(ctx);
// 10 tools registered: Commit, CreateBranch, Amend, SquashCommits,
// GetProjectStatus, MoveFileChanges, GetCommitDetails, GetBranchChanges,
// SplitBranch, SplitCommit

saucier.fire_tools(system_prompt, messages, &mut toolset)?;
```

Each station uses only its designated tools (enforced by policy, not by code). The Brigade
trusts discipline over locks.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Free-form ReAct agent | Unpredictable timing. The Brigade does not improvise during service. |
| Multi-agent in one process | One agent per station, one process per service. Complexity stays in coordination (Sous), not concurrency. |
| Direct file editing | Violates the patch-based workflow. The Brigade's rule: nothing leaves the kitchen that has not passed inspection. Direct edits bypass the pass. |

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is the Tournant's station. PRs are service orders that travel
between kitchens (repositories). Each PR carries structured messages in the Brigade's
kitchen order format.

### Forge Adapter (Tournant Interface)

```rust
pub trait ForgeKitchen: Send + Sync {
    fn open_ticket(&self, repo: &KitchenRef, order: &ServiceOrder) -> Result<TicketId>;
    fn call_order(&self, ticket: &TicketId, msg: &KitchenMessage) -> Result<()>;
    fn read_orders(&self, ticket: &TicketId) -> Result<Vec<KitchenMessage>>;
    fn mark_ticket(&self, ticket: &TicketId, label: &str) -> Result<()>;
    fn find_tickets(&self, repo: &KitchenRef, filter: &TicketFilter) -> Result<Vec<TicketId>>;
}
```

The interface is intentionally small. Only operations that every forge supports (create PR,
comment, label, search) are included. The GitHub adapter is the reference implementation.

### Structured Comment Schema (BCT-Agent-V1)

```json
{
  "$schema": "bct-agent-v1",
  "order_type": "FIRE | READY | REFIRE | 86 | BEHIND | STATUS",
  "station": "rotisseur",
  "sender": {
    "agent": "rotisseur",
    "org": "brigade-de-cuisine-tactique",
    "did": "did:key:z6Mk..."
  },
  "receiver": "@sous | @all | @chef",
  "order": {
    "description": "Generate authentication module patch",
    "timing": {"budget": 38000, "used": 12000},
    "dependencies": ["other-org/other-repo#42"]
  },
  "acknowledge": "required | optional",
  "service_order": "BCT-2026-001"
}
```

The order types map to kitchen communications:
- **FIRE:** Begin work on this task
- **READY:** Station work complete, awaiting pass inspection
- **REFIRE:** Work rejected at pass, redo with corrections
- **86:** Task cannot be completed (blocked, out of budget, infeasible)
- **BEHIND:** Station is running behind timing (warning)
- **STATUS:** Status report (no action required)

Comments are wrapped in code fences:

````
```but-agent
{ ... JSON ... }
```
````

### Cross-Repo Dependencies

Dependencies are "coordination tickets" -- orders that span multiple kitchens:

```
Kitchen A (repo A): PR #42 -- FIRE: build auth module
Kitchen B (repo B): PR #17 -- FIRE: build auth tests (depends on A#42)
```

Tournant tracks these dependencies as a DAG. A dependency is resolved when the upstream PR
is merged or when the upstream agent posts a READY message with a patch handoff. Cycles are
detected on insertion and rejected with a structured error.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Webhook-based event system | Requires infrastructure. The Brigade needs only a kitchen and raw ingredients (Git + forge API). |
| Git notes | Not visible in forge UIs. PR comments are visible to both agents and humans. |
| Custom message bus | Violates "no proprietary dependencies." The forge IS the message bus. |

---

## 5. Agent Memory and Identity (RFP 3.5)

### Mise en Place Memory

Mise en place is the French culinary term for "everything in its place." Before service
begins, every ingredient is washed, cut, portioned, and placed in a labeled container at
the cook's station. The cook does not search for ingredients during service. The cook
reaches.

Mise en place memory applies this principle to agent knowledge. All memories are pre-staged
in named containers, organized by category, and indexed for instant retrieval. There is no
search at query time. The search happened during prep.

### Core Concepts

| Kitchen Concept | Memory Equivalent | Description |
|----------------|-------------------|-------------|
| **Container** | Named memory bucket | A labeled vessel holding related memories |
| **Portion** | Individual memory entry | A single fact, pattern, or observation |
| **Station Layout** | Container index | Map of containers by category and priority |
| **Prep List** | Context plan | List of containers needed for a specific task |
| **Walk-in** | Archive | Cold storage for containers not needed for current service |
| **Pantry** | Long-term shared memory | Ingredients available to all agents across sessions |

### Storage Layout

```
refs/but-ai/mise/<agent-id>/
  layout.json                    -- Station layout (container index)
  containers/
    auth-patterns/
      portions.json              -- [{id, content, freshness, priority}]
      meta.json                  -- {name, category, portion_count, last_prepped}
    patch-workflow/
      portions.json
      meta.json
    token-limits/
      portions.json
      meta.json
  prep-lists/
    default.json                 -- Containers to load for any task
    security-task.json           -- Containers to load for security-related tasks
    coordination-task.json       -- Containers for cross-repo coordination
  walk-in/
    deprecated-patterns/         -- Archived containers (expired but preserved)
      portions.json
      meta.json
  pantry/                        -- Shared long-term memory (read-only view)
    index.json
```

### Container Schema

```json
{
  "name": "auth-patterns",
  "category": "security",
  "portions": [
    {
      "id": "p-001",
      "content": "JWT with refresh tokens is the standard auth pattern in this codebase",
      "tags": ["auth", "jwt", "security"],
      "prepped": "2026-03-28T10:00:00Z",
      "freshness": "fresh",
      "priority": "high",
      "ttl": "30d",
      "use_count": 14
    }
  ],
  "last_prepped": "2026-03-28T10:00:00Z",
  "total_portions": 3
}
```

### Retrieval: The Reach, Not The Search

When the agent needs memory during execution, it does NOT formulate a query and search.
Instead:

1. **Before service:** Garde prepares a **prep list** for the task. The prep list is a set
   of container names determined by task analysis (tags from the task description are
   matched to container categories).
2. **During service:** The agent **reaches** for a named container. Retrieval is a direct
   lookup by container name -- O(1), not O(n).
3. **If a container is missing:** The agent escalates to Garde, who checks the walk-in
   (archive) and the pantry (shared memory). This is expensive (requires a sub-query) and
   is treated as an exception, not the normal path.

This is fundamentally different from search-based memory:

| Approach | During Service | Latency | Token Cost |
|----------|---------------|---------|-----------|
| Vector search | Formulate query, compute similarity, rank results | O(n) | ~300 tokens per query |
| Graph traversal | Identify entry points, BFS/DFS, score paths | O(V+E) | ~200 tokens per query |
| **Mise en place** | Reach for named container | **O(1)** | **~50 tokens (name lookup)** |

The trade-off: mise en place requires prep time (Garde must prepare containers before
service). But the Brigade's philosophy is that prep time is never wasted -- it is invested.

### Relevance Scoring

Relevance is determined at prep time, not query time:

1. **Task analysis:** Extract tags from the task description
2. **Container matching:** Match tags to container categories
3. **Priority ordering:** Within matched containers, sort portions by:
   - Priority (high/medium/low, set by Garde based on historical use)
   - Freshness (fresh > aging > stale)
   - Use count (frequently used > rarely used)
4. **Prep list generation:** The top-K containers are added to the prep list, ordered by
   relevance to the current task

### TTL and Expiration

Portions have freshness states:

| State | Age vs. TTL | Location | Retrieval |
|-------|-------------|----------|-----------|
| **Fresh** | < 50% | Active container | Instant (O(1)) |
| **Aging** | 50-80% | Active container | Instant, flagged as aging |
| **Stale** | 80-100% | Active container | Instant, flagged as stale (use with caution) |
| **Expired** | > 100% | Walk-in (archive) | Requires explicit request |
| **Pinned** | N/A | Active container | Never expires (identity, core patterns) |

### Compaction Survival

When the LLM context window is compacted:

1. **Prep lists survive.** They are small (just container names) and essential for
   rehydration.
2. **Pinned portions survive.** They are marked as permanent and always included.
3. **Container summaries survive.** Each container has a one-line summary in `meta.json`.
   These summaries form a "station layout" that fits in ~400 tokens.
4. **After compaction:** The agent reloads from the prep list, reaching for containers
   as needed. The containers are in Git; they were never in the context window. Only the
   *prepped portions* were in context, and those are regenerated from the containers.

### Long-Term Storage (Pantry)

The pantry is a shared branch readable by all agents:

```
refs/but-ai/mise/shared/pantry/
  index.json             -- Global container index
  orgs/
    brigade-de-cuisine-tactique/
      containers/
        ...
    other-org/
      containers/
        ...
```

Any agent can contribute containers to the pantry. The index maps container names to
locations. Cross-org retrieval follows the same O(1) pattern: reach for the named container
in the pantry.

### Identity

Agent identity is a special container in mise en place:

```json
{
  "name": "identity",
  "category": "core",
  "portions": [
    {
      "id": "identity",
      "content": {
        "name": "rotisseur",
        "org": "brigade-de-cuisine-tactique",
        "station": "roast",
        "capabilities": ["agent-loop", "tool-calling", "patch-generation"],
        "authorization": {
          "branches": ["brigade/rotisseur/*", "feat/*"],
          "max_patch_lines": 1000,
          "repos": ["gitbutler/gitbutler"]
        },
        "signing_key": "openwallet:did:key:z6Mk..."
      },
      "freshness": "pinned",
      "priority": "critical"
    }
  ]
}
```

---

## 6. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent commit is signed via OpenWallet. The signing key is stored in the agent's
identity container (pinned, never expires).

### Key Management

```
but ai identity prep --station rotisseur --org brigade-de-cuisine-tactique
  -> Creates OpenWallet DID: did:key:z6Mk...
  -> Stores in identity container
  -> Registers on refs/but-ai/identity/
```

### Authorization Model

The Brigade's authorization model mirrors kitchen hierarchy:

```json
{
  "org": "brigade-de-cuisine-tactique",
  "hierarchy": {
    "chef": {
      "agents": ["chef"],
      "authority": {
        "branches": ["*"],
        "max_patch_lines": 5000,
        "can_approve": true,
        "can_refire": true
      }
    },
    "sous": {
      "agents": ["sous"],
      "authority": {
        "branches": ["brigade/*", "feat/*", "fix/*"],
        "max_patch_lines": 2000,
        "can_approve": true,
        "can_refire": true
      }
    },
    "station": {
      "agents": ["saucier", "garde", "rotisseur", "tournant"],
      "authority": {
        "branches": ["brigade/<self>/*", "feat/*"],
        "max_patch_lines": 1000,
        "can_approve": false,
        "can_refire": false
      }
    }
  }
}
```

### Verification Chain

1. Extract DID from commit signature
2. Look up agent identity in `refs/but-ai/identity/`
3. Determine agent's rank in the hierarchy
4. Verify: branch permission, patch size, authority level
5. Verify: the commit was approved (signature from Chef or Sous present in commit metadata)

The Brigade adds a unique requirement: **station commits must be counter-signed by Sous or
Chef.** A commit from Rotisseur alone is insufficient -- it must have passed the pass.
This is enforced by requiring a co-signature (Sous signs the assembled patch, proving it
was inspected).

### Key Lifecycle

| Event | Kitchen Protocol |
|-------|-----------------|
| **Provisioning** | "New cook" -- DID created, identity container prepped |
| **Rotation** | "Key change" -- new key, rotation commit signed with old key |
| **Revocation (routine)** | "End of service" -- key retired, commits remain valid |
| **Revocation (compromise)** | "Contamination" -- key quarantined, all commits flagged for review, re-inspection of all outputs signed with compromised key |

The contamination protocol is the Brigade's unique contribution: when a key is compromised,
every commit signed with that key is treated as potentially contaminated, just as a kitchen
treats every dish from a contaminated station as suspect.

---

## 7. Token Budget (RFP 3.7)

### Budget Table (Frontier Model: Claude Opus)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,400 | 0 | Once per session | Identity container, tool descriptions (10 tools), station layout (mise en place index), workspace state. |
| **Task ingestion** | 2,400 | 250 | Once per task | Read PR body / issue / CLI order. Output: structured service order. |
| **Planning (Sous)** | 2,000 | 700 | Once per task | Decompose into station orders, assign timing, create prep list. |
| **Mise en place** | 800 | 100 | Once per task | Load prep list containers. Low cost because containers are pre-organized. |
| **Tool call (per call)** | 1,100 | 350 | ~8 per task | Parameter formulation (200 out) + result processing (900 in). |
| **Patch generation** | 2,000 | 3,600 | Once per task | Accumulated station results (2,000 in). Unified diff (3,600 for 200 lines). |
| **Commit message** | 500 | 150 | Once per task | Terse conventional commit. The Brigade wastes no words. |
| **Memory retrieval** | 200 | 50 | 2 per task | O(1) container reach. Minimal: just the container name + load. |
| **Coordination event** | 1,300 | 400 | 2 per task | Read PR orders (1,000 in) + post kitchen order (400 out). |
| **Pass inspection** | 1,500 | 300 | Once per task | Chef reviews assembled output. May trigger refire (adds ~3,000 tokens). |
| **TOTAL (typical task)** | **25,200** | **9,550** | -- | 200-line feature, 3 files, 2 cross-repo deps, 8 tool calls, 2 memory retrievals, 2 coordination events, 1 pass inspection. |

**Grand total: ~34,750 tokens per typical task.**

### Justification

- **Mise en place at 800+100 tokens:** This is the Brigade's efficiency advantage. Because
  containers are pre-organized and loaded by name (not searched), memory costs are minimal.
  Compare to search-based systems at ~600+150 per retrieval.
- **Memory retrieval at 200+50 per retrieval:** O(1) lookup by container name. The tag
  matching happened at prep time, not query time. This is 3-4x cheaper than search-based
  retrieval.
- **Pass inspection at 1,500+300:** The quality gate costs tokens. The Brigade accepts this
  cost because it prevents more expensive failures downstream (a bad patch that must be
  reverted costs far more than a refire that catches it at the pass).
- **8 tool calls:** GetProjectStatus (1) + GetBranchChanges (2) + CreateBranch (1) +
  MoveFileChanges (1-2) + Commit (1) + GetCommitDetails (1) = 7-8 calls.

### Timing Enforcement

The Brigade tracks timing per station:

```
Station     | Budget | Fired  | Ready  | Status
------------+--------+--------+--------+---------
Garde       | 5,000  | 4,200  | --     | READY
Saucier     | 4,000  | 3,800  | --     | READY
Rotisseur   | 15,000 | 12,400 | --     | IN SERVICE
Tournant    | 6,000  | 3,200  | --     | IN SERVICE
Sous        | 8,000  | 5,100  | --     | EXPEDITING
Chef        | 3,000  | 0      | --     | AWAITING PASS
```

Alerts: **BEHIND** at 70% (station pacing warning). **86** at 90% (station halts, produces
what it has). Reserve 5% held for guaranteed partial output and structured error.

---

## 8. Testing Strategy (RFP 4.5)

### Provider-Agnostic Testing

- **Mock Saucier:** A `MockProvider` that returns deterministic responses. All integration
  tests use the mock. No live API calls in CI.
- **Provider tasting:** Each provider is tested against a canonical "tasting menu" of
  requests. The tasting menu covers: tool calling, streaming, structured output, error
  responses, timeout handling.
- **Replay service:** Recorded production sessions (request/response pairs) are replayed
  in CI to detect regressions.

### Patch Workflow (Kitchen Round-Trip)

- **Full service test:** Create a known workspace. Run `but ai fire` on a known order.
  Apply the patch. Verify the result. This is the kitchen equivalent of "cook the dish,
  taste the dish."
- **86 test:** Simulate budget exhaustion at each station. Verify partial patch validity.
- **Refire test:** Simulate Chef rejecting the patch. Verify that the refire produces a
  corrected output within the remaining budget.
- **Conflict test:** Apply a patch to a dirty workspace. Verify structured error and
  workspace integrity.

### Cross-Repo Coordination Testing

- **Mock kitchen:** A `MockForgeKitchen` that simulates PR lifecycle in memory.
- **Order format validation:** All structured messages validated against BCT-Agent-V1 schema.
- **Dependency DAG tests:** Cycle detection, topological ordering, readiness checking.
- **Multi-kitchen simulation:** Simulate two agents in two repos exchanging orders and
  patches via the mock forge.

### Token Budget Testing

- **Timing precision:** Mock provider returns exact token counts. Verify BEHIND and 86
  alerts fire at correct thresholds.
- **Budget overflow:** Set budget to N. Send an order requiring N+1. Verify graceful 86
  (valid partial output + structured error).
- **Station isolation:** Verify that one station exceeding budget does not corrupt another
  station's output.

---

## 9. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum tokens per service |
| `but-ai.mise.branch` | string | `refs/but-ai/mise/<agent-id>` | Git ref for mise en place |
| `but-ai.mise.maxContainers` | integer | 200 | Maximum active containers per agent |
| `but-ai.mise.maxPortions` | integer | 50 | Maximum portions per container |
| `but-ai.mise.defaultTtl` | string | `30d` | Default TTL for portions |
| `but-ai.mise.prepBudgetPct` | float | 0.15 | Budget % allocated to mise en place |
| `but-ai.coordination.schema` | string | `bct-agent-v1` | Structured comment schema |
| `but-ai.coordination.forge` | string | `github` | Default forge adapter |
| `but-ai.identity.wallet` | string | (required) | OpenWallet endpoint URL |
| `but-ai.timing.behindPct` | float | 0.70 | BEHIND warning threshold |
| `but-ai.timing.eightySixPct` | float | 0.90 | 86 (halt) threshold |
| `but-ai.timing.reservePct` | float | 0.05 | Reserve for partial output |
| `but-ai.pass.coSignRequired` | bool | true | Require Sous/Chef co-signature |
| `but-ai.provider.pluginPath` | string | (PATH) | Additional provider plugin search |

---

## 10. Migration Path

The Brigade's migration follows the kitchen's "menu transition" protocol. You never change
the menu mid-service. You prep the new menu, run it in parallel, verify it, then switch.

| Phase | Action | Verification |
|-------|--------|-------------|
| 1. **Parallel prep** | `but-ai mcp` exposes identical `gitbutler_update_branches`. Old and new run side-by-side. | Diff outputs for identical inputs. |
| 2. **Expanded menu** | New tools added. Legacy tool unchanged. | Existing clients unaffected. |
| 3. **Last call** | Legacy tool deprecated (warning in response). | One release cycle notice. |
| 4. **New menu** | Legacy tool removed. `but-ai mcp` is canonical. | Full test suite green. |

---

*[PASS] SERVICE ORDER BCT-2026-001 COMPLETE. Proposal filed by Brigade de Cuisine Tactique.*
