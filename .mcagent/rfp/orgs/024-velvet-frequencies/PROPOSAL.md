# Velvet Frequencies — Proposal for `but-ai` Plugin

*Key signature: A minor*
*"A good patch resonates with the codebase it enters. A great patch makes the whole codebase sing."*

---

## 1. Plugin Architecture

### 1.1 Approach

`but-ai` is a standalone Rust binary discovered via PATH by `find_external_subcommand()` in `crates/but/src/alias.rs`. It operates in two modes:

- **CLI mode:** `but ai <subcommand>` — interactive and scripted use
- **MCP mode:** `but ai mcp` — stdio-based MCP server compatible with the `rmcp` crate

The binary is structured as a thin dispatcher that routes to internal modules based on the subcommand. We call this the **mixing desk** — each subcommand is a channel strip, and the dispatcher is the master bus.

### 1.2 Design

```
but-ai (binary)
  |-- cli/          # CLI argument parsing (clap)
  |-- mcp/          # MCP server (rmcp ServerHandler)
  |-- agent/        # Autonomous agent loop
  |-- memory/       # Harmonic memory system
  |-- forge/        # Forge adapter layer
  |-- signing/      # OpenWallet integration
  |-- provider/     # Provider plugin mechanism
```

Subcommands:

| Command | Description |
|---------|-------------|
| `but ai run <task>` | Execute a task description as an agent |
| `but ai mcp` | Start MCP server on stdio |
| `but ai memory query <text>` | Query agent memory |
| `but ai memory store <text>` | Store a memory entry |
| `but ai memory gc` | Run memory garbage collection (expire stale entries) |
| `but ai status` | Show agent status, budget, and recent activity |
| `but ai sign <commit>` | Sign a commit via OpenWallet |
| `but ai identity` | Show or create agent identity |

Environment variables consumed:

| Variable | Usage |
|----------|-------|
| `BUT_WORKSPACE_DIR` | Root of Git workspace; used to initialize `but-ctx::Context` |
| `BUT_OUTPUT_FORMAT` | Controls output format: `human`, `json`, `shell` |
| `BUT_JSON` | Shorthand for JSON output |

### 1.3 WASI Graceful Degradation

Plugin discovery is gated behind `#[cfg(not(feature = "wasi"))]`. Under WASI:

- `but ai` is not available as a subcommand (no PATH-based discovery)
- The `but-ai` MCP server can still run as a standalone WASI module communicating over stdio, but it must be invoked directly, not through `but`
- Agent capabilities degrade to "read-only advisory" — the agent can analyze and suggest but cannot produce patches, because the patch application requires filesystem access that WASI sandboxing restricts
- Memory retrieval remains functional (Git branch reads are supported in WASI via `gix`)

The degradation is explicit: running `but ai` under WASI prints a structured error with code `WASI_UNSUPPORTED` listing which capabilities are available and which are not.

### 1.4 Trade-offs

**Considered:** Building `but-ai` as a new crate within the workspace instead of a standalone binary.
**Rejected:** The RFP mandates PATH-based plugin discovery. A workspace crate would require modifying `but`'s build to include the new crate, violating the "must not modify existing crates" constraint.

**Considered:** Implementing MCP as a separate binary (`but-ai-mcp`).
**Rejected:** Two binaries doubles the discovery and deployment surface. A single binary with a `mcp` subcommand is cleaner and mirrors the existing pattern in `crates/but/src/command/legacy/mcp/`.

---

## 2. Provider-Agnostic AI Interface

### 2.1 Approach

We use the existing `but-llm` crate as-is. No modifications, no wrappers, no new LLM clients. The four existing providers (OpenAI, Anthropic, Ollama, LMStudio) work through `LLMProvider::from_git_config()`. We add a **provider plugin mechanism** for future providers that does not require recompiling `but-ai`.

### 2.2 Design

The provider plugin mechanism is a frequency-multiplexing pattern:

1. `but-ai` reads `gitbutler.aiModelProvider` from Git config
2. If the value matches a built-in provider (`openai`, `anthropic`, `ollama`, `lmstudio`), route to `but-llm` directly
3. If the value does not match, search for a provider plugin executable: `but-ai-provider-<name>` on PATH
4. Provider plugins communicate via stdio JSON-RPC, implementing a minimal interface:

```json
{
  "methods": [
    "initialize",
    "tool_calling_loop",
    "stream_response",
    "structured_output",
    "response",
    "capabilities"
  ]
}
```

The `capabilities` method returns which features the provider supports (tool calling, streaming, structured output). `but-ai` degrades gracefully when a provider lacks capabilities — if a provider does not support tool calling, the agent falls back to a plan-then-execute loop using `response` + manual tool dispatch.

### 2.3 MCP Tool Surface

All 10 workspace tools from `WorkspaceToolset` are registered in the MCP server via the `tool_router` macro from `rmcp`. The MCP server's `ServerInfo` maintains backward compatibility:

```rust
ServerInfo {
    name: "GitButler MCP Server",
    version: "2.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

The old `gitbutler_update_branches` tool is preserved as a compatibility shim that internally routes through the new tool surface. Existing MCP clients continue working without modification.

### 2.4 Trade-offs

**Considered:** A dynamic library (.so/.dylib) plugin system for providers.
**Rejected:** Dynamic libraries create deployment nightmares on cross-platform targets and are incompatible with WASI. Executable plugins on PATH are the same pattern `but` itself uses — consistency over cleverness.

**Considered:** gRPC for provider plugin communication.
**Rejected:** gRPC requires a runtime, code generation, and adds 3MB+ to the binary. JSON-RPC over stdio is simple, debuggable, and requires zero dependencies beyond `serde_json`.

---

## 3. The But Agent

### 3.1 Approach

The But Agent operates as an autonomous loop inside `but ai run`. It reads a task, plans, executes tool calls, and produces `INDEX.patch` + `COMMIT.msg` as its sole write primitive. It never edits files directly. It never calls `git commit` or `but commit`. It produces patches; the orchestrator applies them.

We call this the **recording session model**: the agent is the musician, the orchestrator is the recording engineer. The musician plays; the engineer presses record.

### 3.2 Design

The agent loop follows a fixed pipeline that mirrors a recording session:

```
1. SOUNDCHECK  — Read task description, initialize context
2. REHEARSAL   — Query memory (Resonance), plan approach
3. TRACKING    — Execute tool calls, gather information
4. MIXDOWN     — Generate INDEX.patch from gathered context
5. MASTERING   — Generate COMMIT.msg, validate patch
6. PRESSING    — Sign via OpenWallet (Envelope), submit
```

Each phase has a budget allocation. If the total budget is B tokens:

| Phase | Budget % | Purpose |
|-------|----------|---------|
| Soundcheck | 5% | Task ingestion |
| Rehearsal | 15% | Memory + planning |
| Tracking | 40% | Tool calls (N calls, budget shared) |
| Mixdown | 25% | Patch generation |
| Mastering | 10% | Commit message + validation |
| Pressing | 5% | Signing overhead |

The agent tracks cumulative usage after every LLM call. When usage exceeds 85% of budget, the agent enters **fadeout mode**: it completes the current phase, skips remaining tool calls, and produces whatever partial patch it has. The partial patch is marked with a `X-Partial: true` header so the orchestrator knows it is incomplete.

### 3.3 Branch Naming

We extend the current `s01.s04` convention with a harmonic encoding:

```
<agent-id>/<task-id>.<dependency-chain>
```

Example: `overtone/feat-auth.s01.s03` means agent Overtone, working on feature "auth", task s03 which depends on s01.

The agent identity prefix ensures branch isolation. The dependency chain is read right-to-left: the rightmost segment is the current task, segments to the left are dependencies.

### 3.4 WorkspaceToolset Integration

All 10 workspace tools are registered via `Toolset::register_tool()` and exposed to the LLM through the tool-calling interface. Tool descriptions are compressed to minimize system prompt size:

| Tool | Compressed Description | Tokens |
|------|----------------------|--------|
| Commit | Create commit on branch | ~15 |
| CreateBranch | Create virtual branch | ~12 |
| Amend | Amend existing commit | ~10 |
| SquashCommits | Squash N commits into 1 | ~14 |
| GetProjectStatus | Full workspace status | ~12 |
| MoveFileChanges | Move changes between branches | ~16 |
| GetCommitDetails | Details of specific commit | ~14 |
| GetBranchChanges | Changes on specific branch | ~14 |
| SplitBranch | Split branch into N branches | ~15 |
| SplitCommit | Split commit into N commits | ~15 |

Total tool description overhead: ~137 tokens.

### 3.5 Trade-offs

**Considered:** Letting the agent choose its own phase order dynamically.
**Rejected:** A fixed pipeline is predictable and debuggable. Dynamic phase ordering sounds flexible but makes budget tracking unreliable and failure diagnosis nearly impossible. Musicians improvise within structure, not without it.

**Considered:** Allowing the agent to call `Commit` directly.
**Rejected:** The RFP mandates INDEX.patch + COMMIT.msg as the write primitive. Direct commits bypass the orchestrator's conflict detection and signing pipeline.

---

## 4. Polyrepo PR-Based Agent Coordination

### 4.1 Approach

PRs are the communication medium. PR comments are structured messages. Cross-repo references create the dependency graph. No message buses, no databases, no SaaS. Git and the forge API are the only infrastructure.

We model this as a **multi-track recording session**: each repository is a track, each PR is a take, and the coordination protocol ensures all tracks stay in sync.

### 4.2 Forge Adapter Interface

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &PullRequest) -> Result<PrId>;
    fn get_pr(&self, repo: &RepoRef, id: PrId) -> Result<PullRequest>;
    fn add_comment(&self, repo: &RepoRef, pr: PrId, comment: &AgentComment) -> Result<CommentId>;
    fn get_comments(&self, repo: &RepoRef, pr: PrId) -> Result<Vec<AgentComment>>;
    fn add_label(&self, repo: &RepoRef, pr: PrId, label: &str) -> Result<()>;
    fn get_labels(&self, repo: &RepoRef, pr: PrId) -> Result<Vec<String>>;
    fn list_prs(&self, repo: &RepoRef, filter: PrFilter) -> Result<Vec<PullRequest>>;
    fn get_pr_diff(&self, repo: &RepoRef, pr: PrId) -> Result<String>;
}
```

Reference implementation: GitHub (REST API via `octocrab` or raw `reqwest`). The adapter is selected by Git config:

```
[but-ai]
    forge = github       # github | gitlab | bitbucket | gitea
    forgeUrl = https://api.github.com
    forgeToken = <token>  # or read from environment
```

### 4.3 PR Comment Schema

Agent-to-agent messages are embedded in PR comments as fenced blocks:

```markdown
<!-- but-ai:message -->
```yaml
version: 1
from: overtone@velvet-frequencies
to: fundamental@velvet-frequencies
type: status_report
task: feat-auth/s03
status: completed
patch: |
  See attached INDEX.patch in commit abc1234
budget:
  used: 23400
  total: 50000
dependencies:
  - org/other-repo#42
  - org/this-repo#17
```
<!-- /but-ai:message -->
```

The schema supports the required message types:

| Type | Description |
|------|-------------|
| `task_assignment` | Assign work to an agent |
| `status_report` | Report task status (completed/blocked/failed) |
| `dependency_declaration` | Declare cross-repo dependency |
| `patch_handoff` | Reference a patch for review |
| `budget_report` | Token usage update |
| `review_request` | Request review from another agent |
| `review_response` | Review feedback (resonant/dissonant/beating) |

Messages are YAML inside HTML comments so they are invisible to human readers but parseable by agents. The `<!-- but-ai:message -->` markers ensure reliable extraction even from rich PR descriptions.

### 4.4 Cross-Repo Coordination

Cross-repo references use the universal format `forge://host/owner/repo#number`:

```
forge://github.com/org/frontend#42
forge://gitlab.example.com/group/backend#17
forge://gitea.local/team/shared-lib#5
```

This format is forge-agnostic and self-describing. The protocol portion identifies the forge type, the host identifies the instance, and the path identifies the specific PR.

Fundamental (the orchestrator agent) maintains a dependency graph as a YAML file on the agent memory branch:

```yaml
# refs/but-ai/memory/coordination/task-graph.yaml
tasks:
  feat-auth/s01:
    status: completed
    pr: forge://github.com/org/backend#42
    depends_on: []
  feat-auth/s02:
    status: in_progress
    pr: forge://github.com/org/frontend#43
    depends_on: [feat-auth/s01]
  feat-auth/s03:
    status: blocked
    pr: forge://gitlab.example.com/group/shared#17
    depends_on: [feat-auth/s01, feat-auth/s02]
```

### 4.5 Trade-offs

**Considered:** Using Git notes for inter-agent communication instead of PR comments.
**Rejected:** Git notes are poorly supported by forges (GitHub does not display them). PR comments are visible, auditable, and work everywhere.

**Considered:** A binary protocol for PR comment messages.
**Rejected:** YAML in HTML comments is human-debuggable. When things break (and they will), you need to be able to read the messages.

---

## 5. Agent Memory and Identity

### 5.1 Approach: Harmonic Resonance Memory

This is the core of our proposal. We store agent memory as **frequencies** — each memory entry is tagged with harmonic descriptors that enable resonance-based retrieval. Related memories amplify each other when retrieved together. Contradictory memories create dissonance, which is surfaced explicitly rather than hidden.

### 5.2 Storage

Memory is stored on a special Git branch: `refs/but-ai/memory/<agent-id>`.

Directory structure:

```
refs/but-ai/memory/resonance/
  identity.yaml           # Agent identity record
  frequencies/
    <hash>.yaml           # Individual memory entries
  index.yaml              # Frequency index for fast retrieval
  harmonics.yaml          # Precomputed harmonic relationships
```

Each memory entry:

```yaml
id: "freq-a4f2c1"
created: "2026-03-28T10:15:00Z"
ttl: 604800                    # 7 days, in seconds
frequency:
  fundamental: 440.0           # Primary relevance score (0-1000 Hz metaphor)
  key: "authentication"        # Semantic key
  octave: 3                    # Abstraction level (1=concrete, 5=architectural)
  harmonics:                   # Related semantic keys
    - "jwt"
    - "session-management"
    - "middleware"
  dissonances:                 # Contradictory semantic keys
    - "stateless"              # (if this memory is about stateful auth)
content: |
  The authentication module uses JWT with refresh tokens.
  Tokens are stored in HTTP-only cookies, not localStorage.
  The refresh endpoint is /api/auth/refresh.
source:
  commit: "abc1234"
  branch: "feat/auth"
  file: "src/auth/jwt.rs"
confidence: 0.92               # How certain the agent is about this memory
observations: 3                 # Number of times this pattern was confirmed
```

### 5.3 Retrieval: Resonance Scoring

When the agent encounters a task, Resonance formulates a **query frequency** from the task description:

1. Extract semantic keys from the task (e.g., "add OAuth support" -> keys: `oauth`, `authentication`, `authorization`)
2. For each key, find memory entries whose `frequency.key` or `frequency.harmonics` match
3. Score each match using harmonic distance:

```
score = base_relevance * harmonic_multiplier * recency_decay * confidence

where:
  base_relevance  = 1.0 if key matches fundamental, 0.5 if matches harmonic
  harmonic_multiplier = 1.0 / (1 + octave_distance)
  recency_decay   = exp(-age_seconds / ttl)
  confidence      = entry.confidence * (entry.observations / max_observations)
```

4. Entries with `dissonances` matching the query keys are flagged but not excluded — they are surfaced with a "dissonance warning" so the agent can reason about contradictions.

5. Results are returned sorted by score, with a configurable cutoff (default: top 5 entries).

### 5.4 Expiration

Memory TTL is enforced lazily and eagerly:

- **Lazy:** On retrieval, expired entries are excluded from results and marked for deletion.
- **Eager:** `but ai memory gc` walks the memory branch and removes expired entries. This can be run on a schedule or manually.

TTL can be extended by "re-observation" — if an agent encounters a pattern that matches an existing memory, the memory's TTL is reset and its `observations` count increments. Frequently-confirmed memories live longer. Rarely-confirmed memories fade.

### 5.5 Compaction Survival

When the LLM context window is compacted, the agent writes critical context to the memory branch before compaction occurs. The protocol:

1. Before compaction, the agent runs a `memory_checkpoint` step that extracts key facts from the current context and stores them as memory entries with `octave: 1` (most concrete) and high confidence.
2. After compaction, the agent runs a `memory_rehydration` step that retrieves the top N entries by recency and injects them into the fresh context.
3. Entries created during checkpoint are tagged `source: compaction` so the agent knows they are rehydrated memories, not original observations.

### 5.6 Long-Term Storage (Overtone Archive)

Long-term memory is stored on a shared branch: `refs/but-ai/archive`. This branch is not agent-specific — all agents can read and write. Entries in the archive have:

- Higher default TTL (30 days vs 7 days for agent-local memory)
- A `contributors` field listing which agents have confirmed the entry
- Cross-repository reachability via `git fetch` of the archive ref from other repos

The archive is indexed by a B-tree stored in `archive/index.yaml`, sorted by semantic key. Retrieval uses the same harmonic scoring as agent-local memory but with a lower base relevance (0.3 instead of 1.0) to prefer local memory over archived memory.

### 5.7 Identity

Each agent's identity is stored at `refs/but-ai/memory/<agent-id>/identity.yaml`:

```yaml
name: "overtone"
organization: "velvet-frequencies"
created: "2026-03-28T09:00:00Z"
capabilities:
  - patch_generation
  - code_architecture
  - dependency_analysis
authorization:
  branches:
    allow: ["feat/*", "fix/*", "overtone/*"]
    deny: ["main", "release/*"]
  repos:
    allow: ["org/backend", "org/frontend"]
    deny: []
  max_patch_lines: 1000
signing_key:
  fingerprint: "SHA256:abc123..."
  provider: "openwallet"
  created: "2026-03-28T09:00:00Z"
  expires: "2027-03-28T09:00:00Z"
```

The identity record is signed by the agent's OpenWallet key. Verification: given a signed commit, extract the signing key fingerprint, look up the identity record on the memory branch, verify the identity record's own signature, and check that the agent's authorization scope covers the target branch.

### 5.8 Trade-offs

**Considered:** Vector embeddings for memory retrieval (standard RAG).
**Rejected:** Vector embeddings require an embedding model, which adds latency, cost, and a dependency on a specific provider. Our harmonic scoring uses semantic keys extracted by the LLM during memory creation — the LLM does the "embedding" at write time, not at read time. This is cheaper and provider-agnostic.

**Considered:** Storing memory in Git notes.
**Rejected:** Git notes have a 1:1 relationship with commits and cannot be queried by content. A dedicated branch with YAML files is more flexible and indexable.

**Considered:** SQLite for the memory index.
**Rejected:** SQLite is not Git-native. The memory store must be fetchable, mergeable, and auditable using standard Git operations. YAML files on a branch satisfy all three.

---

## 6. Signed Commits via OpenWallet

### 6.1 Approach

Every commit produced by an agent is signed using an OpenWallet-managed key. The signing flow is handled exclusively by the Envelope agent, which is the only entity with access to the signing key.

### 6.2 Design

Signing follows the ADSR model (Attack, Decay, Sustain, Release):

**Attack (Key Provisioning):**
```
but ai identity create --name overtone --org velvet-frequencies
```
This generates a new OpenWallet keypair, stores the public key in the identity record on the memory branch, and registers the key with the OpenWallet credential store.

**Decay (Authorization Check):**
Before signing, Envelope verifies:
1. The signing agent's identity record exists and is valid
2. The target branch is within the agent's `authorization.branches.allow` list
3. The target repo is within the agent's `authorization.repos.allow` list
4. The patch size does not exceed `authorization.max_patch_lines`

If any check fails, the signing is refused with a structured error.

**Sustain (Active Use):**
The key is used for signing via OpenWallet's CLI or SDK:
```
openwallet sign --key <fingerprint> --input <commit-hash>
```

The resulting signature is embedded in the commit using Git's standard signing mechanism (`commit.gpgSign` equivalent).

**Release (Revocation):**
Key revocation is handled through two mechanisms:
- **Rotation:** `but ai identity rotate --name overtone` generates a new keypair, updates the identity record, and adds the old key to a `revoked_keys` list with reason `rotation` and a `valid_until` timestamp. Commits signed before the rotation remain valid.
- **Compromise:** `but ai identity revoke --name overtone --reason compromise` adds the key to the `revoked_keys` list with reason `compromise` and no `valid_until` timestamp. Commits signed with the compromised key are considered suspect. Verification tools flag them.

### 6.3 Authorization Model

Authorization is declarative and stored in the identity record. The model supports:

| Constraint | Example | Enforcement |
|------------|---------|-------------|
| Branch pattern | `feat/*` allowed, `main` denied | Checked by Envelope before signing |
| Repository scope | `org/backend` allowed, `org/secret` denied | Checked by Envelope before signing |
| Patch size limit | max 1000 lines | Checked by Envelope before signing |
| Time window | Valid 09:00-18:00 UTC | Checked by Envelope before signing |
| Dependency requirement | Must depend on an approved PR | Checked by Fundamental during coordination |

The authorization chain is: **Identity record (signed) -> Authorization scope -> Commit signature -> Verification**. Given a signed commit, a verifier can reconstruct the entire chain by reading the memory branch.

### 6.4 Git Config Keys

```
[but-ai.signing]
    provider = openwallet           # Signing provider (only openwallet for now)
    keyStore = ~/.openwallet/keys   # Path to key store
    autoRotateDays = 90             # Auto-rotate keys after N days
    requireAuthorization = true     # Enforce authorization checks (default: true)
```

### 6.5 Trade-offs

**Considered:** Supporting multiple signing providers (GPG, SSH, OpenWallet).
**Rejected:** The RFP mandates OpenWallet. Supporting others adds complexity with no benefit for this proposal.

**Considered:** Storing revocation lists on a central server.
**Rejected:** The RFP mandates no proprietary dependencies. Revocation lists are stored on the memory branch, fetchable via Git.

---

## 7. Token Budget

### 7.1 Budget Table

Model: Claude Opus 4 (200K context, ~$15/M input, ~$75/M output)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,200 | 0 | Once per session | Agent identity (400), tool descriptions (800), workspace state summary (1,200), memory schema (400), coordination protocol (400) |
| **Task ingestion** | 2,500 | 500 | Once per task | PR body (~1,500), branch metadata (~500), issue description (~500). Output: task summary. |
| **Planning** | 4,000 | 1,500 | Once per task | Task description + memory results (~2,500), current workspace state (~1,500). Output: phased plan with budget allocations. |
| **Tool call (per call)** | 1,800 | 600 | 5-8 per task | Tool result parsing (~1,200), context from previous calls (~600). Output: next action decision + parameters. |
| **Patch generation** | 6,000 | 4,000 | Once per task | Accumulated context (~3,000), file contents (~2,000), plan (~1,000). Output: full INDEX.patch content. |
| **Commit message** | 1,500 | 300 | Once per task | Patch summary + branch context. Output: COMMIT.msg. |
| **Memory retrieval** | 2,000 | 800 | 1-3 per task | Query formulation (~500), result parsing (~1,500). Output: structured memory summary. |
| **Coordination event** | 1,500 | 600 | 0-2 per task | PR comment parsing (~1,000), cross-repo context (~500). Output: structured response. |
| **TOTAL (typical task)** | **35,900** | **14,500** | -- | 200-line feature across 3 files, 2 cross-repo deps, 6 tool calls, 2 memory retrievals, 1 coordination event |

**Total: ~50,400 tokens per typical task.**

At Claude Opus 4 pricing: ~$0.54 input + ~$1.09 output = **~$1.63 per task**.

### 7.2 Budget Optimizations

1. **Lazy tool registration:** Tool descriptions are included in the system prompt only for tools the agent is likely to use. Resonance only needs 3 tools (~40 tokens) vs Overtone who needs 5 (~70 tokens). Savings: ~60 tokens per session for specialized agents.

2. **Incremental context:** After the first tool call, subsequent calls carry only the delta from the previous context, not the full accumulated state. Savings: ~30% reduction in per-call input tokens for sessions with >5 tool calls.

3. **Compressed memory injection:** Memory entries are summarized to key facts before injection into context. A 500-token memory entry becomes a ~80-token summary. Savings: ~840 tokens per retrieval cycle (assuming 2 retrievals of 3 entries each).

4. **Harmonic pre-filtering:** Memory retrieval uses keyword matching before LLM scoring, reducing the number of entries the LLM needs to evaluate. This is a zero-token optimization — it reduces latency, not token usage.

### 7.3 Budget Enforcement

The agent maintains a running token counter. After each LLM call, the counter is updated using the response's `usage` field. The enforcement thresholds:

| Threshold | Action |
|-----------|--------|
| 70% | Warning logged. Agent proceeds normally. |
| 85% | Fadeout mode. Complete current phase, skip optional phases. |
| 95% | Emergency stop. Produce partial patch immediately. |
| 100% | Hard stop. No more LLM calls. Submit whatever exists. |

---

## 8. Testing Strategy

### 8.1 Provider-Agnostic Testing

- **Mock provider:** A test provider that returns deterministic responses, registered as `but-ai-provider-mock` on PATH. Used for all integration tests.
- **Provider compliance suite:** A set of test scenarios that every provider must pass (tool calling, streaming, structured output). Run against each provider in CI with recorded responses (VCR-style).

### 8.2 Patch Workflow Validation

- **Round-trip tests:** Generate INDEX.patch from known file states, apply it, verify the result matches expected output. Tests cover: single-file patches, multi-file patches, patches with renames, patches with binary files (expected to fail gracefully).
- **Partial patch tests:** Simulate budget exhaustion at various phases and verify that partial patches are valid (apply cleanly, even if incomplete).

### 8.3 Cross-Repo Coordination Testing

- **Mock forge server:** A lightweight HTTP server that implements the `ForgeAdapter` interface for testing. Returns canned PR data, accepts comment posts, validates the YAML schema.
- **Multi-repo simulation:** Spin up 3 bare Git repos with a mock forge, run a coordination scenario, verify the dependency graph is correctly maintained.

### 8.4 Token Budget Testing

- **Budget enforcement tests:** Configure an agent with a known budget, feed it tasks of known token cost, verify that fadeout and emergency stop trigger at the correct thresholds.
- **Budget accuracy tests:** Compare estimated token costs (from the budget table) against actual usage from the mock provider. Flag estimates that are off by more than 20%.

### 8.5 Memory System Testing

- **Retrieval accuracy tests:** Store known memory entries, query with various semantic distances, verify that harmonic scoring produces the expected ranking.
- **Expiration tests:** Store entries with short TTL, advance time, verify they are excluded from retrieval and removed by GC.
- **Compaction survival tests:** Simulate context compaction by clearing the context and running rehydration, verify that critical memories are recovered.

---

## 9. Git Config Keys (Complete)

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.forge` | string | `github` | Forge type |
| `but-ai.forgeUrl` | string | (per forge) | Forge API URL |
| `but-ai.forgeToken` | string | (env) | Forge API token |
| `but-ai.agent.tokenBudget` | int | `50000` | Max tokens per task |
| `but-ai.agent.fadeoutThreshold` | float | `0.85` | Budget fraction triggering fadeout |
| `but-ai.agent.maxReviewRounds` | int | `3` | Max review iterations before escalation |
| `but-ai.memory.branch` | string | `refs/but-ai/memory` | Memory branch prefix |
| `but-ai.memory.archiveBranch` | string | `refs/but-ai/archive` | Shared archive branch |
| `but-ai.memory.defaultTtl` | int | `604800` | Default memory TTL in seconds (7 days) |
| `but-ai.memory.maxRetrievalDepth` | int | `5` | Max entries per retrieval |
| `but-ai.memory.archiveTtl` | int | `2592000` | Archive TTL in seconds (30 days) |
| `but-ai.signing.provider` | string | `openwallet` | Signing provider |
| `but-ai.signing.keyStore` | string | `~/.openwallet/keys` | Key store path |
| `but-ai.signing.autoRotateDays` | int | `90` | Key rotation interval |
| `but-ai.signing.requireAuthorization` | bool | `true` | Enforce authorization checks |
| `but-ai.provider.pluginPrefix` | string | `but-ai-provider-` | PATH prefix for provider plugins |

---

## 10. Migration Path

The migration from the current MCP server to `but-ai` is designed for zero downtime:

1. **Phase 1 (coexistence):** `but-ai mcp` runs alongside the existing MCP server. The old `gitbutler_update_branches` tool is available in both. Clients can connect to either.
2. **Phase 2 (redirect):** The existing MCP server is modified (one line) to forward requests to `but-ai mcp` via stdio. Clients see no change.
3. **Phase 3 (removal):** The old MCP server code in `crates/but/src/command/legacy/mcp/mod.rs` is removed. `but-ai mcp` is the sole MCP server.

Each phase can be deployed independently. Rollback at any phase is trivial — remove `but-ai` from PATH and the old server resumes.

---

*Key signature: A minor*
*Tempo: 108 BPM (andante)*
*Mill floor: Attic*
