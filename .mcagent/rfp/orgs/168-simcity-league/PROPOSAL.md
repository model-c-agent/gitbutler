# PROPOSAL.md — SimCity League

**Season:** 2026 RFP Championship
**Match Clock:** Unlimited (but we're tracking tokens like it isn't)
**Team MPI Target:** 90+

---

## Pre-Game Scouting Report

We have read the RFP the way we read a new city map: terrain first, constraints second, opportunities third. The terrain is the GitButler codebase — 70+ crates, a mature tool system, a provider abstraction that is already 80% of what we need. The constraints are the patch-based workflow, the OpenWallet mandate, and the forge-agnosticism requirement. The opportunities are in the spaces between: the memory system, the coordination protocol, and the token budget optimization.

This is a championship-level build. We are going to zone it tight, connect it well, and bring it in under budget.

---

## 1. Plugin Architecture (RFP 3.1) — The City Plan

### Approach

The `but-ai` plugin is a single Rust crate in the workspace. Think of it as a new district in an existing city — it connects to existing infrastructure (roads = `but-llm`, `but-tools`, `but-ctx`) rather than building its own.

### Design

#### District Layout (Crate Structure)

```
crates/but-ai/
  src/
    main.rs              -- City hall: entry point, mode dispatch
    cli/
      mod.rs             -- Street grid: subcommand routing
      agent.rs           -- The players
      zones.rs           -- Memory zone management
    mcp/
      mod.rs             -- Transit hub: MCP server
      bridge.rs          -- Tool registration bridge to WorkspaceToolset
    coordination/
      mod.rs             -- Inter-city transport: forge adapters
      github.rs          -- GitHub adapter (reference implementation)
      schema.rs          -- PR comment schema
    memory/
      mod.rs             -- Parks department: memory management
      zones.rs           -- Zone definitions and rules
      scoring.rs         -- Relevance scoring (the MPI of memory)
    budget/
      mod.rs             -- Treasury: token budget tracking
  Cargo.toml
```

#### Mode Dispatch

`but ai` dispatches to CLI mode. `but ai mcp` starts the MCP server. The MCP server implements `ServerHandler` via `rmcp` and registers all ten WorkspaceToolset tools plus new AI-specific tools (memory query, agent status, budget report).

#### WASI Degradation

Under WASI, plugin discovery is disabled. The plugin can still be invoked directly as `but-ai mcp` for MCP server mode. CLI mode is unavailable through the `but` parent command but available through direct invocation. We treat this like a city with reduced transit service — fewer routes, but the core network still runs.

### Trade-offs

| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| Workspace crate | Shared types, single build | Coupling | **Selected** — density wins |
| Standalone binary | Independence | Type duplication, dual builds | Rejected — sprawl |
| Feature flag in `but` | Zero overhead | Violates RFP 4.6 | Rejected — zoning violation |

### New Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.budget.perSession` | 50000 | Token budget per session |
| `but-ai.memory.baseRef` | `refs/but-ai/zones` | Memory zone storage base |
| `but-ai.forge.adapter` | `github` | Active forge adapter |
| `but-ai.forge.github.token` | (none) | GitHub API token |
| `but-ai.provider.pluginDir` | `~/.local/share/but-ai/providers` | External provider plugins |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2) — The Transit Network

### Approach

We use `but-llm` as-is. No new LLM client. The four existing providers are four transit lines — they all go to the same destinations, just via different routes. We add a plugin mechanism for new lines without rebuilding the station.

### Design

#### Provider Bridge

At startup, `LLMProvider::from_git_config()` creates the active provider. All agent interactions use `tool_calling_loop_stream` with a token callback that feeds our budget tracker. The streaming variant gives us real-time token counting — we know our budget position at every moment, like a city finance department that tracks expenditures in real time rather than waiting for quarterly reports.

#### Provider Plugin Mechanism

New providers are loaded from shared libraries in `but-ai.provider.pluginDir`. Each library exports a `create_provider` function that returns a struct implementing a simplified provider interface. The plugin ABI is versioned — we do not load plugins compiled against a different ABI version. This is like a transit standard: any bus manufacturer can build a bus, but it has to fit the bus stops.

```rust
pub trait ProviderPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn tool_calling_loop(
        &self,
        system: &str,
        messages: Vec<ChatMessage>,
        tools: &[ToolDefinition],
    ) -> Result<(String, Vec<ToolCall>)>;
    fn stream_response(
        &self,
        system: &str,
        messages: Vec<ChatMessage>,
        on_token: Box<dyn Fn(&str) + Send>,
    ) -> Result<String>;
}
```

#### MCP Tool Surface

All ten workspace tools plus:

| New Tool | Description |
|----------|-------------|
| `query_memory` | Search agent memory by zone and relevance |
| `report_budget` | Current token budget status |
| `agent_status` | Current agent state and progress |
| `coordination_status` | Cross-repo dependency status |

### Trade-offs

We considered gRPC for provider plugins (better type safety, language-agnostic). Rejected — it adds a runtime dependency and a network hop. In transit terms: we are not going to build a ferry terminal when a bridge will do.

---

## 3. The But Agent (RFP 3.3) — The Players

### Approach

The agent is a three-player team executing practiced plays. Each player has a position (see AGENTS.md), and the team coordinates through a shared game clock (token budget) and playbook (named plays for common patterns).

### Design

#### Game Flow (Task Execution)

```
KICKOFF: Task arrives (PR body, CLI arg, issue reference)
    |
    v
SCOUTING (Gridlock): Read task, query memory zones, assess terrain
    |
    v
ZONING (Densifier): Plan implementation — which zones, what density, what structure
    |
    v
BUILD (Densifier): Produce INDEX.patch + COMMIT.msg
    |      |
    |      v (concurrent)
    |   CONNECT (Gridlock): Wire up coordination — PR comments, dependency updates
    |
    v
SUSTAINABILITY CHECK (Greenline): Review patch for long-term health, update memory
    |
    v
FINAL WHISTLE: Output signed commit or partial result
```

#### Patch Production

Densifier is the primary patch producer. He uses `GetProjectStatus` to survey the workspace, `GetBranchChanges` to understand existing work, and produces a complete INDEX.patch in a single concentrated output. The patch is a unified diff against the current index. Densifier does not make direct file edits, does not call `git commit`, and does not call `but commit`. He produces patches. Period.

#### Branch Naming

We extend the `s01.s04` convention with a zone prefix:

```
zone/<zone-type>/<dependency-chain>
```

Examples:
- `zone/residential/s01` — first step, residential zone (personal/identity changes)
- `zone/commercial/s01.s03` — third step, commercial zone (shared library changes), depends on s01
- `zone/industrial/s02` — second step, industrial zone (compute-intensive, batch operations)

The zone prefix maps to our memory system (see Section 5) and provides immediate visual information about the nature of the work.

#### Token Budget Enforcement

Greenline monitors the budget continuously. Thresholds:

| Level | Trigger | Action |
|-------|---------|--------|
| Green | < 70% used | Normal play |
| Yellow | 70-85% used | "Two-minute warning" — agents must estimate remaining work |
| Red | 85-95% used | "Overtime" — wind-down, produce partial result |
| Final | > 95% used | "Final whistle" — halt, output whatever is complete |

Budget enforcement is not a crude kill switch. When Greenline calls "overtime," the team executes a practiced play: each agent saves its current state to memory and produces whatever partial output is valid. This is like a tournament where the clock runs out — you are scored on what you built, not what you planned to build.

#### Progress Reporting

```json
{
  "play": "The Grid",
  "quarter": 2,
  "score": {
    "transit": 82,
    "density": 88,
    "sustainability": 79
  },
  "clock": {
    "budget": 50000,
    "used": 28400,
    "remaining": 21600,
    "status": "green"
  },
  "agents": {
    "gridlock": { "status": "connecting", "tokens": 9200 },
    "densifier": { "status": "building", "tokens": 12800 },
    "greenline": { "status": "monitoring", "tokens": 6400 }
  }
}
```

### Trade-offs

We considered a single-agent design (simpler, fewer tokens). Rejected — a single agent cannot specialize, and specialization is what wins championships. The three-agent overhead (roughly 20% more tokens than a single agent) is offset by higher quality output, just as a three-person relay team covers more ground than one person running three times.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4) — Inter-City Transit

### Approach

PRs are transit lines between cities (repos). Each PR connects two stations (branches) and carries passengers (structured messages). The coordination protocol is a transit map: clear routes, defined stops, and a schedule that every participant can read.

### Design

#### Forge Adapter Interface

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, params: &PrParams) -> Result<PrRef>;
    fn add_comment(&self, pr: &PrRef, msg: &CoordinationMessage) -> Result<()>;
    fn read_comments(&self, pr: &PrRef) -> Result<Vec<CoordinationMessage>>;
    fn set_labels(&self, pr: &PrRef, labels: &[&str]) -> Result<()>;
    fn get_status(&self, pr: &PrRef) -> Result<PrStatus>;
}
```

Reference implementation: GitHub REST API. The adapter is stateless — each call authenticates independently. No persistent connections, no webhooks, no background polling. You check the schedule when you need to know when the next train arrives.

#### PR Comment Schema

Messages are embedded in PR comments with a zone-labeled schema:

```json
{
  "schema": "simcity-league/coord/v1",
  "zone": "commercial",
  "sender": { "agent": "gridlock", "org": "168-simcity-league" },
  "type": "task | status | dependency | handoff | budget",
  "timestamp": "2026-03-28T14:00:00Z",
  "payload": { ... }
}
```

The `zone` field classifies the message using our memory system's zoning. Commercial-zone messages are shared context; industrial-zone messages are compute-heavy operations.

#### Cross-Repo Dependencies

Dependencies are tracked in a "transit map" stored at `refs/but-ai/zones/commercial/deps.json`:

```json
{
  "routes": [
    {
      "from": { "repo": "gitbutler/but-tools", "pr": 42 },
      "to": { "repo": "gitbutler/but-ai", "pr": 7 },
      "status": "active",
      "zone": "commercial"
    }
  ]
}
```

When a dependency is resolved (PR merged), the transit map is updated and a status message is posted to all downstream PRs.

### Trade-offs

| Option | Verdict | Reason |
|--------|---------|--------|
| Webhook notifications | Rejected | Requires a server — not on our transit map |
| Polling on schedule | **Selected** | Simple, stateless, works with any forge |
| Central coordination repo | Rejected | Single point of failure, like a single transit hub |

---

## 5. Agent Memory and Identity (RFP 3.5) — Zoning-Map Memory

### Approach

Memory is organized into zones, just like a city. Each zone has different rules about what can be stored there, how long it persists, how it interacts with adjacent zones, and who can access it. This is not a metaphor bolted onto a key-value store. The zoning rules are the data model.

### Design

#### Zone Definitions

| Zone | Contents | Persistence | Access | Adjacent Zones |
|------|----------|-------------|--------|---------------|
| **Residential** | Personal agent memory — working context, current task state, preferences | Short (session TTL) | Agent-private | Commercial |
| **Commercial** | Shared team memory — API patterns, code conventions, team decisions | Medium (30-day TTL) | Team-shared | Residential, Industrial |
| **Industrial** | Compute-intensive artifacts — large diffs, batch results, analysis outputs | Medium (14-day TTL) | Team-shared, write-restricted | Commercial |
| **Park** | Long-term knowledge — architectural decisions, precedents, lessons learned | Long (180-day TTL) | Read-public, write-restricted | All zones |
| **Historic** | Archived memory — expired entries preserved for reference | Permanent | Read-only | Park |

#### Storage Structure

```
refs/but-ai/zones/
  residential/<agent-id>/
    <entry-hash>.json
  commercial/
    <entry-hash>.json
    deps.json            -- Cross-repo dependency transit map
  industrial/
    <entry-hash>.json
  park/
    <entry-hash>.json
  historic/
    <entry-hash>.json
```

#### Zoning Rules (Adjacency Constraints)

Just as a city does not allow a factory next to a school, the memory system enforces adjacency rules:

1. **Residential entries cannot reference Industrial entries directly.** Personal working context should not depend on compute-heavy artifacts. If an agent needs industrial data, it must go through the Commercial zone (create a summary in commercial, reference the summary from residential).

2. **Industrial entries cannot be created without a Commercial entry justifying them.** You cannot build a factory without a commercial permit. Every compute-heavy operation must have a shared-context entry explaining why it exists.

3. **Park entries can only be created by promotion from Commercial.** You cannot create long-term knowledge directly. It must first exist as shared team memory and then be promoted when the team agrees it has lasting value.

4. **Historic entries are immutable.** Once archived, a memory entry cannot be modified. It can only be annotated (a new entry in Park that references the historic entry and adds context).

#### Relevance Scoring

Relevance is scored using a zone-weighted model:

```
score = (keyword_sim * 0.30) + (zone_proximity * 0.25) + (recency * 0.25) + (access_freq * 0.20)
```

- **Keyword similarity** (30%): BM25 scoring between query terms and entry content/keywords.
- **Zone proximity** (25%): Entries in the same zone as the querying agent's current work score higher. Adjacent zones score next highest. Non-adjacent zones score lowest.
- **Recency** (25%): Recently created or accessed entries score higher.
- **Access frequency** (20%): Entries accessed across multiple sessions represent established patterns.

#### Expiration

Each zone has a default TTL (see table above). Entries can have custom TTLs that are shorter but not longer than the zone maximum. When an entry expires, it is not deleted — it is moved to the Historic zone. The Historic zone is append-only and never expires.

Expiration is checked at the start of each session, not continuously. This is a batch operation (like a city's annual reassessment), not a real-time process.

#### Compaction Survival

When context is compacted, each agent writes a "snapshot" to its Residential zone:

```json
{
  "type": "compaction_snapshot",
  "zone": "residential",
  "content": "Working on auth refactor. Patch 60% complete. Key finding: tokens stored in plaintext in config.toml. Next step: credential rotation.",
  "references": ["commercial/abc123", "park/def456"]
}
```

Rehydration reads the snapshot and its referenced entries from Commercial and Park zones, reconstructing working context from the zone hierarchy.

#### Long-Term Storage

The Park zone serves as long-term memory. Promotion to Park requires consensus: at least two agents must flag an entry for promotion. Park entries are searchable across sessions and, via forge-based references, across repositories.

Cross-repo memory sharing uses the coordination protocol: an agent can reference a Park entry from another repo in a PR comment, and the receiving agent can fetch it via the forge adapter. Memory stays in its home repo — no copying, no central store.

#### Identity

Agent identity is stored in `refs/but-ai/zones/park/identity/<agent-id>.json`:

```json
{
  "agent_id": "gridlock",
  "display_name": "Gridlock (Maya Torres)",
  "org": "168-simcity-league",
  "position": "Transit Architect",
  "capabilities": ["coordination", "forge_adapters", "pr_communication"],
  "authorization": {
    "branches": ["zone/*", "feat/*"],
    "max_patch_lines": 800,
    "signing_authority": true
  },
  "openwallet_key_id": "scl-gridlock-2026",
  "seasons_played": 7,
  "created": "2021-09-01T00:00:00Z"
}
```

Identity is in the Park zone because it is long-lived, read-public, and foundational.

### Trade-offs

| Option | Verdict | Reason |
|--------|---------|--------|
| Flat key-value memory | Rejected | No structure = sprawl. Memory without zoning is a suburb. |
| Embedding-based search | Rejected | Opaque, costly, and our domain is structured enough for keyword+zone scoring |
| Central memory service | Rejected | Violates git-native requirement. We build in the city, not in the cloud. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6) — Building Permits

### Approach

Every commit is a building permit. It proves who built what, where they were authorized to build, and that the work was inspected. No permit, no building. No signature, no commit.

### Design

#### Key Provisioning

Each agent is provisioned an OpenWallet key at identity creation. The key is referenced in the identity record and used for all commits. Key management:

| Event | Action | Analogy |
|-------|--------|---------|
| Provisioning | New key generated, registered in identity | Contractor license issued |
| Rotation | New key generated, old key enters 14-day overlap | License renewal |
| Revocation (compromise) | Key immediately invalidated, all sessions halted | License suspended |
| Revocation (routine) | Key invalidated at session end | License expired |

#### Authorization Model

Authorization is zone-based, matching the memory system:

```json
{
  "agent": "densifier",
  "zones": {
    "residential": { "access": "read", "commit": false },
    "commercial": { "access": "read-write", "commit": true },
    "industrial": { "access": "read-write", "commit": true },
    "park": { "access": "read", "commit": false }
  },
  "branch_patterns": ["zone/commercial/*", "zone/industrial/*", "feat/*"],
  "max_patch_lines": 500
}
```

Agents can only commit to zones they are authorized for. Greenline can commit to Park (promotions). Densifier can commit to Commercial and Industrial (code). Gridlock can commit to Commercial (coordination).

#### Verification

A signed commit is verified by:
1. Extract signing key from commit.
2. Look up agent identity in `refs/but-ai/zones/park/identity/`.
3. Check authorization policy against the commit's target branch and zone.
4. Verify patch size is within the agent's limit.
5. Verify the commit timestamp falls within the key's validity period.

### Trade-offs

Zone-based authorization adds complexity versus a simple branch-pattern model. We chose it because it aligns the authorization model with the memory model — one system of zones governs both access and identity. In city terms: the zoning map is both the land use plan and the building code.

---

## 7. Token Budget (RFP 3.7) — The City Treasury

### Budget Table

Frontier model: Claude Opus. Typical task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,800 | 0 | Once per session | Three agent profiles, 10 tool descriptions, zone rules |
| **Task ingestion** | 2,200 | 400 | Once per task | PR body, issue, branch metadata, memory zone query |
| **Planning (zoning)** | 1,800 | 1,000 | Once per task | Densifier's structural plan |
| **Tool call (per call)** | 700 | 350 | 5 per task | Status, branch ops, commit details |
| **Patch generation** | 2,000 | 3,200 | Once per task | Densifier's single-shot output |
| **Commit message** | 300 | 250 | Once per task | With zone and dependency metadata |
| **Memory zone query** | 500 | 200 | 2 per task | Zone-specific relevance search |
| **Coordination event** | 900 | 450 | 2 per task | Gridlock's forge operations |
| **Sustainability review** | 1,200 | 600 | Once per task | Greenline's impact assessment |
| **TOTAL (typical task)** | **16,700** | **9,050** | -- | **25,750 total tokens** |

### Budget Analysis

Our total is 25,750 tokens for a typical task. This is lean — we budgeted the way Densifier zones: maximum value per token. The three-agent overhead is approximately 3,500 tokens (14%) compared to a single-agent approach. We consider this the "infrastructure investment" — it costs more upfront but produces higher-quality output with fewer failures.

### Optimization Plays

1. **Zone-scoped memory loading.** Only load memory entries from relevant zones. A transit task does not need industrial memory.
2. **Densifier's single-shot approach.** No iteration means no wasted tokens on discarded drafts.
3. **Greenline's budget monitoring.** Real-time awareness prevents overspending. Teams that do not track their budget always overspend, like cities without a finance department.

---

## 8. Testing Strategy — Scrimmage Protocol

### 8.1 Provider-Agnostic Testing

A `MockProvider` implements the `ProviderPlugin` trait with deterministic responses. Test fixtures define scenarios: "when the agent calls GetProjectStatus, return this JSON." All four providers are tested through the same fixture set — the provider is a variable, the behavior is the constant.

### 8.2 Patch Workflow Testing

Round-trip tests: create workspace state, run agent, capture INDEX.patch, apply patch to clean workspace, verify result. We also test the "contested workspace" scenario: another agent modifies the workspace between patch generation and application. The patch must either apply cleanly or fail with a clear error — no silent corruption.

### 8.3 Cross-Repo Coordination Testing

A `MockForge` implements `ForgeAdapter` with in-memory PR storage. Tests simulate multi-repo scenarios: create PRs in two mock repos, post coordination messages, resolve dependencies, verify transit map updates. The mock forge records all API calls for verification.

### 8.4 Token Budget Testing

The `MockProvider` reports configurable token counts. Tests verify:
- Green/Yellow/Red/Final threshold triggers
- Partial patch production at budget exhaustion
- Budget reporting accuracy in structured output
- Greenline's sustainability review costs match estimates

### 8.5 Scrimmage Games

Full integration tests that simulate a complete task: task ingestion through coordination through patch production through signing. These are our scrimmages — played on a test field with mock providers and forges, but running the real agent code at game speed.

---

## 9. Trade-Off Summary — Season Recap

| Decision | Chose | Over | Why |
|----------|-------|------|-----|
| Crate location | Workspace | Standalone | Density: shared types, single build |
| Agent count | 3 | 1 | Specialization wins championships |
| Memory model | Zoning-map | Flat KV / embeddings | Structure, adjacency rules, sustainability |
| Provider plugins | Shared libraries | WASM / gRPC | Simplicity, no runtime dep |
| Review process | Sustainability check | Full sequential review | Balanced speed and quality |
| Iteration model | Single-shot (Densifier) | Iterative | Token efficiency, all-or-nothing clarity |
| Cross-repo coordination | Polling via forge API | Webhooks | Stateless, no server, works everywhere |
| Authorization model | Zone-based | Branch-pattern only | Unified with memory model |

---

*FINAL SCORE: Proposal complete. Under budget. Clock has 0 tokens remaining because we spent them all on this document, and it was worth every one.*

*"You don't win by building the biggest city. You win by building the best one."*
— Greenline, Season 5 halftime talk
