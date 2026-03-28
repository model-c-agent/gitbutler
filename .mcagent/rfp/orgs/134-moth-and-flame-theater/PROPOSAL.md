# Proposal — Moth & Flame Theater

**RFP Response: `but ai` Plugin for GitButler CLI**
**Organization:** Moth & Flame Theater (Org 134)
**Domain:** Theater Production | **Philosophy:** Artist Commune
**Date:** 2026-03-28

---

## Executive Summary

Moth & Flame Theater proposes a `but-ai` plugin conceived as a theatrical production. The repository is the performance space. Agents are actors. PRs are scenes. The audience — human developers, CI systems, downstream agents — moves through the production and constructs meaning from what they witness.

The central claim: agent coordination is a staging problem, not a scheduling problem. The question is not "when should this happen?" but "where should this happen, in relation to what, and who should witness it?" This reframing — from temporal to spatial, from scheduling to blocking — unlocks a coordination model that is naturally parallel, inherently observable, and deeply tolerant of the unexpected.

Our distinctive contribution is the script/cue memory system: memories stored as theatrical scripts with blocking notes (spatial relationships between memories), cue sheets (triggers for memory retrieval), and rehearsal marks (reliability signals based on usage history).

The 132 redundant `but status` calls are an audience problem: agents are wandering the performance space, checking every room because no one told them where the scene is happening. Our cue system tells them.

---

## 1. Plugin Architecture (RFP Section 3.1)

### Approach

`but-ai` is a Rust binary crate in the workspace, PATH-discoverable. The architecture is modeled on a stage management system: a cue-driven orchestrator that coordinates a company of specialized agents.

### Design

**CLI Mode:**
```
but ai rehearse <task>    — Dry-run a task (plan without executing)
but ai perform <task>     — Execute a task (produces INDEX.patch + COMMIT.msg)
but ai cue <agent> <cue>  — Manually call a cue for a specific agent
but ai script <query>     — Query the script/cue memory system
but ai house <pr>         — Manage front-of-house (PR creation, coordination)
but ai curtain            — End current task session, archive memories
but ai mcp                — Start MCP server on stdio
```

**MCP Server Mode:**
Drop-in replacement preserving `gitbutler_update_branches`. New tools:

| New Tool | Description |
|----------|-------------|
| `rehearse_task` | Dry-run: plan task, estimate cost, preview blocking |
| `perform_task` | Full execution: produce patches via cue-driven workflow |
| `call_cue` | Trigger a specific cue in the production sequence |
| `query_script` | Search memory by cue trigger, blocking notes, or content |
| `house_manage` | PR operations: create, comment, coordinate |
| `curtain_call` | End session: archive, summarize, report |
| `budget_report` | Token usage as "running time" (how long the show has been) |

**Git Config Keys:**

| Key | Purpose | Default |
|-----|---------|---------|
| `but-ai.venue` | Description of the workspace's "character" (auto-detected) | auto |
| `but-ai.memoryBranch` | Ref prefix for script/cue memory | `refs/script/` |
| `but-ai.tokenBudget` | Per-task token budget ("show length") | `58000` |
| `but-ai.agentIdentity` | OpenWallet key reference | none (required) |
| `but-ai.maxScenes` | Maximum scenes (patches) per task | `4` |
| `but-ai.rehearsalThreshold` | Minimum rehearsal marks for memory to be considered reliable | `3` |
| `but-ai.deadAirTimeout` | Seconds without cue before agents signal standby | `30` |
| `but-ai.cueFormat` | Structured cue format (numbered or named) | `numbered` |

**WASI Degradation:**
Under WASI, PATH discovery is unavailable. Degradation:
- Memory operations (script/cue store) work fully — Git-native, no filesystem dependencies.
- Rehearsal mode works fully — read-only, plans without executing.
- Performance mode works but without cue-based interruption (no background threads for Stage monitoring). Cues are processed sequentially.
- The "venue detection" feature (auto-detecting codebase character) is reduced — some filesystem heuristics are unavailable.
- WASI component interface for direct loading by WASI hosts.

### Trade-offs

**Considered:** Implementing the cue system as an event bus (pub/sub). **Rejected:** event buses add infrastructure dependency. The cue system is implemented as a state machine where Stage advances the cue pointer and agents respond. No external message broker required.

**Considered:** Allowing agents to call cues themselves (peer-to-peer coordination). **Rejected:** in theater, cues are called by one authority (the stage manager) for a reason — distributed cue-calling causes chaos. Stage is the single coordinator. Agents can request cues but cannot call them directly.

---

## 2. Provider-Agnostic AI Interface (RFP Section 3.2)

### Approach

`but-llm` is the sole backend. Moth & Flame wraps it with a cue-aware layer that aligns LLM interactions with the production's cue sheet.

### Design

**Cue-Aware Provider Wrapper:**
```rust
pub struct ProductionProvider {
    inner: LLMProvider,
    cue_sheet: CueSheet,
    current_cue: CueId,
    budget: ShowBudget,
}

impl ProductionProvider {
    /// Tool-calling loop synchronized with the cue sheet.
    /// Between each tool call, Stage advances the cue pointer.
    /// The system prompt is updated with cue-relevant context.
    pub fn cued_tool_loop(
        &self,
        system_message: &str,
        messages: Vec<ChatMessage>,
        tools: &mut impl Toolset,
        model: &str,
        cue_sheet: &CueSheet,
    ) -> anyhow::Result<SceneResult>;
}

pub struct SceneResult {
    pub output: String,
    pub cues_executed: Vec<CueId>,
    pub scene_complete: bool,
    pub blocking_notes: Vec<BlockingNote>,  // Spatial relationships discovered
}
```

The key innovation: the cue sheet provides structure to the tool-calling loop. Instead of a flat sequence of tool calls, the loop follows a dramatic structure — setup, action, resolution — with cues marking transitions between phases.

**New Provider Mechanism:**
```rust
pub trait CastMember: Send + Sync {
    /// Each provider is a "cast member" with a particular dramatic range.
    fn name(&self) -> &str;
    fn dramatic_range(&self) -> DramaticRange;  // What kinds of tasks this provider handles well
    fn create_backend(&self, config: &ProviderConfig) -> Result<Box<dyn LLMBackend>>;
}
```

Loaded from `but-ai.providerPluginDir`. The `dramatic_range` metadata (complex reasoning, fast response, creative output) supports future provider routing.

### Trade-offs

**Considered:** Different providers for different "acts" of the production (cheap model for survey, expensive for patch). **Deferred:** same reasoning as other proposals. Single-model budget for the RFP; multi-model routing as future work.

**Considered:** Streaming for all operations. **Adopted selectively:** streaming via `tool_calling_loop_stream` for patch generation (so progress is visible), non-streaming for cue execution and memory operations (where latency matters more than visibility).

---

## 3. The But Agent (RFP Section 3.3)

### Approach

The But Agent is a theatrical production. Stage calls cues. Lead performs (writes patches). Understudy splits scenes. Dramaturg provides context. House presents the result. Every operation is cue-driven, every output is a scene, and every scene is independently reviewable.

### Design

**Task Lifecycle (The Production):**
```
ACT I: REHEARSAL
  CUE 1: Dramaturg retrieves relevant memories
  CUE 2: Stage reads task description and workspace state
  CUE 3: Stage creates cue sheet (production plan)

ACT II: PERFORMANCE
  CUE 4: Lead enters on work branch
  CUE 5: Lead reads target code (scene preparation)
  CUE 6: Lead produces INDEX.patch (the performance)
  CUE 7: Lead produces COMMIT.msg (the curtain speech)
  [If scene split needed:]
    CUE 6a: Understudy decomposes into scenes
    CUE 6b-6n: Lead performs each scene separately

ACT III: HOUSE
  CUE 8: House creates PR (opens the house)
  CUE 9: House posts coordination comments (program notes)
  CUE 10: House manages cross-repo references (touring schedule)
  CUE 11: Dramaturg archives session as memory (strike and store)
```

**The Cue Sheet:**
```json
{
  "production": "task_042_auth_refactor",
  "acts": [
    {
      "name": "rehearsal",
      "cues": [
        {"id": "CUE-1", "agent": "dramaturg", "action": "retrieve_memory", "params": {"query": "auth middleware"}},
        {"id": "CUE-2", "agent": "stage", "action": "read_workspace", "params": {}},
        {"id": "CUE-3", "agent": "stage", "action": "create_cue_sheet", "params": {}}
      ]
    },
    {
      "name": "performance",
      "cues": [
        {"id": "CUE-4", "agent": "lead", "action": "enter_branch", "params": {"branch": "scene/s01.s03/lead/auth-refactor"}},
        {"id": "CUE-5", "agent": "lead", "action": "read_code", "params": {"files": ["src/middleware/auth.rs"]}},
        {"id": "CUE-6", "agent": "lead", "action": "produce_patch", "params": {}},
        {"id": "CUE-7", "agent": "lead", "action": "produce_commit_msg", "params": {}}
      ]
    },
    {
      "name": "house",
      "cues": [
        {"id": "CUE-8", "agent": "house", "action": "create_pr", "params": {}},
        {"id": "CUE-9", "agent": "house", "action": "post_coordination", "params": {}},
        {"id": "CUE-10", "agent": "house", "action": "cross_repo_refs", "params": {}},
        {"id": "CUE-11", "agent": "dramaturg", "action": "archive_session", "params": {}}
      ]
    }
  ]
}
```

**Branch Naming:**
```
scene/s01.s03/lead/auth-refactor
│     │       │     └── Scene description
│     │       └── Actor (agent) name
│     └── Dependency chain (act structure)
└── Namespace (production prefix)
```

**Token Budget Enforcement ("Running Time"):**
```rust
pub struct ShowBudget {
    total_running_time: u32,   // Total token budget
    current_time: u32,         // Tokens consumed
    intermission_reserve: u32, // Reserved for Act III (House operations): 8000 tokens
    curtain_reserve: u32,      // Reserved for final COMMIT.msg + memory archival: 2000 tokens
}
```

When the show approaches its total running time:
1. Stage calls "places for curtain" — signals Lead to wrap up the current scene.
2. Lead produces whatever patch it has (even partial).
3. COMMIT.msg documents the incomplete performance.
4. Act III proceeds normally (House still creates PR, coordination comments).

The intermission reserve ensures that coordination is never skipped. A show without a curtain call is unprofessional.

**WorkspaceToolset Exposure:**
Tools are loaded per act:

| Act | Tools Loaded | Rationale |
|-----|-------------|-----------|
| I: Rehearsal | GetProjectStatus, GetBranchChanges, GetCommitDetails | Read-only, context gathering |
| II: Performance | Commit, CreateBranch, Amend, SplitBranch, SplitCommit | Write and decomposition tools |
| III: House | MoveFileChanges, SquashCommits | Presentation and cleanup tools |

Per-act loading saves ~1,000 tokens compared to loading all 10 tools simultaneously.

### Trade-offs

**Considered:** Allowing Lead to self-direct without cues. **Rejected:** self-directed agents go off-script. The cue system provides guardrails that prevent scope creep (Lead only modifies what's in the cue sheet) without preventing creativity (Lead decides how to implement each cue).

**Considered:** Allowing multiple simultaneous performances (parallel agents). **Adopted with constraint:** multiple Lead agents can perform simultaneously in different scenes, but only if the scenes are in different parts of the codebase (non-overlapping blocking). Stage verifies non-overlap before calling parallel cues.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP Section 3.4)

### Approach

PRs are scenes in a multi-venue production. Cross-repo coordination is a touring schedule. PR comments follow a theatrical protocol that encodes the scene's relationship to the larger production.

### Design

**PR Comment Schema (Stage Protocol):**
```json
{
  "protocol": "stage/v1",
  "type": "cue | scene_report | dependency | blocking_note | budget_runtime",
  "agent": "house@moth-and-flame",
  "production": "task_042_auth_refactor",
  "act": "II",
  "timestamp": "2026-03-28T14:30:00Z",
  "payload": { }
}
```

The `production` field ties all comments across all repos to a single production. The `act` field indicates where in the production lifecycle this comment was generated. This allows an observer to reconstruct the full production from its distributed scenes.

**Forge Adapter Interface:**
```rust
pub trait VenueAdapter: Send + Sync {
    fn create_scene(&self, venue: &VenueRef, scene: &Scene) -> Result<SceneId>;
    fn post_program_note(&self, venue: &VenueRef, scene: SceneId, note: &str) -> Result<NoteId>;
    fn read_program_notes(&self, venue: &VenueRef, scene: SceneId) -> Result<Vec<ProgramNote>>;
    fn set_genre(&self, venue: &VenueRef, scene: SceneId, genre: &str) -> Result<()>;
    fn get_scene(&self, venue: &VenueRef, scene: SceneId) -> Result<SceneInfo>;
    fn search_scenes(&self, venue: &VenueRef, query: &str) -> Result<Vec<SceneSummary>>;
}
```

(`create_scene` = create PR, `post_program_note` = post comment, `set_genre` = add label. The theatrical naming makes the mapping explicit.)

Reference implementation: GitHub REST.

**Cross-Repo as Touring:**
```json
{
  "type": "dependency",
  "payload": {
    "touring_from": "github.com/org/auth-service#87",
    "relation": "this scene requires that scene to complete first",
    "blocking_note": "The auth token format established in auth-service#87 determines the parser we build here",
    "tour_schedule": {
      "expected_completion": "2026-03-29T00:00:00Z",
      "fallback": "If auth-service#87 is delayed, perform with mock token format"
    }
  }
}
```

The `blocking_note` is unique to Moth & Flame — it explains the spatial relationship between the two scenes, not just the temporal dependency. Why does scene B depend on scene A? What is the nature of the connection? This information is absent from most dependency declarations and invaluable for understanding cross-repo coordination failures.

**Structured Message Types:**

| Type | Purpose | Key Fields |
|------|---------|------------|
| `cue` | Trigger action in another agent/repo | `agent`, `action`, `cue_id` |
| `scene_report` | Report scene (task) completion/status | `status`, `act`, `scenes_completed` |
| `dependency` | Declare cross-venue dependency | `touring_from`, `blocking_note` |
| `blocking_note` | Describe spatial relationship between scenes | `scene_a`, `scene_b`, `relationship` |
| `budget_runtime` | Report show running time | `total`, `elapsed`, `acts_remaining` |

### Trade-offs

**Considered:** Using a central "program" document that lists all scenes across all repos. **Rejected:** central documents go stale. The program is assembled dynamically from distributed scene reports, the same way an audience assembles the narrative of an immersive production from the fragments they witness.

**Considered:** Using Git notes for cross-repo coordination. **Rejected:** Git notes are poorly supported across forges. PR comments are universally available and render well on all forge UIs.

---

## 5. Agent Memory and Identity (RFP Section 3.5)

### Approach: Script/Cue Memory

Memories are stored as a theatrical script with three types of annotations: blocking notes (spatial relationships), cue sheets (retrieval triggers), and rehearsal marks (reliability through repetition). A memory is not a fact — it is a line in a script that has been rehearsed, blocked, and cued.

### Design

**Storage Medium:**
Memories are stored in Git refs under `refs/script/<agent-id>/`:
```
refs/script/lead/lines/mem_001.json           — Individual memory "lines"
refs/script/lead/blocking/spatial_map.json     — Blocking notes (spatial relationships)
refs/script/lead/cues/trigger_sheet.json       — Cue triggers for memory retrieval
refs/script/lead/rehearsal/marks.json          — Rehearsal marks per memory
refs/script/lead/identity.json                 — Agent identity
refs/script/shared/production_bible.json       — Cross-session shared memory
```

Each memory line:
```json
{
  "line_id": "mem_001",
  "content": "The auth middleware validates JWT tokens with RS256, 15-minute expiry",
  "blocking": {
    "position": "src/middleware/auth.rs",
    "adjacent_to": ["mem_003_session_management", "mem_007_token_refresh"],
    "scene": "authentication"
  },
  "cues": [
    {
      "trigger": "agent encounters auth-related file modification",
      "priority": "primary"
    },
    {
      "trigger": "agent encounters JWT or token in task description",
      "priority": "secondary"
    }
  ],
  "rehearsal_marks": 7,
  "last_rehearsed": "2026-03-28T14:00:00Z",
  "created_during": "task_012",
  "ttl": "30d",
  "confidence": 0.92
}
```

**Blocking Notes (Spatial Relationships):**
Blocking notes describe where memories are in relation to each other and in relation to the codebase:
```json
{
  "spatial_map": [
    {
      "scene": "authentication",
      "memories": ["mem_001", "mem_003", "mem_007"],
      "codebase_region": "src/middleware/",
      "spatial_notes": "These memories form a tight cluster — changes to any require checking all three"
    },
    {
      "scene": "database",
      "memories": ["mem_012", "mem_015"],
      "codebase_region": "src/db/",
      "spatial_notes": "Isolated scene — rarely interacts with authentication"
    }
  ],
  "cross_scene_links": [
    {
      "from_scene": "authentication",
      "to_scene": "database",
      "nature": "Session tokens are stored in the database — changes to auth format affect DB schema",
      "strength": "weak"
    }
  ]
}
```

**Cue Sheets (Retrieval Triggers):**
Cues define when a memory should be automatically retrieved:
```json
{
  "triggers": [
    {
      "condition": "task_description CONTAINS 'auth' OR 'token' OR 'jwt'",
      "retrieve": ["mem_001", "mem_003", "mem_007"],
      "priority": "primary"
    },
    {
      "condition": "files_modified INCLUDES 'src/middleware/*'",
      "retrieve": ["mem_001"],
      "priority": "primary"
    },
    {
      "condition": "branch_name MATCHES 'scene/*/lead/auth-*'",
      "retrieve": ["mem_001", "mem_003"],
      "priority": "secondary"
    }
  ]
}
```

This is proactive retrieval — memories surface before the agent asks for them, based on contextual triggers. Reactive retrieval (explicit queries) is also supported, but the cue-based system ensures that critical context is never forgotten.

**Rehearsal Marks (Reliability Signals):**
A memory's rehearsal mark count indicates how many times it has been retrieved and confirmed useful by the consuming agent:
- 0 marks: Unrehearsed. New memory, untested. Low confidence.
- 1-2 marks: Read-through. The memory has been seen but not stressed.
- 3-5 marks: Rehearsed. The memory has been used in multiple contexts and confirmed.
- 6+ marks: Performance-ready. The memory is deeply reliable.

Only the consuming agent can increment a rehearsal mark (via a "memory confirmed" signal). Retrieval without confirmation does not count as rehearsal — merely reading a script is not rehearsing it.

**Relevance Scoring:**
1. **Cue match** (0.4 weight): Does the current context match any of the memory's cue triggers?
2. **Blocking proximity** (0.3 weight): Is the memory in the same "scene" (codebase region) as the current task?
3. **Rehearsal depth** (0.2 weight): How many times has this memory been rehearsed?
4. **Freshness** (0.1 weight): When was the memory last rehearsed?

**Expiration:**
Memories expire based on TTL. Expired memories are archived to `refs/script/<agent-id>/archive/` with all annotations preserved. Archived memories can be retrieved for historical context ("how did we handle this last season?").

**Compaction Survival:**
When the context window compacts:
1. All memories with rehearsal marks >= `rehearsalThreshold` (default: 3) are preserved in full.
2. Memories with 1-2 marks are summarized (content only, blocking and cues stripped).
3. Unrehearsed memories are listed by ID only (available on demand).
4. The post-compaction system prompt includes a "promptbook" — a compressed summary of the production's memory state (~1,500 tokens).

**Long-Term Storage (Production Bible):**
The shared production bible (`refs/script/shared/production_bible.json`) accumulates cross-session knowledge:
```json
{
  "recurring_scenes": [
    {
      "scene": "authentication",
      "established_memories": ["mem_001", "mem_003", "mem_007"],
      "total_rehearsals": 23,
      "notes": "Well-rehearsed scene. Any change should be approached with full blocking review."
    }
  ],
  "company_vocabulary": {
    "auth": "JWT-based token validation in src/middleware/auth.rs",
    "session": "Server-side session management in src/session/"
  }
}
```

**Identity:**
```json
{
  "name": "lead",
  "organization": "moth-and-flame-theater",
  "role": "principal_actor_patch_author",
  "capabilities": ["patch_generation", "style_matching", "scene_performance"],
  "authorization_scope": {
    "branches": ["scene/*", "feat/*", "fix/*"],
    "max_patch_lines": 500,
    "max_scenes_per_task": 4
  },
  "openwallet_key_ref": "did:web:mothandflame.theater/agents/lead",
  "created": "2026-03-28T00:00:00Z"
}
```

### Trade-offs

**Considered:** Flat memory without blocking or cues. **Rejected:** flat memory retrieval is keyword matching. Cue-based retrieval is context-triggered. The difference is the difference between searching a database and having a stage manager hand you the relevant script page at the right moment.

**Considered:** Automatic rehearsal marking (any retrieval increments the mark). **Rejected:** automatic marking inflates reliability. A memory retrieved but not used is a line read but not rehearsed. Only confirmed usage counts.

---

## 6. Signed Commits via OpenWallet (RFP Section 3.6)

### Approach

Every agent commit is signed via OpenWallet. The signature encodes the production context: which cue produced the commit, which scene it belongs to, and which act of the production it falls within.

### Design

**Signing Flow:**
```
1. Lead produces INDEX.patch + COMMIT.msg (CUE 6-7)
2. House prepares commit object (CUE 8)
3. Commit is signed with agent's OpenWallet key
4. Signature claims:
   - Agent identity (name, role, company)
   - Production reference (task ID, act, scene)
   - Cue reference (which cue authorized this commit)
   - Authorization scope (branches, max lines, max scenes)
   - Show runtime (tokens used / total budget)
5. Signed commit is pushed
```

**Authorization Model:**
Scene-based authorization:
- Agents are authorized per-scene (codebase region). Lead can commit to `scene/*` but not directly to `main`.
- Scene boundaries are defined in the blocking notes — Lead can only modify files within the scenes listed in the cue sheet.
- Cross-scene commits (modifying files in two different scenes) require explicit authorization from Stage (an additional cue).

```json
{
  "authorization_rules": [
    { "branches": "scene/*", "scenes": ["authentication", "database", "api"], "max_lines": 500 },
    { "branches": "feat/*", "scenes": ["*"], "max_lines": 300 },
    { "branches": "fix/*", "scenes": ["*"], "max_lines": 200 }
  ]
}
```

**Key Lifecycle:**
- **Provisioning:** Per-agent keys via OpenWallet. Key is tied to the agent's "cast membership" — membership in the company.
- **Rotation:** Seasonal rotation (90 days, matching a theatrical "season"). New keys are announced in the production bible.
- **Revocation (departure):** When an agent leaves the company, their key is retired. Historical commits (past performances) remain verifiable.
- **Revocation (compromise):** Key added to OpenWallet revocation list. Commits signed with the compromised key are flagged. The production bible is updated with a note about which scenes/productions may be affected.

### Trade-offs

**Considered:** Per-scene signing (each scene gets its own key). **Rejected:** key proliferation. One key per agent is sufficient. The scene context is encoded in the credential claims, not the key identity.

---

## 7. Token Budget (RFP Section 3.7)

### Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,000 | 0 | Once per session | Agent identity (300), act-gated tools (1,200-1,800), promptbook (1,000), venue description (500) |
| **Act I: Rehearsal** | 3,000 | 1,000 | Once per task | Memory retrieval (1,500 in), workspace state (1,500 in). Cue sheet output (1,000 out). |
| **Act II: Performance** | | | | |
| — Scene preparation | 2,000 | 300 | Once per scene | Target code reading (2,000 in). Scene plan (300 out). |
| — Patch generation | 3,500 | 4,500 | Once per scene | Context (3,500 in). Patch (4,000 out) + COMMIT.msg (500 out). |
| — Tool calls | 500 | 150 | ~6 per task | Parameters (150 out), result (500 in). |
| **Act III: House** | 1,200 | 1,500 | Once per task | PR template (200 in), coordination context (1,000 in). PR body (800 out), coordination comments (700 out). |
| **Memory archival** | 300 | 600 | Once per task | Session summary (300 in). Memory lines, blocking notes, cue triggers (600 out). |
| **Cue overhead** | 800 | 200 | ~11 cues per task | Stage reads workspace state between cues (800 in total). Cue calls (200 out total). |
| **TOTAL (typical task)** | **19,500** | **10,050** | -- | 200-line, 3-file feature with 2 cross-repo dependencies |

**Grand total: ~29,550 tokens per typical task.**

### Budget by Act

| Act | Input | Output | % of Total |
|-----|-------|--------|-----------|
| I: Rehearsal | 3,000 | 1,000 | 14% |
| II: Performance | 9,000 | 5,850 | 50% |
| III: House | 1,200 | 1,500 | 9% |
| Cross-cutting (system prompt, cues, memory) | 6,300 | 1,700 | 27% |

Act II (the actual performance) consumes exactly half the budget. The rest is production overhead — necessary for coordination, memory, and presentation. The 27% cross-cutting cost includes the system prompt (amortized across the session), cue overhead, and memory operations.

### Budget Justification

The 29,550-token total is realistic for a 200-line feature. The largest single cost is patch generation (8,000 tokens for code context + patch output), consistent with frontier model benchmarks.

The rehearsal phase (4,000 tokens) is the "cost of preparation." Moth & Flame's philosophy — inhabit the space before performing in it — means that Act I is never skipped. The preparation cost pays for itself by preventing mid-performance surprises that would cost more tokens to handle.

The House overhead (2,700 tokens) is significant but justified: well-formatted PRs with coordination context reduce human review time, and cross-repo coordination comments prevent dependency deadlocks. A show without front-of-house management is a show without an audience.

---

## 8. Testing Strategy

### Provider-Agnostic Testing
- **Mock cast member** (mock provider) with configurable responses per act. Act I returns fixed memory results. Act II returns fixed patches.
- **Cue sequencing tests:** Verify that cues are executed in order, that dependent cues wait for predecessors, and that parallel cues in non-overlapping scenes execute simultaneously.

### Patch Workflow Validation
- **Scene round-trip tests:** Generate cue sheet → execute cues → produce patches → apply patches → verify workspace state.
- **Scene split tests:** Task requires >1 scene. Verify that Understudy correctly decomposes the task and each scene's patch applies independently.
- **Curtain-at-budget tests:** Trigger budget exhaustion mid-Act-II. Verify that the partial patch is valid and Act III still executes.

### Cross-Repo Coordination Testing
- **Mock venue adapter** with stage-protocol comment validation.
- **Touring tests:** Simulate a production spanning two repos. Verify that blocking notes correctly describe the spatial relationship and that dependency resolution works when the upstream scene completes.

### Token Budget Testing
- **Per-act budget tests:** Verify that act budgets are respected and that the intermission reserve is not consumed by Act II.
- **Running time accuracy tests:** Compare reported "show runtime" against actual tokens consumed. Must match within 5%.

---

## 9. Migration Path

The migration is staged as a three-act production:

1. **Act I (Read-through):** `but ai mcp` serves alongside the existing server. `gitbutler_update_branches` is preserved and internally routed through the cue system (the old tool is a one-cue production). No change for existing clients.
2. **Act II (Rehearsal):** Existing clients migrate to `perform_task`. The transition is rehearsed in staging environments before going live.
3. **Act III (Opening Night):** Old MCP server decommissioned. `but-ai` is the sole MCP server. The production bible inherits any useful memories from the transition period.

The curtain rises when the audience is ready. Not before.

---

*Submitted by Moth & Flame Theater. We don't perform for the audience. We perform with them.*
