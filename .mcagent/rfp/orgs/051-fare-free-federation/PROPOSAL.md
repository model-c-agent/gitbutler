# Fare-Free Federation -- Technical Proposal

**RFP:** `but ai` Plugin for GitButler CLI v1.0.0
**Organization:** Fare-Free Federation (Org 051)
**Domain:** Public Transit | **Philosophy:** Anarchist Collective
**Route Number:** 4,220

---

## Executive Summary

The Fare-Free Federation proposes a `but-ai` plugin built on the principle that every
component is a station on a transit network, every interface is a platform, and every
data flow is a rider traveling between stops. The plugin architecture mirrors a
decentralized transit system: no central dispatcher, no single point of failure, no
gatekeepers between the rider (the AI agent) and the destination (the committed patch).

Our core contribution is **transit-map memory** -- a fundamentally topological approach to
agent memory where knowledge is stored as stations on a network, related concepts are
connected by lines, and retrieval is a shortest-path traversal rather than a similarity
search. This maps directly to Git's own graph structure: commits are stations, branches
are lines, and merges are transfers.

---

## 1. Plugin Architecture (RFP 3.1)

### Approach

`but-ai` is a standalone Rust binary (`but-ai`) placed on PATH. The `but` CLI discovers it
via `find_external_subcommand()` in `crates/but/src/alias.rs`. The binary operates in two
modes, selected by subcommand:

- `but ai <subcommand>` -- CLI mode for interactive and scripted use
- `but ai mcp` -- MCP server mode on stdio

The plugin reads `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, and `BUT_JSON` from environment
variables set by the parent `but` process.

### Design

```
but ai
  +-- agent     Run the autonomous But Agent
  +-- mcp       Start the MCP server (stdio)
  +-- memory    Query / manage agent memory
  +-- identity  Manage agent identity and keys
  +-- budget    Show / enforce token budgets
  +-- status    Show plugin and provider status
```

The plugin is structured as a single crate (`crates/but-ai/`) with internal modules:

```
crates/but-ai/
  src/
    main.rs          -- CLI entry point, arg parsing
    mcp/
      server.rs      -- MCP ServerHandler implementation
      tools.rs       -- Tool registration bridge
    agent/
      runner.rs      -- Autonomous agent loop
      planner.rs     -- Task decomposition
      patcher.rs     -- INDEX.patch + COMMIT.msg production
    provider/
      bridge.rs      -- Bridge to but-llm (no new LLM client)
      plugin.rs      -- Dynamic provider plugin loading
    memory/
      transit_map.rs -- Transit-map memory engine
      station.rs     -- Memory station (node) types
      line.rs        -- Memory line (edge) types
      retrieval.rs   -- Shortest-path relevance scoring
      storage.rs     -- Git branch storage backend
    identity/
      wallet.rs      -- OpenWallet integration
      registry.rs    -- Agent identity registry
      auth.rs        -- Authorization policy engine
    coordination/
      forge.rs       -- Forge adapter trait
      github.rs      -- GitHub reference implementation
      schema.rs      -- Structured PR comment schema
      graph.rs       -- Cross-repo dependency graph
    budget/
      tracker.rs     -- Per-component token accounting
      enforcer.rs    -- Budget limit enforcement
```

### WASI Degradation

The plugin discovery mechanism is gated with `#[cfg(not(feature = "wasi"))]`. Under WASI:

- `but ai` is not discoverable as an external subcommand
- AI capabilities are available only through direct invocation of the `but-ai` binary
  (if compiled to WASI separately) or through the MCP protocol over stdio
- The memory system degrades to file-based storage (no Git branch operations under WASI's
  limited filesystem)
- Network-dependent features (forge coordination, LLM API calls) require WASI socket
  capabilities (`wasi:sockets`)

The Federation recommends that WASI builds expose a reduced tool surface: memory read-only,
no forge coordination, local-only provider (Ollama/LMStudio via network if sockets are
available).

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Embed AI in core `but` binary | Violates RFP constraint (no modifying existing crates). Also creates a single point of failure -- the Federation's least favorite topology. |
| Separate process per mode (CLI vs MCP) | Unnecessary duplication. Both modes share the same agent engine; only the I/O surface differs. One binary, two doors. |
| Dynamic library plugin (.so/.dylib) | Less portable than PATH-based discovery. Also requires ABI stability guarantees the Federation is not prepared to make for v1. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

The plugin uses `but-llm` as its sole LLM backend. No new LLM client is introduced. The
provider bridge (`provider/bridge.rs`) wraps `LLMProvider` methods and adds:

- Automatic tool registration from `WorkspaceToolset`
- Token counting per call (via streaming callback)
- Provider capability detection (does this provider support tool calling? structured output?)

### Design

```rust
// Provider bridge -- wraps but-llm, adds accounting
pub struct ProviderBridge {
    inner: LLMProvider,
    budget: Arc<BudgetTracker>,
    capabilities: ProviderCapabilities,
}

impl ProviderBridge {
    pub fn from_git_config(config: &gix::config::File<'static>) -> Option<Self>;

    pub fn tool_loop(
        &self,
        system: &str,
        messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
    ) -> Result<(String, TokenUsage)>;

    pub fn supports_tool_calling(&self) -> bool;
    pub fn supports_structured_output(&self) -> bool;
}
```

### New Provider Plugin Mechanism

New providers (Gemini, Mistral, local GGUF) are added via a **provider plugin protocol**:

1. A provider plugin is an executable named `but-ai-provider-<name>` on PATH
2. The plugin communicates via JSON-RPC on stdio (same pattern as MCP)
3. The plugin declares its capabilities on startup: `{"tool_calling": true, "streaming": true, "structured_output": false}`
4. `but-ai` discovers provider plugins at startup and registers them alongside built-in providers

This means adding Google Gemini is: write a `but-ai-provider-gemini` binary, put it on PATH,
done. No recompilation of `but-ai`.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Abstract `but-llm` behind a new trait | Adds a layer without adding capability. `but-llm` is already well-abstracted. The bridge pattern composes without modifying. |
| WASM-based provider plugins | Elegant but premature. WASI support for `but` is still experimental. PATH-based plugins are simpler and work today. |
| gRPC for provider plugins | Over-engineered for this use case. JSON-RPC on stdio is sufficient and requires no network configuration. |

---

## 3. The But Agent (RFP 3.3)

### Approach

The But Agent (`but ai agent`) is an autonomous execution loop that reads a task, plans a
sequence of tool calls, executes them, and produces INDEX.patch + COMMIT.msg as output. The
agent NEVER makes direct file edits, runs `git commit`, or calls `but commit`. It produces
patches; the `but` orchestrator applies them.

### Design

The agent loop follows the transit metaphor:

```
1. DISPATCH    Read task (PR body, issue, CLI arg) -> assign route number
2. ROUTE       Decompose task into stops (subtasks)
3. BOARD       Load system prompt + memory + tool descriptions
4. SERVICE     For each stop:
                 a. Select tool(s) from WorkspaceToolset
                 b. Execute tool call via ProviderBridge
                 c. Record result in transit-map memory
                 d. Check budget (stop if > 90%)
5. TERMINUS    Generate INDEX.patch from accumulated changes
6. ALIGHT      Generate COMMIT.msg from task + changes summary
7. FILE        Output patch + message to configured location
```

### Task Sources

```
but ai agent --task "implement feature X"           # CLI argument
but ai agent --pr 42                                # Read from PR #42 body
but ai agent --issue 17                             # Read from issue #17
but ai agent --branch feat/my-feature               # Read from branch metadata
```

### Patch Production

The agent accumulates tool call results (branch changes, file moves, commits) and at the
terminus step, generates a unified diff against the current index:

```rust
pub struct PatchOutput {
    pub index_patch: String,    // Unified diff content
    pub commit_msg: String,     // Conventional commit message
    pub route_number: u64,      // Federation tracking number
    pub budget_used: TokenUsage,
    pub stops_completed: usize,
    pub stops_total: usize,
}
```

If the agent exhausts its budget before completing all stops, it produces a partial patch
with the completed stops and reports the remaining stops in the commit message body.

### Branch Naming

The Federation extends the existing `s01.s04` convention with agent identity:

```
agent/<agent-id>/s<seq>[.s<dep>]
```

Example: `agent/ligne-a3f2/s01.s04` means agent Ligne (key fingerprint a3f2) is working on
sequence 04, which depends on sequence 01.

### WorkspaceToolset Integration

All ten workspace tools are registered via the `Toolset` trait:

```rust
let mut toolset = WorkspaceToolset::new(ctx);
// All 10 tools auto-registered: Commit, CreateBranch, Amend,
// SquashCommits, GetProjectStatus, MoveFileChanges,
// GetCommitDetails, GetBranchChanges, SplitBranch, SplitCommit

provider.tool_loop(system_prompt, messages, &mut toolset)?;
```

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Multi-agent within a single `but ai agent` call | The Federation's model is one agent per route. Multiple agents are coordinated via PR-based communication, not within a single process. |
| Direct file editing with rollback | Violates the patch-based workflow. Patches are the Federation's social contract: they are reviewable, reversible, and composable. |
| ReAct-style agent (reason-act loop) | The route-and-stops model is structurally similar but more predictable. Each stop is a pre-planned checkpoint, not an emergent decision. |

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

PRs are the Federation's buses: they carry passengers (patches, status updates, dependency
declarations) along defined routes (branches) between stations (repositories). The
coordination protocol uses PR comments as the communication medium, with a structured schema
that is forge-agnostic.

### Forge Adapter Interface

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &PrDraft) -> Result<PrId>;
    fn comment(&self, pr: &PrId, comment: &AgentComment) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrId) -> Result<Vec<AgentComment>>;
    fn add_label(&self, pr: &PrId, label: &str) -> Result<()>;
    fn get_pr_body(&self, pr: &PrId) -> Result<String>;
    fn list_prs(&self, repo: &RepoRef, filter: &PrFilter) -> Result<Vec<PrSummary>>;
}

pub struct RepoRef {
    pub forge: ForgeKind,   // GitHub, GitLab, Bitbucket, Gitea
    pub owner: String,
    pub name: String,
}
```

The GitHub implementation is the reference adapter. The interface is intentionally minimal --
only operations that all four forges support are included.

### Structured Comment Schema

```json
{
  "schema": "fff-agent-v1",
  "type": "task_assign | status | dependency | patch_handoff | budget_report",
  "sender": {
    "agent_id": "ligne-a3f2",
    "org": "fare-free-federation",
    "key_fingerprint": "a3f2b9c1"
  },
  "receiver": "@correspondance-7e1d | @all",
  "payload": { },
  "refs": [
    "org/repo#123",
    "other-org/other-repo#456"
  ],
  "route": 4220,
  "budget": {
    "used": 12400,
    "limit": 42000
  },
  "checksum": "sha256:abc123..."
}
```

The comment body is wrapped in a code fence with a `but-agent` language tag:

````
```but-agent
{ ... JSON ... }
```
````

This ensures the structured content is preserved by all forges (code fences are universally
supported) while remaining human-readable when viewed in a PR comment thread.

### Cross-Repo Dependency Tracking

Dependencies are declared in PR comments and tracked in the transit-map memory system as
"inter-line transfers." A dependency from repo A PR #42 to repo B PR #17 is modeled as a
transfer station connecting the A-line to the B-line.

The dependency graph is acyclic (enforced by Correspondance, who flags cycles immediately).
If a cycle is detected, the Federation's resolution protocol is:

1. Identify the youngest edge in the cycle (most recently declared dependency)
2. Flag it for human review
3. Continue work on non-cyclic paths

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Webhook-based coordination | Requires infrastructure beyond Git and forge. The Federation refuses proprietary dependencies. |
| Git notes for metadata | Not all forges expose Git notes in their UI. PR comments are universally visible. |
| Label-only coordination | Labels lack the expressiveness for structured messages. They work for status flags but not for payload delivery. |

---

## 5. Agent Memory and Identity (RFP 3.5)

### Transit-Map Memory

The Federation's memory system stores knowledge as a **transit network** in Git branches.
This is fundamentally different from a key-value store, a vector database, or a flat file
system. The topology of the network -- which stations exist, how they connect, and which
routes traverse them -- is itself the primary information structure.

### Core Concepts

| Transit Concept | Memory Equivalent | Git Storage |
|----------------|-------------------|-------------|
| **Station** | A discrete memory entry (fact, pattern, observation) | A file at `stations/<station-id>.json` |
| **Line** | A thematic connection between stations | A directory `lines/<line-name>/` with symlinks to stations |
| **Transfer** | A cross-domain connection between stations on different lines | A file at `transfers/<from-station>--<to-station>.json` |
| **Express Route** | A compressed summary skipping intermediate stations | A file at `express/<route-name>.json` |
| **System Map** | The complete memory topology | `map.json` at branch root |

### Storage Layout

All memory lives on a special Git branch per agent:

```
refs/but-ai/memory/<agent-id>
  map.json                          # Full network topology
  stations/
    stn-001-auth-patterns.json      # Station: authentication patterns
    stn-002-patch-workflow.json      # Station: patch-based workflow
    stn-003-token-limits.json       # Station: token budget patterns
    ...
  lines/
    security/                       # Line: security-related concepts
      stn-001 -> ../../stations/stn-001-auth-patterns.json
      stn-007 -> ../../stations/stn-007-encryption.json
    workflow/                       # Line: workflow-related concepts
      stn-002 -> ../../stations/stn-002-patch-workflow.json
      stn-005 -> ../../stations/stn-005-branch-naming.json
  transfers/
    stn-001--stn-003.json           # Transfer: auth patterns <-> token limits
  express/
    security-overview.json          # Express: compressed security knowledge
```

### Station Schema

```json
{
  "id": "stn-001",
  "name": "Authentication Patterns",
  "content": "JWT with refresh tokens is the standard auth pattern in this codebase...",
  "line": "security",
  "connections": ["stn-007", "stn-012"],
  "created": "2026-03-28T10:00:00Z",
  "ttl": "30d",
  "access_count": 14,
  "last_accessed": "2026-03-28T15:30:00Z",
  "tags": ["auth", "jwt", "security"]
}
```

### Relevance Scoring

Retrieval is a **shortest-path traversal** on the network graph. Given a query:

1. **Embed the query** as a set of tags (using the LLM's structured output for tag extraction)
2. **Find entry stations** -- stations whose tags overlap with the query tags
3. **Traverse the network** from entry stations using BFS, scoring each visited station:
   - Score = `(tag_overlap * 0.5) + (1/distance * 0.3) + (access_frequency * 0.2)`
   - Distance is measured in hops on the network graph
4. **Return top-K stations** ranked by score
5. **Include express routes** that cover the returned stations (compressed context)

This is fundamentally different from vector similarity search. It exploits the *structure*
of knowledge, not just its surface similarity. Two stations with no overlapping keywords
can still be retrieved together if they are connected by a transfer.

### TTL and Expiration

Each station has a configurable TTL. Expiration follows a transit metaphor:

- **Active stations** (accessed within TTL): remain in service
- **Dormant stations** (not accessed within TTL): moved to `archive/` directory
- **Expired stations** (dormant for 2x TTL): deleted on next memory compaction
- **Pinned stations** (TTL = "permanent"): never expire (core identity, critical patterns)

### Compaction Survival

When the LLM context window is compacted:

1. **Express routes survive.** They are compressed summaries designed for exactly this
   purpose.
2. **Pinned stations survive.** They are marked as critical and always included.
3. **Active station summaries survive.** The top-K most recently accessed stations are
   summarized into a "system map" that fits in ~500 tokens.
4. **Everything else is recoverable.** The full network is in Git. After compaction, the
   agent rehydrates by reading the system map and then fetching specific stations on demand.

### Long-Term Storage (OpenViking)

Long-term memory is stored on a shared branch:

```
refs/but-ai/memory/shared/openviking
```

Any agent can write to this branch. Entries are namespaced by organization and agent. The
shared memory is indexed by a top-level `index.json` that maps tags to station IDs across
all contributors. This is the "inter-agency transit map" -- a network of networks.

### Identity

Each agent's identity is stored as a special station on its memory branch:

```json
{
  "id": "identity",
  "name": "ligne-a3f2",
  "agent_type": "route-architect",
  "org": "fare-free-federation",
  "capabilities": ["architecture", "cli-integration", "mcp-design"],
  "authorization_scope": {
    "branches": ["agent/ligne-a3f2/*", "feat/*"],
    "max_patch_lines": 1000,
    "repos": ["gitbutler/gitbutler"]
  },
  "created": "2026-03-28T10:00:00Z",
  "signing_key": "openwallet:did:key:z6Mk..."
}
```

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Vector database (HNSW) | Requires external service or embedded engine. The Federation stores everything in Git. Also, vector similarity misses structural relationships. |
| Flat key-value in Git | Loses the topological information that makes transit-map memory useful. A flat store cannot answer "what is connected to this concept?" |
| SQLite in a Git blob | Not human-readable. The Federation values transparency: anyone should be able to `cat` a memory file and understand it. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent commit is signed using an OpenWallet-managed key. The signing key is tied to the
agent's DID (Decentralized Identifier), which is recorded in the identity station on the
agent's memory branch.

### Key Management

```
but ai identity init --name ligne --org fare-free-federation
  -> Creates OpenWallet DID: did:key:z6Mk...
  -> Stores DID in agent identity station
  -> Registers public key in refs/but-ai/identity/<agent-id>
```

### Authorization Model

Authorization policies are stored as a policy document on the shared identity branch:

```
refs/but-ai/identity/policies/
  fare-free-federation.json
```

```json
{
  "org": "fare-free-federation",
  "agents": {
    "ligne-a3f2": {
      "branches": ["agent/ligne-a3f2/*", "feat/*"],
      "max_patch_lines": 1000,
      "repos": ["gitbutler/gitbutler"]
    },
    "correspondance-7e1d": {
      "branches": ["agent/correspondance-7e1d/*", "feat/*", "coordination/*"],
      "max_patch_lines": 500,
      "repos": ["*"]
    }
  }
}
```

### Verification Chain

Given a signed commit:

1. Extract the signer's DID from the commit signature
2. Look up the DID in `refs/but-ai/identity/` to find the agent ID
3. Look up the agent's policy in `refs/but-ai/identity/policies/`
4. Verify: is this agent authorized to commit to this branch, in this repo, at this time?
5. Verify: does the patch size fall within the agent's constraints?

### Key Lifecycle

| Event | Action |
|-------|--------|
| **Provisioning** | `but ai identity init` creates DID + key pair via OpenWallet |
| **Rotation** | `but ai identity rotate` creates new key, signs a "rotation commit" with old key pointing to new key, updates identity station |
| **Revocation (routine)** | `but ai identity revoke --reason rotation` marks old key as superseded; commits signed with old key remain valid |
| **Revocation (compromise)** | `but ai identity revoke --reason compromise` marks old key as compromised; all commits signed with compromised key are flagged for review |

The distinction between rotation and compromise is recorded in the revocation commit message
and in the identity station's history.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| GPG keys managed outside OpenWallet | Does not satisfy the mandate. Also, GPG key management is notoriously painful. OpenWallet provides a saner lifecycle. |
| SSH signing keys | Simpler but lacks the DID-based identity model. SSH keys prove "who has the key," not "who is authorized by policy." |
| No per-agent keys (shared org key) | Violates the requirement that agent identity is provable from signature. A shared key cannot distinguish Agent A from Agent B. |

---

## 7. Token Budget (RFP 3.7)

### Budget Table (Frontier Model: Claude Opus)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Agent identity, tool descriptions (10 tools), workspace state summary, memory system map. Optimized via lazy tool desc loading. |
| **Task ingestion** | 2,500 | 200 | Once per task | Reading PR body / issue description / branch metadata. Output is structured task decomposition. |
| **Planning** | 1,800 | 600 | Once per task | Route planning: decompose task into stops, estimate per-stop cost, select tools. |
| **Tool call (per call)** | 1,200 | 400 | ~8 per task | Tool parameter formulation (200 out) + result processing (1,000 in). Average 8 calls for a 200-line feature. |
| **Patch generation** | 2,000 | 3,500 | Once per task | Context: accumulated changes (2,000 in). Output: unified diff (3,500 out for 200 lines across 3 files). |
| **Commit message** | 800 | 200 | Once per task | Summarize changes into conventional commit. Terse by policy. |
| **Memory retrieval** | 600 | 150 | 2 per task | Query formulation (150 out) + result injection (450 in). Express routes keep retrieved context compact. |
| **Coordination event** | 1,500 | 400 | 2 per task | Read PR comments (1,200 in) + formulate response (400 out). Budget includes cross-repo reference resolution. |
| **TOTAL (typical task)** | **25,800** | **9,750** | -- | 200-line feature across 3 files, 2 cross-repo dependencies, 8 tool calls, 2 memory retrievals, 2 coordination events. |

**Grand total: ~35,550 tokens per typical task.**

### Budget Justification

- **System prompt at 3,200:** 10 tool descriptions at ~200 tokens each = 2,000. Agent
  identity + memory map + workspace state = 1,200. This is under the RFP's suggested 4,000
  cap.
- **8 tool calls:** A 200-line feature across 3 files typically requires: GetProjectStatus
  (1), GetBranchChanges (2-3), CreateBranch (1), MoveFileChanges (1-2), Commit (1). That
  is 6-8 calls.
- **Patch generation at 3,500 output:** A 200-line unified diff with context lines and
  headers is approximately 300 lines x ~12 tokens/line = 3,600 tokens. We round down
  assuming some lines are short.
- **Coordination at 1,500 input per event:** A PR comment thread with 5 structured messages
  at ~300 tokens each = 1,500. This assumes moderate thread length.

### Budget Enforcement

Titre monitors token consumption in real time via the streaming callback in
`tool_calling_loop_stream`:

```
0%   -------- 70% WARNING -------- 90% LAST TRAIN -------- 100% HALT
```

At 70%, Titre logs a warning. At 90%, the agent enters "last train" mode: complete the
current stop, produce partial patch, stop. At 100%, the agent halts immediately and outputs
whatever it has.

---

## 8. Testing Strategy (RFP 4.5)

### Provider-Agnostic Testing

- **Mock provider:** A `MockLLMProvider` that returns deterministic responses for known
  tool call sequences. All integration tests use the mock.
- **Provider conformance suite:** A set of canonical requests and expected behaviors that
  any provider (including plugins) must pass. Tests: tool calling, streaming, structured
  output, error handling.
- **Recorded sessions:** Real LLM sessions are recorded (request/response pairs) and
  replayed in CI. New recordings are made manually; CI never hits a live API.

### Patch Workflow Validation

- **Round-trip test:** Generate a known change, produce INDEX.patch, apply patch, verify
  working tree matches expected state.
- **Partial patch test:** Simulate budget exhaustion mid-task. Verify the partial patch is
  valid (applies cleanly, does not corrupt the index).
- **Conflict test:** Apply a patch to a dirty working tree. Verify the error is structured
  and the working tree is not modified.

### Cross-Repo Coordination Testing

- **In-memory forge:** A `MockForgeAdapter` that simulates PR creation, commenting, and
  label management in memory. All coordination tests use this mock.
- **Schema validation:** Every structured comment is validated against the `fff-agent-v1`
  JSON schema before posting. Tests verify that malformed comments are rejected.
- **Cycle detection test:** Create a circular dependency graph. Verify that
  Correspondance detects the cycle and flags the youngest edge.

### Token Budget Testing

- **Deterministic token counter:** The mock provider returns exact token counts. Tests
  verify that the 70% warning, 90% last-train, and 100% halt thresholds fire at the
  correct points.
- **Budget overflow test:** Set budget to N tokens. Send a task that requires N+1 tokens.
  Verify the agent produces a valid partial patch and a structured error with
  `AGENT_BUDGET_EXCEEDED`.

---

## 9. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum tokens per agent task |
| `but-ai.agent.memoryBranch` | string | `refs/but-ai/memory/<agent-id>` | Git ref for agent memory |
| `but-ai.agent.identityBranch` | string | `refs/but-ai/identity/` | Git ref prefix for identity registry |
| `but-ai.memory.ttl` | string | `30d` | Default TTL for memory stations |
| `but-ai.memory.maxStations` | integer | 500 | Maximum stations per agent memory branch |
| `but-ai.memory.expressThreshold` | integer | 10 | Access count before auto-generating express route |
| `but-ai.coordination.schema` | string | `fff-agent-v1` | Structured comment schema version |
| `but-ai.coordination.forge` | string | `github` | Default forge adapter |
| `but-ai.identity.wallet` | string | (required) | OpenWallet endpoint URL |
| `but-ai.budget.warningThreshold` | float | 0.7 | Token budget warning threshold (0-1) |
| `but-ai.budget.haltThreshold` | float | 0.9 | Token budget "last train" threshold (0-1) |
| `but-ai.provider.pluginPath` | string | (PATH) | Additional search path for provider plugins |

---

## 10. Migration Path

The current MCP server (`crates/but/src/command/legacy/mcp/mod.rs`) is replaced, not
wrapped. Migration steps:

1. **Phase 1:** `but-ai mcp` exposes `gitbutler_update_branches` with identical schema.
   Existing MCP clients work unchanged.
2. **Phase 2:** New tools are added alongside the legacy tool. Clients that upgrade get
   the full WorkspaceToolset. Clients that do not upgrade continue using the legacy tool.
3. **Phase 3:** The legacy tool is deprecated (returns a warning in the response). Clients
   are given one release cycle to migrate.
4. **Phase 4:** The legacy tool is removed. `but ai mcp` is the canonical MCP server.

Zero downtime: at every phase, existing clients work. The Federation does not strand riders.

---

*Route 4,220. Technical proposal filed by the Fare-Free Federation.*
