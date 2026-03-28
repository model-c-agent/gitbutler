# Proposal: `but-ai` Plugin -- The Textile Morphology Lab

**Submitted by:** The Textile Morphology Lab (Org 083)
**Domain:** Fashion Design -- Computational Material Science
**Date:** 2026-03-28

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Plugin Architecture (RFP 3.1)](#2-plugin-architecture-rfp-31)
3. [Provider-Agnostic AI Interface (RFP 3.2)](#3-provider-agnostic-ai-interface-rfp-32)
4. [The But Agent (RFP 3.3)](#4-the-but-agent-rfp-33)
5. [Polyrepo PR-Based Agent Coordination (RFP 3.4)](#5-polyrepo-pr-based-agent-coordination-rfp-34)
6. [Agent Memory and Identity (RFP 3.5)](#6-agent-memory-and-identity-rfp-35)
7. [Signed Commits via OpenWallet (RFP 3.6)](#7-signed-commits-via-openwallet-rfp-36)
8. [Token Budget (RFP 3.7)](#8-token-budget-rfp-37)
9. [Testing Strategy](#9-testing-strategy)
10. [Trade-offs and Alternatives](#10-trade-offs-and-alternatives)
11. [Configuration Reference](#11-configuration-reference)

---

## 1. Executive Summary

The Textile Morphology Lab proposes a `but-ai` plugin built on the structural principle that **arrangement determines behavior**. Just as the interlacement pattern of warp and weft threads determines a fabric's mechanical properties, the interlacement pattern of persistent memory (warp) and task-specific context (weft) determines an agent's cognitive properties.

Our central innovation is **woven memory** -- a memory scheme where long-term persistent context forms the warp (fixed, pre-tensioned, structural) and task-specific context forms the weft (dynamic, task-by-task, layered). The weave pattern -- plain, twill, or satin -- determines how warp and weft interact, and the operator (Osei, our heddle controller) adjusts the pattern in real time based on task demands.

The plugin is implemented in Rust as `crates/but-ai`, using the existing `but-llm` and `but-tools` crates without modification.

---

## 2. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a PATH-discovered Rust executable functioning in CLI mode (`but ai <subcommand>`) and MCP server mode (`but ai mcp`).

### Design

**Binary structure:**

```
but-ai
  ├── but ai weave <task>       -- Execute a task (primary command)
  ├── but ai inspect             -- Inspect memory fabric state
  ├── but ai mcp                 -- Start MCP server on stdio
  ├── but ai agent --task <desc> -- Autonomous agent mode
  └── but ai loom --status       -- Show loom configuration (warp, weft, pattern)
```

The `weave` command is our primary verb. Every task execution is a weaving operation: the agent takes the warp (persistent context), runs the weft (task-specific work) through it, and produces a fabric (output patches and updated memory).

**Crate structure:**

```
crates/but-ai/
  src/
    lib.rs           -- Core library
    loom/            -- Memory weaving engine
      warp.rs        -- Long-term context management
      weft.rs        -- Task-specific context management
      pattern.rs     -- Weave pattern selection and application
      heddle.rs      -- Dynamic pattern adjustment (Osei's logic)
    shuttle/         -- Cross-repo coordination
    selvedge/        -- Validation and edge integrity
    fabric/          -- Output production (INDEX.patch + COMMIT.msg)
  bin/
    main.rs          -- Binary entry point
```

**Environment variables:**

The plugin reads `BUT_WORKSPACE_DIR` (maps to "loom workspace"), `BUT_OUTPUT_FORMAT` (maps to output weave pattern), and `BUT_JSON` (activates structured output).

**WASI degradation:**

Under WASI, the plugin operates as an "off-loom" weaver -- it can produce patches from pre-loaded context but cannot coordinate with external looms (no forge API, no cross-repo). The warp is loaded from a static configuration rather than queried dynamically. This produces a stiffer, less adaptive fabric, but it is still a fabric. The loom metaphor holds: an off-loom weaving technique (tablet weaving, frame weaving) produces narrower, simpler fabrics but uses the same fundamental principles.

**MCP compatibility:**

The MCP server implements `ServerHandler` via `rmcp`, maintaining backward compatibility with the existing `gitbutler_update_branches` tool while exposing the expanded toolset. The server name becomes `"GitButler AI Loom"`, version `"2.0.0"`, with capabilities including tools, memory inspection, and loom status.

### Trade-offs

**Alternative considered: Separate binary and library packages.** We chose a single crate with both library and binary targets. The loom metaphor maps naturally to a unified structure where the binary is the frame that holds the loom, and the library is the weaving mechanism inside it.

**Alternative considered: Plugin-based command discovery within but-ai.** Rejected as over-engineering. The five top-level commands (`weave`, `inspect`, `mcp`, `agent`, `loom`) are sufficient for the foreseeable command surface.

---

## 3. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

We use `but-llm` exclusively. The four existing providers are sufficient for production use. For extensibility, we define a **spool adapter** -- a configuration-driven mechanism for registering new providers without recompilation.

### Design

**Spool adapter interface:**

In textile manufacturing, a spool holds thread ready for the loom. A spool adapter holds a provider ready for the AI system.

```rust
pub trait SpoolAdapter: Send + Sync {
    fn provider_name(&self) -> &str;
    fn supports_tools(&self) -> bool;
    fn supports_streaming(&self) -> bool;
    fn max_context_length(&self) -> usize;
    fn to_llm_config(&self, git_config: &gix::config::File) -> Result<LLMProviderConfig>;
}
```

New providers implement `SpoolAdapter` and are registered via Git config:

```ini
[but-ai "spool.gemini"]
    adapter-path = "/path/to/gemini-spool"
    thread-weight = "heavy"   # indicates token-dense responses
```

**Tool registration:**

All 10 `WorkspaceToolset` tools are registered through the `Toolset` trait. The tools are presented to the LLM with descriptions that follow our thread metaphor:

- `Commit` -> "Bind the current weft pass into the fabric"
- `CreateBranch` -> "Set up a new warp for a separate fabric"
- `GetProjectStatus` -> "Inspect the current state of the loom"

These descriptions are cosmetic for the LLM's benefit. The underlying tool behavior is identical to the standard `WorkspaceToolset` implementation.

**Provider capability adaptation:**

Providers that support tool calling use `tool_calling_loop`. Providers that support only text use `structured_output` with a JSON schema describing the desired tool calls. Providers that support neither fall back to `response` with explicit tool-calling instructions embedded in the system prompt. Each fallback level produces a coarser fabric -- less precise, more token-expensive -- but still functional.

### Trade-offs

**Alternative considered: Custom LLM client optimized for textile terminology.** Rejected per RFP (disqualifying). We use `but-llm` as-is.

**Alternative considered: Compile-time provider registry.** Rejected because adding a new provider should not require recompiling the plugin. The spool adapter approach allows runtime registration.

---

## 4. The But Agent (RFP 3.3)

### Approach

The agent operates as a loom with five stations (see AGENTS.md): Tanaka sets the warp, Marchetti runs the weft, Osei controls the heddle, Lindqvist inspects the selvedge, and Nakamura carries the shuttle. Every task execution is a weaving operation that produces INDEX.patch + COMMIT.msg as the finished fabric.

### Design

**Agent lifecycle (one weaving cycle):**

```
1. WARP:     Tanaka establishes foundational context and architectural constraints
2. THREAD:   Osei selects the weave pattern and lifts the appropriate warp threads
3. SHUTTLE:  Marchetti runs the weft (generates code changes) through the warp
4. BEAT:     Marchetti compacts the weft against the existing fabric (refines changes)
5. INSPECT:  Lindqvist checks the selvedge (validates boundaries and consistency)
6. CUT:      Final INDEX.patch + COMMIT.msg produced; fabric is cut from the loom
```

The weaving metaphor maps precisely to the software development cycle. Each "pick" (pass of the shuttle) adds one layer of change. The "beat" (pressing the weft against the warp with the reed) corresponds to refining the raw change against the architectural constraints. The "selvedge check" ensures the edges of the fabric (module boundaries, API contracts) are intact.

**Patch production:**

Marchetti produces unified diffs against the current index. The diff format is standard `git diff` output, ensuring compatibility with `git apply`. The commit message includes loom metadata:

```
feat(auth): add middleware validation for session tokens

Thread-count: 3W/5F (3 warp concerns, 5 weft operations)
Weave-pattern: twill
Inspected-by: lindqvist
```

**Branch naming:**

```
weave/<agent-id>/<pattern>/<task-id>[.<dep-id>]
```

Example: `weave/marchetti/twill/t015.t012` -- Marchetti, twill pattern task, task 15, depends on task 12.

The pattern in the branch name (`plain`, `twill`, `satin`) indicates the complexity level of the task, which helps other agents estimate the cognitive load of reviewing the work.

**Token budget enforcement:**

At 80% budget consumption, Osei switches the weave pattern from twill (complex, detailed) to plain (simple, dense). At 90%, Osei switches to a "tabby" pattern -- the simplest possible weave, one-over-one-under, producing the minimum viable output. At 95%, the loom halts and whatever fabric has been completed is cut and submitted as a partial INDEX.patch with explicit annotations about the unfinished portion.

**Progress reporting:**

```json
{
  "phase": "SHUTTLE",
  "agent": "marchetti",
  "loom_position": "weft",
  "pick_number": 3,
  "picks_estimated": 5,
  "tokens_used": 18000,
  "tokens_budget": 50000,
  "weave_pattern": "twill",
  "fabric_integrity": 0.92
}
```

### Trade-offs

**Alternative considered: Single-pass generation (no iterative refinement).** Rejected because single-pass patches have higher defect rates. The multi-pass weaving approach (rough pass, refinement pass, edge-case pass) mirrors our experience with fabric prototyping: the first sample is never the final product.

**Alternative considered: Dynamic agent count (spawn more agents as needed).** Rejected because adding looms mid-production introduces coordination overhead that exceeds the benefit for tasks at this scale. Our five agents are calibrated for the expected task complexity.

---

## 5. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is modeled as **multi-loom production** -- the textile manufacturing process where multiple looms produce fabrics that must be cut and assembled into a single garment. Each repository is a loom. The shuttle (Nakamura) carries coordination threads between looms.

### Design

**Forge adapter (Loom-to-Loom Interface):**

```rust
pub trait LoomBridge: Send + Sync {
    fn send_shuttle(&self, target: &RepoRef, message: &ShuttleMessage) -> Result<MessageId>;
    fn receive_shuttles(&self, since: DateTime<Utc>) -> Result<Vec<ShuttleMessage>>;
    fn track_fabric(&self, pr: &PrId) -> Result<FabricStatus>;
    fn attach_pattern(&self, pr: &PrId, pattern: &CoordinationPattern) -> Result<()>;
    fn list_connected_looms(&self, pr: &PrId) -> Result<Vec<RepoRef>>;
}
```

The `LoomBridge` trait maps directly to forge operations: `send_shuttle` creates a PR comment, `receive_shuttles` reads PR comments, `track_fabric` checks PR status, etc. We provide a GitHub implementation and document the mapping for GitLab, Bitbucket, and Gitea.

**PR comment schema (Shuttle Message):**

```json
{
  "$schema": "but-ai/shuttle/v1",
  "type": "task | status | dependency | handoff | budget",
  "from_loom": { "agent": "nakamura", "org": "083-textile-morphology", "repo": "owner/repo" },
  "to_loom": { "agent": "target", "org": "target-org", "repo": "owner/other" },
  "thread": {
    "task_ref": "weave/marchetti/twill/t015",
    "status": "weaving",
    "dependencies": ["owner/other#42"],
    "budget_remaining": 32000,
    "pattern_complexity": "twill"
  },
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Embedded in PR comments as:

````markdown
```but-ai-shuttle
{ ... }
```
````

**Dependency tracking:**

Nakamura maintains a "cut plan" -- a dependency graph mapping which fabrics (PRs) from which looms (repos) must be assembled in which order. The cut plan is stored as a warp-level memory entry (high-priority, long TTL) and updated with each coordination event.

**Forge-agnosticism:**

The `LoomBridge` trait uses only operations available on all major forges: create comment, read comments, check status, add labels. The shuttle message schema is plain JSON in a code fence, requiring no forge-specific features beyond Markdown rendering.

### Trade-offs

**Alternative considered: Webhook-based coordination.** Rejected because webhooks require a running server, which violates the RFP's "no services beyond forge" requirement. Polling PR comments is less efficient but requires no infrastructure.

**Alternative considered: Git-notes-based coordination.** Rejected because Git notes are not reliably rendered by all forges, making them invisible to human reviewers who may need to audit coordination.

---

## 6. Agent Memory and Identity (RFP 3.5)

### Approach: Woven Memory

Our memory system models agent memory as a **woven fabric** with two thread systems:

- **Warp threads**: Long-term persistent context. These are the threads that are set in the loom before weaving begins. They represent architectural knowledge, coding conventions, team preferences, and cross-session learning. Warp threads are high-tension (strongly maintained) and run the full length of the agent's operational history.

- **Weft threads**: Task-specific context. These are the threads woven through the warp during each task. They represent the current task description, recent observations, intermediate results, and coordination context. Weft threads are added with each shuttle pass and have shorter lifespans.

The **weave pattern** determines how warp and weft interact. Osei, the heddle controller, selects the pattern:

- **Plain weave**: Every warp thread interacts with every weft thread. Dense, balanced memory retrieval. Used for unfamiliar tasks where all persistent context might be relevant.
- **Twill weave**: Weft threads skip some warp threads, creating diagonal lines. Lighter, faster retrieval. Used for familiar tasks where only a subset of persistent context is relevant.
- **Satin weave**: Long "floats" where the weft rides over many warp threads. Surface-level retrieval only. Used for quick, routine tasks where deep context is unnecessary.

### Design

**Storage:**

Memory is stored on a special Git branch structured as a loom:

```
refs/but-ai/loom/<agent-id>/
  warp/
    arch-001.json       -- "Module X uses middleware pattern"
    conv-002.json       -- "Team uses snake_case"
    style-003.json      -- "Error messages include context object"
  weft/
    task-current/
      obs-001.json      -- "File auth.rs has 3 functions"
      plan-001.json     -- "Step 1: add validation, Step 2: update tests"
    task-archive/
      t014/             -- Completed task context (compressed)
      t013/             -- Completed task context (compressed)
  pattern/
    active.json         -- Current weave pattern configuration
    history.json        -- Pattern selection history (for learning)
  selvedge/
    integrity.json      -- Memory fabric integrity metrics
```

**Memory entry structure:**

```json
{
  "id": "arch-001",
  "thread_type": "warp",
  "tension": 0.85,
  "content": "The authentication module uses a middleware chain pattern",
  "created_at": "2026-03-15T10:00:00Z",
  "last_interlaced": "2026-03-28T09:00:00Z",
  "interlacement_count": 14,
  "ttl_seconds": 2592000,
  "color": "structural",
  "position_in_warp": 3,
  "connected_weft_threads": ["obs-001", "plan-001"],
  "embedding_vector": [0.12, -0.34, ...],
  "source_commit": "abc123def"
}
```

The `tension` field (0.0-1.0) represents the thread's structural importance. High-tension warp threads are critical to the fabric's integrity. Low-tension threads can be removed without structural damage. Tension is calculated from access frequency, dependency count, and age.

**Relevance scoring:**

```
score = 0.35 * embedding_similarity(query, thread)
      + 0.25 * tension(thread)
      + 0.20 * interlacement_recency(thread)
      + 0.10 * pattern_match(current_pattern, thread.position)
      + 0.10 * color_match(task_type, thread.color)
```

The `pattern_match` component is unique to woven memory: it weights threads that are "up" (active) in the current weave pattern more heavily than threads that are "down" (inactive). This means the same query returns different results depending on the active weave pattern, which is the desired behavior -- a twill-pattern task should retrieve different context than a plain-pattern task.

**Expiration:**

Warp threads have long TTLs (30 days default) and are renewed whenever they are interlaced with a weft thread. Weft threads expire at task completion -- the completed task's weft is compressed and archived in `task-archive/`. If a pattern of weft threads recurs across multiple tasks (Osei detects this by comparing interlacement patterns), the recurring pattern is promoted to a warp thread -- it has become persistent context.

**Compaction survival:**

During compaction, the fabric is "fulled" -- a textile finishing process that shrinks the fabric, making it denser and more durable. Warp threads survive fulling; they are the structural backbone. Weft threads are compressed: their content is summarized, but their interlacement metadata (which warp threads they connected to) is preserved. This allows the agent to reconstruct the pattern of the lost weft from the surviving warp, even if the weft content itself is gone.

**Long-term storage:**

The `task-archive/` directory serves as the long-term store. Archived tasks retain their compressed weft and their interlacement pattern with the warp. Cross-repository woven memory is stored on a shared branch:

```
refs/but-ai/shared-loom/
  patterns/        -- Successful weave patterns (which warp-weft combinations worked)
  thread-library/  -- Reusable warp threads from other repos
```

**Identity:**

Agent identity is encoded as a **maker's mark** -- a signed JSON document that includes:

```json
{
  "agent_id": "marchetti",
  "org_id": "083-textile-morphology",
  "loom_position": "weft",
  "capabilities": ["patch_generation", "code_review", "refactoring"],
  "authorization": {
    "branch_patterns": ["weave/*", "feat/*"],
    "max_patch_lines": 800,
    "repos": ["owner/main-repo"]
  },
  "signing_key": "openwallet:083-textile:marchetti",
  "created_at": "2026-03-01T00:00:00Z"
}
```

### Trade-offs

**Alternative considered: Flat memory with tagging.** Rejected because tags do not capture the structural relationship between persistent and task-specific context. The warp/weft distinction is fundamental: it determines how memories interact, not just how they are categorized.

**Alternative considered: Hierarchical memory (tree structure).** Rejected because trees impose a single parent-child relationship. Woven memory allows any weft thread to interlace with any warp thread, creating a many-to-many relationship that better models how task-specific context relates to persistent knowledge.

---

## 7. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent has an OpenWallet signing key. Every INDEX.patch + COMMIT.msg pair is signed before submission. The signature proves provenance: this agent, with this maker's mark, authorized to weave on these branches, produced this fabric.

### Design

**Key hierarchy:**

```
Organization Key (083-textile-morphology)
  ├── Warp Key (tanaka)      -- Architectural authority
  ├── Weft Key (marchetti)   -- Patch production authority
  ├── Heddle Key (osei)      -- Memory management authority
  ├── Selvedge Key (lindqvist) -- Validation authority
  └── Shuttle Key (nakamura) -- Coordination authority
```

Key names encode loom positions, making it immediately clear what each key authorizes.

**Authorization model:**

Authorization is encoded in the maker's mark (identity document). Loom-position-specific constraints:

- **Warp agents** (Tanaka): May modify architectural files, configuration, and interface definitions. May not generate feature patches.
- **Weft agents** (Marchetti): May generate feature patches within their branch scope. May not modify architectural files without Warp approval.
- **Heddle agents** (Osei): May modify memory branches only. No codebase write access.
- **Selvedge agents** (Lindqvist): Read-only codebase access. May add validation annotations to PRs.
- **Shuttle agents** (Nakamura): May create PR comments and cross-repo references. No direct codebase write access.

This separation of concerns prevents any single compromised key from granting full access.

**Key lifecycle:**

| Event | Action | Textile Analogy |
|-------|--------|-----------------|
| Provisioning | Org key signs agent key | Threading a new bobbin |
| Rotation | New key issued, old key archived | Re-threading with fresh yarn |
| Compromise | Key revoked, all signed work quarantined | Cutting contaminated threads from the loom |
| Decommission | Key archived permanently | Removing a bobbin from service |

Rotation and revocation are signed operations stored on the memory branch. Compromised keys trigger a "fabric inspection" -- all commits signed by the compromised key are re-examined for integrity.

### Trade-offs

**Alternative considered: Single team key.** Rejected because loom-position-based keys provide meaningful access control. A weft agent should not be able to modify architecture, and the key structure enforces this.

---

## 8. Token Budget (RFP 3.7)

Estimates for Claude Opus on a typical task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Loom config, tool descriptions, active warp summary |
| **Task ingestion** | 1,800 | 400 | Once per task | Reading task description, inspecting target files |
| **Warp setting (Tanaka)** | 3,000 | 1,500 | Once per task | Architectural analysis, constraint identification |
| **Pattern selection (Osei)** | 1,200 | 600 | Once per task | Choosing weave pattern based on task type |
| **Shuttle pass (per pass)** | 1,200 | 1,800 | 3 per task | Marchetti's iterative code generation |
| **Selvedge check (Lindqvist)** | 1,500 | 800 | Once per task | Boundary validation, consistency check |
| **Commit message** | 400 | 300 | Once per task | Including loom metadata |
| **Memory retrieval (Osei)** | 1,000 | 400 | 2 per task | Warp query, interlacement lookup |
| **Memory storage** | 400 | 600 | 1 per task | Weft archival, warp tension update |
| **Coordination (Nakamura)** | 1,200 | 700 | 2 per task | Shuttle messages, dependency tracking |
| **TOTAL (typical task)** | **22,100** | **12,300** | -- | **34,400 total tokens** |

### Justification

The total of ~34,000 tokens is competitive because the weave pattern mechanism reduces wasted retrieval. In a plain-weave task (unfamiliar domain), all warp threads are activated, costing more retrieval tokens. In a twill-weave task (familiar domain), only relevant warp threads are activated, saving retrieval tokens. Most tasks in a mature project are twill or satin, so the average cost decreases over time as the agent's warp becomes better calibrated.

The system prompt stays under 3,500 tokens by storing the full warp inventory in memory and including only a summary in the prompt. The active weave pattern is described in approximately 200 tokens.

The three shuttle passes (Marchetti's iterative refinement) are the largest cost component. For simple tasks, the agent may complete in one or two passes, reducing total cost to ~28,000 tokens.

---

## 9. Testing Strategy

### Provider-agnostic testing

A `MockLoom` implements `LLMProvider` with deterministic thread outputs. Each test case defines a warp (persistent context), a weft (task input), and an expected fabric (output patch). The mock returns pre-defined tool call sequences, allowing full lifecycle testing without API calls.

### Patch workflow validation

Round-trip testing for INDEX.patch:

1. Set up a known codebase state (the "blank fabric")
2. Run the agent on a defined task (the "weaving operation")
3. Capture the produced INDEX.patch
4. Apply it to a fresh copy of the blank fabric
5. Verify the result matches the expected output

Edge cases tested: partial patches (budget exhaustion), multi-file patches, patches that modify existing code versus patches that add new files.

### Cross-repo coordination

A `MockForge` implements `LoomBridge` with an in-memory message store. Tests simulate multi-loom production:

1. Loom A (repo 1) produces a fabric that depends on Loom B (repo 2)
2. Shuttle message sent from A to B
3. B completes its fabric and sends a handoff message to A
4. A integrates the handoff and completes its fabric

Rate limiting, network errors, and malformed messages are simulated.

### Token budget enforcement

Tests use `MockLoom` with configurable token counts:

- Normal operation: all three shuttle passes complete within budget
- Pattern downgrade: budget pressure triggers twill-to-plain switch
- Emergency halt: budget exhaustion produces a valid partial patch
- Pattern learning: repeated tasks trigger weft-to-warp promotion

### Woven memory

Memory tests verify:

- Warp/weft separation (warp survives task completion, weft is archived)
- Weave pattern selection (plain for unfamiliar, twill for familiar, satin for routine)
- Compaction survival (warp threads survive, weft threads are summarized)
- Interlacement recording (which weft threads connected to which warp threads)
- Weft-to-warp promotion (recurring patterns detected and promoted)
- Thread tension calculation (high-use threads have higher tension)

---

## 10. Trade-offs and Alternatives

| Decision | Chosen | Alternative | Why |
|----------|--------|-------------|-----|
| Memory model | Woven (warp/weft) | Flat key-value | Warp/weft distinction captures persistent vs. ephemeral naturally |
| Weave patterns | Three patterns (plain/twill/satin) | Single retrieval strategy | Adaptive retrieval reduces wasted tokens on familiar tasks |
| Agent structure | 5-agent loom | Single agent | Loom positions provide natural separation of concerns |
| Patch generation | 3-pass iterative | Single-pass | Multi-pass catches more defects, mirrors textile prototyping |
| Forge abstraction | 5-method trait | Rich API | Minimal surface ensures portability |
| WASI fallback | Off-loom mode (reduced but functional) | No AI in WASI | Partial weaving better than no weaving |
| Memory promotion | Automatic (pattern detection) | Manual | Reduces operator overhead, leverages weaving history |
| Key structure | Loom-position-based | Flat per-agent | Position-based keys enforce role-specific access |

---

## 11. Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.loom.defaultPattern` | string | `twill` | Default weave pattern for new tasks |
| `but-ai.loom.maxWarpThreads` | integer | 200 | Maximum long-term memory entries |
| `but-ai.loom.maxWeftThreads` | integer | 100 | Maximum per-task context entries |
| `but-ai.loom.warpTTL` | integer | 2592000 | Warp thread TTL in seconds (30 days) |
| `but-ai.loom.promotionThreshold` | integer | 3 | Weft recurrence count to trigger warp promotion |
| `but-ai.loom.tensionDecay` | float | 0.95 | Daily tension decay factor for unused threads |
| `but-ai.agent.tokenBudget` | integer | 50000 | Total token budget per task |
| `but-ai.agent.patternDowngrade` | float | 0.80 | Budget fraction triggering pattern simplification |
| `but-ai.agent.haltThreshold` | float | 0.95 | Budget fraction triggering loom halt |
| `but-ai.memory.branch` | string | `refs/but-ai/loom` | Base ref for loom storage |
| `but-ai.shuttle.schema` | string | `but-ai/shuttle/v1` | Shuttle message schema version |
| `but-ai.shuttle.pollInterval` | integer | 30 | Seconds between shuttle message polls |
| `but-ai.identity.orgKey` | string | -- | OpenWallet organization key ID |
| `but-ai.identity.keyPrefix` | string | -- | OpenWallet agent key prefix |
| `but-ai.spool.<name>.adapter-path` | string | -- | Path to external provider spool adapter |

---

*"The loom does not care what story the fabric tells. It cares that the threads interlace correctly, that the tension is even, and that the selvedge holds. Our plugin does the same: it ensures structural integrity, and leaves meaning to the developer."*
-- Dr. Yuki Tanaka-Rhodes, Lab Director
