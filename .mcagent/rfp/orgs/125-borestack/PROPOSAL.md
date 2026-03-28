# Proposal — BoreStack

**RFP Response: `but ai` Plugin for GitButler CLI**
**Organization:** BoreStack (Org 125)
**Domain:** Mining Engineering | **Philosophy:** Startup Hustle
**Date:** 2026-03-28

---

## Executive Summary

BoreStack proposes a `but-ai` plugin modeled on the exploration drilling workflow: survey the terrain, identify targets, drill precisely, and log every layer. The core insight: token expenditure is exploration expenditure. Every tool call is a meter of depth. Every patch is a borehole. The goal is to maximize the information value (useful code changes) per token spent, the same way BoreStack maximizes the mineral value per meter drilled.

Our plugin's distinctive contribution is the core-sample memory system: memories stored as layered cylinders of context, where each layer preserves the conditions under which it was formed. You can analyze the surface layer for current information or drill deeper into the sample to understand the geological history that produced the current state.

The 132 redundant `but status` calls are failed seismic surveys — each one pinging the subsurface but none building a coherent model. Our plugin runs the survey once, builds a model, and drills from the model.

---

## 1. Plugin Architecture (RFP Section 3.1)

### Approach

`but-ai` is a Rust binary crate (`crates/but-ai/`) in the workspace, discoverable via PATH as `but-ai`. The architecture mirrors a drill rig: a surface controller (CLI/MCP) directs subsurface operations (agent tools) based on a geological model (workspace state).

### Design

**CLI Mode:**
```
but ai survey         — Run seismic survey of workspace (maps structure, identifies targets)
but ai drill <task>   — Execute a task (produces INDEX.patch + COMMIT.msg)
but ai core <query>   — Query core-sample memory store
but ai log            — Display drill log (recent agent operations with depth estimates)
but ai assay <ref>    — Analyze a specific commit or branch (equivalent to assaying a core sample)
but ai mcp            — Start MCP server on stdio
```

**MCP Server Mode:**
Drop-in replacement. `gitbutler_update_branches` preserved for backward compatibility. New tools:

| New Tool | Description |
|----------|-------------|
| `run_survey` | Map workspace structure, return terrain report |
| `drill_task` | Execute task with seismic guidance, produce patches |
| `query_core` | Search core-sample memory by depth, composition, or content |
| `drill_log` | Return recent operations as a structured log |
| `assay_commit` | Analyze a commit's composition (files, changes, dependencies) |
| `budget_depth` | Report token usage as drilling depth (tokens spent / estimated total) |

**Git Config Keys:**

| Key | Purpose | Default |
|-----|---------|---------|
| `but-ai.surveyDepth` | How deep the initial survey should probe (number of commits to analyze) | `50` |
| `but-ai.memoryBranch` | Ref prefix for core-sample memory storage | `refs/core/` |
| `but-ai.tokenBudget` | Per-task token budget (the "drilling budget") | `50000` |
| `but-ai.maxBoreholeDepth` | Maximum tokens for a single patch operation | `20000` |
| `but-ai.agentIdentity` | OpenWallet key reference | none (required) |
| `but-ai.targetConfidence` | Minimum survey confidence before drilling begins | `0.7` |
| `but-ai.compactionLayers` | Number of layers retained during memory compaction | `10` |

**WASI Degradation:**
Under WASI, PATH discovery is unavailable. Degradation path:
- Survey operations work fully (read-only Git operations, no filesystem-specific syscalls).
- Core-sample memory operations work fully (Git-native storage).
- Drill operations produce patches normally (the patch is the output, not a filesystem operation).
- The drill rig's background monitoring (Seismic's continuous survey updates) is disabled — surveys must be triggered explicitly.
- WASI component interface available for direct loading.

### Trade-offs

**Considered:** Building the plugin as a Python wrapper around `but` CLI calls. **Rejected:** latency is unacceptable. Each CLI call adds ~50ms of process startup. A 10-tool-call task would add 500ms of pure overhead. Rust-native in the workspace eliminates this.

**Considered:** Implementing the survey as a persistent index (like a search engine). **Rejected:** indices go stale. BoreStack learned this from geology — a seismic survey from last month may not reflect current conditions if there has been tectonic activity (major refactoring). Instead, the survey is run fresh for each task, with cached results used only as a warm start.

---

## 2. Provider-Agnostic AI Interface (RFP Section 3.2)

### Approach

`but-llm` is the sole backend. BoreStack wraps it with a depth-aware layer that tracks token expenditure as "drilling depth."

### Design

**Depth-Aware Provider Wrapper:**
```rust
pub struct DrillProvider {
    inner: LLMProvider,
    budget: DrillBudget,
    current_depth: u32,  // Tokens spent = meters drilled
}

impl DrillProvider {
    /// Tool-calling loop that tracks depth and can abort at max depth.
    pub fn drill_loop(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
        model: &str,
        max_depth: u32,
    ) -> anyhow::Result<DrillResult>;
}

pub struct DrillResult {
    pub output: String,
    pub depth_reached: u32,       // Tokens spent
    pub target_reached: bool,     // Did the agent complete the task?
    pub core_sample: CoreSample,  // Memory artifact from this drill
    pub partial_patch: Option<Patch>, // If target not reached
}
```

Every LLM interaction is metered. The `current_depth` counter increments with each token consumed. When depth approaches `max_depth`, the agent enters "core retrieval" mode — it stops drilling and brings whatever it has to the surface (produces the patch, even if incomplete).

**New Provider Mechanism:**
Pluggable provider adapters loaded from `but-ai.providerPluginDir`:

```rust
pub trait DrillBit: Send + Sync {
    /// Different drill bits for different rock types.
    /// Metaphor: different providers have different strengths.
    fn name(&self) -> &str;
    fn hardness_rating(&self) -> f32;  // How well does this provider handle complex tasks?
    fn create_backend(&self, config: &ProviderConfig) -> Result<Box<dyn LLMBackend>>;
}
```

The "drill bit" metaphor carries information: some providers are better for "hard rock" (complex reasoning tasks) and others for "soft sediment" (simple, fast tasks). The metadata helps with future model routing.

### Trade-offs

**Considered:** Auto-selecting providers based on task complexity. **Deferred:** the RFP requests a single-model budget. The `hardness_rating` metadata is stored but not used for routing in the initial implementation.

**Considered:** Using `structured_output` for all agent responses to enforce schema. **Rejected for patches:** structured output constrains output length and format. Patches vary wildly in structure. Free-form output via `tool_calling_loop` is more appropriate for patch generation. Structured output is used for survey reports and memory operations.

---

## 3. The But Agent (RFP Section 3.3)

### Approach

The But Agent follows the exploration drilling workflow: Survey → Target → Drill → Log. The agent never drills without a survey, never targets without confidence, and always logs the result.

### Design

**Task Lifecycle:**
```
1. SURVEY     — Seismic maps workspace structure (cheap, read-only)
2. TARGET     — Seismic identifies drilling target (which files, which functions)
3. PLAN       — Auger creates drilling plan (approach angle, estimated depth)
4. DRILL      — Auger produces INDEX.patch + COMMIT.msg
5. LOG        — CoreLog stores the result as a core sample
6. COORDINATE — Mux manages cross-repo PRs and dependencies
```

**The Survey Report:**
```json
{
  "terrain": {
    "total_files": 142,
    "active_terrain": ["src/middleware/", "src/auth/"],
    "stable_formation": ["src/config/", "src/utils/"],
    "fault_zones": ["src/legacy/mcp/"],
    "recent_activity": {
      "last_7_days": ["src/tools/workspace.rs", "src/auth/token.rs"],
      "change_velocity": "moderate"
    }
  },
  "targets": [
    {
      "file": "src/middleware/auth.rs",
      "function": "validate_token",
      "confidence": 0.85,
      "estimated_depth": 3500,
      "rock_type": "metamorphic",
      "rationale": "Function modified 4 times in last 30 days, high churn suggests active area"
    }
  ],
  "estimated_total_depth": 15000,
  "recommendation": "Drill target 1 first, estimated budget sufficient"
}
```

**Branch Naming:**
```
bore/s01.s03/auger/auth-token-validation
│    │       │      └── Target description
│    │       └── Agent name
│    └── Dependency chain
└── Namespace (borehole prefix)
```

**Token Budget Enforcement:**
```rust
pub struct DrillBudget {
    total_depth: u32,           // Total token budget
    current_depth: u32,         // Tokens consumed
    surface_reserve: u32,       // Reserved for final patch + COMMIT.msg (2000 tokens)
    abort_threshold: f32,       // Fraction of budget at which to start abort sequence (0.85)
}
```

At 85% depth, the agent enters "core retrieval" mode:
1. Stops exploring (no more GetProjectStatus or GetBranchChanges calls).
2. Produces the best patch possible with remaining budget.
3. Writes COMMIT.msg documenting what was completed and what remains.
4. Logs the incomplete borehole in CoreLog for future resumption.

**WorkspaceToolset Exposure:**
Tools are loaded in phases matching the drilling workflow:

| Phase | Tools Loaded | Rationale |
|-------|-------------|-----------|
| SURVEY | GetProjectStatus, GetBranchChanges, GetCommitDetails | Read-only reconnaissance |
| TARGET | (No additional tools needed — targeting uses survey data) | — |
| DRILL | Commit, CreateBranch, Amend | Write operations |
| LOG | (No tools — memory operations are Git-native) | — |
| COORDINATE | MoveFileChanges, SplitBranch, SquashCommits | Branch management |

Phase-gated tool loading saves ~1,200 tokens on the system prompt compared to loading all 10 tools at once.

### Trade-offs

**Considered:** Skipping the survey for simple tasks. **Rejected:** BoreStack's first rule is "never drill blind." Even a simple task benefits from knowing the terrain. The survey for a simple task is cheap (~500 tokens) and often reveals complications that would have been expensive to discover mid-drill.

**Considered:** Allowing the agent to drill multiple boreholes in parallel. **Rejected:** parallel drilling causes resource contention (two patches modifying the same file). Auger drills sequentially. Parallelism is handled at a higher level by Mux coordinating multiple Auger instances on different targets.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP Section 3.4)

### Approach

PRs are drilling reports shared between sites. The coordination protocol is forge-agnostic and uses structured comments that encode the "drill log" — a record of what was done, what was found, and what dependencies exist.

### Design

**PR Comment Schema (Drill Protocol):**
```json
{
  "protocol": "drill/v1",
  "type": "survey_report | drill_result | dependency | core_handoff | budget_depth",
  "agent": "auger@borestack",
  "site": "github.com/org/repo",
  "timestamp": "2026-03-28T14:30:00Z",
  "depth": 15000,
  "payload": { }
}
```

The `depth` field is always present — it tells other agents how much budget was consumed, enabling them to estimate whether the referenced work was a shallow probe or a deep investigation.

**Forge Adapter Interface:**
```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &SiteRef, pr: &DrillingPR) -> Result<PrId>;
    fn post_drill_log(&self, repo: &SiteRef, pr: PrId, log: &DrillLog) -> Result<CommentId>;
    fn read_drill_logs(&self, repo: &SiteRef, pr: PrId) -> Result<Vec<DrillLog>>;
    fn add_label(&self, repo: &SiteRef, pr: PrId, label: &str) -> Result<()>;
    fn get_pr(&self, repo: &SiteRef, pr: PrId) -> Result<PrInfo>;
    fn search_prs(&self, repo: &SiteRef, query: &str) -> Result<Vec<PrSummary>>;
}
```

Reference implementation: GitHub REST. The adapter normalizes forge-specific behaviors (e.g., GitHub's comment editing vs. Gitea's comment API differences).

**Cross-Repo Dependencies:**
Dependencies are modeled as "connected boreholes" — drill results in one site that inform drilling in another:

```json
{
  "type": "dependency",
  "payload": {
    "depends_on": "github.com/org/auth-service#42",
    "relation": "stratigraphic",
    "description": "Core sample from auth-service shows token format change — our parsing layer must match",
    "core_reference": "refs/core/auger/sess_2026-03-28_001"
  }
}
```

The `core_reference` is unique to BoreStack: it points to the actual core sample (memory) that documents the dependency, allowing the dependent agent to inspect the evidence rather than trusting a textual claim.

**Structured Message Types:**

| Type | Purpose | Key Fields |
|------|---------|------------|
| `survey_report` | Share terrain analysis with other sites | `terrain`, `targets`, `estimated_depth` |
| `drill_result` | Report borehole outcome | `target_reached`, `depth`, `core_sample_ref` |
| `dependency` | Declare cross-site dependency | `depends_on`, `core_reference` |
| `core_handoff` | Transfer a core sample (memory) between agents | `core_ref`, `layers_summary` |
| `budget_depth` | Report drilling budget status | `total`, `drilled`, `remaining` |

### Trade-offs

**Considered:** Using Git bundles for cross-repo core sample transfer. **Rejected:** bundles add Git complexity and require push access to the remote. PR comments with core sample references are sufficient — the consuming agent can fetch the memory ref via standard Git fetch.

**Considered:** Real-time coordination via SSE (Server-Sent Events) from the forge. **Rejected:** not all forges support SSE. BoreStack uses polling with configurable intervals (`but-ai.pollInterval`, default: 60 seconds for active tasks).

---

## 5. Agent Memory and Identity (RFP Section 3.5)

### Approach: Core-Sample Memory

Memories are layered cylinders of context. Each memory preserves the stratigraphy of how it was formed: surface layers contain recent information, deeper layers contain the historical context that explains the surface. You can analyze a memory at any depth.

### Design

**Storage Medium:**
Core samples are stored in Git refs under `refs/core/<agent-id>/`:
```
refs/core/auger/samples/sess_2026-03-28_001.json   — Individual core sample
refs/core/auger/index/stratigraphic.json            — Index by geological era (task phase)
refs/core/auger/index/compositional.json            — Index by content type
refs/core/auger/index/spatial.json                  — Index by codebase location
refs/core/auger/identity.json                       — Agent identity
refs/core/shared/formation_map.json                 — Shared geological model of the codebase
```

Each core sample:
```json
{
  "sample_id": "core_2026-03-28_001",
  "agent": "auger",
  "borehole": {
    "target": "src/middleware/auth.rs::validate_token",
    "depth_reached": 15000,
    "target_reached": true
  },
  "layers": [
    {
      "depth": 0,
      "era": "current",
      "type": "observation",
      "content": "validate_token uses JWT with RS256, 15-minute expiry",
      "confidence": 0.95,
      "formed_during": "task_042"
    },
    {
      "depth": 1,
      "era": "recent",
      "type": "change",
      "content": "Expiry was changed from 30 to 15 minutes in task_038",
      "confidence": 0.90,
      "formed_during": "task_038"
    },
    {
      "depth": 2,
      "era": "historical",
      "type": "design_decision",
      "content": "JWT was chosen over session tokens for stateless scaling",
      "confidence": 0.80,
      "formed_during": "task_012"
    },
    {
      "depth": 3,
      "era": "foundational",
      "type": "architecture",
      "content": "Auth middleware was originally HTTP Basic Auth, replaced in v2.0",
      "confidence": 0.70,
      "formed_during": "task_001"
    }
  ],
  "ttl": "30d",
  "last_validated": "2026-03-28T14:30:00Z"
}
```

**Layer Taxonomy:**

| Layer Type | Description | Typical Depth |
|------------|-------------|---------------|
| `observation` | Current state, directly verified | 0 (surface) |
| `change` | Recent modifications, context of current state | 1-2 |
| `design_decision` | Why the code is the way it is | 3-5 |
| `architecture` | Foundational choices, rarely questioned | 6+ |
| `fossil` | Artifacts from deleted/replaced code | Variable |

**Retrieval:**
CoreLog scores memories by:
1. **Compositional similarity** (0.4 weight): Does the query match the content of any layer in the core sample? Surface layers are weighted more heavily than deep layers.
2. **Spatial proximity** (0.3 weight): Is the core sample's borehole near the current task's target (same file, same directory, same module)?
3. **Stratigraphic coherence** (0.2 weight): Does the core sample's layer structure make sense for the current task? A query about architecture benefits from samples with deep architecture layers; a query about recent changes benefits from samples with rich surface layers.
4. **Sample freshness** (0.1 weight): When was the core sample last validated?

**Expiration and Compaction:**
- TTL is per-sample, not per-layer. When a sample expires, all its layers expire together — the stratigraphy is the unit.
- Compaction reduces layer count: deep layers are merged. A sample with 10 layers compacts to 5 by merging adjacent layers of the same type. The merged layer's confidence is the average of the constituent layers.
- Expired samples are moved to `refs/core/<agent-id>/archive/`, not deleted. They can be retrieved for geological surveys of the codebase's history.

**Compaction Survival:**
When the LLM context is compacted:
1. Surface layers (depth 0-1) of the most relevant core samples are preserved in full.
2. Deeper layers are compressed to one-line summaries.
3. The post-compaction system prompt includes a "formation map" — a summary of the known geological structure of the codebase, costing ~1,500 tokens.

**Long-Term Storage (Formation Map):**
The shared formation map (`refs/core/shared/formation_map.json`) is a collaborative model of the codebase's geology, contributed to by all agents across sessions:
```json
{
  "formations": [
    {
      "path": "src/middleware/",
      "rock_type": "metamorphic",
      "description": "Heavily modified middleware layer, multiple refactoring events",
      "last_major_event": "v2.0 auth rewrite",
      "known_fault_zones": ["auth.rs", "rate_limit.rs"],
      "core_sample_refs": ["core_042", "core_038", "core_012"]
    }
  ]
}
```

**Identity:**
```json
{
  "name": "auger",
  "organization": "borestack",
  "role": "primary_driller",
  "capabilities": ["patch_generation", "deep_analysis", "partial_patch_recovery"],
  "authorization_scope": {
    "branches": ["bore/*", "feat/*", "fix/*"],
    "max_patch_lines": 500,
    "max_depth": 20000
  },
  "openwallet_key_ref": "did:web:borestack.io/agents/auger",
  "created": "2026-03-28T00:00:00Z",
  "key_rotation_policy": "90d"
}
```

### Trade-offs

**Considered:** Flat memory (one layer per memory). **Rejected:** flat memories lose geological context. Knowing that "the auth module uses JWT" is less valuable than knowing the full stratigraphy: JWT was chosen for stateless scaling (architecture), replacing session tokens (fossil), with the expiry recently changed from 30 to 15 minutes (change). The stratigraphy explains the surface.

**Considered:** Unlimited layers per core sample. **Rejected:** deep samples are expensive to retrieve and analyze. Maximum layer count is 10 (configurable). Beyond 10, older layers are merged during compaction.

---

## 6. Signed Commits via OpenWallet (RFP Section 3.6)

### Approach

Every agent commit is signed via OpenWallet. The signature includes drilling metadata: depth reached, target achieved, and the core-sample reference that documents the work.

### Design

**Signing Flow:**
```
1. Auger produces INDEX.patch + COMMIT.msg
2. Mux prepares commit object and attaches drilling metadata
3. Commit is signed with agent's OpenWallet key
4. Signature claims:
   - Agent identity (name, org, role)
   - Borehole reference (branch name, depth)
   - Core sample reference (memory ref documenting the work)
   - Authorization scope (branches, max depth)
   - Budget: depth drilled / max depth
5. Signed commit is pushed
```

**Authorization Model:**
Depth-based authorization:
- Agents have a `max_depth` (maximum tokens per borehole). An agent authorized for 20,000 tokens cannot produce a patch that costs more.
- Branch patterns define where the agent can drill.
- Core sample references in the signature create an evidence chain: the commit references the memory that documents the analysis that justified the change.

```json
{
  "authorization_rules": [
    { "branches": "bore/*", "max_depth": 20000, "max_lines": 500 },
    { "branches": "feat/*", "max_depth": 15000, "max_lines": 300 },
    { "branches": "fix/*", "max_depth": 10000, "max_lines": 200 }
  ]
}
```

**Key Lifecycle:**
- **Provisioning:** Per-agent keys via OpenWallet. Key DID stored in identity record.
- **Rotation:** 90-day rotation. During rotation, old key is marked "retired" in identity record.
- **Revocation (routine):** Retired keys remain valid for verifying historical commits.
- **Revocation (compromise):** Key added to OpenWallet revocation list. Commits signed with compromised key flagged for review. Core samples referenced by those commits are marked as "contaminated" in the memory system — their confidence is reduced but they are not deleted (contamination is noted, not erased).

### Trade-offs

**Considered:** Signing core samples (memories) in addition to commits. **Deferred:** adds complexity. Core samples are referenced by signed commits, so the trust chain extends from commit signature to core sample. Direct core sample signing is a future enhancement.

---

## 7. Token Budget (RFP Section 3.7)

### Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,000 | 0 | Once per session | Agent identity (300), phase-gated tools (1,200-1,800), formation map summary (800), workspace terrain (700) |
| **Survey (Seismic)** | 2,500 | 500 | Once per task | GetProjectStatus + GetBranchChanges + commit sampling. Output: survey report. |
| **Targeting** | 1,000 | 400 | Once per task | Analyze survey results, select drilling target. |
| **Planning** | 1,500 | 500 | Once per task | Create drilling plan: approach, estimated depth, abort conditions. |
| **Tool call (per call)** | 500 | 150 | ~8 per task | Tool parameters (150 out), tool result (500 in). |
| **Drilling (patch generation)** | 3,500 | 4,500 | Once per task | Target code (3,500 in). Patch (4,000 out) + COMMIT.msg (500 out). |
| **Core logging** | 500 | 800 | Once per task | Structured core sample creation: layers, metadata, indices. |
| **Memory retrieval** | 700 | 200 | 2 per task | Query (200 out), core sample results (500 in per retrieval). |
| **Coordination** | 1,000 | 600 | 1 per task | PR creation, drill-protocol comments, dependency declarations. |
| **TOTAL (typical task)** | **19,400** | **9,450** | -- | 200-line, 3-file feature with 1 cross-repo dependency |

**Grand total: ~28,850 tokens per typical task.**

### Budget Justification

The budget is front-loaded in the survey phase (3,000 tokens) and the drilling phase (8,000 tokens). The survey is the cheapest investment with the highest ROI: 3,000 tokens of survey consistently saves 5,000+ tokens of drilling by identifying the right target on the first attempt.

Core logging (1,300 tokens) is the "cost of learning." Every borehole produces a core sample whether or not the borehole reached its target. BoreStack treats this as mandatory overhead — the memory is the long-term asset, even when the patch is the short-term deliverable.

The total team budget from AGENTS.md (50,000 tokens) accommodates one primary task (28,850 tokens), inter-agent communication (~6,000), survey overhead (~5,000), and a 20% contingency reserve (~10,000). The contingency is the drilling equivalent of "unexpected geology" — complications that the survey didn't predict.

---

## 8. Testing Strategy

### Provider-Agnostic Testing
- **Mock drill bit** (mock LLM provider) that returns deterministic responses per phase. Survey phase returns fixed terrain. Drill phase returns fixed patches.
- **Provider parity tests:** Same task executed against all four providers via mock. Verify identical survey → target → drill → log lifecycle regardless of provider.

### Patch Workflow Validation
- **Round-trip borehole tests:** Generate survey → produce drilling plan → generate patch → apply patch → verify workspace state matches target.
- **Abort-at-depth tests:** Set artificially low `maxBoreholeDepth`, verify that the agent produces a valid partial patch and documents incomplete work in COMMIT.msg.
- **Contaminated target tests:** Modify the workspace between survey and drill. Verify that the agent detects the discrepancy (terrain changed since survey) and re-surveys before drilling.

### Cross-Repo Coordination Testing
- **Mock forge adapter** with drill-protocol comment validation.
- **Connected borehole tests:** Simulate a dependency where repo A's core sample informs repo B's drilling target. Verify that the core reference is correctly resolved and the dependency chain is auditable.

### Token Budget Testing
- **Depth tracking tests:** Compare budget tracker's reported depth against actual tokens consumed. Must match within 5%.
- **Abort threshold tests:** Verify that the 85% abort threshold triggers graceful core retrieval and that the resulting partial patch is valid.
- **Survey ROI tests:** Compare tokens consumed by tasks with and without surveys. Verify that surveyed tasks consistently consume fewer total tokens.

---

## 9. Migration Path

1. **Phase 1 (Survey):** Deploy `but ai mcp` alongside the existing MCP server. Both serve simultaneously. `gitbutler_update_branches` is proxied through the new system, with survey data collected passively.
2. **Phase 2 (Drill):** Existing clients migrate to `drill_task`. The old tool is internally routed through Seismic → Auger, so the behavior is identical but the infrastructure is new.
3. **Phase 3 (Decommission):** Old MCP server retired. The drill log from Phase 1-2 passive data collection provides the initial formation map for Phase 3 operations.

The migration itself is a borehole: we survey the current system (Phase 1), drill through it (Phase 2), and log the result (Phase 3).

---

*Submitted by BoreStack. We don't guess what's underground. We go look.*
