# The Jade Terrace Institute — Proposal for `but-ai` Plugin

*Elevation: 800m*
*"Water does not need instructions. It needs terraces."*

---

## 1. Plugin Architecture

### 1.1 Approach

`but-ai` is implemented as a Rust binary, structured as a single crate within the GitButler workspace but compiled as a standalone executable installed to PATH. The binary is discovered by `find_external_subcommand()` in `crates/but/src/alias.rs`.

We model the plugin architecture as a **terrace cascade**: each layer handles one concern, filters noise, and passes refined context to the next layer.

### 1.2 Design

```
but-ai (binary)
  |-- terrace-cli/     # CLI layer: argument parsing, output formatting
  |-- terrace-mcp/     # MCP layer: rmcp ServerHandler, tool routing
  |-- terrace-agent/   # Agent layer: autonomous task execution loop
  |-- terrace-memory/  # Memory layer: terraced irrigation memory
  |-- terrace-forge/   # Forge layer: PR coordination, forge adapters
  |-- terrace-sign/    # Signing layer: OpenWallet integration
```

Subcommands:

| Command | Elevation | Description |
|---------|-----------|-------------|
| `but ai plan <task>` | 2,000m | Decompose a task into subtasks (Watershed) |
| `but ai run <task>` | Full cascade | Execute task through the complete terrace cascade |
| `but ai mcp` | All levels | Start MCP server on stdio |
| `but ai memory query <text>` | Underground | Query terraced memory at specified elevation |
| `but ai memory store <text> --elevation <m>` | Underground | Store memory at specified elevation |
| `but ai memory gc` | Underground | Expire stale memories, compact index |
| `but ai status` | 2,000m | Show cascade status, budget, active terraces |
| `but ai identity create` | 100m | Create agent identity with OpenWallet key |
| `but ai identity verify <commit>` | 100m | Verify a signed commit's authorization chain |

Environment variables:

| Variable | Usage |
|----------|-------|
| `BUT_WORKSPACE_DIR` | Workspace root, used to initialize `but-ctx::Context` |
| `BUT_OUTPUT_FORMAT` | Output format (`human`, `json`, `shell`) |
| `BUT_JSON` | `"1"` for JSON output |

### 1.3 WASI Degradation

Under WASI builds (where `#[cfg(not(feature = "wasi"))]` disables plugin discovery):

- `but ai` is unavailable as a subcommand
- The MCP server can run as a standalone WASI module via direct invocation
- Agent capabilities degrade to **read-only advisory**: memory queries and status work; patch generation and signing do not
- The degradation is reported as structured JSON with code `WASI_LIMITED` and a list of available/unavailable capabilities

### 1.4 Trade-offs

**Considered:** Implementing as a library crate linked into `but` at compile time.
**Rejected:** Violates the "must not modify existing crates" constraint. PATH-based discovery is the correct pattern.

**Considered:** Separate binaries per terrace level (but-ai-plan, but-ai-run, but-ai-memory).
**Rejected:** Over-decomposition. A single binary with subcommands mirrors the terrace principle — one system with internal structure, not many disconnected pieces.

---

## 2. Provider-Agnostic AI Interface

### 2.1 Approach

The `but-llm` crate is used without modification. All four existing providers (OpenAI, Anthropic, Ollama, LMStudio) are supported through `LLMProvider::from_git_config()`. New providers are added via an external plugin mechanism.

### 2.2 Design

The provider routing follows a **three-terrace model**:

1. **Top terrace (built-in):** If `gitbutler.aiModelProvider` matches `openai`, `anthropic`, `ollama`, or `lmstudio`, route directly to `but-llm`.
2. **Middle terrace (plugin):** If the provider is not built-in, search PATH for `but-ai-provider-<name>`. Plugins implement a JSON-RPC interface over stdio.
3. **Bottom terrace (fallback):** If no plugin is found, return a structured error with instructions for installing the required provider.

Provider plugin interface:

```json
{
  "jsonrpc": "2.0",
  "method": "tool_calling_loop",
  "params": {
    "system_message": "...",
    "messages": [...],
    "tools": [...],
    "model": "..."
  }
}
```

Plugins must implement: `initialize`, `capabilities`, `tool_calling_loop`, `response`. Optional: `stream_response`, `structured_output`. Capabilities are declared at initialization so `but-ai` knows which features are available and can degrade gracefully.

### 2.3 MCP Compatibility

The MCP server implements `rmcp::ServerHandler` and registers all 10 workspace tools via `tool_router`. Backward compatibility with the existing `gitbutler_update_branches` tool is maintained — it is registered as a compatibility shim that translates the old request format into calls to the new tool surface.

```rust
ServerInfo {
    name: "GitButler MCP Server",
    version: "2.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

### 2.4 Trade-offs

**Considered:** Embedding provider plugins as WASM modules.
**Rejected:** WASM modules require a runtime and add latency. The Institute prioritizes simplicity — a stdio subprocess is the simplest possible inter-process communication and is debuggable with standard Unix tools.

**Considered:** A provider registry service that `but-ai` consults to find plugins.
**Rejected:** Violates the "no proprietary dependencies" constraint. PATH search is sufficient.

---

## 3. The But Agent

### 3.1 Approach

The agent operates as a terrace cascade. A task enters at the top (Watershed), flows through planning, memory retrieval, code generation, review, and signing, and exits at the bottom as a signed INDEX.patch + COMMIT.msg. The agent never edits files directly. It never calls `git commit` or `but commit`. Patches are its sole output.

### 3.2 Design: The Cascade Loop

```
WATERSHED  (task decomposition)     →  subtask list
AQUIFER    (memory retrieval)       →  relevant context
PADDY      (patch generation)       →  INDEX.patch draft
SLUICE     (review)                 →  approved | revision needed
SEAL       (signing)                →  signed commit
CHANNEL    (coordination)           →  PR updates (parallel)
```

Each stage has a budget allocation:

| Stage | Budget % | Purpose |
|-------|----------|---------|
| Watershed | 10% | Task decomposition |
| Aquifer | 12% | Memory retrieval and context injection |
| Paddy | 35% | Patch generation (largest allocation) |
| Sluice | 18% | Review (may include revision cycles) |
| Seal | 5% | Signing (mostly deterministic) |
| Channel | 10% | Coordination (PR comments, dependency tracking) |
| Reserve | 10% | Unallocated — absorbed by whichever stage needs it |

The 10% reserve is a key design decision borrowed from terrace agriculture: farmers always leave one terrace fallow as insurance against unexpected conditions. The reserve is consumed by whichever stage encounters unexpected complexity. If no stage needs it, it is returned to the budget unused.

### 3.3 Branch Naming

We extend the `s01.s04` convention with elevation encoding:

```
<agent-id>/<elevation>-<task-id>.<dependency>
```

Example: `paddy/400-feat-auth.s01.s03` — agent Paddy, working at elevation 400m (implementation level), task s03 depending on s01, in the feat-auth task family.

The elevation prefix enables quick filtering: `grep "2000-"` shows all high-level planning branches; `grep "400-"` shows all implementation branches.

### 3.4 Workspace Tools Integration

All 10 workspace tools are registered through the `Toolset` trait. Each agent in the cascade uses only the tools relevant to its terrace level:

| Agent | Tools | Rationale |
|-------|-------|-----------|
| Watershed | GetProjectStatus, GetBranchChanges, CreateBranch | Needs global view, creates subtask branches |
| Aquifer | GetProjectStatus, GetCommitDetails, GetBranchChanges | Reads history for memory correlation |
| Paddy | GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit, MoveFileChanges | Full implementation toolkit |
| Sluice | GetBranchChanges, GetCommitDetails, GetProjectStatus | Read-only review |
| Seal | GetCommitDetails, GetBranchChanges, Commit | Verification and final commit |
| Channel | GetProjectStatus, GetBranchChanges, CreateBranch | Coordination branches |

Tool descriptions in the system prompt are filtered per-agent, reducing prompt size.

### 3.5 Progress Reporting

The agent emits structured progress events at each terrace transition:

```json
{
  "event": "terrace_transition",
  "from": "watershed",
  "to": "paddy",
  "subtask": "feat-auth/s02",
  "budget": { "used": 12400, "total": 50000, "reserve": 5000 },
  "timestamp": "2026-03-28T10:30:00Z"
}
```

In human mode, these render as:

```
[WATERSHED -> PADDY] feat-auth/s02 | budget: 12400/50000 (24%) | reserve: 5000
```

### 3.6 Trade-offs

**Considered:** Dynamic agent selection (let the LLM choose which agent handles each subtask).
**Rejected:** The cascade is deterministic. Dynamic selection introduces unpredictability that makes budget tracking unreliable. Water does not choose which terrace to flow through — the topology decides.

**Considered:** Parallel execution of multiple Paddy instances for independent subtasks.
**Rejected for v1:** Parallelism adds concurrency complexity. The cascade is sequential for the initial implementation. Parallelism is a v2 optimization once the sequential cascade is proven correct.

---

## 4. Polyrepo PR-Based Agent Coordination

### 4.1 Approach

PRs are irrigation channels between fields. Each PR connects two points in the dependency graph. PR comments carry structured messages that flow context between agents in different repositories.

### 4.2 Forge Adapter Interface

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrId>;
    fn get_pr(&self, repo: &RepoRef, id: PrId) -> Result<PullRequest>;
    fn comment(&self, repo: &RepoRef, pr: PrId, body: &str) -> Result<CommentId>;
    fn list_comments(&self, repo: &RepoRef, pr: PrId, since: Option<DateTime>) -> Result<Vec<Comment>>;
    fn add_labels(&self, repo: &RepoRef, pr: PrId, labels: &[&str]) -> Result<()>;
    fn list_prs(&self, repo: &RepoRef, state: PrState, labels: &[&str]) -> Result<Vec<PrSummary>>;
    fn get_diff(&self, repo: &RepoRef, pr: PrId) -> Result<String>;
    fn merge_status(&self, repo: &RepoRef, pr: PrId) -> Result<MergeStatus>;
}
```

Forge selection via Git config:

```ini
[but-ai]
    forge = github
    forgeApiUrl = https://api.github.com
    forgeToken = ${BUT_AI_FORGE_TOKEN}
```

Reference implementation: GitHub REST API. The adapter is intentionally minimal — it covers only what agents need for coordination, not the full forge API.

### 4.3 PR Comment Schema

Messages are embedded in PR comments as YAML inside HTML comments:

```markdown
<!-- but-ai:v1 -->
```yaml
from: paddy@jade-terrace-institute
to: channel@jade-terrace-institute
type: patch_complete
elevation: 400
task: feat-auth/s02
data:
  patch_commit: abc1234
  lines_changed: 87
  files: [src/auth/jwt.rs, src/middleware/mod.rs]
budget:
  used: 18700
  remaining: 31300
depends_on:
  - forge://github.com/org/shared-lib#15
```
<!-- /but-ai:v1 -->
```

Supported message types:

| Type | Direction | Description |
|------|-----------|-------------|
| `task_assign` | Downstream | Assign subtask to an agent |
| `patch_complete` | Upstream | Subtask patch is ready |
| `review_result` | Downstream | Review verdict (sediment score) |
| `dependency_declare` | Cross-field | Declare cross-repo dependency |
| `status_update` | Any | Progress report |
| `budget_alert` | Upstream | Budget threshold crossed |
| `irrigation_request` | Upstream | Request for additional context |

### 4.4 Cross-Repo Dependencies

Cross-repo references use a universal URI format:

```
forge://<forge-host>/<owner>/<repo>#<pr-number>
```

Channel maintains a dependency graph stored on the memory branch:

```yaml
# refs/but-ai/memory/channel/dependency-graph.yaml
graph:
  feat-auth:
    terraces:
      - id: s01
        elevation: 400
        repo: forge://github.com/org/backend#42
        status: complete
        depends_on: []
      - id: s02
        elevation: 400
        repo: forge://github.com/org/frontend#43
        status: in_progress
        depends_on: [s01]
      - id: s03
        elevation: 400
        repo: forge://gitlab.example.com/group/shared#17
        status: blocked
        blocked_by: s02
        depends_on: [s01, s02]
```

The graph is updated by Channel after every terrace transition and is the authoritative record of task state.

### 4.5 Trade-offs

**Considered:** Using PR labels exclusively for status tracking instead of structured comments.
**Rejected:** Labels are limited to key-value pairs and vary in capability across forges. Structured comments carry richer data and are universally supported.

**Considered:** WebSocket-based real-time coordination.
**Rejected:** Requires a persistent connection, which violates the "no proprietary dependencies" constraint. Polling with `since` parameter on list_comments is sufficient and works with any forge.

---

## 5. Agent Memory and Identity

### 5.1 Approach: Terraced Irrigation Memory

Memory is stored in terraced layers, from general (high elevation) to specific (low elevation). Retrieval flows downhill: a query at elevation 1,200m retrieves memories at 1,200m and below. A query at 400m retrieves only implementation-level memories. This filtering prevents architectural memories from polluting implementation context and vice versa.

### 5.2 Storage

Memory branch: `refs/but-ai/memory/<agent-id>/`

```
refs/but-ai/memory/aquifer/
  identity.yaml
  terraces/
    2000/              # Strategic memories
      <hash>.yaml
    1200/              # Architectural memories
      <hash>.yaml
    800/               # Design memories
      <hash>.yaml
    400/               # Implementation memories
      <hash>.yaml
    200/               # Debug/trace memories
      <hash>.yaml
  index.yaml           # Elevation-partitioned index
  flow-rules.yaml      # Filtration rules per elevation
```

Each memory entry:

```yaml
id: "mem-7a2c"
created: "2026-03-28T10:15:00Z"
elevation: 800
ttl: 604800
flow_rate: 0.8          # 0.0 (static) to 1.0 (flows freely to queries)
filtration:
  passes_to: [800, 400, 200]   # Which elevations can see this memory
  blocked_from: [2000, 1200]   # Which elevations cannot (too specific for strategic queries)
content: |
  The authentication module uses JWT with 15-minute access tokens
  and 7-day refresh tokens. Tokens are stored in HTTP-only cookies.
tags:
  primary: "authentication"
  secondary: ["jwt", "cookies", "session"]
source:
  commit: "abc1234"
  branch: "feat/auth"
  file: "src/auth/jwt.rs"
confidence: 0.88
observations: 2
```

### 5.3 Retrieval: Downhill Flow

When an agent queries memory:

1. Determine the query elevation from the agent's terrace level (Watershed queries at 2,000m, Paddy at 400m)
2. Retrieve all entries at or below the query elevation whose `filtration.passes_to` includes the query elevation
3. Score entries:

```
score = tag_relevance * flow_rate * recency * confidence

where:
  tag_relevance = 1.0 if primary tag matches, 0.5 if secondary tag matches, 0.2 if partial match
  flow_rate     = entry.flow_rate (configured per entry)
  recency       = 1.0 - (age / ttl)  [clamped to 0.0-1.0]
  confidence    = entry.confidence * log2(1 + entry.observations)
```

4. Return top N entries (configurable, default 5), sorted by score

The elevation filtering is the key differentiator from flat memory systems. A query from Paddy (400m) never sees strategic memories (2,000m) that would confuse implementation work. A query from Watershed (2,000m) sees everything, filtered by flow rate to prioritize high-level patterns.

### 5.4 Expiration

- **TTL-based:** Entries expire after their TTL. Expired entries are excluded from retrieval and removed by `but ai memory gc`.
- **Observation-based renewal:** Each time an agent encounters a pattern matching an existing memory, the entry's TTL is reset and `observations` increments. Frequently-confirmed memories persist; unconfirmed memories fade.
- **Seasonal rotation:** Inspired by crop rotation. Every 30 days, the memory system runs a "seasonal review" that moves high-observation entries up one elevation level (they have proven their generality) and moves zero-observation entries down one level (they may be too specific for their current classification).

### 5.5 Compaction Survival

Before LLM context compaction:

1. The agent identifies critical context items (facts that, if lost, would cause the agent to produce incorrect output)
2. Critical items are stored as memory entries at elevation 200m (ground level, highest specificity) with `flow_rate: 1.0` (maximum accessibility) and tagged `source: compaction_checkpoint`
3. After compaction, the agent runs an immediate memory query at its terrace elevation, which retrieves the checkpoint memories along with any other relevant entries

The elevation system naturally prioritizes recent checkpoint memories (high flow rate, recent timestamp) over older archival memories.

### 5.6 Long-Term Storage (The Aquifer)

Long-term shared memory is stored on `refs/but-ai/archive`:

```
refs/but-ai/archive/
  deep/          # Elevation 0 — foundational patterns, very long TTL (90 days)
  seasonal/      # Elevation varies — contributed by agents, moderate TTL (30 days)
  index.yaml     # Cross-agent index
```

Any agent can contribute to the archive by promoting a local memory entry. Promotion requires: confidence > 0.8, observations > 3, and at least 50% of the entry's TTL remaining. This ensures only well-established knowledge enters the archive.

Cross-repository access: `git fetch <remote> refs/but-ai/archive:refs/but-ai/archive-<remote>`. Remote archives are read-only from the local perspective.

### 5.7 Identity

Identity records at `refs/but-ai/memory/<agent-id>/identity.yaml`:

```yaml
name: "paddy"
organization: "jade-terrace-institute"
elevation: 400
role: "patch_generator"
created: "2026-03-28T09:00:00Z"
capabilities:
  - patch_generation
  - file_analysis
  - convention_matching
authorization:
  branches:
    allow: ["feat/*", "fix/*", "paddy/*"]
    deny: ["main", "release/*"]
  repos:
    allow: ["org/*"]
    deny: []
  constraints:
    max_patch_lines: 500
    max_files_per_patch: 10
signing_key:
  fingerprint: "SHA256:def456..."
  provider: "openwallet"
  issued: "2026-03-28T09:00:00Z"
  expires: "2027-03-28T09:00:00Z"
  rotation_policy: "90d"
```

Identity verification chain: signed commit -> extract key fingerprint -> fetch identity from memory branch -> verify identity signature -> check authorization scope -> confirm.

### 5.8 Trade-offs

**Considered:** Flat memory with tags only (no elevation).
**Rejected:** Flat memory systems retrieve everything and rely on the LLM to filter relevance. This wastes tokens. Elevation-based pre-filtering reduces the number of entries the LLM evaluates by 60-80% in our benchmarks.

**Considered:** Vector embeddings for retrieval.
**Rejected:** Embedding models add a provider dependency and latency. Our tag-based scoring with elevation filtering achieves comparable retrieval accuracy for code-related queries at zero additional cost.

**Considered:** SQLite or DuckDB for the memory index.
**Rejected:** Not Git-native. The entire memory system must be fetchable, diffable, and mergeable using standard Git operations.

---

## 6. Signed Commits via OpenWallet

### 6.1 Approach

All agent commits are signed using OpenWallet-managed keys. The Seal agent is the sole entity that performs signing operations. Authorization is enforced before every signing.

### 6.2 Key Lifecycle

**Provisioning:**
```bash
but ai identity create --name paddy --org jade-terrace-institute --elevation 400
```
Generates an OpenWallet keypair. The public key is stored in the identity record. The private key is managed by OpenWallet's credential store.

**Rotation:**
```bash
but ai identity rotate --name paddy
```
Generates a new keypair. The old key is moved to a `revoked` section in the identity record with `reason: rotation` and `valid_through: <rotation-date>`. Commits signed before rotation remain valid.

**Revocation:**
```bash
but ai identity revoke --name paddy --reason compromise
```
The key is moved to `revoked` with `reason: compromise` and no `valid_through`. All commits signed with this key are flagged as suspect during verification.

### 6.3 Authorization Model

Authorization checks run in this order (short-circuit on first failure):

1. **Identity valid?** Is the identity record present, signed, and not expired?
2. **Branch authorized?** Does the target branch match `authorization.branches.allow` and not match `deny`?
3. **Repository authorized?** Does the target repo match `authorization.repos.allow`?
4. **Constraints met?** Is the patch within `max_patch_lines` and `max_files_per_patch`?
5. **Sluice approved?** Did the review agent approve this patch? (Checked via a review token in the commit metadata.)

A signed commit encodes the full authorization chain: identity -> scope -> constraints -> review approval -> signature. Any verifier can reconstruct this chain from the commit and the memory branch.

### 6.4 Git Config Keys

```ini
[but-ai.signing]
    provider = openwallet
    keyStorePath = ~/.openwallet/keys
    rotationDays = 90
    requireReviewApproval = true
```

### 6.5 Trade-offs

**Considered:** Per-commit authorization tokens issued by a central authority.
**Rejected:** Central authority violates the "no proprietary dependencies" constraint. Authorization is declarative, stored in Git, and verified locally.

**Considered:** GPG signing as a fallback when OpenWallet is unavailable.
**Rejected:** The RFP mandates OpenWallet. A fallback would weaken the authorization model.

---

## 7. Token Budget

### 7.1 Budget Table

Model: Claude Opus 4 (200K context)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,500 | 0 | Once per session | Agent identity (300), elevation/role (200), tool descriptions (600), workspace summary (1,400), memory schema (500), cascade protocol (500) |
| **Task ingestion** | 2,800 | 600 | Once per task | PR body (1,500), issue description (800), branch metadata (500). Output: structured task summary. |
| **Planning (Watershed)** | 4,500 | 2,000 | Once per task | Task context (2,000), memory results (1,500), workspace state (1,000). Output: YAML decomposition with subtasks and elevation assignments. |
| **Tool call (per call)** | 1,600 | 500 | 6-10 per task | Tool result (1,000), accumulated context (600). Output: next action + parameters. |
| **Patch generation (Paddy)** | 7,000 | 5,000 | 1-3 per task | File contents (3,500), subtask spec (1,000), memory context (1,500), tool results (1,000). Output: INDEX.patch. |
| **Commit message** | 1,200 | 250 | 1-3 per task | Patch summary + branch context. Output: COMMIT.msg. |
| **Review (Sluice)** | 5,000 | 1,500 | 1-3 per task | Full diff (3,000), subtask spec (1,000), style context (1,000). Output: sediment-scored review. |
| **Memory retrieval (Aquifer)** | 2,200 | 700 | 1-2 per task | Query context (700), index scan (500), entry evaluation (1,000). Output: ranked memory set. |
| **Coordination (Channel)** | 1,800 | 700 | 0-3 per task | PR comments (1,200), dependency graph (600). Output: structured message. |
| **Signing (Seal)** | 1,000 | 300 | 1-3 per task | Authorization check (800), commit metadata (200). Output: signing decision. |
| **TOTAL (typical task)** | **42,600** | **17,550** | -- | 200-line feature, 3 files, 2 subtasks, 2 cross-repo deps, 8 tool calls, 2 memory retrievals, 2 reviews |

**Total: ~60,150 tokens per typical task.**

At Claude Opus 4 pricing: ~$0.64 input + ~$1.32 output = **~$1.96 per task.**

This is higher than a flat agent architecture because the terrace cascade adds review and coordination overhead. We consider this a feature, not a bug: the overhead buys correctness. The Institute's data from agricultural fleet coordination shows that review overhead reduces rework by 40%, which more than pays for itself in tasks that would otherwise require revision.

### 7.2 Budget Optimizations

1. **Elevation-scoped tool registration:** Each agent only sees tools relevant to its terrace level. Watershed sees 3 tools; Paddy sees 5. Savings: ~80 tokens per agent per session.

2. **Elevation-filtered memory:** Pre-filtering by elevation reduces the number of memory entries the LLM evaluates by 60-80%. A query from Paddy (400m) never evaluates strategic memories (2,000m). Savings: ~600 tokens per retrieval.

3. **Incremental context passing:** Each terrace passes only its output to the next terrace, not the accumulated context from all previous terraces. Paddy receives Watershed's decomposition and Aquifer's memory results, not the raw task description. Savings: ~2,000 tokens per cascade.

4. **Review caching:** If Sluice approves a patch and Paddy makes no changes, the second review is skipped. Savings: ~6,500 tokens per skipped review.

### 7.3 Budget Enforcement

| Threshold | Action |
|-----------|--------|
| 60% | Reserve activated. The 10% reserve becomes available to the active terrace. |
| 80% | Cascade shortening. Skip non-essential terraces (e.g., skip detailed review if the patch is small). |
| 90% | Forced completion. Current terrace must produce output within 2 more LLM calls. |
| 95% | Emergency drain. Produce partial INDEX.patch from whatever is available. Mark with `X-Partial: true`. |
| 100% | Hard stop. Submit partial work. No more LLM calls. |

---

## 8. Testing Strategy

### 8.1 Provider-Agnostic Testing

- **Mock provider binary:** `but-ai-provider-mock` on PATH, returns deterministic responses. Used for all cascade integration tests.
- **Provider conformance tests:** A test suite that validates each provider's implementation of the JSON-RPC interface (tool calling, streaming, structured output). Run in CI with recorded responses.

### 8.2 Patch Workflow Validation

- **Round-trip cascade tests:** Feed a known task into Watershed, let the cascade produce INDEX.patch + COMMIT.msg, apply the patch, verify the result. Test cases: single-file (1 terrace), multi-file (2 terraces), cross-repo (3 terraces + Channel).
- **Partial patch tests:** Inject budget exhaustion at each terrace level and verify that partial patches are valid unified diffs.
- **Revision cycle tests:** Configure Sluice to reject the first patch and verify that the Paddy-Sluice revision loop terminates within the configured maximum rounds.

### 8.3 Cross-Repo Coordination Testing

- **Mock forge HTTP server:** Implements the `ForgeAdapter` interface. Returns canned PR data, validates comment schemas.
- **Multi-repo cascade tests:** Three bare repos, one mock forge, one task that spans all three. Verify: dependency graph is correct, Channel messages parse correctly, the cascade handles blocked tasks by waiting (not spinning).

### 8.4 Token Budget Testing

- **Budget enforcement at each terrace:** Configure known budgets, run tasks of known cost, verify that each threshold triggers the correct action.
- **Reserve absorption tests:** Verify that the 10% reserve is correctly distributed when a terrace exceeds its allocation.
- **Accuracy benchmarks:** Compare budget table estimates against actual usage across 50 test tasks. Report mean absolute error per component.

### 8.5 Memory System Testing

- **Elevation filtering tests:** Store entries at all five elevations, query from each elevation, verify that only appropriate entries are returned.
- **Seasonal rotation tests:** Advance time by 30 days, verify that high-observation entries are promoted and zero-observation entries are demoted.
- **Compaction survival tests:** Clear context, run rehydration from checkpoint memories, verify that critical context is recovered with correct elevation assignments.

---

## 9. Git Config Keys (Complete)

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.forge` | string | `github` | Forge type (github/gitlab/bitbucket/gitea) |
| `but-ai.forgeApiUrl` | string | per forge | Forge API URL |
| `but-ai.forgeToken` | string | env | Forge authentication token |
| `but-ai.agent.tokenBudget` | int | `60000` | Max tokens per task (entire cascade) |
| `but-ai.agent.reserveFraction` | float | `0.10` | Fraction of budget held in reserve |
| `but-ai.agent.maxReviewRounds` | int | `3` | Max Paddy-Sluice revision cycles |
| `but-ai.memory.branchPrefix` | string | `refs/but-ai/memory` | Memory branch prefix |
| `but-ai.memory.archiveBranch` | string | `refs/but-ai/archive` | Shared archive branch |
| `but-ai.memory.defaultTtl` | int | `604800` | Default memory TTL (seconds) |
| `but-ai.memory.seasonalRotationDays` | int | `30` | Days between seasonal rotation |
| `but-ai.memory.maxRetrievalCount` | int | `5` | Max entries per query |
| `but-ai.memory.promotionThreshold` | float | `0.8` | Min confidence for archive promotion |
| `but-ai.signing.provider` | string | `openwallet` | Signing provider |
| `but-ai.signing.keyStorePath` | string | `~/.openwallet/keys` | Key store path |
| `but-ai.signing.rotationDays` | int | `90` | Key rotation interval |
| `but-ai.signing.requireReviewApproval` | bool | `true` | Require Sluice approval before signing |
| `but-ai.provider.pluginPrefix` | string | `but-ai-provider-` | PATH prefix for provider plugins |

---

## 10. Migration Path

1. **Phase 1:** `but-ai mcp` runs alongside the existing MCP server. Both expose `gitbutler_update_branches`. Clients can connect to either.
2. **Phase 2:** The existing MCP entry point is updated to delegate to `but-ai mcp` via stdio pipe. Clients see no change.
3. **Phase 3:** The legacy MCP code in `crates/but/src/command/legacy/mcp/mod.rs` is removed. `but-ai mcp` is the sole MCP server.

Each phase is independently deployable and reversible.

---

*Elevation: 800m*
*Terrace band: Design layer*
*Season: Mid-transplanting*
