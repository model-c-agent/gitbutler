# Proposal: `but-ai` Plugin -- The Serengeti Systems Ecology Lab

**Submitted by:** The Serengeti Systems Ecology Lab (Org 073)
**Domain:** Wildlife Conservation -- Computational Ecology
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

The Serengeti Systems Ecology Lab proposes a `but-ai` plugin modeled on savanna ecosystem dynamics. We treat the agent-codebase relationship as an organism-ecosystem relationship: agents consume resources (tokens), produce artifacts (patches), interact through trophic links (dependencies), and operate within carrying capacity constraints (budgets). Our central innovation is **ecosystem memory** -- a memory scheme where memories form a food web with trophic levels, where removing a foundational memory cascades through its dependents exactly as removing a keystone species cascades through an ecosystem.

We implement `but-ai` as a Rust crate (`crates/but-ai`) with a companion binary (`but-ai`) that functions as both a CLI subcommand and an MCP server. The plugin uses the existing `but-llm` crate for all LLM interactions and the existing `but-tools` crate for workspace operations. It introduces no new LLM clients and modifies no existing crates.

---

## 2. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a standalone Rust executable placed on PATH. It is discovered by `find_external_subcommand()` in `crates/but/src/alias.rs` and invoked as `but ai <subcommand>`.

### Design

**Binary structure:**

```
but-ai
  ├── cli mode:    but ai <subcommand> [args]
  ├── mcp mode:    but ai mcp
  └── agent mode:  but ai agent --task <description>
```

**Environment consumption:**

The binary reads `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, and `BUT_JSON` from environment. All output respects the format flag: `human` produces colored terminal output with ecological status indicators, `json` produces structured JSON, `shell` produces key=value pairs.

**Crate organization:**

```
crates/but-ai/         -- library crate (core logic)
crates/but-ai-bin/     -- binary crate (CLI entry point, MCP server)
```

The library crate is separated from the binary to enable testing without spawning processes.

**WASI degradation:**

When running under WASI (`#[cfg(feature = "wasi")]`), the plugin cannot be discovered via PATH. We provide a `but ai --embedded` mode where the AI capabilities are compiled as a library function rather than a subprocess. In WASI builds, the agent operates with reduced capabilities: no cross-repo coordination (no network access), no forge API calls, but full memory management and patch generation from pre-loaded context. This mirrors ecological island biogeography -- a population cut off from migration maintains its local ecosystem but cannot participate in metapopulation dynamics.

### Trade-offs

**Alternative considered: Single binary (no library crate).** Rejected because testing a single binary requires process spawning, which is slow and fragile. The library crate enables unit testing of all logic without subprocess overhead.

**Alternative considered: Dynamic plugin loading via shared libraries.** Rejected because dynamic linking introduces platform-specific complexity and is incompatible with the WASI target. PATH-based discovery is simpler and sufficient.

---

## 3. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

We use `but-llm` as-is. The existing `LLMProviderKind` enum (OpenAI, Anthropic, Ollama, LMStudio) and the five interaction methods (`tool_calling_loop`, `tool_calling_loop_stream`, `stream_response`, `structured_output`, `response`) are sufficient for all agent operations.

### Design

**Provider adapter layer:**

```rust
pub trait ProviderAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn supports_tool_calling(&self) -> bool;
    fn supports_structured_output(&self) -> bool;
    fn max_context_tokens(&self) -> usize;
    fn configure(&self, config: &gix::config::File) -> anyhow::Result<LLMProviderConfig>;
}
```

New providers (Gemini, Mistral, local GGUF) implement this trait. The adapter produces an `LLMProviderConfig` that `but-llm` already knows how to consume. This means new providers are added without modifying `but-llm` or recompiling `but-ai` -- the adapter is loaded from a configuration-specified path.

**Provider discovery:**

New providers are registered via Git config:

```ini
[but-ai "provider.gemini"]
    adapter = "path/to/gemini-adapter"
    priority = 10
```

The plugin scans `but-ai.provider.*` config sections at startup and loads adapters. Built-in providers (the four in `LLMProviderKind`) have implicit adapters that require no configuration.

**Tool exposure:**

All 10 `WorkspaceToolset` tools are registered via the `Toolset` trait and exposed to every provider that supports tool calling. Providers that do not support tool calling (some Ollama models) fall back to `structured_output` with a tool-call schema, or `response` with explicit instructions. This is ecological niche partitioning -- different providers occupy different capability niches, and the system adapts the interaction pattern to the provider's capabilities.

### Trade-offs

**Alternative considered: Introducing a new unified LLM client.** Rejected per RFP Section 5.2 (disqualifying factor). `but-llm` is the only LLM interface.

**Alternative considered: Requiring all providers to support tool calling.** Rejected because this would exclude useful local models (many Ollama models lack tool calling support). The fallback mechanism allows the system to work with reduced capability rather than failing entirely.

---

## 4. The But Agent (RFP 3.3)

### Approach

The But Agent operates as a trophic web of six specialized agents (see AGENTS.md), coordinated by Nyerere (the lead modeler). The agent reads task descriptions, decomposes them into trophic-level operations, and produces INDEX.patch + COMMIT.msg as its sole write primitive.

### Design

**Agent lifecycle:**

```
1. OBSERVE:  Nyerere reads task, queries memory, maps dependencies
2. MODEL:    Nyerere constructs system model, identifies cascade risks
3. PLAN:     Nyerere decomposes task into operational orders
4. PRODUCE:  Makena generates patches; Baruti updates memory
5. VALIDATE: Kiptoo runs the Fieldworker Gauntlet
6. EMIT:     Final INDEX.patch + COMMIT.msg produced
```

**Patch production:**

The agent never calls `but commit` or makes direct file edits. Makena generates unified diffs against the current index. The diff is written to `INDEX.patch` in the agent's working directory. The commit message is written to `COMMIT.msg`. Both files are produced atomically -- either both exist and are valid, or neither exists.

**Branch naming:**

We extend the existing `s01.s04` convention with ecological metadata:

```
agent/<agent-id>/<trophic-level>/<task-id>[.<dependency-id>]
```

Example: `agent/nyerere/L3/t042.t039` -- Agent Nyerere, trophic level 3, task 42, depends on task 39.

**Token budget enforcement:**

Tendaji monitors token consumption across all agents. At 50% consumption, he issues an advisory. At 75%, he issues a warning. At 90%, he triggers "dry season protocol" -- all agents reduce output complexity and focus on the critical path. At 95%, Nyerere issues a mandatory halt, and whatever partial work exists is packaged as a valid (but incomplete) INDEX.patch with explicit annotations about what was deferred.

**Progress reporting:**

All progress is reported via structured output:

```json
{
  "phase": "PRODUCE",
  "agent": "makena",
  "trophic_level": 1,
  "tokens_used": 12400,
  "tokens_budget": 50000,
  "patches_generated": 2,
  "patches_validated": 1,
  "ecosystem_health": 0.87
}
```

The `ecosystem_health` metric is a composite score (0.0-1.0) that aggregates token budget status, memory coherence, and coordination status. It is the agent equivalent of an ecosystem health index.

### Trade-offs

**Alternative considered: Flat agent architecture (single agent, no specialization).** Rejected because a single agent cannot effectively perform observation, generation, validation, and coordination within a single context window. Specialization allows each agent to maintain a focused context, reducing per-agent token consumption even though total consumption is higher.

**Alternative considered: Hierarchical command structure (like RAC).** Rejected because our lab's experience with ecosystem modeling shows that rigid hierarchies are brittle -- removing the top node collapses the system. A trophic web is more resilient because energy (information) flows through multiple paths.

---

## 5. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

We model cross-repo coordination as species migration between habitat patches. Each repository is a habitat. PRs are migration corridors. Agent work moves along these corridors following dependency gradients, just as animals follow resource gradients between patches.

### Design

**Forge adapter interface:**

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &PullRequest) -> Result<PrId>;
    fn comment_on_pr(&self, pr: &PrId, comment: &AgentComment) -> Result<CommentId>;
    fn get_pr_comments(&self, pr: &PrId) -> Result<Vec<AgentComment>>;
    fn add_labels(&self, pr: &PrId, labels: &[&str]) -> Result<()>;
    fn get_pr_status(&self, pr: &PrId) -> Result<PrStatus>;
    fn list_related_prs(&self, pr: &PrId) -> Result<Vec<PrRef>>;
}
```

We provide a GitHub reference implementation using the REST API. The interface is deliberately minimal -- only operations available on all forges (GitHub, GitLab, Bitbucket, Gitea/Forgejo) are included.

**PR comment schema:**

```json
{
  "$schema": "but-ai/coordination/v1",
  "type": "task_assignment | status_report | dependency | patch_handoff | budget_report",
  "from": { "agent": "zawadi", "org": "073-serengeti", "repo": "owner/repo" },
  "to": { "agent": "target-agent", "org": "target-org", "repo": "owner/other-repo" },
  "payload": {
    "task": "implement authentication middleware",
    "status": "in_progress",
    "dependencies": ["owner/other-repo#42", "owner/third-repo#17"],
    "budget": { "used": 24000, "total": 50000 },
    "patch_ref": "agent/makena/L1/t042"
  },
  "classification": "GREEN",
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Comments are embedded in a code fence with language tag `but-ai-coordination` to distinguish them from human comments:

````markdown
```but-ai-coordination
{ ... schema above ... }
```
````

**Cross-repo dependency tracking:**

Zawadi maintains a dependency graph where nodes are PRs (identified by `forge:owner/repo#number`) and edges are dependency relationships. The graph is stored in the ecosystem memory as a high-priority Producer memory (trophic level 1), ensuring it is available to all agents and cascades appropriately if invalidated.

**Forge-agnosticism:**

The `RepoRef` type is forge-agnostic:

```rust
pub struct RepoRef {
    pub forge: ForgeKind, // GitHub, GitLab, Bitbucket, Gitea
    pub owner: String,
    pub name: String,
}
```

All coordination operations go through the `ForgeAdapter` trait. No agent code touches forge-specific APIs directly.

### Trade-offs

**Alternative considered: Git-native coordination (no forge API).** Rejected because Git alone cannot provide the notification and discussion features that PRs offer. However, the coordination schema is stored in Git branches (as part of ecosystem memory), so the forge is used for notification only -- all persistent state is in Git.

**Alternative considered: Rich comment schema with inline diffs.** Rejected because large comments hit forge API rate limits and size limits. Patch handoffs reference branches, not inline diffs.

---

## 6. Agent Memory and Identity (RFP 3.5)

### Approach: Ecosystem Memory

Our memory system models agent memory as a **food web**. Memories are organisms occupying trophic levels. Primary memories (Producers) are foundational -- they represent core facts about the codebase, its architecture, its conventions. Secondary memories (Primary Consumers) are derived -- they represent conclusions, patterns, and associations drawn from primary memories. Tertiary memories (Secondary Consumers) are meta-cognitive -- they represent the agent's understanding of its own performance, its error patterns, its successful strategies.

Removing a Producer memory cascades through the food web. If the architectural memory "module X uses pattern Y" is invalidated, all Consumer memories that depend on it ("when working on module X, prefer approach Z because of pattern Y") are flagged for review. This cascade is the central mechanism for maintaining memory coherence.

### Design

**Storage:**

Memories are stored as JSON files on a special-purpose branch:

```
refs/but-ai/memory/<agent-id>/
  producers/
    arch-001.json    -- "The authentication module uses middleware pattern"
    conv-002.json    -- "The team uses snake_case for function names"
  primary-consumers/
    strat-001.json   -- "When modifying auth, update middleware chain"
    patt-002.json    -- "snake_case violations are likely copy-paste errors"
  secondary-consumers/
    meta-001.json    -- "I am more accurate on auth tasks than data tasks"
  decomposed/
    arch-001-v1.json -- Expired version of arch-001, preserved for pattern analysis
```

**Memory entry structure:**

```json
{
  "id": "arch-001",
  "trophic_level": "producer",
  "content": "The authentication module uses a middleware chain pattern",
  "created_at": "2026-03-15T10:00:00Z",
  "last_accessed": "2026-03-28T09:00:00Z",
  "access_count": 14,
  "ttl_seconds": 2592000,
  "dependencies": [],
  "dependents": ["strat-001", "patt-002"],
  "relevance_tags": ["authentication", "middleware", "architecture"],
  "cascade_impact": 0.72,
  "embedding_vector": [0.12, -0.34, ...],
  "source_commit": "abc123def"
}
```

**Relevance scoring:**

Retrieval uses a composite score:

```
score = 0.4 * embedding_similarity(query, memory)
      + 0.3 * trophic_importance(memory)
      + 0.2 * recency_factor(memory)
      + 0.1 * access_frequency(memory)
```

`trophic_importance` is the Nyerere Cascade Index -- a measure of how many downstream memories depend on this one. High-cascade-impact memories score higher because they represent foundational knowledge.

**Expiration:**

Memories expire based on their trophic level:

| Trophic Level | Default TTL | Renewal Condition |
|---------------|-------------|-------------------|
| Producer | 30 days | Any dependent is accessed |
| Primary Consumer | 14 days | Direct access or source Producer is accessed |
| Secondary Consumer | 7 days | Direct access only |
| Decomposed | Permanent | Never expires (archived) |

When a Producer memory expires without renewal, all its dependents enter a "review" state. Kiptoo evaluates whether the dependents are still valid without their Producer. Valid dependents are re-rooted to a new Producer or promoted to Producer status themselves. Invalid dependents are decomposed (archived).

**Compaction survival:**

When the context window is compacted, the agent preserves memories in trophic order: Producers first, then Primary Consumers, then Secondary Consumers. This ensures that foundational knowledge survives compaction even when derived knowledge is lost. The trophic structure allows rapid reconstruction of Consumer memories from surviving Producers -- if you know the foundational facts, you can re-derive the conclusions.

**Long-term storage:**

The decomposed archive (`decomposed/`) serves as the long-term memory store. All expired memories are archived here with their complete lifecycle metadata (creation, access history, expiration cause, cascade effects). This archive is searchable and serves as training data for improving the cascade impact estimator.

Cross-repository long-term memory is stored on a shared branch:

```
refs/but-ai/shared-memory/
  patterns/       -- Cross-repo architectural patterns
  conventions/    -- Cross-repo coding conventions
  failures/       -- Cross-repo failure patterns (learning from past cascades)
```

This shared memory branch can be fetched across repositories, enabling agents to learn from other agents' experiences without requiring a central service.

**Identity:**

Each agent's identity is stored as a signed JSON document on the memory branch:

```json
{
  "agent_id": "nyerere",
  "org_id": "073-serengeti",
  "capabilities": ["architecture", "planning", "observation"],
  "authorization_scope": {
    "branches": ["agent/*", "feat/*"],
    "max_patch_lines": 1000,
    "repos": ["owner/main-repo", "owner/lib-repo"]
  },
  "signing_key_id": "openwallet:073-serengeti:nyerere",
  "created_at": "2026-03-01T00:00:00Z",
  "trophic_level": "secondary_consumer"
}
```

The identity document is signed with the agent's OpenWallet key, creating a self-referential proof: the identity document proves the agent is who it claims to be, and the signature proves the agent controls the key listed in the identity document.

### Trade-offs

**Alternative considered: Flat memory (key-value store with TTL).** Rejected because flat memory has no mechanism for cascade invalidation. When a foundational fact changes, all derived knowledge must be manually identified and updated. The trophic web automates this.

**Alternative considered: Graph database for memory.** Rejected because the RFP mandates Git-native storage. A graph database would require an external service.

---

## 7. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every agent has an OpenWallet-managed signing key. Every commit produced by any agent is signed with that key. The signature chain proves: this agent, in this organization, with these capabilities, was authorized to make this change.

### Design

**Key hierarchy:**

```
Organization Key (073-serengeti)
  ├── Agent Key (nyerere)
  ├── Agent Key (kiptoo)
  ├── Agent Key (makena)
  ├── Agent Key (baruti)
  ├── Agent Key (zawadi)
  └── Agent Key (tendaji)
```

The organization key signs all agent keys. Each agent key signs that agent's commits. Verification requires checking both the agent signature and the organization endorsement.

**Authorization model:**

Authorization is encoded in the agent's identity document (see Section 6). The authorization scope specifies:

- **Branch patterns:** Which branches the agent may commit to (glob patterns)
- **Patch size limits:** Maximum lines changed per commit
- **Repository scope:** Which repositories the agent may operate in
- **Trophic constraints:** Which trophic levels of memory the agent may modify

A signed commit is valid only if the committing agent's authorization scope permits the operation. Verification is performed by the `but` orchestrator before applying any INDEX.patch.

**Key lifecycle:**

| Event | Action |
|-------|--------|
| Agent provisioned | Organization key signs new agent key; agent identity document created |
| Routine rotation | New key generated, old key marked "rotated" (commits remain valid) |
| Compromise detected | Old key marked "compromised" (commits require re-validation); all dependent keys re-signed |
| Agent decommissioned | Key marked "decommissioned"; authorization scope set to empty |

Rotation and revocation are recorded as signed entries on the memory branch, creating an auditable lifecycle log.

**Distinguishing rotation from compromise:**

Rotated keys have a `successor_key_id` field pointing to the replacement. Compromised keys have a `compromise_detected_at` timestamp and no successor. Commits signed by a rotated key are trusted up to the rotation timestamp. Commits signed by a compromised key require manual re-validation from the compromise detection timestamp backward.

### Trade-offs

**Alternative considered: Shared organization key for all agents.** Rejected because it prevents per-agent authorization and makes key compromise catastrophic (all agents affected).

**Alternative considered: Per-commit authorization tokens.** Rejected as over-engineered. The identity document with authorization scope is sufficient for the current use case.

---

## 8. Token Budget (RFP 3.7)

Estimates for a frontier model (Claude Opus) executing a typical task: implement a 200-line feature across 3 files with 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,500 | 0 | Once per session | Agent identity, tool descriptions, trophic-level context, workspace summary |
| **Task ingestion** | 2,000 | 500 | Once per task | PR body, branch metadata, dependency chain |
| **Planning (Nyerere)** | 4,000 | 2,000 | Once per task | Dependency mapping, cascade risk assessment, operational decomposition |
| **Tool call (per call)** | 800 | 400 | ~8 per task | GetProjectStatus (2x), GetBranchChanges (3x), Commit (2x), CreateBranch (1x) |
| **Patch generation (Makena)** | 3,000 | 4,000 | Once per task | Reading 3 files (~1000 tokens each), producing 200 lines of unified diff |
| **Commit message** | 500 | 300 | Once per task | Structured message with trophic metadata |
| **Memory retrieval (Baruti)** | 1,500 | 500 | 2 per task | Query formulation, trophic traversal, result injection |
| **Memory storage** | 500 | 800 | 1 per task | Creating/updating Producer and Consumer memories |
| **Validation (Kiptoo)** | 2,000 | 1,000 | Once per task | Fieldworker Gauntlet: consistency, completeness, cascade check |
| **Coordination (Zawadi)** | 1,500 | 800 | 2 per task | Reading PR comments, formulating responses, dependency tracking |
| **Budget monitoring (Tendaji)** | 300 | 200 | ~4 per task | Lightweight burn-rate checks |
| **TOTAL (typical task)** | **25,500** | **14,700** | -- | **40,200 total tokens** |

### Justification

The total of ~40,000 tokens is higher than a single-agent approach would require (~25,000) but produces higher-quality outputs because each agent maintains a focused context. The trophic specialization means no single agent needs to hold the entire problem in context simultaneously.

The system prompt is kept under 4,000 tokens by compressing tool descriptions into a summary format and storing detailed descriptions in memory (retrieved on demand). The trophic-level context adds approximately 400 tokens to the base system prompt.

Memory retrieval costs are low because the trophic structure allows targeted traversal -- the agent queries at a specific trophic level rather than searching all memories. This reduces the number of embedding comparisons from O(n) to O(n/k) where k is the number of trophic levels.

---

## 9. Testing Strategy

### Provider-agnostic behavior

We test against a mock LLM provider that implements the `LLMProvider` interface with deterministic responses. The mock provider returns pre-defined tool call sequences for known inputs, allowing us to test the full agent lifecycle without live API calls. Each of the four real providers (OpenAI, Anthropic, Ollama, LMStudio) has a provider-specific integration test that runs against a live endpoint in CI (gated behind a feature flag).

### Patch workflow validation

INDEX.patch round-trip testing:

1. Generate a known codebase state
2. Apply a known set of changes
3. Produce INDEX.patch via the agent
4. Apply INDEX.patch to a clean copy of the original state
5. Verify the result matches the known changed state

This test runs for every patch-generating code path, including the partial-patch path triggered by budget exhaustion.

### Cross-repo coordination

We test against a mock forge that implements the `ForgeAdapter` trait with an in-memory PR store. The mock forge simulates latency, rate limiting, and error conditions. Cross-repo tests use two mock repositories with interdependent tasks, verifying that dependency tracking, status reporting, and patch handoff work correctly.

### Token budget enforcement

Budget tests use the mock LLM provider with configurable token counts per response. We test:

- Normal operation (budget sufficient)
- Dry season protocol activation (budget at 90%)
- Mandatory halt (budget at 95%)
- Partial patch production under budget pressure
- Tendaji's burn-rate prediction accuracy

### Ecosystem memory

Memory tests verify:

- Cascade invalidation (removing a Producer triggers review of all dependents)
- Trophic-level ordering during compaction (Producers survive, Consumers may not)
- Relevance scoring accuracy (motif-based retrieval outperforms keyword-only retrieval)
- TTL expiration and renewal mechanics
- Cross-repo shared memory fetch and merge

---

## 10. Trade-offs and Alternatives

| Decision | Chosen | Alternative | Why |
|----------|--------|-------------|-----|
| Memory model | Ecosystem (trophic web) | Flat key-value | Cascade invalidation is essential for memory coherence |
| Agent structure | Multi-agent trophic web | Single monolithic agent | Specialization reduces per-agent context load |
| Branch naming | Trophic-encoded | Simple sequential | Trophic encoding enables automated dependency inference |
| Provider extension | Config-registered adapters | Compile-time enum extension | Runtime extensibility avoids recompilation |
| Forge abstraction | Minimal trait (6 methods) | Rich trait (20+ methods) | Minimal surface ensures portability across forges |
| WASI fallback | Embedded library mode | No AI in WASI | Partial capability is better than none |
| Memory storage | JSON on Git branches | SQLite database | Git-native storage per RFP requirement |
| Relevance scoring | Composite (embedding + trophic + recency + frequency) | Embedding-only | Trophic importance captures structural relevance that embeddings miss |

---

## 11. Configuration Reference

All configuration via Git config, consistent with existing patterns.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.agent.tokenBudget` | integer | 50000 | Total token budget per task |
| `but-ai.agent.drySeasonThreshold` | float | 0.90 | Budget fraction that triggers dry season protocol |
| `but-ai.agent.haltThreshold` | float | 0.95 | Budget fraction that triggers mandatory halt |
| `but-ai.memory.branch` | string | `refs/but-ai/memory` | Base ref for memory storage |
| `but-ai.memory.producerTTL` | integer | 2592000 | Producer memory TTL in seconds (default 30 days) |
| `but-ai.memory.primaryConsumerTTL` | integer | 1209600 | Primary Consumer TTL (14 days) |
| `but-ai.memory.secondaryConsumerTTL` | integer | 604800 | Secondary Consumer TTL (7 days) |
| `but-ai.memory.cascadeThreshold` | float | 0.5 | Cascade impact threshold for mandatory review |
| `but-ai.memory.maxEntries` | integer | 1000 | Maximum memory entries (carrying capacity) |
| `but-ai.coordination.schema` | string | `but-ai/coordination/v1` | PR comment schema version |
| `but-ai.coordination.retryAttempts` | integer | 3 | Forge API retry count |
| `but-ai.identity.orgKey` | string | -- | OpenWallet organization key ID |
| `but-ai.identity.agentKeyPrefix` | string | -- | OpenWallet agent key ID prefix |
| `but-ai.provider.<name>.adapter` | string | -- | Path to external provider adapter |
| `but-ai.provider.<name>.priority` | integer | 0 | Provider priority (higher = preferred) |

Each key is justified by a specific requirement: budget keys support RFP 3.3 (token budget enforcement), memory keys support RFP 3.5 (memory and identity), coordination keys support RFP 3.4 (polyrepo coordination), identity keys support RFP 3.6 (OpenWallet signing), and provider keys support RFP 3.2 (provider-agnostic interface).

---

*"We model ecosystems because we believe that the behavior of a system is determined by the interactions between its parts, not by the parts themselves. The `but-ai` plugin is no different. Its value lies not in any single agent or any single feature, but in the web of interactions that holds them together."*
-- Dr. Amara Nyerere, Lab Director
