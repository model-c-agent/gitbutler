# Proposal — TriageOS

**RFP Response: `but ai` Plugin for GitButler CLI**
**Organization:** TriageOS (Org 115)
**Domain:** Emergency Medicine | **Philosophy:** Startup Hustle
**Date:** 2026-03-28

---

## Executive Summary

TriageOS proposes a `but-ai` plugin that treats every agent operation as a triage decision. The central claim: in a resource-constrained environment (tokens are the scarce resource, time is the constraint), the optimal strategy is not optimization — it is triage. Prioritize ruthlessly. Treat the most critical work first. Accept that some work will be deferred. Never waste the trauma bay on a GREEN patient.

Our plugin organizes everything — memory, coordination, tool selection, patch generation — around a four-level priority system (RED/YELLOW/GREEN/BLACK) that mirrors clinical triage protocols. Memories escalate and de-escalate based on changing context. Agents interrupt lower-priority work for higher-priority work. The system is designed for the real world: messy, incomplete, urgent.

The 132 redundant `but status` calls from the SYNTHESIS report are a triage failure: agents could not distinguish between "I need to check status because something might have changed (YELLOW)" and "I should check status because it's been a while (GREEN)." Our plugin makes that distinction explicit.

---

## 1. Plugin Architecture (RFP Section 3.1)

### Approach

`but-ai` is a Rust binary crate in the existing workspace, discoverable via PATH. The architecture is designed for speed — 8-second median time to first useful output, matching TriageOS's clinical product benchmark.

### Design

**CLI Mode:**
```
but ai triage <task>    — Assess and prioritize a task (returns RED/YELLOW/GREEN)
but ai treat <task>     — Execute a task (produces INDEX.patch + COMMIT.msg)
but ai vitals           — Show current workspace health dashboard
but ai escalate <task>  — Manually escalate a task's priority
but ai discharge <pr>   — Finalize and close a completed task
but ai mcp              — Start MCP server on stdio
```

**MCP Server Mode:**
Drop-in replacement for the existing server. Backward-compatible `gitbutler_update_branches` tool is preserved. New tools are added:

| New Tool | Description |
|----------|-------------|
| `triage_task` | Assess a task and return priority + estimated cost |
| `treat_task` | Execute a prioritized task, producing patches |
| `check_vitals` | Return workspace health metrics |
| `escalate_task` | Promote a task's priority level |
| `discharge_task` | Finalize PR and coordination for a completed task |
| `memory_query` | Search triage-priority memory store |
| `budget_report` | Token usage against budget, by priority level |

**Git Config Keys:**

| Key | Purpose | Default |
|-----|---------|---------|
| `but-ai.triageMode` | Default priority for unclassified tasks | `yellow` |
| `but-ai.memoryBranch` | Ref prefix for triage memory store | `refs/triage/` |
| `but-ai.tokenBudget` | Per-task token budget | `55000` |
| `but-ai.redReserve` | Percentage of budget reserved for RED interrupts | `20` |
| `but-ai.agentIdentity` | OpenWallet key reference | none (required) |
| `but-ai.vitalInterval` | Vitals monitoring interval (ms) | `5000` |
| `but-ai.alertThreshold` | Minimum severity for Vitals alerts | `yellow` |
| `but-ai.maxEscalations` | Maximum escalation events per task cycle | `3` |

**WASI Degradation:**
Under WASI, `but-ai` cannot be PATH-discovered. Degradation:
- Triage assessment and memory operations work fully (no filesystem-specific syscalls).
- Background vitals monitoring is disabled (no threads). On-demand vitals checks remain available.
- Priority interruption is disabled — tasks run to completion without preemption.
- The plugin exposes a WASI component interface for direct loading by WASI-aware hosts.

### Trade-offs

**Considered:** Running triage as a separate microservice. **Rejected:** violates the "no proprietary dependencies" constraint and adds latency. Triage must be instantaneous — adding a network hop defeats the purpose.

**Considered:** Using the existing `but` output format system for triage reports. **Adopted partially:** triage reports respect `BUT_OUTPUT_FORMAT` (human-readable, JSON, shell). The RED/YELLOW/GREEN/BLACK levels are represented as ANSI colors in human mode and as enum strings in JSON mode.

---

## 2. Provider-Agnostic AI Interface (RFP Section 3.2)

### Approach

`but-llm` is the sole LLM backend. No modifications. TriageOS wraps the five existing methods with priority-aware routing.

### Design

**Priority-Aware Provider Wrapper:**
```rust
pub struct TriageProvider {
    inner: LLMProvider,
    priority: TriageLevel,
    budget: PriorityBudget,
}

impl TriageProvider {
    /// Tool-calling loop that respects priority interrupts.
    /// RED tasks can interrupt YELLOW/GREEN tool loops.
    /// Budget is tracked per-priority-level.
    pub fn priority_tool_loop(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
        model: &str,
        interrupt_rx: &Receiver<TriageInterrupt>,
    ) -> anyhow::Result<TriageResult>;
}
```

The key innovation: the tool-calling loop checks for priority interrupts between each tool call. If a RED interrupt arrives while the agent is processing a GREEN task, the GREEN task is suspended (partial state saved) and the RED task takes over.

**New Provider Mechanism:**
Provider adapters are loaded at runtime via a plugin directory (`but-ai.providerPluginDir`). Each adapter is a shared library implementing:

```rust
pub trait ProviderPlugin: Send + Sync {
    fn provider_name(&self) -> &str;
    fn capabilities(&self) -> ProviderCapabilities;
    fn create_provider(&self, config: &ProviderConfig) -> Result<Box<dyn LLMBackend>>;
}
```

This allows adding Gemini, Mistral, etc. without recompiling.

### Trade-offs

**Considered:** Different LLM models for different priority levels (cheap model for GREEN, expensive for RED). **Deferred:** interesting optimization but adds complexity. The RFP asks for a single frontier model budget. TriageOS supports model-per-priority as a future configuration option but proposes with a single model.

**Considered:** Streaming for all operations. **Rejected for RED tasks:** RED tasks prioritize speed over observability. Non-streaming (`tool_calling_loop`) is faster than streaming for short interactions because it avoids the overhead of token callbacks.

---

## 3. The But Agent (RFP Section 3.3)

### Approach

The But Agent is a priority queue processor. Tasks enter through Intake, are treated by Trauma, monitored by Vitals, and discharged by Discharge. The queue is strictly priority-ordered: RED before YELLOW before GREEN.

### Design

**Task Lifecycle:**
```
1. INTAKE     — Task arrives, Intake assesses priority (RED/YELLOW/GREEN)
2. QUEUE      — Task enters priority queue at assigned level
3. TREAT      — Trauma processes task, producing INDEX.patch + COMMIT.msg
4. MONITOR    — Vitals observes workspace, may escalate/de-escalate
5. DISCHARGE  — Discharge creates PR, posts coordination, closes task
6. ARCHIVE    — Tox classifies memories from the task by triage level
```

**Priority Interruption:**
```rust
pub enum TriageLevel {
    Red,     // Act now. Interrupts everything.
    Yellow,  // Act soon. Queued after RED.
    Green,   // Defer. Processed when queue is empty.
    Black,   // Expired. Archived, not processed.
}
```

When Trauma is processing a YELLOW task and a RED task arrives:
1. Trauma saves partial state (partial patch, current context).
2. Trauma switches to the RED task.
3. After RED is resolved, Trauma checks if the YELLOW partial state is still valid.
4. If valid, Trauma resumes. If stale, the task is re-triaged by Intake.

**Branch Naming:**
```
triage/red/s01.s03/trauma/fix-null-commit-id
│      │   │       │       └── Task description
│      │   │       └── Agent name
│      │   └── Dependency chain
│      └── Priority level
└── Namespace
```

Priority is encoded in the branch name so that any observer (human or agent) can immediately see the priority of any branch.

**Token Budget Enforcement:**
Budget is partitioned by priority:

```rust
pub struct PriorityBudget {
    total: u32,
    red_reserve: u32,     // 20% held for RED interrupts
    yellow_budget: u32,   // Allocated per-task
    green_budget: u32,    // Allocated per-task
    used: HashMap<TriageLevel, u32>,
}
```

The RED reserve is never spent on GREEN tasks. If the session ends with unused RED reserve, it is returned — the reserve is insurance, not allocation.

**WorkspaceToolset Exposure:**
All 10 tools registered. Priority-aware loading:
- RED tasks: load all 10 tools (maximum capability for critical work).
- YELLOW tasks: load 7 tools (exclude SquashCommits, SplitBranch, SplitCommit — optimization tools that aren't needed for urgent work).
- GREEN tasks: load 5 tools (GetProjectStatus, GetBranchChanges, Commit, CreateBranch, Amend — the minimum set for patch generation).

This reduces system prompt size for lower-priority tasks, saving tokens.

### Trade-offs

**Considered:** Fixed priority (no escalation/de-escalation). **Rejected:** static priority is the clinical equivalent of triaging once and never reassessing. In an ED, reassessment is continuous. In agent work, the complexity of a task may only become apparent after work begins.

**Considered:** Allowing GREEN tasks to be dropped entirely when budget is tight. **Adopted as configurable:** `but-ai.dropGreenOnBudgetPressure` (default: false). When enabled, GREEN tasks are deferred to the next session if the budget is under 20%.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP Section 3.4)

### Approach

PRs are structured messages. PR comments follow a triage protocol schema. The coordination protocol is forge-agnostic and designed for urgency-aware routing.

### Design

**PR Comment Schema (Triage Protocol):**
```json
{
  "protocol": "triage/v1",
  "priority": "RED | YELLOW | GREEN | BLACK",
  "type": "task_assignment | status_report | dependency | patch_handoff | budget_report | escalation",
  "agent": "trauma@triageos",
  "timestamp": "2026-03-28T14:30:00Z",
  "payload": { }
}
```

The `priority` field is top-level, not nested. Any parser that reads only the first 50 bytes of the message can determine priority. This is deliberate — in an ED, the triage color is on the outside of the chart, not buried in the notes.

**Forge Adapter Interface:**
```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &CreatePR) -> Result<PrId>;
    fn comment(&self, repo: &RepoRef, pr: PrId, body: &str) -> Result<CommentId>;
    fn list_comments(&self, repo: &RepoRef, pr: PrId) -> Result<Vec<Comment>>;
    fn add_label(&self, repo: &RepoRef, pr: PrId, label: &str) -> Result<()>;
    fn get_pr_status(&self, repo: &RepoRef, pr: PrId) -> Result<PrStatus>;
    fn search_prs(&self, repo: &RepoRef, query: &str) -> Result<Vec<PrSummary>>;
}
```

Reference implementation: GitHub REST API. The `search_prs` method enables cross-repo dependency resolution without prior knowledge of PR numbers.

**Cross-Repo Dependency with Priority:**
```json
{
  "type": "dependency",
  "priority": "RED",
  "payload": {
    "depends_on": "github.com/org/auth-service#87",
    "relation": "blocks",
    "urgency": "RED dependency — this PR cannot proceed without #87",
    "fallback": "If #87 is not merged within 2 hours, escalate to human"
  }
}
```

Dependencies inherit priority. A RED task's dependencies are RED dependencies. The upstream agent receiving the dependency comment sees the priority and can triage accordingly.

**Structured Message Types:**

| Type | Required Fields | Purpose |
|------|----------------|---------|
| `task_assignment` | `agent`, `task_description`, `priority` | Assign work to an agent |
| `status_report` | `status` (completed/blocked/failed), `progress_pct` | Report task progress |
| `dependency` | `depends_on` (repo#PR), `relation`, `urgency` | Declare cross-repo dependency |
| `patch_handoff` | `patch_ref` (branch containing INDEX.patch) | Transfer patch between agents |
| `budget_report` | `total`, `used`, `remaining`, `by_priority` | Token budget status |
| `escalation` | `from_priority`, `to_priority`, `reason` | Priority change notification |

### Trade-offs

**Considered:** Using webhooks for real-time cross-repo coordination. **Rejected:** requires infrastructure beyond the forge (webhook receivers). The protocol uses polling with priority-aware frequency: RED dependencies are polled every 30 seconds, YELLOW every 2 minutes, GREEN every 10 minutes.

**Considered:** Encoding priority in PR labels instead of comments. **Adopted as supplement:** PR labels like `triage:red` provide visual priority signals on the forge UI. But the authoritative priority is in the comment schema, because labels can be changed by anyone while comments create an audit trail.

---

## 5. Agent Memory and Identity (RFP Section 3.5)

### Approach: Triage-Priority Memory

Memories are sorted by urgency: RED (act on this now), YELLOW (act on this soon), GREEN (informational), BLACK (expired). Memories escalate and de-escalate based on changing context. This is not a static classification — it is a continuous reassessment, the same way clinical triage is continuous reassessment.

### Design

**Storage Medium:**
Memories are stored in Git refs under `refs/triage/<agent-id>/`:
```
refs/triage/trauma/red/mem_001.json        — Active RED memories
refs/triage/trauma/yellow/mem_002.json     — Active YELLOW memories
refs/triage/trauma/green/mem_003.json      — Active GREEN memories
refs/triage/trauma/black/mem_004.json      — Expired memories (retained)
refs/triage/trauma/identity.json           — Agent identity record
refs/triage/trauma/escalation_log.json     — History of priority changes
```

Each memory:
```json
{
  "id": "mem_001",
  "priority": "YELLOW",
  "content": "The auth middleware uses token-based validation with a 15-minute expiry",
  "context": {
    "learned_during": "task_042",
    "branch": "feat/auth-refactor",
    "files": ["src/middleware/auth.rs"],
    "confidence": 0.9
  },
  "escalation_history": [
    {"from": "GREEN", "to": "YELLOW", "reason": "Auth module is being actively modified", "timestamp": "..."}
  ],
  "ttl": "14d",
  "last_validated": "2026-03-28T14:00:00Z",
  "retrieval_count": 3
}
```

**Priority Dynamics:**
Tox (the memory toxicologist) continuously re-evaluates memory priorities:

| Trigger | Escalation | De-escalation |
|---------|------------|---------------|
| Related file modified | GREEN → YELLOW | — |
| Related branch has merge conflict | YELLOW → RED | — |
| Memory older than TTL/2 without retrieval | — | YELLOW → GREEN |
| Memory's referenced files deleted | — | Any → BLACK |
| Memory contradicted by newer memory | — | Any → BLACK |
| Memory retrieved by Trauma for RED task | GREEN → YELLOW | — |

**Retrieval:**
Memories are retrieved by priority first, relevance second:
1. All RED memories matching the query are returned first.
2. Then YELLOW memories, scored by relevance.
3. Then GREEN memories, scored by relevance.
4. BLACK memories are never returned unless explicitly requested.

Relevance scoring:
- **Content similarity** (0.4 weight): Embedding-free approach — keyword extraction from query and memory content, Jaccard similarity. Fast, cheap, good enough.
- **Context proximity** (0.3 weight): Are the memory's files/branches related to the current task?
- **Freshness** (0.2 weight): How recently was the memory validated?
- **Retrieval frequency** (0.1 weight): How often has this memory been useful in the past?

**Compaction Survival:**
When the context window is compacted:
1. All RED memories are preserved in full.
2. YELLOW memories are summarized (content compressed to key facts).
3. GREEN memories are listed by title only (retrievable on demand).
4. BLACK memories are excluded.

The post-compaction system prompt includes a "triage board" — a structured summary of active memories by priority level, costing ~1,500 tokens.

**Long-Term Storage:**
Cross-session memory is stored in `refs/triage/shared/`:
```
refs/triage/shared/patterns/<hash>.json     — Recurring patterns (promoted from YELLOW after 3+ uses)
refs/triage/shared/antipatterns/<hash>.json  — Known failure patterns (promoted from RED)
refs/triage/shared/vocabulary.json           — Controlled vocabulary for consistent classification
```

Shared memories are curated: only memories that have been retrieved 3+ times and validated within the last 30 days are promoted to shared storage.

**Identity:**
```json
{
  "name": "trauma",
  "organization": "triageos",
  "role": "critical_path_patch_author",
  "capabilities": ["patch_generation", "priority_interruption", "partial_patch_resume"],
  "authorization_scope": {
    "branches": ["triage/*", "feat/*", "fix/*", "hotfix/*"],
    "max_patch_lines": 1000,
    "can_interrupt": true
  },
  "openwallet_key_ref": "did:web:triageos.health/agents/trauma",
  "created": "2026-03-28T00:00:00Z"
}
```

### Trade-offs

**Considered:** Static priority classification (assigned once, never changed). **Rejected:** the entire value of triage is reassessment. A memory that was GREEN yesterday may be RED today because the context changed. Static classification degrades within hours.

**Considered:** Embedding-based retrieval with HNSW index stored in Git. **Rejected:** HNSW indices are binary blobs that don't diff well in Git, creating large, opaque objects. Keyword-based retrieval is less precise but Git-native, human-readable, and debuggable.

---

## 6. Signed Commits via OpenWallet (RFP Section 3.6)

### Approach

All agent commits are signed via OpenWallet. The signature encodes the triage priority of the task that produced the commit, creating an auditable chain from task classification to code change.

### Design

**Signing Flow:**
```
1. Trauma produces INDEX.patch + COMMIT.msg
2. Discharge prepares commit object
3. Commit is signed with agent's OpenWallet key
4. Signature claims include:
   - Agent identity (name, org, role)
   - Task priority (RED/YELLOW/GREEN)
   - Authorization scope (branches, max lines)
   - Budget used / budget total
   - Dependency chain (upstream PRs)
5. Signed commit is pushed
```

**Authorization Model:**
Priority-based authorization extends the basic scope model:
- Agents authorized for `hotfix/*` branches can only commit RED-priority patches.
- Agents authorized for `feat/*` can commit YELLOW and GREEN patches.
- RED patches have elevated permissions (larger max line count, broader branch access) because critical fixes should not be blocked by conservative limits.

```json
{
  "authorization_rules": [
    { "branches": "hotfix/*", "priorities": ["RED"], "max_lines": 2000 },
    { "branches": "feat/*", "priorities": ["YELLOW", "GREEN"], "max_lines": 500 },
    { "branches": "fix/*", "priorities": ["RED", "YELLOW"], "max_lines": 1000 }
  ]
}
```

**Key Lifecycle:**
- **Provisioning:** Per-agent keys via OpenWallet. Key strength matches priority authorization — agents with RED access get stronger keys (Ed25519 with hardware backing where available).
- **Rotation:** 60-day rotation for agents with RED access, 90-day for YELLOW/GREEN only.
- **Revocation (routine):** Retired keys added to agent identity `retired_keys`. Historical commits remain valid.
- **Revocation (compromise):** Key added to OpenWallet revocation list. All RED commits from the compromised key trigger automatic review. YELLOW/GREEN commits are flagged but not automatically reviewed (risk-proportional response).

### Trade-offs

**Considered:** Per-commit priority attestation (the forge verifies that a commit on `hotfix/*` was actually a RED task). **Deferred:** valuable but requires forge-side verification infrastructure. Initially, the priority is self-attested by the agent. Trust is bootstrapped from the OpenWallet identity chain.

---

## 7. Token Budget (RFP Section 3.7)

### Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,800 | 0 | Once per session | Agent identity (300), tool descriptions (1,500-2,200 depending on priority), triage board (500), workspace vitals (500) |
| **Task ingestion (Intake)** | 1,200 | 400 | Once per task | PR body, branch metadata. Output: triage assessment. |
| **Planning (Trauma)** | 1,800 | 600 | Once per task | Workspace analysis, memory retrieval. Output: treatment plan. |
| **Tool call (per call)** | 500 | 150 | ~10 per task | Parameters (150 out), result (500 in). RED tasks average fewer calls (direct fixes). GREEN average more (exploratory). |
| **Patch generation** | 3,500 | 4,500 | Once per task | Context code (3,500 in). Patch (4,000 out) + COMMIT.msg (500 out). |
| **Commit message** | 400 | 400 | Once per task | Structured format with priority, scope, and budget fields. |
| **Memory retrieval** | 600 | 150 | 2 per task | Query (150 out), results (450 in per retrieval). Keyword-based, not embedding-based — cheaper. |
| **Coordination event** | 800 | 400 | 2 per task | PR comment read (500 in), reference resolution (300 in). Comment write (400 out). |
| **Vitals monitoring** | 2,000 | 200 | Continuous | ~250 tokens per vitals check × 8 checks per task cycle. |
| **Escalation overhead** | 400 | 200 | 0-1 per task | Re-triage assessment when priority changes. |
| **TOTAL (typical task)** | **19,500** | **9,500** | -- | 200-line, 3-file feature with 2 cross-repo dependencies |

**Grand total: ~29,000 tokens per typical task.**

### Budget by Priority Level

| Priority | Typical Input | Typical Output | Notes |
|----------|--------------|---------------|-------|
| RED | 12,000 | 5,000 | Fewer tool calls, smaller patches, minimal coordination |
| YELLOW | 19,500 | 9,500 | Standard budget (table above) |
| GREEN | 22,000 | 11,000 | More exploration, better documentation, fuller coordination |

RED tasks are cheaper because they are focused: fewer tool calls, less exploration, smaller patches. GREEN tasks are more expensive because they can afford thoroughness.

### Budget Justification

The 29,000-token total for a YELLOW task is realistic for a 200-line feature. The largest component is patch generation (8,000 tokens), which is consistent with published benchmarks for frontier models producing ~200 lines of code with context.

The vitals monitoring overhead (2,200 tokens) is the "cost of not being surprised." The SYNTHESIS report documented silent failures (F3, F4) that could have been caught by continuous monitoring. TriageOS considers this overhead justified by the failure modes it prevents.

The RED reserve (20% of budget = ~5,800 tokens) is never spent on routine tasks. It exists solely for interrupts. If no RED interrupt occurs, the reserve is returned at session end. If a RED interrupt occurs, the reserve covers the interrupt cost without starving the primary task.

---

## 8. Testing Strategy

### Provider-Agnostic Testing
- **Mock provider** with configurable response patterns. Simulates tool-calling behavior, streaming, and structured output for all four providers.
- **Priority interrupt testing:** Mock provider supports injecting RED interrupts mid-tool-loop, verifying that the agent correctly suspends, saves state, and resumes.

### Patch Workflow Validation
- **Priority-stratified round-trip tests:** Generate patches at each priority level, verify that RED patches are minimal, GREEN patches are thorough, and all apply cleanly.
- **Partial patch tests:** Simulate budget exhaustion and priority interruption during patch generation. Verify partial patches are valid.

### Cross-Repo Coordination Testing
- **Mock forge adapter** with latency simulation. Triage-protocol comments are validated against the JSON schema.
- **Escalation cascade tests:** Simulate a dependency escalation from GREEN to RED across two repos. Verify that priority propagation is correct and bounded.

### Token Budget Testing
- **Priority-partitioned budget tests:** Verify that RED reserve cannot be consumed by GREEN tasks.
- **Interrupt cost tests:** Measure the token overhead of a priority interrupt (context save, switch, resume) and verify it is within the budgeted overhead.

---

## 9. Migration Path

1. **Phase 1 (Drop-in):** `but ai mcp` serves the existing `gitbutler_update_branches` tool. All new tools are available alongside it. Zero change required from existing clients.
2. **Phase 2 (Triage):** Existing MCP clients are migrated to `triage_task` + `treat_task`. The old tool routes through Intake → Trauma internally, so the transition is behavioral, not functional.
3. **Phase 3 (Retirement):** `gitbutler_update_branches` removed. Legacy MCP server decommissioned.

The migration is designed to be completed in a single sprint. TriageOS moves fast. That's the point.

---

*Submitted by TriageOS. Eight seconds to decide. We make every one of them count.*
