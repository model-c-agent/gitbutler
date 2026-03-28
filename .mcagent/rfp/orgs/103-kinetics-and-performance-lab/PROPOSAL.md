# Proposal — The Kinetics & Performance Lab

**RFP Response: `but ai` Plugin for GitButler CLI**
**Organization:** The Kinetics & Performance Lab (Org 103)
**Domain:** Sports Analytics | **Philosophy:** Academic Research Lab
**Date:** 2026-03-28

---

## Executive Summary

The Kinetics & Performance Lab proposes a `but-ai` plugin built on the principle that observation frequency determines the quality of insight. Our approach treats the GitButler workspace as a biomechanical system — a complex, dynamic entity whose behavior is invisible at low resolution and interpretable at high resolution. The plugin captures agent behavior as high-frequency motion data, stores it in a motion-capture memory system, and provides playback tools that let agents and humans analyze repository evolution at different temporal scales.

Our core insight: the 132 redundant `but status --json` calls identified in the SYNTHESIS report are not a caching problem. They are a frame rate problem. Agents poll because they have no high-frequency observation channel. We build that channel.

---

## 1. Plugin Architecture (RFP Section 3.1)

### Approach

`but-ai` is implemented as a Rust binary crate (`crates/but-ai/`) within the existing workspace. It is discoverable via `find_external_subcommand()` in `crates/but/src/alias.rs` as a PATH-based executable.

### Design

**CLI Mode:**
```
but ai observe    — Start high-frequency workspace observation
but ai analyze    — Run pattern analysis on captured motion data
but ai agent      — Execute an autonomous task (produces INDEX.patch + COMMIT.msg)
but ai replay     — Play back a session at configurable speed
but ai memory     — Query/manage the motion-capture memory store
but ai mcp        — Start MCP server on stdio
```

**MCP Server Mode:**
The MCP server implements `ServerHandler` via `rmcp`, maintaining backward compatibility with the existing `gitbutler_update_branches` tool while exposing the full `WorkspaceToolset` (10 tools) plus four new tools:

| New Tool | Description |
|----------|-------------|
| `observe_frame` | Capture a single workspace state frame |
| `replay_session` | Retrieve session frames at specified playback speed |
| `query_memory` | Search motion-capture memory by relevance |
| `budget_status` | Report current token usage against budget |

**Environment Variables:**
All three plugin environment variables (`BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`) are consumed. The plugin adds:

| New Git Config Key | Purpose | Default |
|--------------------|---------|---------|
| `but-ai.frameRate` | Default observation frequency (ms between frames) | `1000` |
| `but-ai.memoryBranch` | Branch prefix for motion-capture memory storage | `refs/motion/` |
| `but-ai.tokenBudget` | Default per-task token budget | `50000` |
| `but-ai.agentIdentity` | Path to agent identity file (OpenWallet key ref) | none (required) |
| `but-ai.replaySpeed` | Default playback speed multiplier for `replay` | `1.0` |
| `but-ai.compactionThreshold` | Max frames before automatic compaction | `10000` |

**WASI Degradation:**
Under WASI builds, `but-ai` cannot be discovered as a PATH plugin. The graceful degradation path:
- The `but-ai` MCP tools are compiled as a WASI component that can be loaded directly by a WASI-aware host (bypassing PATH discovery).
- Observation frequency is reduced (no background threads in WASI), but on-demand frame capture is available.
- Memory operations remain fully functional (Git-native, no filesystem-specific syscalls).

### Trade-offs

**Considered:** Implementing the plugin as a standalone process that communicates with `but` via IPC (Unix sockets, named pipes). **Rejected** because: adds complexity, introduces another failure mode (IPC channel failure), and is incompatible with WASI. The PATH-based plugin model is simpler and sufficient.

**Considered:** Implementing observation as a daemon process. **Rejected** because: daemon management is complex, and the existing `but` sync daemon already causes contention (F4). Instead, observation is event-driven — triggered by tool calls and state changes, not by a timer.

---

## 2. Provider-Agnostic AI Interface (RFP Section 3.2)

### Approach

We use `but-llm` as-is. No modifications, no new LLM client. The `but-ai` plugin consumes `LLMProvider::from_git_config()` and routes all LLM interactions through the five existing methods.

### Design

**Provider Routing Layer:**
```rust
pub struct MotionAIProvider {
    inner: LLMProvider,
    budget: TokenBudget,
    frame_buffer: FrameBuffer,
}

impl MotionAIProvider {
    /// Tool-calling loop with frame capture and budget tracking.
    /// Wraps LLMProvider::tool_calling_loop, capturing workspace state
    /// before and after each tool call.
    pub fn observed_tool_loop(
        &self,
        system_message: &str,
        chat_messages: Vec<ChatMessage>,
        tool_set: &mut impl Toolset,
        model: &str,
    ) -> anyhow::Result<(String, Vec<Frame>)>;
}
```

The wrapper adds two capabilities: (1) frame capture around each tool call, and (2) budget tracking that halts the loop when tokens approach the limit.

**New Provider Mechanism:**
For adding providers beyond the existing four, we define a `ProviderAdapter` trait:

```rust
pub trait ProviderAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn supports_tool_calling(&self) -> bool;
    fn supports_streaming(&self) -> bool;
    fn translate_request(&self, req: &LLMRequest) -> anyhow::Result<HttpRequest>;
    fn translate_response(&self, resp: HttpResponse) -> anyhow::Result<LLMResponse>;
}
```

Provider adapters are loaded from shared libraries (`.so`/`.dylib`) found at a configurable path (`but-ai.providerPluginDir`). This allows adding Gemini, Mistral, or local GGUF models without recompiling `but-ai`.

### Trade-offs

**Considered:** Building adapters into the `but-ai` binary with feature flags. **Rejected** because: requires recompilation for each new provider, violating the RFP requirement. Dynamic loading adds runtime complexity but enables the extensibility the RFP demands.

**Considered:** Using `but-llm`'s structured_output for all agent responses. **Rejected** because: structured output constrains the model's reasoning. We use structured output for budget reports and coordination messages, but freeform output for patch generation and commit messages.

---

## 3. The But Agent (RFP Section 3.3)

### Approach

The But Agent operates as a three-phase biomechanical cycle: Observation (capture workspace state), Analysis (identify patterns and plan), and Motion (produce patches). The agent produces `INDEX.patch` + `COMMIT.msg` exclusively — no direct file edits, no `git commit`, no `but commit`.

### Design

**Task Lifecycle:**
```
1. CAPTURE    — Strobe captures initial workspace state at high frequency
2. RETRIEVE   — Replay queries memory for relevant prior work
3. PLAN       — Gait decomposes task into movement phases
4. EXECUTE    — Gait produces INDEX.patch + COMMIT.msg
5. VERIFY     — Strobe captures post-task state, Sync validates
6. RECORD     — Replay stores the session as a new motion-capture memory
```

**Branch Naming:**
We extend the existing `s01.s04` convention with kinematic metadata:

```
s01.s04/gait/auth-refactor
│   │   │     └── Task description
│   │   └── Agent name
│   └── Dependency (depends on s01)
└── Sequence number
```

The agent name in the branch encodes identity. The dependency chain encodes the kinematic sequence — the order of movements.

**Token Budget Enforcement:**
```rust
pub struct TokenBudget {
    total: u32,
    used_input: u32,
    used_output: u32,
    reserve: u32,  // Held for graceful shutdown (COMMIT.msg + partial patch)
}

impl TokenBudget {
    pub fn can_afford(&self, estimated_cost: u32) -> bool {
        self.used_input + self.used_output + estimated_cost + self.reserve <= self.total
    }

    pub fn enter_graceful_shutdown(&mut self) -> bool {
        // Release reserve for final patch production
        self.reserve = 0;
        true
    }
}
```

When the budget approaches the limit, the agent enters "graceful shutdown" — it stops planning, produces whatever partial patch it has, and writes a COMMIT.msg that documents the incomplete state.

**WorkspaceToolset Exposure:**
All 10 workspace tools are registered with the LLM via the `Toolset` trait. The `observed_tool_loop` wrapper ensures that every tool call is captured as a motion frame:

```
Frame N:   GetProjectStatus → {workspace state}
Frame N+1: CreateBranch("s01.s03/gait/auth-refactor") → {branch created}
Frame N+2: GetBranchChanges → {diff data}
...
Frame N+K: Commit(INDEX.patch, COMMIT.msg) → {commit result}
```

### Trade-offs

**Considered:** Allowing the agent to operate without token budget limits. **Rejected** because: unbounded agents are unbounded costs. The budget is a constraint that forces efficiency, the same way a motion-capture session has a fixed recording duration.

**Considered:** Pre-loading all workspace tools into the system prompt. **Rejected** because: the 10 tool descriptions consume ~2,500 tokens. Instead, we use lazy tool registration — only tools relevant to the current task phase are loaded. Observation phase: GetProjectStatus, GetBranchChanges, GetCommitDetails. Action phase: Commit, CreateBranch, Amend. This reduces the per-call tool overhead by ~60%.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP Section 3.4)

### Approach

PRs are the coordination medium. PR comments carry structured messages. The protocol is forge-agnostic, requiring only: PR creation, PR comments, and PR labels (or title encoding as fallback).

### Design

**PR Comment Schema (Motion Protocol):**
```json
{
  "protocol": "motion/v1",
  "type": "task_assignment | status_report | dependency | patch_handoff | budget_report",
  "agent": "gait@kinetics-lab",
  "timestamp": "2026-03-28T14:30:00Z",
  "payload": {
    // Type-specific fields
  }
}
```

Comments are wrapped in a code block with language tag `motion-protocol` to prevent forge rendering from corrupting the JSON:

````
```motion-protocol
{ "protocol": "motion/v1", ... }
```
````

**Forge Adapter Interface:**
```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &PullRequest) -> Result<PrId>;
    fn comment(&self, repo: &RepoRef, pr: PrId, body: &str) -> Result<CommentId>;
    fn list_comments(&self, repo: &RepoRef, pr: PrId) -> Result<Vec<Comment>>;
    fn add_label(&self, repo: &RepoRef, pr: PrId, label: &str) -> Result<()>;
    fn get_pr(&self, repo: &RepoRef, pr: PrId) -> Result<PullRequest>;
}
```

Reference implementation: GitHub (via REST API). The adapter handles pagination, rate limiting, and authentication.

**Cross-Repo Coordination:**
Dependencies between repos are declared as motion-protocol comments with explicit repo references:

```json
{
  "type": "dependency",
  "payload": {
    "depends_on": "github.com/org/other-repo#42",
    "relation": "blocks",
    "description": "Auth middleware change must land before this PR"
  }
}
```

The Sync agent monitors dependencies and updates status when upstream PRs are merged.

### Trade-offs

**Considered:** Using PR labels for structured metadata instead of comments. **Rejected** as primary mechanism because: labels have character limits, are not versioned, and some forges limit the number of labels. Labels are used as supplementary signals (e.g., `motion:red` for priority) but not for structured data.

**Considered:** Using a dedicated coordination branch instead of PR comments. **Rejected** because: it adds Git complexity and doesn't work well cross-repo. PR comments are natively cross-repo (you can reference any PR by URL).

---

## 5. Agent Memory and Identity (RFP Section 3.5)

### Approach: Motion-Capture Memory

Memories are stored as high-frequency time-series — motion-capture sessions that can be played back at different speeds to reveal different patterns. This is a direct translation of the Lab's core research methodology.

### Design

**Storage Medium:**
Memories are stored in Git refs under `refs/motion/<agent-id>/`:
```
refs/motion/gait/sessions/2026-03-28T14-30-00Z.json    — Session recording
refs/motion/gait/index/subject.json                      — Subject index
refs/motion/gait/index/temporal.json                     — Temporal index
refs/motion/gait/index/kinematic.json                    — Kinematic index (rate of change)
refs/motion/gait/identity.json                           — Agent identity record
```

Each session is a sequence of frames:
```json
{
  "session_id": "sess_2026-03-28_001",
  "agent": "gait",
  "task": "implement auth middleware",
  "frames": [
    {
      "frame_number": 0,
      "timestamp": "2026-03-28T14:30:00.000Z",
      "type": "workspace_state",
      "data": { "branches": [...], "status": "clean" },
      "delta_from_previous": null
    },
    {
      "frame_number": 1,
      "timestamp": "2026-03-28T14:30:01.200Z",
      "type": "tool_call",
      "data": { "tool": "GetProjectStatus", "result_hash": "abc123" },
      "delta_from_previous": { "new_tool_call": true }
    }
  ],
  "summary": "Implemented auth middleware token validation...",
  "ttl": "30d",
  "playback_speeds": {
    "1x": "all frames",
    "10x": "every 10th frame",
    "100x": "key frames only (tool calls, state changes)"
  }
}
```

**Retrieval:**
Replay scores memories by relevance using three signals:
1. **Textual similarity** — Embedding-based similarity between the query and the session summary (computed via `but-llm`'s `response` method with a similarity prompt).
2. **Kinematic similarity** — How similar is the pattern of tool usage? A task that used GetProjectStatus → CreateBranch → Commit in the past is relevant to a task that is about to do the same sequence.
3. **Temporal proximity** — Recent memories score higher, with exponential decay (half-life configurable via `but-ai.memoryHalfLife`, default 14 days).

Combined score: `0.5 * textual + 0.3 * kinematic + 0.2 * temporal`

**Expiration:**
Each memory has a TTL set at creation. Replay monitors TTLs and transitions expired memories from "active" to "archived." Archived memories are not deleted — they are moved to `refs/motion/<agent-id>/archive/` and excluded from default retrieval. They can be explicitly queried for historical analysis.

**Compaction Survival:**
When the LLM context window is compacted, the agent preserves persistent memory by:
1. Writing all critical memories to the Git-native store before compaction.
2. After compaction, the system prompt includes a "memory rehydration" section: a compressed summary of the most relevant memories, injected by Replay.
3. The rehydration section is bounded at 2,000 tokens (configurable) and includes pointers to the full memory refs for on-demand retrieval.

**Long-Term Storage (MotionDB):**
The Lab's existing MotionDB concept extends to the agent domain. Long-term memory is stored in a dedicated repository (or branch) that agents across sessions and tasks can contribute to and query. The structure:
```
refs/motiondb/patterns/<pattern-hash>.json    — Reusable patterns
refs/motiondb/vocabulary/<term>.json          — Controlled vocabulary entries
refs/motiondb/sessions/<session-hash>.json    — Archived sessions
```

**Identity:**
Each agent has an identity record in `refs/motion/<agent-id>/identity.json`:
```json
{
  "name": "gait",
  "organization": "kinetics-and-performance-lab",
  "capabilities": ["patch_generation", "branch_management", "pattern_analysis"],
  "authorization_scope": {
    "branches": ["feat/*", "fix/*"],
    "max_patch_lines": 500,
    "repos": ["gitbutler/gitbutler"]
  },
  "openwallet_key_ref": "did:web:kinetics-lab.example/agents/gait",
  "created": "2026-03-28T00:00:00Z",
  "key_rotation_policy": "90d"
}
```

### Trade-offs

**Considered:** Storing memories as flat key-value entries. **Rejected** because: flat entries lose temporal context. A memory about "the auth module was refactored" is less useful than a motion-capture session showing the sequence of changes that constituted the refactor. The temporal structure is the insight.

**Considered:** Using an embedding database (HNSW, etc.) for retrieval. **Rejected** because: the RFP requires Git-native storage. Embeddings require a separate index that is not Git-native. Instead, we use LLM-based relevance scoring at query time, which is more expensive per query but requires no external infrastructure.

---

## 6. Signed Commits via OpenWallet (RFP Section 3.6)

### Approach

Every agent commit is signed using an OpenWallet-managed key. The signing key is provisioned per-agent and tied to the agent's identity record.

### Design

**Signing Flow:**
```
1. Agent produces INDEX.patch + COMMIT.msg
2. Orchestrator applies patch and creates commit object
3. Commit is signed using the agent's OpenWallet key
4. Signature includes agent identity claims:
   - Agent name and organization
   - Authorized branch patterns
   - Task reference (PR/issue that authorized the work)
   - Token budget used
5. Signed commit is pushed to the repository
```

**Authorization Model:**
The identity record (Section 5) defines the authorization scope. Before signing, the orchestrator verifies:
1. The target branch matches `authorization_scope.branches`
2. The patch size is within `authorization_scope.max_patch_lines`
3. The target repo is in `authorization_scope.repos`
4. The agent's key is not revoked

Verification is performed by checking the OpenWallet revocation list and the agent's identity record in `refs/motion/<agent-id>/identity.json`.

**Key Lifecycle:**
- **Provisioning:** New agent keys are generated via OpenWallet CLI and registered in the identity record. The key's DID (Decentralized Identifier) is stored in the identity record.
- **Rotation:** Keys are rotated on a configurable schedule (`key_rotation_policy`). During rotation, the old key is marked as "retired" (not "revoked") and remains valid for verifying historical commits. The new key is activated.
- **Revocation for rotation:** Old key DID is added to a `retired_keys` list in the identity record. Historical commits remain verifiable.
- **Revocation for compromise:** Key DID is added to the OpenWallet revocation list. All commits signed with the compromised key are flagged as suspect. The agent's identity record is updated with a `compromised_key_event` entry that records the timestamp and scope of the compromise.

**Commits as Authorization Tokens:**
A signed commit encodes the full authorization chain:
```
Signer: did:web:kinetics-lab.example/agents/gait
Authorized-By: ISSUE#42 (task assignment)
Scope: feat/* branches, max 500 lines
Budget: 18000/20000 tokens used
Timestamp: 2026-03-28T14:45:00Z
```

This information is embedded in the commit's signature metadata (OpenWallet credential claims), making the commit itself a verifiable authorization artifact.

### Trade-offs

**Considered:** Using GPG keys instead of OpenWallet. **Rejected** because: the RFP mandates OpenWallet. GPG also lacks the credential/claims model needed for authorization encoding.

**Considered:** Signing patches instead of commits. **Rejected** because: patches are intermediate artifacts. The commit is the durable, auditable unit. Signing the commit signs the final result, not an intermediate step.

---

## 7. Token Budget (RFP Section 3.7)

### Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Agent identity (400), tool descriptions (1,800 with lazy loading), workspace state summary (600), memory context (400) |
| **Task ingestion** | 1,500 | 300 | Once per task | PR body (~800), branch metadata (~400), issue description (~300). Output: task decomposition plan. |
| **Planning** | 2,000 | 800 | Once per task | Workspace analysis (1,200), memory retrieval results (800). Output: movement plan (sequence of operations). |
| **Tool call (per call)** | 600 | 200 | ~8 per task | Tool parameters (100 out), tool result (500 in), frame capture overhead (100 in). |
| **Patch generation** | 3,000 | 4,000 | Once per task | Target code context (3,000 in). Generated patch (3,500 out) + COMMIT.msg (500 out). |
| **Commit message** | 500 | 300 | Once per task | Context from patch (500 in). Structured commit message (300 out). |
| **Memory retrieval** | 800 | 200 | 2 per task | Query formulation (200 out), relevance scoring (400 in), memory injection (400 in per retrieval). |
| **Coordination event** | 1,000 | 500 | 1 per task | PR comment read (600 in), cross-repo reference (400 in). Comment write (500 out). |
| **Frame capture overhead** | 1,600 | 0 | ~8 per task | 200 tokens per frame × 8 frames. State delta encoding. |
| **TOTAL (typical task)** | **18,800** | **8,300** | -- | 200-line, 3-file feature with 1 cross-repo dependency |

**Grand total: ~27,100 tokens per typical task.**

### Budget Justification

The budget is dominated by two components: patch generation (7,000 tokens) and tool calls (6,400 tokens for ~8 calls). This reflects the Lab's observation that the actual "motion" — writing code — is the expensive part, and the observation overhead (frame capture) is relatively cheap.

The system prompt is kept under 3,200 tokens through lazy tool loading: only 4-5 tools are loaded per phase (observation tools in the observation phase, action tools in the action phase), saving ~1,000 tokens compared to loading all 10 tools upfront.

Memory retrieval is budgeted conservatively at 2 retrievals per task. Complex tasks may require more, but the exponential decay in relevance scoring means that the third and subsequent retrievals rarely add actionable information.

The Lab's total team budget (48,000 tokens across 4 agents, from AGENTS.md) accommodates the per-task cost (27,100) plus inter-agent communication overhead (~8,000), background observation (~5,000), and a 15% contingency reserve (~8,000).

---

## 8. Testing Strategy

### Provider-Agnostic Testing
- **Mock LLM provider** implementing `LLMProvider` that returns deterministic responses. Allows testing the full agent lifecycle without live API calls.
- **Provider parity tests:** Each test case is run against all four providers via the mock. The mock simulates provider-specific behaviors (e.g., different tool-calling formats for OpenAI vs. Anthropic).

### Patch Workflow Validation
- **Round-trip tests:** Generate a known `INDEX.patch`, apply it via the orchestrator, verify the resulting commit matches the expected state.
- **Partial patch tests:** Simulate budget exhaustion mid-patch, verify that the partial patch is valid (applies without error) and that the COMMIT.msg documents the incomplete state.
- **Conflict tests:** Apply a patch to a workspace that has changed since the patch was generated. Verify that conflict detection and reporting work correctly.

### Cross-Repo Coordination Testing
- **Mock forge adapter** implementing `ForgeAdapter` that simulates PR creation, commenting, and label management in memory.
- **Multi-repo scenario tests:** Simulate a task that spans two repositories, verify that dependency declarations, status reports, and patch handoffs follow the motion protocol schema.

### Token Budget Testing
- **Budget enforcement tests:** Run agent tasks with artificially low budgets, verify graceful shutdown at each budget threshold.
- **Budget accuracy tests:** Run agent tasks with a counting mock LLM that reports exact token usage, verify that the budget tracking matches within 5%.

### Frame Capture Testing
- **Frame integrity tests:** Capture frames during a multi-tool-call session, verify that frame sequence numbers are contiguous, timestamps are monotonic, and delta encoding is correct.
- **Playback tests:** Capture a session, play it back at 1x, 10x, and 100x speeds, verify that the correct frames are included at each speed.

---

## 9. Migration Path

The migration from the current MCP server (`crates/but/src/command/legacy/mcp/mod.rs`) to `but-ai` is designed for zero downtime:

1. **Phase 1:** `but-ai mcp` implements the existing `gitbutler_update_branches` tool alongside new tools. Existing MCP clients continue to work unchanged.
2. **Phase 2:** Existing MCP clients are updated to use the new tools. `gitbutler_update_branches` is deprecated but remains functional.
3. **Phase 3:** `gitbutler_update_branches` is removed. The legacy MCP server is decommissioned.

Each phase is independently deployable. No phase depends on the completion of a subsequent phase.

---

*Submitted by The Kinetics & Performance Lab. Every movement tells a story. We read the ones that haven't been written yet.*
