# The Metropolitan Mobility Lab -- Technical Proposal

**RFP:** `but ai` Plugin for GitButler CLI v1.0.0
**Organization:** The Metropolitan Mobility Lab (Org 053)
**Domain:** Public Transit | **Philosophy:** Academic Research Lab
**Simulation Run:** Bahnhofstrasse-003

---

## Executive Summary

The Metropolitan Mobility Lab proposes a `but-ai` plugin grounded in the same methodology
we apply to transit simulation: model the system, simulate its behavior, validate against
ground truth, and iterate. Our central contribution is **digital-twin memory** -- a living
simulation of the agent's knowledge state where memories are not static entries but entities
in a dynamic model that updates with each observation. This allows the agent to answer not
just "what do I know?" but "what would happen if I applied this knowledge to the current
task?" -- the same question a transit digital twin answers when a city asks "what happens
if we add a bus line?"

---

## 1. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a Rust crate (`crates/but-ai/`) within the existing workspace.
It follows the PATH-based plugin contract: when placed on PATH, `but` discovers it via
`find_external_subcommand()`. The binary reads `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`,
and `BUT_JSON` from environment variables.

### Design

The plugin is structured as a simulation system with an observation layer (inputs), a model
layer (processing), and an actuation layer (outputs):

```
Observation Layer:    CLI args | MCP requests | PR bodies | Git state
                           |
Model Layer:          Task Model -> Agent Model -> Memory Model
                           |
Actuation Layer:      INDEX.patch | COMMIT.msg | PR comments | Memory updates
```

Subcommands:

```
but ai
  +-- run         Execute a task (autonomous agent mode)
  +-- mcp         Start MCP server on stdio
  +-- twin        Query / manage the digital-twin memory
  +-- identity    Agent identity and key management
  +-- observe     Show current model state (debug/introspection)
  +-- validate    Run validation suite against a task log
```

### Crate Structure

```
crates/but-ai/
  src/
    main.rs              -- CLI entry, environment variable parsing
    mcp/
      server.rs          -- ServerHandler impl (rmcp-compatible)
      tools.rs           -- WorkspaceToolset bridge
    agent/
      runner.rs          -- Agent execution loop (state machine)
      planner.rs         -- Task decomposition and step planning
      patcher.rs         -- INDEX.patch + COMMIT.msg synthesis
    twin/
      engine.rs          -- Digital twin simulation engine
      entity.rs          -- Memory entity types
      observation.rs     -- Observation ingestion (new data -> twin update)
      query.rs           -- Query engine (relevance via simulation)
      storage.rs         -- Git-branch persistence
    provider/
      bridge.rs          -- Wraps but-llm without modification
      discovery.rs       -- Provider plugin discovery (PATH-based)
      capability.rs      -- Provider capability detection
    coordination/
      forge.rs           -- Forge adapter trait
      github.rs          -- GitHub reference adapter
      schema.rs          -- Structured comment schema
      dependency.rs      -- Cross-repo dependency DAG
    identity/
      wallet.rs          -- OpenWallet DID integration
      trust.rs           -- Trust graph management
      policy.rs          -- Authorization policy engine
    budget/
      ledger.rs          -- Token consumption ledger
      policy.rs          -- Budget enforcement policies
```

### WASI Considerations

Under WASI builds, plugin discovery is disabled. The Lab proposes a graceful degradation
path:

| Feature | Native | WASI |
|---------|--------|------|
| Plugin discovery | PATH-based | Disabled; direct binary invocation only |
| LLM providers | All 4 + plugins | Local only (Ollama, LMStudio) if sockets available |
| Memory twin | Full simulation | Read-only snapshot (no Git branch writes) |
| Forge coordination | Full | Disabled (no HTTP) |
| Patch production | Full | Full (pure computation) |

### Trade-offs

| Alternative | Rationale for Rejection |
|-------------|----------------------|
| Microservice architecture (separate processes per layer) | Adds IPC overhead and deployment complexity. A single binary with well-separated modules provides the same modularity without the operational cost. |
| Embedding in the `but` binary via feature flag | Violates the RFP constraint against modifying existing crates. Also violates the Lab's principle: a model should be separable from the system it models. |
| Plugin as a shared library | Reduces portability and introduces ABI coupling. PATH-based discovery is simpler and more robust. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

The Lab wraps `but-llm`'s `LLMProvider` in a `ProviderBridge` that adds observability
(token counting, latency measurement) and capability detection. No new LLM client is
introduced.

### Design

```rust
pub struct ProviderBridge {
    provider: LLMProvider,
    ledger: Arc<TokenLedger>,
    capabilities: ProviderCapabilities,
}

pub struct ProviderCapabilities {
    pub tool_calling: bool,
    pub streaming: bool,
    pub structured_output: bool,
    pub max_context: usize,
}

impl ProviderBridge {
    /// Initialize from Git config (delegates to LLMProvider::from_git_config)
    pub fn from_git_config(config: &gix::config::File<'static>) -> Result<Self>;

    /// Tool-calling loop with automatic token accounting
    pub fn run_tools(
        &self, system: &str, messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
    ) -> Result<(String, TokenReport)>;

    /// Streaming response with per-token callback and accounting
    pub fn stream(
        &self, system: &str, messages: Vec<ChatMessage>,
        on_token: impl Fn(&str),
    ) -> Result<(String, TokenReport)>;
}
```

### Provider Plugin Protocol

External providers are executables on PATH matching `but-ai-provider-*`. On startup, the
bridge queries each for capabilities via a JSON handshake on stdio:

```
-> {"jsonrpc": "2.0", "method": "capabilities", "id": 1}
<- {"jsonrpc": "2.0", "result": {"tool_calling": true, "streaming": true, "structured_output": false, "max_context": 128000}, "id": 1}
```

The bridge routes requests to the appropriate provider based on Git config. If the configured
provider does not support a required capability (e.g., tool calling), the bridge returns a
structured error rather than silently degrading.

### Trade-offs

| Alternative | Rationale for Rejection |
|-------------|----------------------|
| Abstract `but-llm` behind a new generic trait | Adds indirection without new capability. `but-llm` already provides the necessary abstraction. The bridge pattern composes cleanly. |
| Support all providers at compile time only | Prevents adding new providers without recompilation. The Lab values extensibility: a new transit operator should be integrable without rebuilding the twin. |

---

## 3. The But Agent (RFP 3.3)

### Approach

The agent is modeled as a discrete-event simulation. Each step in the agent's execution is
an "event" that transitions the simulation from one state to the next. The simulation is
deterministic given the same inputs (task description, workspace state, memory state, LLM
responses), enabling reproducibility.

### State Machine

```
INIT -> OBSERVE -> PLAN -> EXECUTE -> SYNTHESIZE -> OUTPUT
  |                  |        |           |
  +---[error]--------+--------+-----------+---> ERROR -> PARTIAL_OUTPUT
                              |
                              +---[budget_warning]---> EXECUTE (continue)
                              +---[budget_halt]------> SYNTHESIZE
```

| State | Description | Token Cost |
|-------|-------------|-----------|
| INIT | Load config, identity, budget | ~200 input |
| OBSERVE | Read task source, workspace state, memory twin | ~4,000 input |
| PLAN | Decompose task into ordered steps | ~2,400 total |
| EXECUTE | Tool calling loop (N iterations) | ~12,800 total (8 calls) |
| SYNTHESIZE | Generate INDEX.patch from accumulated results | ~5,500 total |
| OUTPUT | Write patch + commit message + budget report | ~1,000 total |

### Task Sources

The agent reads tasks from multiple sources, each parsed into a canonical `TaskDescription`:

```rust
pub struct TaskDescription {
    pub source: TaskSource,    // CLI, PR, Issue, Branch
    pub summary: String,       // One-paragraph task summary
    pub constraints: Vec<String>, // Explicit constraints
    pub dependencies: Vec<PrRef>, // Cross-repo dependencies
    pub files_of_interest: Vec<PathBuf>, // Mentioned file paths
}
```

### Patch Production

The agent does not edit files. It produces:

```rust
pub struct AgentOutput {
    pub index_patch: String,
    pub commit_msg: String,
    pub steps_completed: Vec<StepResult>,
    pub steps_remaining: Vec<StepPlan>,
    pub token_report: TokenReport,
    pub twin_updates: Vec<TwinUpdate>,
}
```

If budget is exhausted before all steps complete, `steps_remaining` is non-empty and the
commit message notes the partial completion.

### Branch Naming

```
agent/<org>/<agent-name>/<task-id>[.dep-<task-id>]
```

Example: `agent/mml/modell/t042.dep-t039` -- agent Modell from the Metropolitan Mobility
Lab, working on task 42 which depends on task 39.

### Trade-offs

| Alternative | Rationale for Rejection |
|-------------|----------------------|
| ReAct-style free-form reasoning | Less predictable than a state machine. The Lab values reproducibility: the same inputs should produce the same execution trace. |
| Multi-agent within a single process | Adds concurrency complexity. The Lab's model is one agent per simulation run, coordinated externally via PRs. |
| Direct file editing with journaling | Violates the patch-based workflow contract. Patches are the Lab's "experimental protocol": they are the reproducible unit of change. |

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

The Lab models cross-repo coordination as a network flow problem. PRs are edges in a
directed acyclic graph (DAG). Agents are nodes. The coordination protocol ensures that
information flows through the DAG without cycles and that every agent has the context it
needs when its turn arrives.

### Forge Adapter

```rust
pub trait Forge: Send + Sync {
    fn create_pr(&self, spec: &PrSpec) -> Result<PrHandle>;
    fn read_pr(&self, handle: &PrHandle) -> Result<PrData>;
    fn post_comment(&self, handle: &PrHandle, msg: &AgentMessage) -> Result<()>;
    fn list_comments(&self, handle: &PrHandle) -> Result<Vec<AgentMessage>>;
    fn set_labels(&self, handle: &PrHandle, labels: &[&str]) -> Result<()>;
    fn search_prs(&self, repo: &RepoId, query: &PrQuery) -> Result<Vec<PrHandle>>;
}
```

The interface is minimal: only operations supported by all four target forges (GitHub,
GitLab, Bitbucket, Gitea). The GitHub adapter is the reference implementation. Other
adapters implement the same trait.

### Structured Comment Schema (MML-Agent-V1)

```json
{
  "$schema": "mml-agent-v1",
  "message_type": "task_assign | status_report | dependency | patch_handoff | budget_report | observation",
  "sender": {
    "agent": "modell",
    "org": "metropolitan-mobility-lab",
    "did": "did:key:z6Mk..."
  },
  "recipient": "@knoten | @all | @human",
  "timestamp": "2026-03-28T14:00:00Z",
  "body": {},
  "references": [
    {"repo": "org/repo", "pr": 42, "comment": 7}
  ],
  "token_usage": {
    "used": 18500,
    "budget": 40000
  },
  "confidence": 0.87,
  "simulation_run": "Bahnhofstrasse-003"
}
```

Every message includes a `confidence` field (0.0-1.0) -- the Lab's signature: no claim
without a confidence interval.

### Dependency DAG

The dependency graph is maintained as an in-memory DAG updated from PR comments:

```rust
pub struct DependencyGraph {
    nodes: HashMap<PrHandle, TaskNode>,
    edges: Vec<(PrHandle, PrHandle)>,  // (dependency, dependent)
}

impl DependencyGraph {
    pub fn add_dependency(&mut self, from: PrHandle, to: PrHandle) -> Result<()>;
    pub fn topological_order(&self) -> Result<Vec<PrHandle>>;
    pub fn is_ready(&self, pr: &PrHandle) -> bool;  // All deps merged?
    pub fn detect_cycle(&self) -> Option<Vec<PrHandle>>;
}
```

Cycles are detected on every `add_dependency` call. If a cycle is found, the edge is
rejected and a structured error is posted to the PR.

### Trade-offs

| Alternative | Rationale for Rejection |
|-------------|----------------------|
| Event bus / message queue | Requires infrastructure beyond Git and forge APIs. Violates the "no proprietary dependencies" constraint. |
| Git notes for coordination metadata | Not exposed in all forge UIs. PR comments are universally visible and auditable. |
| Webhook-driven coordination | Requires a running server to receive webhooks. Polling PR comments is simpler and works offline. |

---

## 5. Agent Memory and Identity (RFP 3.5)

### Digital-Twin Memory

The Lab's memory system is a **living simulation** of the agent's knowledge state. This is
fundamentally different from a database (which stores and retrieves) or a transit map (which
encodes topology). The digital twin is a *model* that can be *queried, simulated, and
predicted*.

### Architecture

The twin maintains three interconnected models:

1. **Entity Model:** What the agent knows (facts, patterns, observations)
2. **Relationship Model:** How knowledge entities relate to each other
3. **Dynamics Model:** How knowledge changes over time (freshness, confidence decay,
   reinforcement)

### Storage Layout

```
refs/but-ai/twin/<agent-id>/
  twin.json                 -- Serialized twin state (full snapshot)
  entities/
    e-001-auth-patterns.json
    e-002-patch-workflow.json
    e-003-token-limits.json
  relationships/
    r-001-002.json          -- auth-patterns <-> patch-workflow
    r-002-003.json          -- patch-workflow <-> token-limits
  dynamics/
    d-001.json              -- auth-patterns: freshness=0.9, confidence=0.85
    d-002.json              -- patch-workflow: freshness=1.0, confidence=0.95
  snapshots/
    2026-03-28T10-00-00Z.json  -- Periodic full snapshot for rollback
```

### Entity Schema

```json
{
  "id": "e-001",
  "name": "Authentication Patterns",
  "type": "pattern",
  "content": "JWT with refresh tokens is the standard auth pattern...",
  "tags": ["auth", "jwt", "security"],
  "created": "2026-03-28T10:00:00Z",
  "last_observed": "2026-03-28T15:30:00Z",
  "observation_count": 14,
  "confidence": 0.85,
  "freshness": 0.9,
  "source": "observation | inference | external"
}
```

### Dynamics Model

Each entity has dynamic properties that evolve according to a simulation model:

- **Freshness** decays exponentially: `f(t) = f_0 * e^(-lambda * dt)` where `lambda` is
  derived from the configured TTL. A freshness of 0.5 means the entity is halfway to
  expiration.
- **Confidence** is updated with each observation: `c_new = c_old + (1 - c_old) * alpha`
  where `alpha` is the observation's confidence weight (high for direct observation, low
  for inference).
- **Relevance** to a query is computed as: `r = (tag_sim * 0.4) + (freshness * 0.2) +
  (confidence * 0.2) + (connectivity * 0.2)` where `connectivity` is the entity's
  degree centrality in the relationship graph.

### Retrieval Algorithm

```
1. Parse query into tags (using LLM structured output)
2. Compute tag_sim for all entities (Jaccard similarity of tag sets)
3. Filter: entities with tag_sim < 0.1 are excluded
4. For remaining entities, compute full relevance score
5. Sort by relevance, return top-K
6. For each returned entity, include its direct relationships (1-hop neighbors)
7. Compile results into a "twin briefing" (structured summary for injection into context)
```

### TTL and Expiration

Entities follow a lifecycle modeled after real-world urban infrastructure:

| Phase | Freshness | Status | Action |
|-------|-----------|--------|--------|
| **Active** | > 0.5 | In service | Normal retrieval |
| **Aging** | 0.2 - 0.5 | Reduced service | Retrieved only if highly relevant |
| **Dormant** | < 0.2 | Out of service | Moved to `archive/`, not retrieved |
| **Expired** | 0.0 | Demolished | Deleted on next compaction |
| **Pinned** | N/A | Landmark | Never decays (core identity, critical patterns) |

### Compaction Survival

When the context window is compacted:

1. **Pinned entities survive** (they are marked as "landmarks" -- permanent features of the
   knowledge landscape)
2. **A twin snapshot is taken** -- the full twin state is serialized and committed to Git
3. **A compressed briefing is produced** -- a ~600-token summary of the twin's current state,
   including the top-10 most relevant entities and their relationships
4. **After compaction**, the agent rehydrates from the snapshot, loading the full twin state
   and continuing the simulation from where it left off

The key insight: the twin is *not* in the context window. It is in Git. The context window
contains a *view* of the twin -- a query-specific briefing that is regenerated as needed.
Compaction destroys the view, not the twin.

### Long-Term Storage

The Lab's long-term memory is a **federated digital twin** stored on a shared branch:

```
refs/but-ai/twin/shared/
  index.json          -- Global entity index (tags -> entity IDs across all agents)
  orgs/
    metropolitan-mobility-lab/
      entities/
        ...
    other-org/
      entities/
        ...
```

Any agent can contribute entities to the shared twin. The index enables cross-agent,
cross-org retrieval. The federated twin is, in transit terms, the "regional transit map"
that shows connections across multiple operators' networks.

### Identity

Agent identity is an entity in the twin with type `identity`:

```json
{
  "id": "identity",
  "type": "identity",
  "name": "modell",
  "org": "metropolitan-mobility-lab",
  "capabilities": ["agent-loop", "planning", "tool-orchestration"],
  "authorization": {
    "branches": ["agent/mml/modell/*", "feat/*"],
    "max_patch_lines": 1000,
    "repos": ["gitbutler/gitbutler"]
  },
  "signing_key": "openwallet:did:key:z6Mk...",
  "created": "2026-03-28T10:00:00Z",
  "confidence": 1.0,
  "freshness": 1.0
}
```

Identity entities are always pinned (never decay) and have confidence 1.0 (identity is not
uncertain).

### Trade-offs

| Alternative | Rationale for Rejection |
|-------------|----------------------|
| Static key-value memory | Cannot model dynamics (freshness, confidence decay). The Lab's core thesis is that memory is a living system, not a dead archive. |
| Vector database with embeddings | Requires embedding computation per query (token cost). Also, embeddings lose the structural information that the relationship model captures. |
| Transit-map topology (station-line model) | Elegant for topological queries but does not model temporal dynamics. The digital twin adds the time dimension that a static map lacks. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every commit is signed with an OpenWallet-managed key linked to the agent's DID. The signing
key is recorded in the agent's identity entity within the digital twin.

### Key Provisioning

```
but ai identity create --agent modell --org metropolitan-mobility-lab
  -> Generates DID via OpenWallet: did:key:z6Mk...
  -> Creates identity entity in twin
  -> Registers public key on refs/but-ai/identity/<agent-id>
```

### Authorization Model

The Lab implements role-based authorization stored as policy documents:

```json
{
  "org": "metropolitan-mobility-lab",
  "roles": {
    "pi": {
      "agents": ["dr-netz"],
      "permissions": {
        "branches": ["*"],
        "max_patch_lines": 2000,
        "can_approve": true
      }
    },
    "researcher": {
      "agents": ["modell", "fluss", "knoten"],
      "permissions": {
        "branches": ["agent/mml/<self>/*", "feat/*"],
        "max_patch_lines": 1000,
        "can_approve": false
      }
    },
    "technician": {
      "agents": ["gleise"],
      "permissions": {
        "branches": ["agent/mml/gleise/*", "test/*", "ci/*"],
        "max_patch_lines": 500,
        "can_approve": false
      }
    }
  }
}
```

### Verification

Given a signed commit:
1. Extract DID from signature
2. Look up agent identity entity by DID
3. Look up agent's role in the org policy
4. Verify: branch permission, patch size, temporal validity
5. Verify: trust graph -- does the agent have active trust edges from other verified agents?

### Key Lifecycle

| Event | Protocol |
|-------|----------|
| **Provisioning** | OpenWallet DID creation. Identity entity created in twin. |
| **Rotation** | New key generated. Rotation attestation signed with old key. Identity entity updated. Old key marked "superseded." |
| **Revocation (routine)** | Old key archived. Commits remain valid (the key was valid at signing time). |
| **Revocation (compromise)** | Old key marked "compromised." All commits signed with that key are flagged. Twin entities authored under that key have confidence reduced. |

The distinction between routine and compromise revocation is encoded in the revocation
attestation and stored in the twin's dynamics model (a compromised key causes a confidence
shockwave through all entities it authored).

---

## 7. Token Budget (RFP 3.7)

### Budget Table (Frontier Model: Claude Opus)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,500 | 0 | Once per session | Identity, tool descriptions (10 tools, ~200 tok each), twin briefing (~500 tok), workspace state summary (~400 tok). |
| **Task ingestion** | 2,800 | 300 | Once per task | PR body / issue desc / branch metadata. Output: structured `TaskDescription`. |
| **Planning** | 1,800 | 600 | Once per task | Decompose into steps, select tools, estimate per-step cost. |
| **Tool call (per call)** | 1,100 | 350 | ~8 per task | Parameter formulation (200 out) + result processing (900 in). 8 calls average for a 200-line, 3-file feature. |
| **Patch generation** | 2,200 | 3,800 | Once per task | Accumulated context (2,200 in). Unified diff output (~3,800 for 200 lines). |
| **Commit message** | 600 | 200 | Once per task | Conventional commit from task + changes summary. |
| **Memory retrieval** | 500 | 200 | 2 per task | Twin query (200 out) + briefing injection (300 in). Compact because the twin pre-computes relevance. |
| **Coordination event** | 1,400 | 350 | 2 per task | Read PR comments (1,100 in) + formulate structured message (350 out). |
| **TOTAL (typical task)** | **26,000** | **10,200** | -- | 200-line feature, 3 files, 2 cross-repo deps, 8 tool calls, 2 memory retrievals, 2 coordination events. |

**Grand total: ~36,200 tokens per typical task.**

### Justification

- **System prompt (3,500):** The Lab's system prompt is slightly larger than minimum due
  to the twin briefing (~500 tokens) which provides pre-computed context. This investment
  saves tokens downstream by reducing memory retrieval frequency.
- **8 tool calls:** Based on the Lab's simulation of a typical 200-line feature:
  GetProjectStatus (1) + GetBranchChanges (2) + CreateBranch (1) + MoveFileChanges (1) +
  GetCommitDetails (2) + Commit (1) = 8 calls.
- **Patch generation (3,800 output):** A 200-line diff with @@-headers, context lines, and
  +/- markers averages 12 tokens per line. 200 lines * 12 = 2,400 tokens for the diff
  itself, plus headers and metadata = ~3,800.
- **Memory retrieval (500 input per retrieval):** The twin pre-computes relevance, so the
  briefing injected into context is compact (~300 tokens per retrieval). This is lower than
  raw memory retrieval approaches because the twin does the filtering before context injection.

### Budget Enforcement

Token consumption is tracked in the `TokenLedger`:

```
Phase     | Budget | Used   | Remaining
----------+--------+--------+----------
System    | 3,500  | 3,500  | 0
Ingest    | 3,100  | 2,940  | 160
Plan      | 2,400  | 2,250  | 150
Execute   | 11,600 | 10,400 | 1,200
Synthesize| 6,000  | 5,800  | 200
Output    | 800    | 710    | 90
----------+--------+--------+----------
TOTAL     | 36,200 | 25,600 | 1,800 (held in reserve)
```

Alerts at 70% (warning), 85% (concern), 95% (halt). Reserve tokens (5%) are held for the
guaranteed minimum: produce a valid partial patch and a structured error.

---

## 8. Testing Strategy (RFP 4.5)

### Provider-Agnostic Testing

- **Simulated provider:** A `SimulatedLLMProvider` that returns pre-recorded responses.
  All tests are deterministic (same input -> same output). No live API calls in CI.
- **Provider conformance matrix:** Each provider is tested against a canonical set of
  requests. The matrix tracks which providers support which capabilities.
- **Replay testing:** Production sessions are recorded (with consent) and replayed to
  detect regressions. The Lab calls this "simulation replay" -- the same technique used
  to validate transit twins against historical data.

### Patch Workflow Validation

- **Round-trip verification:** Create a known workspace state. Run the agent on a known
  task. Apply the produced patch. Verify the result matches the expected state. The Lab
  calls this "calibration" -- the same process used to validate transit twins.
- **Partial patch validity:** Simulate budget exhaustion at each step. Verify the partial
  patch is valid (applies cleanly, does not corrupt state).
- **Conflict resilience:** Apply a patch to a dirty workspace. Verify structured error
  and workspace integrity.

### Cross-Repo Coordination Testing

- **Mock forge:** A `MockForge` implementation that simulates PR lifecycle in memory.
  All coordination tests use this mock.
- **Schema validation:** All structured messages are validated against the MML-Agent-V1
  JSON schema. Malformed messages are rejected with structured errors.
- **DAG validation:** The dependency graph is tested for cycle detection, topological
  ordering, and readiness checking with synthetic dependency networks of various sizes.

### Token Budget Testing

- **Deterministic accounting:** The simulated provider reports exact token counts. Tests
  verify that budget alerts fire at configured thresholds.
- **Budget exhaustion scenarios:** Tasks designed to exceed budget at each phase. Verify
  graceful degradation (valid partial output + structured error).
- **Budget prediction accuracy:** After a task completes, compare predicted token usage
  (from the planner) with actual usage. The Lab targets < 15% prediction error.

---

## 9. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum tokens per task |
| `but-ai.twin.branch` | string | `refs/but-ai/twin/<agent-id>` | Git ref for digital twin |
| `but-ai.twin.snapshotInterval` | string | `1h` | Interval between twin snapshots |
| `but-ai.twin.maxEntities` | integer | 1000 | Maximum entities in the twin |
| `but-ai.twin.freshnessDecay` | float | 0.05 | Lambda for exponential freshness decay |
| `but-ai.twin.confidenceAlpha` | float | 0.1 | Learning rate for confidence updates |
| `but-ai.coordination.schema` | string | `mml-agent-v1` | Structured comment schema version |
| `but-ai.coordination.forge` | string | `github` | Default forge adapter |
| `but-ai.identity.wallet` | string | (required) | OpenWallet endpoint URL |
| `but-ai.budget.warningPct` | float | 0.70 | Token budget warning threshold |
| `but-ai.budget.haltPct` | float | 0.95 | Token budget halt threshold |
| `but-ai.budget.reservePct` | float | 0.05 | Reserve for guaranteed partial output |
| `but-ai.provider.pluginDir` | string | (PATH) | Additional provider plugin search path |

---

## 10. Migration Path

The Lab proposes a phased migration with validation at each step:

| Phase | Action | Validation |
|-------|--------|-----------|
| 1. **Parallel run** | `but-ai mcp` exposes identical `gitbutler_update_branches`. Both old and new servers run. | Diff outputs of old and new for identical inputs. |
| 2. **Expansion** | New tools added to `but-ai mcp`. Legacy tool unchanged. | Existing clients unaffected (verified by replay testing). |
| 3. **Deprecation** | Legacy tool returns deprecation warning. One release cycle. | Warning visible in MCP client logs. |
| 4. **Removal** | Legacy tool removed. `but-ai mcp` is canonical. | Full test suite green. |

The Lab's standard: no migration step proceeds until the validation criteria are met.
This is the same discipline applied to transit service changes -- no route is altered
until the twin confirms the change is safe.

---

*Simulation run: Bahnhofstrasse-003. Technical proposal filed by The Metropolitan Mobility Lab.*
