# PROPOSAL.md — Papadopoulos Telecom

*"Papou Kostas connected two villages with copper wire. We are connecting repositories with code. Different wire. Same family. The line holds."*

---

## A Word Before We Begin

We are a family business. We have been connecting people for four generations. We are not the fastest company. We are not the most innovative company. We are the company whose infrastructure is still running after everyone else's has been replaced three times.

We bring to this RFP what we bring to every project: reliability, warmth, and the understanding that technology serves people, not the other way around. Our proposal is built the way we build everything: connection by connection, tested by Eleni, monitored by Nikos, and signed off over Sunday lunch.

---

## 1. Plugin Architecture (RFP 3.1) — Laying the Line

### Approach

Sofia designs. Kostas builds. The `but-ai` plugin is a Rust crate in the workspace, wired into the existing infrastructure the way we wire a new island into the network: using the existing backbone, not building a parallel one.

### Design

```
crates/but-ai/
  src/
    main.rs              -- The exchange: call routing (mode dispatch)
    copper/              -- Core infrastructure (Kostas's layer)
      mod.rs             -- Foundational types and traits
      context.rs         -- Environment context handling
    fiber/               -- Quality and reliability (Eleni's layer)
      mod.rs
      testing.rs         -- Verification utilities
      redundancy.rs      -- Fallback mechanisms
    wireless/            -- Operations (Nikos's layer)
      mod.rs
      monitoring.rs      -- System health, budget tracking
      provider.rs        -- Provider management
    fiveg/               -- Modern integration (Sofia's layer)
      mod.rs
      cli.rs             -- CLI subcommands
      mcp.rs             -- MCP server
      forge.rs           -- Forge adapter
    exchange/            -- Telephone-exchange memory system
      mod.rs
      switchboard.rs     -- Memory routing
      connections.rs     -- Direct and operator-assisted connections
      partyline.rs       -- Shared context
      longdistance.rs    -- Cross-repo memory
  Cargo.toml
```

The layers are named for the generations, not because we are sentimental (though we are), but because the names communicate the purpose. The copper layer is foundational — it carries everything. The fiber layer is quality — it verifies and documents. The wireless layer is operational — it monitors and maintains. The 5G layer is integration — it connects to the outside world.

### Two Modes

- **CLI mode** (`but ai`): For working at the command line. Nikos's preferred interface — direct, no frills.
- **MCP mode** (`but ai mcp`): For integration with AI clients. Sofia's preferred interface — standards-based, interoperable.

Both modes read the environment variables (`BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`) at startup and store them immutably. Nikos's rule: "Don't change the configuration while the system is running. You wouldn't retune an antenna while it's transmitting."

### WASI

Under WASI, plugin discovery does not work. The plugin can still be invoked directly as `but-ai mcp`. Eleni's rule: "Always have a backup route." Direct invocation is the backup route.

### Git Config Keys

| Key | Default | Purpose | Who Uses It |
|-----|---------|---------|-------------|
| `but-ai.exchange.baseRef` | `refs/but-ai/exchange` | Memory exchange root | Sofia |
| `but-ai.agent.tokenBudget` | 50000 | Session budget | Nikos |
| `but-ai.agent.reserve` | 5000 | Reserved for post-session cleanup | Nikos |
| `but-ai.forge.adapter` | `github` | Active forge | Sofia |
| `but-ai.forge.github.token` | (none) | GitHub API token | Sofia |
| `but-ai.provider.pluginDir` | `~/.local/share/but-ai/providers` | External providers | Nikos |
| `but-ai.reliability.fallbackProvider` | (none) | Backup provider | Eleni |

The `fallbackProvider` key is Eleni's contribution. If the primary provider fails, the system switches to the fallback. Eleni has been insisting on redundant paths since the Corfu Incident, and she is right.

### Trade-offs

| Decision | Chosen | Why |
|----------|--------|-----|
| Workspace crate | Yes | Uses existing backbone. We do not build parallel networks. |
| Standalone binary | No | Would require duplicating shared types. Waste of wire. |
| Feature flag in but | No | Violates RFP 4.6. We do not modify other people's infrastructure. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2) — The Backbone

### Approach

The four LLM providers are four carriers on the network. `but-llm` is the backbone that connects them all. We use the backbone as-is — Nikos's philosophy: "If the backbone works, don't touch the backbone."

### Design

Nikos handles provider management. At startup, `LLMProvider::from_git_config()` selects the active provider. `tool_calling_loop_stream` is the primary interface — the streaming callback feeds Nikos's budget tracker.

#### Provider Plugins

New providers are added as shared libraries in `but-ai.provider.pluginDir`. Each one must implement:

```rust
pub trait TelecomProvider: Send + Sync {
    fn name(&self) -> &str;
    fn signal_strength(&self) -> ProviderHealth;
    fn connect(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &[ToolSchema],
        on_token: Option<Box<dyn Fn(&str) + Send>>,
    ) -> Result<ProviderResponse>;
}
```

The `signal_strength` method returns the provider's current health status. Nikos monitors this continuously. If signal strength drops below threshold, he switches to the fallback provider. The same logic he uses for his wireless towers: if the signal drops, you do not wait for complaints — you switch to the backup antenna.

#### Fallback Chain

Providers are arranged in a fallback chain:

```
Primary (from git config)
  -> Fallback (from but-ai.reliability.fallbackProvider)
    -> Offline mode (structured error, no LLM calls)
```

Eleni insisted on the third level. "What happens when both providers are down? The system must still tell you what happened. A dead line is worse than a busy signal."

#### MCP Tool Registration

All ten workspace tools registered through `tool_router`. The bridge translates between `Tool` trait's JSON Schema and MCP declarations. Nikos verifies each registration at startup — a faulty tool registration is like a faulty connection in the exchange: it does not just fail, it corrupts the routing for everyone.

### Trade-offs

We considered building a provider abstraction on top of `but-llm`. Nikos vetoed it: "You do not put a switch between a switch and the backbone. It adds latency and a failure point." We use `but-llm` directly.

---

## 3. The But Agent (RFP 3.3) — The Exchange

### Approach

The agent operates as a family: four members, each contributing according to their expertise, coordinated through the morning coffee and the Sunday lunch.

### Design

#### The Daily Work

```
MORNING COFFEE (All)
  - Review system state (Nikos)
  - Read task description (Sofia)
  - Plan the work (Kostas and Sofia together)
  - Allocate token budget (Nikos, with Yiayia Maria's constraint)

WORK SESSION
  1. Kostas builds the foundation (INDEX.patch for core changes)
  2. Eleni reviews Kostas's patch (verification, redundancy check)
  3. Sofia handles coordination (forge messages, memory updates)
  4. Nikos monitors everything (budget, provider health, system state)

EVENING REVIEW (All)
  - Summary of work completed (Sofia)
  - System health report (Nikos)
  - Documentation update (Eleni)
  - Quality assessment (Eleni)
```

#### The Patch

Kostas produces INDEX.patch + COMMIT.msg. He does not edit files directly. He does not call `git commit` or `but commit`. He produces the patch, the same way Papou Kostas produced a telephone connection: the wire is laid, the connection is tested, and only then is the circuit closed.

Kostas's commit messages are distinctive:

```
Add credential rotation mechanism for authentication module

The authentication module needed a way to rotate credentials
without dropping active connections. We built this the way we
build all connections: the new path is established before the
old path is released. No user loses their session during rotation.

The mechanism is configured via git config (but-ai.auth.rotationInterval).
Default: 72 hours. This is conservative by modern standards but
appropriate for the systems we serve. Papou Kostas maintained
his copper lines on a weekly schedule. Some things benefit from
regular attention.

Reviewed by: Eleni (redundancy verified)
Monitored by: Nikos (budget within allocation)
Coordinated by: Sofia (upstream dependency on but-tools#42 resolved)
```

#### Branch Naming

We name branches for what they connect, not what they contain:

```
connect/<source>-to-<destination>/<dependency-chain>
```

Examples:
- `connect/auth-to-rotation/s01` — Connecting the auth module to the new rotation mechanism
- `connect/provider-to-tools/s01.s02` — Connecting the provider layer to the tool system, s02 depends on s01
- `connect/memory-to-forge/s03` — Connecting the memory exchange to the forge adapter

This naming convention makes the dependency graph readable as a network map. You can look at the branches and see what is connected to what.

#### Budget Management

Nikos manages the budget the way he manages the family's finances: carefully, transparently, and with a reserve for emergencies.

| Budget Zone | Allocation | Purpose |
|------------|------------|---------|
| Foundation | 35% | Kostas's core work |
| Quality | 20% | Eleni's reviews and documentation |
| Operations | 15% | Nikos's monitoring and maintenance |
| Coordination | 20% | Sofia's forge and memory work |
| Reserve | 10% | Emergency fund (never touched unless necessary) |

Budget thresholds:

| Level | Trigger | What Nikos Does |
|-------|---------|----------------|
| Comfortable | < 60% used | Normal operations |
| Careful | 60-80% | Nikos tightens allocation, non-essential work deferred |
| Tight | 80-95% | Wind down, complete current work, save state |
| Empty | > 95% | Stop. Save everything. Report to the family. |

The reserve is Nikos's insurance policy. In thirty years of running the network, he has learned that something always goes wrong, and having a reserve is cheaper than not having one.

#### Progress Reporting

```json
{
  "session": "pt-2026-0328-001",
  "family_status": "working",
  "agents": {
    "kostas": { "status": "building", "task": "credential rotation foundation", "tokens": 8200 },
    "eleni": { "status": "reviewing", "task": "Kostas patch verification", "tokens": 4100 },
    "nikos": { "status": "monitoring", "task": "system health", "tokens": 2800 },
    "sofia": { "status": "coordinating", "task": "upstream dependency resolution", "tokens": 5400 }
  },
  "budget": { "total": 50000, "used": 20500, "reserve": 5000, "remaining": 24500 },
  "network_health": "all connections nominal"
}
```

### Trade-offs

Four agents cost more than one. The family overhead — morning coffee, reviews, documentation — adds approximately 7,000 tokens (15%) per session. We consider this the cost of reliability. A solo agent is faster but has no redundancy. When the solo agent fails, everything fails. When Kostas's patch has an issue, Eleni catches it. When the budget runs tight, Nikos finds the optimization. The family is the redundancy.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4) — Long-Distance Calls

### Approach

Sofia handles all cross-repo coordination. PRs are telephone calls between repos. PR comments are conversations on the line. The forge adapter is the telephone exchange that routes the calls.

### Design

#### Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn dial(&self, params: &CallParams) -> Result<CallRef>;  // Create PR
    fn speak(&self, call: &CallRef, msg: &Message) -> Result<()>;  // Add comment
    fn listen(&self, call: &CallRef) -> Result<Vec<Message>>;  // Read comments
    fn check_line(&self, call: &CallRef) -> Result<LineStatus>;  // PR status
}
```

We name our methods for what they do, not what protocol they use. `dial`, `speak`, `listen`, `check_line` — these are the operations of a telephone call, and they map exactly to the operations of PR-based coordination. GitHub reference implementation provided.

#### Message Schema

```json
{
  "schema": "papadopoulos-telecom/exchange/v1",
  "line_type": "direct | operator | party | longdistance",
  "caller": {
    "agent": "sofia",
    "family": "199-papadopoulos-telecom"
  },
  "type": "greeting | update | request | handoff | budget",
  "timestamp": "2026-03-28T09:00:00Z",
  "payload": { ... }
}
```

The `line_type` indicates the routing: direct messages go to a specific agent, operator-assisted messages need routing help, party-line messages are broadcast to all agents, and long-distance messages cross repository boundaries.

#### Message Types

**Greeting** (task assignment):
```json
{
  "to": "tank-agent (org 188)",
  "message": "Good morning. We have a shared dependency on the tool registration interface. Shall we coordinate? We are available to review your changes.",
  "suggested_schedule": "We will check for your response at our next work session."
}
```

**Update** (status):
```json
{
  "task": "credential rotation foundation",
  "status": "complete",
  "notes": "Kostas finished the foundation. Eleni has verified it. The patch is ready for integration. Tokens used: 14,200.",
  "warmth": "We hope your work is going well also."
}
```

We include warmth in our messages because we are a family business, and business between families should be warm. This is not a protocol requirement. It is a Papadopoulos requirement.

#### Cross-Repo Dependencies

Dependencies tracked in `refs/but-ai/exchange/longdistance/routes.json`:

```json
{
  "routes": [
    {
      "from": { "repo": "gitbutler/but-ai", "pr": 7 },
      "to": { "repo": "gitbutler/but-tools", "pr": 42 },
      "status": "awaiting",
      "established": "2026-03-28T09:00:00Z",
      "notes": "We need the updated tool registration interface before we can complete the MCP bridge."
    }
  ]
}
```

Sofia checks dependencies at the start of each work session. If a dependency is unresolved, she sends a polite follow-up. If it has been resolved, she updates the route and proceeds. Patience is a family trait.

### Trade-offs

| Option | Verdict | Reason |
|--------|---------|--------|
| Webhooks | Rejected | Requires infrastructure we do not control. We learned from the 5G Spectrum Problem: do not depend on systems you do not own. |
| Polling | **Selected** | Works with any forge. No dependencies. Like checking the mailbox — simple and reliable. |
| Central coordination service | Rejected | Single point of failure. The Corfu Incident taught us: always have a backup route. |

---

## 5. Agent Memory and Identity (RFP 3.5) — The Telephone Exchange

### Approach

Memory is organized as a telephone exchange. Connections between memories are explicit, routable, and maintained with the same care that we maintain our physical network connections.

This is **telephone-exchange memory**: memories are connected through a switchboard with four types of connections, each with different routing characteristics and costs.

### Design

#### The Four Connection Types

**1. Direct Connections (Frequently Accessed Pairs)**

A direct connection is a dedicated line between two memory entries that are frequently accessed together. Like the dedicated line between the Pyrgos exchange and the Katakolo exchange — used so often that a permanent connection is more efficient than routing through the switchboard each time.

- **Storage:** `refs/but-ai/exchange/direct/<connection-id>.json`
- **TTL:** 30 days (renewed automatically when both endpoints are accessed)
- **Cost:** Low retrieval cost (pre-routed)
- **Example:** A direct connection between "credential rotation pattern" and "session management trait" because they are always used together

```json
{
  "connection_id": "dc-auth-session-001",
  "endpoint_a": { "key": "credential-rotation-pattern", "content": "Rotate credentials without dropping connections. New path before old path releases." },
  "endpoint_b": { "key": "session-management-trait", "content": "SessionProvider trait with renew() and invalidate(). Extensible for credential rotation." },
  "established": "2026-03-20T00:00:00Z",
  "last_used": "2026-03-28T10:00:00Z",
  "usage_count": 14,
  "ttl_expires": "2026-04-27T00:00:00Z"
}
```

**2. Operator-Assisted Connections (Rare Queries)**

An operator-assisted connection requires active routing — the switchboard (relevance scoring system) must find the right memory entry for an unfamiliar query. Like calling a number you have never called before: the operator looks it up in the directory and connects you.

- **Storage:** Entries scattered across `refs/but-ai/exchange/directory/<hash>.json`
- **TTL:** 60 days (individual entries)
- **Cost:** Higher retrieval cost (requires relevance scoring)
- **Example:** Querying "how does the codebase handle concurrent writes?" when you have never encountered this topic before

```json
{
  "entry_id": "dir-concurrent-writes-001",
  "content": "The codebase uses a Context struct with interior mutability via Arc<Mutex<>>. Concurrent writes are serialized at the Context level. Tool calls hold the lock for the duration of the call.",
  "keywords": ["concurrent", "writes", "context", "mutex", "serialization"],
  "directory_category": "architecture",
  "created": "2026-02-15T00:00:00Z",
  "ttl_expires": "2026-04-15T00:00:00Z",
  "accessed_count": 3
}
```

**3. Party Lines (Shared Context)**

A party line is a shared memory space that all agents can access simultaneously. Like the party lines of early telephony: multiple subscribers on the same circuit, each able to hear what the others are saying. Party-line memory holds the team's shared context — the current task description, the workspace state summary, the active dependencies.

- **Storage:** `refs/but-ai/exchange/partyline/<topic>.json`
- **TTL:** Session duration (party lines are established at the morning coffee and dissolved at the evening review)
- **Cost:** Very low (pre-loaded at session start)
- **Example:** The current task description, the workspace state, the active branch list

```json
{
  "topic": "current-task",
  "content": "Implement credential rotation for the authentication module. Three files: auth/mod.rs, auth/rotation.rs (new), config/settings.rs. Budget: 50,000 tokens. Upstream dependency: but-tools#42.",
  "participants": ["kostas", "eleni", "nikos", "sofia"],
  "established": "2026-03-28T09:00:00Z",
  "ttl": "session"
}
```

**4. Long-Distance Connections (Cross-Repo Memory)**

A long-distance connection links memory entries across repositories. Like the submarine cables that connect the islands: expensive to establish, valuable once running, and requiring special routing.

- **Storage:** `refs/but-ai/exchange/longdistance/<route-id>.json`
- **TTL:** 90 days
- **Cost:** Highest (requires forge API calls to access)
- **Example:** A reference to an architectural pattern in the but-tools repo that is relevant to the but-ai plugin

```json
{
  "route_id": "ld-but-tools-pattern-001",
  "local_endpoint": "credential-rotation-pattern",
  "remote": {
    "repo": "gitbutler/but-tools",
    "ref": "refs/but-ai/exchange/directory/tool-registration-pattern-001"
  },
  "notes": "The tool registration pattern in but-tools uses the same trait extension approach we use for credential rotation. Relevant for maintaining consistency.",
  "established": "2026-03-25T00:00:00Z",
  "last_accessed": "2026-03-28T10:00:00Z"
}
```

#### The Switchboard (Relevance Scoring)

When an agent queries memory, the switchboard routes the query:

1. **Check direct connections first.** If the query matches an endpoint of an existing direct connection, return both endpoints. This is the fastest path — the connection is already established.

2. **Check party lines.** If the query matches a party-line topic, return the shared context. This is the second-fastest path — party lines are pre-loaded.

3. **Route through the directory.** For queries that do not match direct connections or party lines, the switchboard performs a directory lookup using relevance scoring:

```
score = (keyword_match * 0.30) + (connection_strength * 0.25) + (recency * 0.25) + (directory_category * 0.20)
```

- **Keyword match** (30%): BM25 similarity between query and entry content/keywords.
- **Connection strength** (25%): Entries that are endpoints of direct connections score higher — they are "well-connected" in the network, indicating established relevance.
- **Recency** (25%): Recently created or accessed entries score higher.
- **Directory category** (20%): Entries in the same category (architecture, operations, coordination) as the querying agent's current work score higher.

4. **Long-distance lookup.** If no local results are sufficient, and the query context suggests cross-repo relevance, the switchboard routes through the long-distance connections. This is the most expensive path and is used only when necessary.

#### Connection Promotion

Connections are promoted based on usage:

- **Directory entry accessed 5+ times for the same pair** -> Automatic promotion to direct connection. The switchboard notices the pattern and establishes a permanent line.
- **Direct connection unused for 14 days** -> Downgraded back to directory entries. The permanent line is disconnected to free routing capacity.
- **Party-line topic persists across 3+ sessions** -> Promoted to directory entry with longer TTL. What was temporary shared context becomes established knowledge.

This promotion system is self-maintaining. The exchange learns which connections are valuable and strengthens them, the way a telephone network adds capacity on busy routes.

#### Expiration

| Connection Type | Default TTL | Renewal Condition |
|-----------------|-------------|-------------------|
| Direct | 30 days | Renewed when both endpoints accessed |
| Directory | 60 days | Renewed on access |
| Party line | Session | Dissolved at session end (unless promoted) |
| Long-distance | 90 days | Renewed on access |

Expired connections are logged in `refs/but-ai/exchange/archive/` — Eleni's insistence. "You do not throw away cable route maps. You never know when you will need to lay a new cable along the same path."

#### Compaction Survival

When context is compacted, Sofia creates a "connection summary" on the party line:

```json
{
  "topic": "compaction-summary",
  "content": "Working on credential rotation. Foundation (Kostas) complete. Review (Eleni) in progress. Key connections: direct/dc-auth-session-001, directory/dir-concurrent-writes-001. Upstream dependency but-tools#42 resolved.",
  "active_connections": ["dc-auth-session-001", "dir-concurrent-writes-001"],
  "participants": ["kostas", "eleni", "nikos", "sofia"]
}
```

Rehydration:
1. Read the party-line compaction summary
2. Re-establish the referenced direct connections
3. Query the directory for any entries matching the current task keywords
4. Resume the work session

The compaction summary is a party line because it is shared context — all agents need it to resume work. The party line is the fastest retrieval path, so rehydration is as fast as possible.

#### Identity

Agent identity stored in `refs/but-ai/exchange/directory/identity/<agent>.json`:

```json
{
  "agent_id": "kostas",
  "family_name": "Papadopoulos",
  "generation": "first",
  "named_for": "Konstantinos 'Papou Kostas' Papadopoulos (1919-2004)",
  "role": "Core Architecture",
  "specialty": "Foundational systems, reliability",
  "capabilities": ["architecture", "patch_generation", "reliability"],
  "authorization": {
    "branches": ["connect/*", "feat/*"],
    "max_patch_lines": 600,
    "signing_authority": true
  },
  "openwallet_key_id": "pt-kostas-2026",
  "family_motto": "The line holds.",
  "created": "2024-01-01T00:00:00Z"
}
```

### Trade-offs

| Decision | Chosen | Why |
|----------|--------|-----|
| Telephone-exchange model | Yes | Explicit connections, routable queries, self-maintaining promotion. Matches our mental model. |
| Flat key-value | No | No connection structure. Like a phone book with no exchange — you can look up numbers but you cannot call anyone. |
| Embedding-based search | No | Opaque routing. We need to understand every connection. Nikos cannot maintain what he cannot inspect. |
| Central memory service | No | Single point of failure. See: Corfu Incident. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6) — The Family Seal

### Approach

Every commit is signed by the family member who produced it. The signature is the family's guarantee: this work was done by us, reviewed by us, and meets our standards.

### Key Management

Each agent is provisioned an OpenWallet key. Sofia handles provisioning (she is the technologist). Nikos monitors key health (he is the operator).

| Event | Process | Family Analogy |
|-------|---------|----------------|
| Provisioning | Sofia generates key, registers in identity | Welcoming a new family member to the business |
| Rotation | Annual, on the anniversary of the agent's creation | Renewing the family's annual telecom license |
| Revocation (compromise) | Immediate. All sessions halted. Family meeting. | Emergency — like a severed cable. Drop everything and fix it. |
| Revocation (routine) | Key retires at session end. | A family member retiring from active service |

### Authorization

Family-role-based authorization:

```json
{
  "kostas": {
    "branches": ["connect/*", "feat/*"],
    "max_patch_lines": 600,
    "requires_review": true,
    "reviewer": "eleni"
  },
  "eleni": {
    "branches": ["connect/*"],
    "max_patch_lines": 200,
    "requires_review": false
  },
  "nikos": {
    "branches": ["ops/*", "connect/*"],
    "max_patch_lines": 150,
    "requires_review": false
  },
  "sofia": {
    "branches": ["connect/*", "coordination/*", "feat/*"],
    "max_patch_lines": 400,
    "requires_review": true,
    "reviewer": "eleni"
  }
}
```

Kostas and Sofia require Eleni's review. Eleni and Nikos do not require review — Eleni because she IS the reviewer, Nikos because his operational changes are small and self-verifying (like adjusting an antenna angle — the signal either improves or it does not).

### Verification

1. Extract signing key.
2. Look up identity in `refs/but-ai/exchange/directory/identity/`.
3. Verify branch authorization.
4. Verify patch size limit.
5. If review required, verify review record exists in the exchange.
6. Verify key validity at commit time.

The family's seal means something. If it is on the commit, the work is good.

---

## 7. Token Budget (RFP 3.7) — The Family Budget

*Yiayia Maria approves this section.*

### Budget Table

Frontier model: Claude Opus. Task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,800 | 0 | Once per session | 4 agent identities, tool descriptions, exchange architecture |
| **Morning coffee** | 1,200 | 400 | Once per task | Task review, work allocation, budget assignment |
| **Network survey (Kostas)** | 1,500 | 300 | Once per task | Thorough workspace assessment |
| **Route planning (Kostas)** | 1,200 | 800 | Once per task | Architecture and load-bearing identification |
| **Tool calls (per call)** | 700 | 350 | 5 per task | Status, branch ops, commits |
| **Patch generation (Kostas)** | 2,000 | 3,000 | Once per task | Single-shot foundation patch |
| **Commit message** | 300 | 500 | Once per task | Warm, contextual, family-style |
| **Eleni's review** | 2,000 | 800 | Once per task | Verification and redundancy check |
| **Memory exchange ops** | 600 | 300 | 3 per task | Switchboard queries, connection updates |
| **Coordination (Sofia)** | 900 | 500 | 2 per task | Forge calls, dependency management |
| **Monitoring (Nikos)** | 400 | 200 | Continuous | Budget tracking, provider health |
| **TOTAL (typical task)** | **18,100** | **9,600** | -- | **27,700 total tokens** |

### Budget Analysis

27,700 tokens for a four-agent family. That is 6,925 tokens per agent — efficient by any standard. Yiayia Maria would approve.

The family overhead (morning coffee, Eleni's review, Nikos's monitoring) is approximately 5,000 tokens (18%). This is the cost of the family's way of working: reviewed, monitored, documented, reliable. A solo agent could complete the task in approximately 20,000 tokens, but without review, without monitoring, and without the generational knowledge that prevents repeating past mistakes.

We consider the 18% overhead an investment in reliability. Papou Kostas spent time every spring maintaining a line that had not carried a signal in thirty years. The family does not cut maintenance to save money. Maintenance is how things last.

### Optimizations

1. **Direct-connection caching.** Frequently accessed memory pairs are pre-loaded at session start. No switchboard routing needed for common queries.
2. **Party-line preloading.** Shared context established at the morning coffee and available to all agents without per-query cost.
3. **Kostas's single-shot approach.** No iteration, no wasted tokens on discarded work. Build it right the first time.
4. **Nikos's lean monitoring.** Small, frequent checks cost less than large, periodic audits.

---

## 8. Testing Strategy — Cable Testing

### 8.1 Provider Testing (Eleni's Protocol)

Mock provider with deterministic responses. All four providers tested through identical scenarios. The fallback chain is tested separately: primary fails, system switches to fallback, system produces correct output. Also test the third level: both providers fail, system produces structured error. Eleni insists on testing the failure paths, not just the success paths.

### 8.2 Patch Workflow (Round-Trip Test)

Create workspace, run agent, capture INDEX.patch, apply to clean workspace, verify. Also test the "contested workspace" scenario. Also test the "partial completion" scenario: budget runs out mid-patch. The partial patch must be valid and applicable, even if incomplete.

### 8.3 Cross-Repo Coordination

Mock forge with in-memory PRs. Simulate multi-family coordination: two repos, dependency chain, messages exchanged. Verify warm messages are well-formed. Verify dependency resolution triggers downstream work.

### 8.4 Token Budget

Mock provider with configurable token counts. Test all budget levels: Comfortable, Careful, Tight, Empty. Verify the reserve is preserved. Verify Nikos's budget optimization actually reduces consumption.

### 8.5 Telephone Exchange Memory

Dedicated tests:
- Direct connections retrieved faster than directory lookups
- Operator-assisted queries return relevant results
- Party lines are accessible to all agents
- Long-distance connections work across mock repos
- Automatic promotion from directory to direct after 5 accesses
- Automatic demotion from direct to directory after 14 days unused
- Expired connections archived, not deleted
- Compaction summary correctly summarizes active connections
- Rehydration from compaction summary restores working context

---

## 9. Trade-Off Summary

| Decision | Our Choice | What We Chose Over | Why |
|----------|-----------|-------------------|-----|
| Crate location | Workspace | Standalone | Use the existing backbone |
| Agent count | 4 (family) | 1 (solo) | Redundancy, review, reliability |
| Memory model | Telephone exchange | Flat KV, embeddings | Explicit connections, self-maintaining promotion |
| Provider fallback | Yes (Eleni's rule) | No fallback | The Corfu Incident. Always have a backup route. |
| Review process | Eleni reviews Kostas and Sofia | No review | Quality is worth the tokens |
| Iteration model | Single-shot (Kostas) | Iterative | Build it right the first time. Wire is expensive. |
| Coordination style | Warm, polite messages | Terse protocol messages | We are a family. Business should be warm. |
| Branch naming | Connection-based | Hash-based or role-based | Names should tell you what they connect |
| Budget reserve | 10% always held back | Spend everything | Something always goes wrong. Be ready. |

---

## A Final Word

We are not the biggest company responding to this RFP. We are not the fastest. We are not the most innovative.

We are the company that has been connecting people for four generations. We are the company whose first telephone line is still standing. We are the company that laid submarine cable using fishing boats because the national company said it could not be done.

We build things that last. We maintain what we build. We connect people who need to be connected.

The line holds.

---

*"If the line holds, the connection holds. If the connection holds, the people can reach each other. That is all that matters."*
— Konstantinos "Papou Kostas" Papadopoulos, 1938-2004

*Submitted with warmth and respect,*
*The Papadopoulos Family*
*Pyrgos, Peloponnese, Greece*
*2026-03-28*
