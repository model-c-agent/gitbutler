# Conservation Strike Force -- Technical Proposal

**RFP:** `but ai` Plugin for GitButler CLI v1.0.0
**Organization:** Conservation Strike Force (Org 072)
**Domain:** Wildlife Conservation | **Philosophy:** Military Precision
**Operation:** Pangolin

---

## Executive Summary

The Conservation Strike Force proposes a `but-ai` plugin built on the doctrine of
zone-based territorial control. The codebase is our territory. The plugin architecture
is our area of operations. Each agent patrols a defined sector with clear boundaries,
clear rules of engagement, and clear reporting lines.

Our core contribution is **patrol-route memory** -- a memory system where knowledge is
compartmentalized by zone, like intelligence in a military operation. Each agent maintains
deep knowledge of its sector and minimal knowledge of others. Memory is mapped to patrol
zones, access is controlled by operational need-to-know, and cross-zone intelligence flows
through controlled declassification channels. This provides both security (a compromised
agent reveals only its zone) and efficiency (each agent's context window is saturated with
relevant sector intelligence, not diluted with cross-sector noise).

---

## 1. Plugin Architecture (RFP 3.1)

### Approach

`but-ai` is a standalone Rust binary organized as a field operations platform. Each module
corresponds to a sector in the area of operations, with defined boundaries, a designated
operator, and controlled interfaces to adjacent sectors.

### Design

```
but ai
  +-- deploy     Execute a task (autonomous agent mode)
  +-- mcp        Start MCP server on stdio
  +-- intel      Query / manage patrol-route memory
  +-- identity   Agent identity and credential management
  +-- sitrep     Show operation status (debug/introspection)
  +-- budget     Show token budget allocation by sector
```

### Crate Structure

```
crates/but-ai/
  src/
    main.rs              -- CLI entry, env var parsing
    mcp/
      server.rs          -- ServerHandler (rmcp-compatible, drop-in replacement)
      tools.rs           -- WorkspaceToolset registration
    ops/
      runner.rs          -- Operation execution loop
      planner.rs         -- Operation order (OPORD) generation
      patcher.rs         -- INDEX.patch + COMMIT.msg synthesis
    intel/
      patrol_memory.rs   -- Patrol-route memory engine
      zone.rs            -- Zone definition and management
      briefing.rs        -- Intelligence briefing generation
      declassify.rs      -- Cross-zone declassification
      storage.rs         -- Git-branch persistence
    provider/
      bridge.rs          -- Wraps but-llm (no modification)
      plugin.rs          -- Provider plugin discovery
      capability.rs      -- Provider capability detection
    coordination/
      forge.rs           -- Forge adapter trait
      github.rs          -- GitHub reference implementation
      schema.rs          -- Structured message schema (tactical format)
      deconflict.rs      -- Cross-repo dependency deconfliction
    identity/
      wallet.rs          -- OpenWallet DID integration
      credentials.rs     -- Agent credential management
      auth.rs            -- Zone-based authorization policies
    budget/
      allocator.rs       -- Sector budget allocation
      monitor.rs         -- Real-time consumption monitoring
```

### WASI Degradation

| Feature | Native | WASI |
|---------|--------|------|
| Plugin discovery | PATH-based | Disabled |
| LLM providers | All 4 + plugins | Local only (if wasi:sockets) |
| Memory | Full patrol-route (read/write) | Reconnaissance only (read-only) |
| Forge coordination | Full | Disabled (no HTTP) |
| Patch production | Full | Full |
| MCP server | Full | Full (stdio) |

Under WASI, the Strike Force operates in "reconnaissance mode" -- intelligence gathering
only, no offensive operations. Agents can read zone memory and produce patches but cannot
coordinate across repos or access remote providers.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Distributed microservices | Multiple attack surfaces. The Strike Force prefers a single hardened perimeter. One binary, one perimeter. |
| Embed in core `but` | Violates RFP constraint. Also: you do not embed the special operations team in the regular infantry. They operate adjacent but independent. |
| Dynamic library plugins | ABI surface increases attack vectors. PATH-based discovery is simpler and more defensible. |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

Sapeur wraps `but-llm` in a provider bridge that adds operational monitoring: token
consumption per engagement (tool call), capability assessment per provider, and a plugin
protocol for additional providers. No modifications to `but-llm` -- the Strike Force
does not modify the terrain; it operates within it.

### Design

```rust
pub struct ProviderBridge {
    provider: LLMProvider,
    monitor: Arc<BudgetMonitor>,
    capabilities: ProviderCapabilities,
}

pub struct ProviderCapabilities {
    pub tool_calling: bool,
    pub streaming: bool,
    pub structured_output: bool,
    pub max_context: usize,
}

impl ProviderBridge {
    pub fn from_git_config(config: &gix::config::File) -> Result<Self>;

    /// Execute engagement (tool-calling loop) with budget monitoring
    pub fn engage(
        &self, system: &str, messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
    ) -> Result<(String, EngagementReport)>;

    /// Streaming response with per-token monitoring
    pub fn observe(
        &self, system: &str, messages: Vec<ChatMessage>,
        on_token: impl Fn(&str),
    ) -> Result<(String, EngagementReport)>;
}
```

### Provider Plugin Protocol

Additional providers (`but-ai-provider-*`) on PATH, JSON-RPC on stdio. The protocol:

1. `capabilities` -- report what the provider supports (recon)
2. `complete` -- generate a completion (engage)
3. `stream` -- streaming completion (sustained engagement)

Adding a new provider is a logistics operation: place the asset on PATH, the bridge
discovers and integrates it.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| New LLM abstraction | Unnecessary duplication. `but-llm` is a proven asset. Do not replace functioning equipment in the field. |
| gRPC for plugins | Over-engineered for the mission. JSON-RPC on stdio is the lightest viable protocol. |

---

## 3. The But Agent (RFP 3.3)

### Approach

The agent execution loop (`but ai deploy`) follows a military operations cycle:

```
1. INTELLIGENCE PREP    Read task, load zone intelligence, assess terrain
2. OPERATION ORDER      Decompose task into sector assignments
3. DEPLOYMENT           Assign operators to sectors with budget allocations
4. PATROL               Execute tool-calling loop within sectors
   4a.                  Select tool (weapon selection)
   4b.                  Execute call (engagement)
   4c.                  Process result (battle damage assessment)
   4d.                  Update zone intelligence
   4e.                  Check budget (ammunition count)
   4f.                  Continue or withdraw
5. CONSOLIDATION        Assemble sector outputs into unified result
6. EXTRACTION           Produce INDEX.patch + COMMIT.msg
7. AFTER ACTION         Update memory, report lessons learned
```

### Task Sources

```
but ai deploy --mission "implement feature X"
but ai deploy --pr 42
but ai deploy --issue 17
but ai deploy --recon-only               # Intelligence gathering, no patch
```

### Patch Production

```rust
pub struct OperationOutput {
    pub index_patch: String,
    pub commit_msg: String,
    pub operation: String,             // Codename (e.g., "pangolin-042")
    pub sectors_complete: Vec<SectorReport>,
    pub sectors_incomplete: Vec<SectorReport>,
    pub budget_report: BudgetReport,
    pub intel_updates: Vec<IntelUpdate>,
}
```

If a sector exhausts its budget, the operator withdraws and reports partial results. The
remaining sectors continue. At consolidation, Ndlovu assembles whatever is available into
a coherent (if partial) patch. A partial extraction is always better than no extraction.

### Branch Naming

```
ops/<callsign>/<operation>[.dep-<operation>]
```

Example: `ops/rhino-3/pangolin-042.dep-pangolin-039` -- RHINO-3 (Sniper) executing
operation Pangolin-042, which depends on Pangolin-039.

### WorkspaceToolset Integration

All ten workspace tools are registered as operational assets:

```rust
let mut toolset = WorkspaceToolset::new(ctx);
// 10 tools: Commit, CreateBranch, Amend, SquashCommits,
// GetProjectStatus, MoveFileChanges, GetCommitDetails,
// GetBranchChanges, SplitBranch, SplitCommit

bridge.engage(system_prompt, messages, &mut toolset)?;
```

Each operator uses tools designated for their sector. Sector boundaries are enforced by
policy and operational discipline, not by code.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Free-form agent loop | Unpredictable. The Strike Force does not conduct unplanned operations. |
| Single-agent design | One operator cannot patrol all zones effectively. Zone-based multi-agent provides better coverage. |
| Direct file editing | Violates the patch-based workflow. An operator does not modify the terrain directly; they submit a patch (report) for command review. |

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is treated as inter-unit communication. PRs are operational
dispatches between units operating in different theaters (repositories). Communication
follows military message format with classification, priority, and structured content.

### Forge Adapter

```rust
pub trait CommsChannel: Send + Sync {
    fn open_dispatch(&self, repo: &TheaterRef, msg: &OperationalDispatch) -> Result<DispatchId>;
    fn send_message(&self, dispatch: &DispatchId, msg: &TacticalMessage) -> Result<()>;
    fn receive_messages(&self, dispatch: &DispatchId) -> Result<Vec<TacticalMessage>>;
    fn classify_dispatch(&self, dispatch: &DispatchId, label: &str) -> Result<()>;
    fn scan_dispatches(&self, repo: &TheaterRef, filter: &DispatchFilter) -> Result<Vec<DispatchId>>;
}
```

Minimal interface. Only operations common to all forges. GitHub is the reference
implementation.

### Structured Message Schema (CSF-Tactical-V1)

```json
{
  "$schema": "csf-tactical-v1",
  "classification": "UNCLASSIFIED | ZONE_RESTRICTED | OPERATION_RESTRICTED",
  "priority": "ROUTINE | PRIORITY | IMMEDIATE | FLASH",
  "message_type": "OPORD | SITREP | INTREQ | CONTACT | CASEVAC | EXFIL",
  "sender": {
    "callsign": "RHINO-3",
    "agent": "sniper",
    "org": "conservation-strike-force",
    "did": "did:key:z6Mk..."
  },
  "recipient": "@RHINO-ACTUAL | @RHINO-2 | @all",
  "operation": "pangolin-042",
  "body": {},
  "references": [
    {"theater": "org/repo", "dispatch": 42}
  ],
  "budget": {
    "allocated": 42000,
    "consumed": 18500,
    "sector": "execution"
  }
}
```

Message types map to military communications:
- **OPORD:** Operation order (task assignment)
- **SITREP:** Situation report (status update)
- **INTREQ:** Intelligence request (memory query to another agent)
- **CONTACT:** Unexpected encounter (blocker, error, conflict)
- **CASEVAC:** Agent down (budget exhausted, unrecoverable error)
- **EXFIL:** Extraction complete (patch ready, operation concluding)

Classification levels control cross-zone visibility:
- **UNCLASSIFIED:** Visible to all agents and humans
- **ZONE_RESTRICTED:** Visible only to agents in the same zone
- **OPERATION_RESTRICTED:** Visible only to agents on the same operation

Comments are wrapped in code fences:

````
```but-agent
{ ... JSON ... }
```
````

### Cross-Repo Deconfliction

Dependencies between operations in different theaters are tracked as a **deconfliction
matrix:**

```rust
pub struct DeconflictionMatrix {
    operations: HashMap<DispatchId, OperationNode>,
    dependencies: Vec<(DispatchId, DispatchId)>,
}

impl DeconflictionMatrix {
    pub fn add_dependency(&mut self, from: DispatchId, to: DispatchId) -> Result<()>;
    pub fn execution_order(&self) -> Result<Vec<DispatchId>>;
    pub fn is_clear(&self, dispatch: &DispatchId) -> bool;
    pub fn detect_circular(&self) -> Option<Vec<DispatchId>>;
}
```

Circular dependencies are "fratricide risks" -- they are detected immediately and rejected.

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Webhook-based events | Requires infrastructure. The Strike Force operates with organic (built-in) capabilities only. The forge is the only comms infrastructure. |
| Centralized coordination server | Single point of failure. Also a single target for compromise. The Strike Force distributes coordination across the forge. |
| Git notes | Not visible in forge UI. PR comments provide operational transparency. |

---

## 5. Agent Memory and Identity (RFP 3.5)

### Patrol-Route Memory

The Strike Force's memory system is organized as a **territory divided into patrol zones.**
Each zone contains intelligence relevant to that sector of the problem space. Agents patrol
their assigned zones, accumulating deep local knowledge. Cross-zone intelligence flows
through controlled channels.

This is fundamentally different from other memory approaches:

| Approach | Structure | Access Pattern | Security |
|----------|-----------|---------------|----------|
| Transit map | Network topology | Shortest-path traversal | Open |
| Digital twin | Living simulation | Simulated queries | Open |
| Mise en place | Named containers | Direct lookup | Open |
| Fermentation | Living cultures | Maturity-ranked harvest | Open |
| **Patrol route** | **Compartmentalized zones** | **Zone-restricted access** | **Need-to-know** |

### Core Concepts

| Military Concept | Memory Equivalent | Description |
|-----------------|-------------------|-------------|
| **Zone** | Memory sector | A defined area of the problem space with clear boundaries |
| **Patrol log** | Zone memory | Time-ordered record of observations within a zone |
| **Intelligence product** | Processed memory | Raw observations analyzed and structured for use |
| **Briefing** | Context injection | Sanitized intelligence prepared for a specific consumer |
| **Dead drop** | Cross-zone transfer | A controlled point where one zone's intelligence is made available to another zone |
| **SIGINT** | Automated observation | Intelligence gathered from signals (code patterns, test results) without active querying |
| **HUMINT** | Direct observation | Intelligence from direct agent interaction with the codebase |

### Storage Layout

```
refs/but-ai/patrol/<agent-id>/
  zones/
    architecture/
      log.json          -- Patrol log (time-ordered observations)
      products.json     -- Processed intelligence products
      access.json       -- Access control (who can read this zone)
    execution/
      log.json
      products.json
      access.json
    memory-system/
      log.json
      products.json
      access.json
    coordination/
      log.json
      products.json
      access.json
    infrastructure/
      log.json
      products.json
      access.json
  dead-drops/
    arch-to-exec.json   -- Declassified intel from architecture to execution
    intel-to-exec.json  -- Declassified intel from intelligence to execution
  operational/
    identity.json       -- Agent identity (operator credentials)
    clearance.json      -- Zone access clearance levels
```

### Patrol Log Schema

```json
{
  "zone": "execution",
  "entries": [
    {
      "id": "pl-001",
      "timestamp": "2026-03-28T10:00:00Z",
      "type": "HUMINT",
      "classification": "ZONE_RESTRICTED",
      "content": "Agent loop requires 8 tool calls average for 200-line feature",
      "tags": ["agent-loop", "tool-calling", "performance"],
      "confidence": 0.85,
      "freshness": 0.95,
      "ttl": "30d",
      "source": "direct_observation",
      "cross_refs": ["pl-003", "pl-007"]
    }
  ]
}
```

### Intelligence Product Schema

```json
{
  "zone": "execution",
  "product_id": "ip-001",
  "title": "Agent Execution Performance Profile",
  "summary": "8 tool calls / 200 lines, ~1,600 tokens per call...",
  "classification": "UNCLASSIFIED",
  "confidence": 0.9,
  "sources": ["pl-001", "pl-003", "pl-007"],
  "valid_until": "2026-04-28T00:00:00Z",
  "dissemination": ["all_zones"]
}
```

### Retrieval: Zone-Based Access

Memory retrieval follows a military intelligence request (IR) pattern:

```
1. Agent submits IR: "I need intelligence about authentication patterns"
2. Tracker determines which zone(s) hold relevant intel
3. For the requester's OWN zone: full access, return raw log entries
4. For OTHER zones: declassification required
   a. Tracker extracts relevant entries from the target zone
   b. Tracker sanitizes (removes zone-specific operational details)
   c. Tracker produces a "briefing" (structured summary for the requester)
   d. Briefing is placed in a dead-drop for the requester
5. Requester retrieves briefing from dead-drop
```

### Relevance Scoring

Within a zone, relevance is scored as:

```
relevance = (tag_overlap * 0.35) + (confidence * 0.25) +
            (freshness * 0.25) + (source_quality * 0.15)
```

Where:
- `tag_overlap`: Jaccard similarity between query tags and entry tags
- `confidence`: entry's confidence level (0-1)
- `freshness`: `1.0 - (age / ttl)`, clamped to [0, 1]
- `source_quality`: HUMINT (1.0) > SIGINT (0.7) > inference (0.4)

Cross-zone retrieval adds a **declassification penalty**: relevance is reduced by 20%
for each zone boundary crossed, reflecting the information loss during sanitization.

### TTL and Expiration

| Status | Freshness | Action |
|--------|-----------|--------|
| **Active** | > 0.5 | Full retrieval, normal operations |
| **Aging** | 0.2-0.5 | Retrieved with caution flag. Requester warned of staleness. |
| **Stale** | < 0.2 | Moved to archive. Not retrieved unless explicitly requested. |
| **Expired** | 0.0 | Purged on next maintenance cycle. |
| **Standing orders** | Pinned | Never expires. Core identity, fundamental patterns. |

### Compaction Survival

When the context window is compacted:

1. **Standing orders survive.** Core identity and fundamental patterns are always retained.
2. **Active zone summaries survive.** Each zone produces a one-paragraph "patrol summary"
   (~100 tokens). Five zones = ~500 tokens of zone awareness.
3. **Dead-drop contents survive.** Recent cross-zone briefings are compact and high-value.
4. **After compaction:** The agent rehydrates from zone summaries and requests specific
   intelligence as needed. The full patrol logs are in Git -- they are never in the context
   window. Only briefings (curated extracts) enter the context.

### Long-Term Storage (Shared Intelligence)

The Strike Force's long-term memory is a **shared intelligence database**:

```
refs/but-ai/patrol/shared/
  index.json            -- Cross-org intelligence index
  orgs/
    conservation-strike-force/
      declassified/     -- Intelligence products cleared for sharing
        ip-001.json
        ip-005.json
    other-org/
      declassified/
        ...
```

Only declassified intelligence products are shared. Raw patrol logs never leave the
originating agent's zone. This is the Strike Force's operational security boundary: what
is learned in the zone stays in the zone unless explicitly declassified.

### Identity (Operator Credentials)

Agent identity is stored as operational credentials:

```json
{
  "callsign": "RHINO-3",
  "agent_name": "sniper",
  "org": "conservation-strike-force",
  "clearance": {
    "zones": ["execution"],
    "classification_level": "ZONE_RESTRICTED"
  },
  "capabilities": ["agent-loop", "tool-calling", "patch-generation"],
  "authorization": {
    "branches": ["ops/rhino-3/*", "feat/*"],
    "max_patch_lines": 1000,
    "repos": ["gitbutler/gitbutler"]
  },
  "signing_key": "openwallet:did:key:z6Mk...",
  "commissioned": "2026-03-28T10:00:00Z"
}
```

### Trade-offs

| Alternative | Why Rejected |
|-------------|-------------|
| Open access memory | No compartmentalization. A compromised agent in an open system can read everything. Zone-based access limits blast radius. |
| Vector similarity search | Does not respect zone boundaries. A query should not return results from zones the requester is not cleared for. |
| Single-agent deep memory | One agent holding all knowledge is a single point of failure. Distributed zone memory is resilient to individual agent failure. |
| Transit-map topology | No access control model. The Strike Force's memory must enforce need-to-know. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every commit is signed with an OpenWallet-managed key tied to the operator's credentials.
The key is the operator's weapon -- it is issued, tracked, and audited.

### Key Provisioning

```
but ai identity commission --callsign RHINO-3 --org conservation-strike-force
  -> Creates OpenWallet DID: did:key:z6Mk...
  -> Stores in operator credentials
  -> Registers on refs/but-ai/identity/
```

### Authorization Model

Zone-based authorization:

```json
{
  "org": "conservation-strike-force",
  "operators": {
    "RHINO-ACTUAL": {
      "agent": "ndlovu",
      "clearance": "TOP",
      "authority": {
        "branches": ["*"],
        "max_patch_lines": 5000,
        "zones": ["*"],
        "can_commission": true,
        "can_decommission": true
      }
    },
    "RHINO-3": {
      "agent": "sniper",
      "clearance": "STANDARD",
      "authority": {
        "branches": ["ops/rhino-3/*", "feat/*"],
        "max_patch_lines": 1000,
        "zones": ["execution"],
        "can_commission": false,
        "can_decommission": false
      }
    }
  }
}
```

### Verification Chain

1. Extract DID from commit signature
2. Look up operator credentials by DID
3. Verify clearance level and zone assignment
4. Verify: branch permission, patch size, zone authority
5. Verify: the commit falls within the operator's zone of responsibility
6. Verify: chain of command -- did ACTUAL authorize this operation?

The Strike Force adds a unique verification step: **zone verification.** A commit that
modifies files outside the operator's assigned zone is flagged, even if the operator has
branch permission. This catches "patrol boundary violations" -- operators who stray
outside their sector.

### Key Lifecycle

| Event | Protocol |
|-------|----------|
| **Commissioning** | New DID, operator credentials created, zone assigned |
| **Rotation** | "Key turnover" -- new key, rotation documented in patrol log |
| **Revocation (routine)** | "Decommission" -- key retired, operator credentials archived |
| **Revocation (compromise)** | "Compromise protocol" -- key revoked immediately, all commits signed with compromised key flagged, zone intelligence reviewed for contamination, adjacent zones alerted |

The compromise protocol includes a **blast radius assessment**: which zones did the
compromised operator have access to? What intelligence was available in those zones? This
assessment informs the remediation scope.

---

## 7. Token Budget (RFP 3.7)

### Budget Table (Frontier Model: Claude Opus)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,400 | 0 | Once per session | Operator credentials, tool descriptions (10 tools), zone summary (~400 tok), workspace state. |
| **Task ingestion** | 2,500 | 300 | Once per task | Read PR body / issue / mission brief. Output: structured OPORD. |
| **Planning (OPORD)** | 2,000 | 700 | Once per task | Terrain analysis, sector assignment, budget allocation. |
| **Tool call (per call)** | 1,100 | 350 | ~8 per task | Engagement: parameter formulation (200 out) + result processing (900 in). |
| **Patch generation** | 2,000 | 3,500 | Once per task | Consolidation of sector results (2,000 in). Unified diff (3,500 for 200 lines). |
| **Commit message** | 500 | 150 | Once per task | Tactical brevity. Operation name + conventional commit. |
| **Intelligence retrieval** | 500 | 150 | 2 per task | Zone-local retrieval: 300 in + 100 out. Dead-drop (cross-zone): 200 in + 50 out. |
| **Coordination event** | 1,300 | 400 | 2 per task | Read dispatches (1,000 in) + send tactical message (400 out). |
| **After action** | 400 | 200 | Once per task | Lessons learned, zone intel updates. |
| **TOTAL (typical task)** | **25,500** | **9,750** | -- | 200-line feature, 3 files, 2 cross-repo deps, 8 tool calls, 2 intel retrievals, 2 coordination events. |

**Grand total: ~35,250 tokens per typical task.**

### Justification

- **System prompt (3,400):** Zone summary is compact (~400 tokens) because each agent only
  knows its zone. Compare to open-access systems that must include summaries of all domains.
- **Intelligence retrieval (500+150):** Zone-local retrieval is cheap (direct access to own
  zone's products). Cross-zone retrieval via dead-drops adds a small overhead but is
  infrequent.
- **After action (400+200):** Low cost because lessons learned are structured (SALUTE
  format) and written to zone logs, not to context.
- **8 tool calls:** Standard for a 200-line, 3-file feature.

### Sector Budget Allocation

```
Sector          | Operator    | Allocated | Consumed | Status
----------------+-------------+-----------+----------+---------
Command         | RHINO-ACTUAL| 5,300     | 4,800    | GREEN
Intelligence    | RHINO-2     | 5,300     | 4,500    | GREEN
Execution       | RHINO-3     | 12,340    | 11,200   | AMBER
Communications  | RHINO-4     | 6,340     | 5,600    | GREEN
Infrastructure  | RHINO-5     | 4,230     | 3,800    | GREEN
Reserve         | --          | 1,740     | 0        | HELD
```

Status codes: **GREEN** (< 70% consumed), **AMBER** (70-90%), **RED** (> 90%, prepare
for withdrawal).

---

## 8. Testing Strategy (RFP 4.5)

### Provider-Agnostic Testing

- **Mock provider:** Deterministic responses, no live API calls. The Strike Force calls
  this "dry fire" -- practicing engagements without live ammunition.
- **Provider qualification:** Each provider tested against a "qualification course"
  (canonical request set). Providers that fail qualification are not deployed.
- **Replay testing:** Recorded operations replayed in CI. Same inputs, same expected
  outputs.

### Patch Workflow (Operational Round-Trip)

- **Full operation test:** Create known workspace -> deploy operation -> apply patch ->
  verify result. "Fire and maneuver" -- execute and validate.
- **Partial extraction test:** Exhaust budget in one sector. Verify other sectors complete
  and partial patch is valid.
- **Contact test:** Apply patch to dirty workspace. Verify structured error and workspace
  integrity.

### Cross-Repo Coordination Testing

- **Mock comms:** `MockCommsChannel` simulates forge operations in memory.
- **Message validation:** All tactical messages validated against CSF-Tactical-V1 schema.
- **Deconfliction test:** Circular dependencies detected and rejected. Execution order
  verified.
- **Multi-theater simulation:** Two agents in two repos exchanging dispatches and patches.

### Token Budget Testing

- **Budget accounting:** Mock provider returns exact token counts. Verify GREEN/AMBER/RED
  alerts at correct thresholds.
- **Sector isolation:** Verify one sector's budget exhaustion does not impact others.
- **Reserve activation:** Verify reserve tokens are used only for guaranteed partial output.

### Zone Security Testing

- **Compartmentalization test:** Agent in zone A attempts to read zone B's raw intelligence.
  Verify access denied.
- **Dead-drop test:** Intelligence placed in dead-drop by zone A is accessible to zone B
  only in declassified form.
- **Compromise simulation:** Mark an agent as compromised. Verify blast radius assessment
  correctly identifies exposed zones.

---

## 9. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Maximum tokens per operation |
| `but-ai.patrol.branch` | string | `refs/but-ai/patrol/<agent-id>` | Git ref for patrol memory |
| `but-ai.patrol.maxZones` | integer | 10 | Maximum zones per agent |
| `but-ai.patrol.defaultTtl` | string | `30d` | Default TTL for patrol log entries |
| `but-ai.patrol.declassifyPenalty` | float | 0.20 | Relevance penalty for cross-zone retrieval |
| `but-ai.patrol.sharedBranch` | string | `refs/but-ai/patrol/shared` | Shared intelligence branch |
| `but-ai.coordination.schema` | string | `csf-tactical-v1` | Tactical message schema |
| `but-ai.coordination.forge` | string | `github` | Default forge adapter |
| `but-ai.identity.wallet` | string | (required) | OpenWallet endpoint URL |
| `but-ai.budget.greenPct` | float | 0.70 | GREEN/AMBER threshold |
| `but-ai.budget.amberPct` | float | 0.90 | AMBER/RED threshold |
| `but-ai.budget.reservePct` | float | 0.05 | Reserve allocation |
| `but-ai.security.zoneEnforcement` | bool | true | Enforce zone-based access control |

---

## 10. Migration Path

The Strike Force's migration follows a "relief in place" protocol. One unit replaces another
in the same position without a gap in coverage.

| Phase | Action | Verification |
|-------|--------|-------------|
| 1. **Forward deploy** | `but-ai mcp` deployed alongside existing MCP server. Both operational. `gitbutler_update_branches` exposed with identical schema. | Diff outputs for identical inputs. Zero gaps. |
| 2. **Expand perimeter** | New tools added to `but-ai mcp`. Legacy tool unchanged. | Existing clients unaffected. No change in coverage. |
| 3. **Warning order** | Legacy tool deprecated (warning in response). One release cycle. | All clients notified. |
| 4. **Relief complete** | Legacy tool removed. `but-ai mcp` assumes full responsibility. | Full test suite green. No sector left unpatrolled. |

---

*Operation Pangolin. Proposal filed by Conservation Strike Force.*
*All sectors clear. RHINO-ACTUAL out.*
