# CropOS — Proposal for `but-ai` Plugin

*Soil composition: 30% durability / 30% velocity / 40% correctness*
*"Plant fast. Harvest clean. Compost everything."*

---

## 1. Plugin Architecture

### 1.1 Approach

`but-ai` is a single Rust binary on PATH, discovered by `find_external_subcommand()` in `crates/but/src/alias.rs`. CropOS ships lean — one binary, no runtime dependencies beyond `but-llm` and `gix`.

We treat the plugin like a farm equipment controller: it does one thing (AI agent operations), it talks to the equipment it needs (Git workspace, LLM providers, forge APIs), and it stays out of the way of everything else.

### 1.2 Design

```
but-ai (binary, ~4MB target)
  |-- cmd/        # CLI argument parsing (clap)
  |-- mcp/        # MCP server (rmcp ServerHandler)
  |-- agent/      # Agent pipeline: Harvester -> Tiller -> Composter
  |-- soil/       # Soil-layer memory system
  |-- forge/      # Forge adapters
  |-- sign/       # OpenWallet signing
```

Subcommands:

| Command | Description |
|---------|-------------|
| `but ai run <task>` | Execute task through the full pipeline |
| `but ai mcp` | Start MCP server on stdio |
| `but ai plan <task>` | Decompose task into subtasks (Harvester only, dry run) |
| `but ai soil query <text>` | Query soil-layer memory |
| `but ai soil store <text> --layer <name>` | Store memory at a specific soil layer |
| `but ai soil compost` | Decompose fresh memories into primitives |
| `but ai soil gc` | Remove expired entries |
| `but ai status` | Agent status, budget, yield metrics |
| `but ai identity` | Create, show, rotate agent identity |

Environment variables:

| Variable | CropOS Usage |
|----------|-------------|
| `BUT_WORKSPACE_DIR` | Field root — the Git workspace |
| `BUT_OUTPUT_FORMAT` | Output mode (`human`, `json`, `shell`) |
| `BUT_JSON` | JSON shorthand |

### 1.3 WASI Degradation

Under WASI:

- `but ai` subcommand unavailable (no PATH discovery)
- MCP server can run standalone via direct invocation
- Memory queries work (Git reads via `gix` are WASI-compatible)
- Patch generation and signing are disabled
- The degradation is explicit: structured error with `WASI_UNSUPPORTED` code and a capability manifest

CropOS's position: WASI support is a v2 concern. The current priority is shipping a working native binary. The WASI hooks are in place but the investment is minimal.

### 1.4 Trade-offs

**Considered:** A multi-crate workspace within `crates/`.
**Rejected:** CropOS has three people. Managing a multi-crate workspace with its own dependency graph would consume 30% of our development time. A single binary with internal modules ships faster and is easier to debug.

**Considered:** Python or Go for faster prototyping.
**Rejected:** The `but` ecosystem is Rust. A Python plugin would require a runtime and create a deployment dependency. Consistency over convenience.

---

## 2. Provider-Agnostic AI Interface

### 2.1 Approach

Use `but-llm` as-is. All four providers supported. New providers via PATH-based plugin executables.

CropOS treats LLM providers like farm equipment protocols: they all do roughly the same thing, they all speak slightly different languages, and the API layer's job is to abstract away the differences.

### 2.2 Design

Provider routing:

1. Read `gitbutler.aiModelProvider` from Git config
2. If built-in (`openai`, `anthropic`, `ollama`, `lmstudio`) -> `but-llm` directly
3. If not built-in -> find `but-ai-provider-<name>` on PATH
4. Plugin implements JSON-RPC over stdio:
   - `capabilities` -> returns supported features (tool_calling, streaming, structured_output)
   - `chat` -> single-turn completion
   - `tool_loop` -> multi-turn tool calling loop
   - `stream` -> streaming response (optional)

The interface is intentionally small. CropOS does not believe in comprehensive plugin interfaces — they are specification debt. A small interface ships faster, is easier to implement, and is easier to test.

### 2.3 MCP Surface

All 10 workspace tools registered via `tool_router`. Backward-compatible `gitbutler_update_branches` shim included. The MCP server's info:

```rust
ServerInfo {
    name: "GitButler MCP Server",
    version: "2.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

### 2.4 Trade-offs

**Considered:** Supporting provider hot-swapping (change provider mid-session).
**Rejected:** Adds complexity for a rare use case. Provider is set at session start and fixed for the session duration. Restart to change providers.

---

## 3. The But Agent

### 3.1 Approach

The agent pipeline is simple: Harvester decomposes, Tiller patches, Composter reviews and signs. Three stages. No orchestration layer — the pipeline is hardcoded and deterministic.

We call this the **three-field rotation**: decompose, grow, harvest. Each stage feeds the next.

### 3.2 Design

```
1. DECOMPOSE  (Harvester)  — Read task, produce subtask list, create PRs
2. GROW       (Tiller)     — For each subtask: produce INDEX.patch + COMMIT.msg
3. HARVEST    (Composter)  — Review each patch, extract knowledge, sign if approved
```

Budget allocation for a total budget B:

| Stage | Budget % | Notes |
|-------|----------|-------|
| Decompose | 12% | Fast. Harvester is lean. |
| Grow | 55% | The main work. Tiller gets the lion's share. |
| Harvest | 28% | Review + memory + signing. Composter is thorough. |
| Margin | 5% | Emergency buffer for revision cycles. |

The 5% margin is CropOS's "seed bank" — held in reserve for unexpected rework. In agricultural terms, you always save some seed. You never plant everything.

### 3.3 Branch Naming

CropOS extends the `s01.s04` convention minimally:

```
<agent>/<task>.<dep-chain>
```

Example: `tiller/auth.s01.s03` — agent Tiller, task s03 (depends on s01), in the "auth" task group.

CropOS intentionally does not add more encoding to branch names. Branch names are for humans. Structured metadata belongs in the memory system.

### 3.4 Workspace Tools

All 10 tools registered via `Toolset::register_tool()`. Tools are exposed per-agent:

| Agent | Tools |
|-------|-------|
| Harvester | GetProjectStatus, GetBranchChanges, CreateBranch, GetCommitDetails |
| Tiller | GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit, CreateBranch, MoveFileChanges, SplitCommit |
| Composter | GetBranchChanges, GetCommitDetails, GetProjectStatus, Commit |

### 3.5 Patch Production

Tiller produces patches in a two-phase process:

1. **Survey phase:** Read target files, understand surrounding context, identify conventions. 40% of Tiller's budget.
2. **Generation phase:** Produce INDEX.patch as a unified diff. Validate by mentally applying the patch to the surveyed file state. 60% of Tiller's budget.

If the patch exceeds 500 lines, Tiller uses `SplitCommit` to break it into smaller logical units. Each unit gets its own COMMIT.msg.

### 3.6 Budget Fadeout

| Usage | Action |
|-------|--------|
| < 80% | Normal operation |
| 80% | Warning. Harvester logs yield-per-acre metrics. |
| 90% | Harvest early. Complete current subtask, skip remaining subtasks. |
| 95% | Emergency. Produce partial patch from whatever Tiller has. Mark `X-Partial: true`. |
| 100% | Stop. Submit whatever exists. |

### 3.7 Trade-offs

**Considered:** Four agents (separate reviewer and signer).
**Rejected:** Three people, three agents. Splitting review and signing creates a coordination boundary that costs tokens. Composter combines them with an internal authorization check.

**Considered:** Dynamic pipeline (let the LLM decide the stage order).
**Rejected:** Deterministic pipelines are debuggable. Dynamic pipelines are not. CropOS ships what it can debug.

---

## 4. Polyrepo PR-Based Agent Coordination

### 4.1 Approach

PRs as communication channels. PR comments as structured messages. Git and forge API only — no external dependencies.

CropOS treats cross-repo coordination like cross-field irrigation: water (context) flows between fields through channels (PRs), controlled by gates (the comment schema).

### 4.2 Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &str, title: &str, body: &str, head: &str, base: &str) -> Result<u64>;
    fn comment(&self, repo: &str, pr_id: u64, body: &str) -> Result<u64>;
    fn get_comments(&self, repo: &str, pr_id: u64) -> Result<Vec<Comment>>;
    fn add_labels(&self, repo: &str, pr_id: u64, labels: &[String]) -> Result<()>;
    fn get_pr(&self, repo: &str, pr_id: u64) -> Result<PullRequest>;
    fn list_prs(&self, repo: &str, labels: &[String]) -> Result<Vec<PullRequest>>;
}
```

Six methods. Intentionally minimal. CropOS does not implement capabilities it does not need.

Reference implementation: GitHub REST API. Forge selection via Git config:

```ini
[but-ai]
    forge = github
    forgeApiUrl = https://api.github.com
    forgeToken = ${BUT_AI_FORGE_TOKEN}
```

### 4.3 PR Comment Schema

```markdown
<!-- crop:v1 -->
```yaml
from: tiller@cropos
type: patch_ready
task: auth/s02
data:
  commit: abc1234
  lines: 142
  files: 3
budget:
  spent: 24000
  remaining: 26000
  yield_per_acre: 38
deps:
  - github.com/org/shared#15
```
<!-- /crop:v1 -->
```

Message types:

| Type | Description |
|------|-------------|
| `task_assign` | Harvester assigns work |
| `patch_ready` | Tiller completed a patch |
| `review_result` | Composter reviewed (approve/reject + fertility score) |
| `dep_declare` | Cross-repo dependency |
| `budget_alert` | Budget threshold crossed |
| `status` | General progress update |

### 4.4 Cross-Repo References

Format: `<forge-host>/<owner>/<repo>#<number>`

Examples:
```
github.com/org/backend#42
gitlab.example.com/group/frontend#17
gitea.local/team/lib#5
```

CropOS uses a flat format (no `forge://` URI scheme) because it is shorter, familiar (matches how developers already write cross-repo references in issues), and parseable with a simple regex.

Harvester tracks dependencies in a YAML file on the memory branch:

```yaml
# refs/but-ai/soil/coordination/deps.yaml
tasks:
  auth/s01: { pr: "github.com/org/backend#42", status: done, deps: [] }
  auth/s02: { pr: "github.com/org/frontend#43", status: wip, deps: [auth/s01] }
  auth/s03: { pr: "gitlab.example.com/group/shared#17", status: blocked, deps: [auth/s01, auth/s02] }
```

### 4.5 Trade-offs

**Considered:** A full forge abstraction with webhooks and status checks.
**Rejected:** CropOS does not have the person-hours to implement and maintain a full forge abstraction. The minimal interface covers coordination. Webhooks and status checks are v2.

**Considered:** ActivityPub for cross-forge communication.
**Rejected:** ActivityPub is a protocol for social networks, not for code coordination. PR comments work.

---

## 5. Agent Memory and Identity

### 5.1 Approach: Soil-Layer Memory

Memory decomposes over time. Fresh observations are stored in the topsoil — easily accessible but volatile. As observations are confirmed and generalized, they decompose downward into deeper layers where they become reusable primitives. Old, unconfirmed memories decay and are eventually removed, like organic matter that was never incorporated into the soil.

### 5.2 Storage

Memory branch: `refs/but-ai/soil/<agent-id>/`

```
refs/but-ai/soil/composter/
  identity.yaml
  layers/
    topsoil/           # Fresh observations, high specificity, short TTL
      <hash>.yaml
    subsoil/           # Confirmed patterns, moderate generality, medium TTL
      <hash>.yaml
    bedrock/           # Fundamental truths, high generality, long TTL
      <hash>.yaml
  compost/             # Entries actively being decomposed
    <hash>.yaml
  index.yaml
```

Each memory entry:

```yaml
id: "soil-3f1a"
created: "2026-03-28T10:15:00Z"
layer: topsoil
ttl: 259200                    # 3 days (topsoil is volatile)
nutrients:                     # What this memory provides
  - type: "pattern"
    key: "authentication"
    strength: 0.7              # How much of this nutrient is available
  - type: "convention"
    key: "error-handling"
    strength: 0.4
content: |
  The auth module uses JWT with 15-min access tokens.
  Refresh tokens last 7 days. Stored in HTTP-only cookies.
source:
  commit: "abc1234"
  file: "src/auth/jwt.rs"
observations: 1                # Confirmed once
decomposition_stage: "fresh"   # fresh -> composting -> decomposed
```

### 5.3 Decomposition Process

The composting process runs on `but ai soil compost`:

1. **Identify candidates:** Entries in `topsoil/` with `observations >= 2` and age > 50% of TTL.
2. **Extract nutrients:** For each candidate, identify the reusable primitives (patterns, conventions, architectural decisions).
3. **Create subsoil entries:** Store extracted primitives in `subsoil/` with higher generality and longer TTL (14 days).
4. **Promote proven subsoil to bedrock:** Entries in `subsoil/` with `observations >= 5` are promoted to `bedrock/` with 90-day TTL.
5. **Decay unconfirmed entries:** Entries that reach their TTL without being observed again are removed.

The composting cycle:

```
topsoil (3-day TTL, specific)
  |  2+ observations
  v
subsoil (14-day TTL, moderate generality)
  |  5+ observations
  v
bedrock (90-day TTL, fundamental truths)
```

### 5.4 Retrieval

When an agent queries soil memory:

1. Search all three layers for entries whose `nutrients[].key` matches the query
2. Score each match:

```
score = nutrient_strength * layer_weight * freshness * observations_bonus

where:
  nutrient_strength = entry.nutrients[matched].strength
  layer_weight      = topsoil: 0.8, subsoil: 1.0, bedrock: 0.6
  freshness         = 1.0 - (age / ttl)
  observations_bonus = min(entry.observations / 5, 1.5)
```

The scoring deliberately weights `subsoil` highest. Topsoil entries are too specific (not yet proven). Bedrock entries are too general (already internalized). Subsoil entries are the sweet spot: confirmed, specific enough to be useful, general enough to be transferable.

3. Return top 5 entries, sorted by score.

### 5.5 Expiration and Decay

- **Topsoil:** 3-day default TTL. Unobserved entries decay after 3 days. This is aggressive by design — fresh observations that are not confirmed are noise.
- **Subsoil:** 14-day TTL. Extended by observation (each observation resets TTL).
- **Bedrock:** 90-day TTL. Only decays if actively invalidated (e.g., the code it describes is deleted).
- **`but ai soil gc`:** Removes expired entries from all layers. Can be run manually or on a schedule.

### 5.6 Compaction Survival

Before context compaction:

1. Composter runs a `soil_checkpoint` that extracts critical facts and stores them in `topsoil/` with `decomposition_stage: "checkpoint"` and maximum TTL (30 days instead of 3 days for checkpoints).
2. After compaction, the agent queries soil at all layers, which returns checkpoint entries (high freshness, maximum strength).
3. Checkpoint entries are marked so the agent knows they are rehydrated memories.

### 5.7 Long-Term Storage (The Compost Heap)

Shared long-term memory on `refs/but-ai/soil/shared/`:

```
refs/but-ai/soil/shared/
  bedrock/         # Shared fundamental truths (contributions from all agents)
  index.yaml
```

Only bedrock-level entries can be promoted to shared storage. Requirements: confidence > 0.85, observations > 5, contributed by at least 2 different agents (for cross-validation). This is intentionally high-bar — the compost heap is the most valuable knowledge resource and contamination would be catastrophic.

Cross-repo: `git fetch <remote> refs/but-ai/soil/shared:refs/but-ai/soil/shared-<remote>`.

### 5.8 Identity

```yaml
# refs/but-ai/soil/tiller/identity.yaml
name: "tiller"
org: "cropos"
role: "patch_generator"
created: "2026-03-28T09:00:00Z"
capabilities: [patch_generation, systems_architecture, convention_matching]
auth:
  branches: { allow: ["feat/*", "fix/*"], deny: ["main"] }
  repos: { allow: ["*"], deny: [] }
  limits: { max_lines: 500, max_files: 15 }
key:
  fingerprint: "SHA256:abc123..."
  provider: "openwallet"
  issued: "2026-03-28T09:00:00Z"
  expires: "2027-03-28T09:00:00Z"
```

### 5.9 Trade-offs

**Considered:** Vector embeddings for memory retrieval.
**Rejected:** Too expensive for a three-person startup. Nutrient-key matching with layer scoring achieves 85% of the retrieval quality at 0% of the embedding cost.

**Considered:** A single flat memory store with tags.
**Rejected:** Flat memory does not decompose. Without the composting process, old observations pile up indefinitely, creating noise that degrades retrieval quality over time. The three-layer system with active decomposition keeps the memory store healthy.

**Considered:** SQLite for the memory index.
**Rejected:** Not Git-native. Must be fetchable via Git for cross-repo sharing.

---

## 6. Signed Commits via OpenWallet

### 6.1 Approach

OpenWallet signing is handled by Composter as part of the review-and-sign pipeline. Authorization is checked before signing. No separate signing agent — CropOS cannot afford the coordination overhead.

### 6.2 Design

**Key provisioning:**
```bash
but ai identity create --name tiller --org cropos
```
Generates OpenWallet keypair. Public key stored in identity record on soil branch.

**Signing flow:**
1. Composter approves patch (review complete, fertility score >= 0.5)
2. Composter checks authorization: branch in scope, repo in scope, patch within limits
3. Composter signs via OpenWallet: `openwallet sign --key <fingerprint> --input <commit-data>`
4. Signed commit produced via orchestrator

**Rotation:** `but ai identity rotate --name tiller` — new keypair, old key archived with `reason: rotation`, old commits remain valid.

**Revocation:** `but ai identity revoke --name tiller --reason compromise` — key archived with `reason: compromise`, commits flagged as suspect.

### 6.3 Authorization Model

| Constraint | Config Path | Enforcement |
|------------|-------------|-------------|
| Branch scope | `auth.branches.allow/deny` | Pattern match before signing |
| Repo scope | `auth.repos.allow/deny` | Pattern match before signing |
| Patch size | `auth.limits.max_lines` | Line count check before signing |
| File count | `auth.limits.max_files` | File count check before signing |

### 6.4 Git Config Keys

```ini
[but-ai.signing]
    provider = openwallet
    keyStore = ~/.openwallet/keys
    rotationDays = 90
```

### 6.5 Trade-offs

**Considered:** Hardware security module (HSM) for key storage.
**Rejected:** CropOS agents run on commodity hardware. HSM is a v2 concern for enterprise deployments.

---

## 7. Token Budget

### 7.1 Budget Table

Model: Claude Opus 4 (200K context)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 2,800 | 0 | Once per session | Agent identity (200), tool descriptions (700), workspace state (1,200), soil memory schema (400), coordination protocol (300) |
| **Task ingestion** | 2,200 | 400 | Once per task | PR body (1,200), branch metadata (500), issue description (500). |
| **Planning (Harvester)** | 3,000 | 1,200 | Once per task | Task + workspace state (2,000), memory results (1,000). Output: subtask YAML. |
| **Tool call (per call)** | 1,500 | 500 | 5-7 per task | Tool result (1,000), prior context (500). |
| **Patch generation (Tiller)** | 8,000 | 6,000 | 1-2 per task | File contents (4,000), subtask spec (1,000), context (2,000), memory (1,000). Output: INDEX.patch. |
| **Commit message** | 1,000 | 200 | 1-2 per task | Patch summary. Output: COMMIT.msg. |
| **Review (Composter)** | 4,500 | 2,000 | 1-2 per task | Diff (2,500), context (1,000), style references (1,000). Output: review + memory entries. |
| **Memory retrieval** | 1,500 | 500 | 1-2 per task | Query (300), index scan (400), entry evaluation (800). Output: ranked results. |
| **Coordination (Harvester)** | 1,200 | 500 | 0-2 per task | PR comments (800), dep graph (400). Output: structured message. |
| **Signing (Composter)** | 800 | 200 | 1-2 per task | Authorization check. Output: signing decision. |
| **TOTAL (typical task)** | **33,500** | **14,500** | -- | 200-line feature, 3 files, 1 subtask, 2 cross-repo deps, 6 tool calls, 1 memory retrieval, 1 review, 1 coordination event |

**Total: ~48,000 tokens per typical task.**

At Claude Opus 4 pricing: ~$0.50 input + ~$1.09 output = **~$1.59 per task.**

CropOS's budget is the leanest of any serious proposal because the three-agent pipeline eliminates inter-agent coordination overhead. There is no orchestrator-to-reviewer handoff, no reviewer-to-signer handoff. The pipeline is a straight line.

### 7.2 Optimizations

1. **Per-agent tool filtering:** Each agent only sees its tools in the system prompt. Savings: ~100 tokens.
2. **Soil-layer pre-filtering:** Nutrient-key matching before LLM scoring reduces memory evaluation cost. Savings: ~400 tokens per retrieval.
3. **Combined review + signing:** Eliminates the review-to-signing handoff. Savings: ~1,500 tokens per task.
4. **Aggressive topsoil TTL:** 3-day TTL keeps the memory store small, reducing index scan cost. Savings: ~200 tokens per retrieval as the store grows.

### 7.3 Yield Metrics

CropOS tracks "yield per acre" (tokens per line of code produced) as its primary efficiency metric:

| Task Complexity | Target Yield | Acceptable Yield |
|----------------|-------------|-----------------|
| Simple (< 50 lines) | < 30 tokens/line | < 50 tokens/line |
| Medium (50-200 lines) | < 45 tokens/line | < 70 tokens/line |
| Complex (> 200 lines) | < 60 tokens/line | < 100 tokens/line |

A typical 200-line task at ~48,000 tokens gives a yield of 240 tokens per line — which is above the target because the denominator includes coordination and memory overhead. The *patch generation* yield (Tiller's budget / lines produced) is ~70 tokens per line, within the medium target.

---

## 8. Testing Strategy

### 8.1 Provider Testing

- **Mock provider:** `but-ai-provider-mock` binary on PATH. Returns canned responses keyed by input hash. All integration tests use this.
- **Conformance tests:** A test suite that validates provider plugins implement the required JSON-RPC methods. Run in CI.

### 8.2 Patch Workflow

- **Round-trip tests:** Feed task into pipeline, collect INDEX.patch, apply to test repo, verify output. Cases: single-file, multi-file, file creation, file deletion.
- **Partial patch tests:** Inject budget exhaustion at each pipeline stage. Verify partial patches are valid diffs.
- **Split tests:** Feed tasks that produce > 500-line patches and verify `SplitCommit` produces valid sub-patches.

### 8.3 Cross-Repo Coordination

- **Mock forge:** Lightweight HTTP server implementing `ForgeAdapter`. Returns canned data.
- **End-to-end:** Two bare repos, one mock forge, one cross-repo task. Verify dependency graph, PR comments, and status tracking.

### 8.4 Token Budget

- **Budget enforcement tests:** Known budget, known task cost, verify fadeout triggers at correct thresholds.
- **Yield tracking tests:** Run 20 test tasks, compute yield per acre, verify within acceptable ranges.

### 8.5 Memory System

- **Layer tests:** Store in topsoil, trigger composting, verify promotion to subsoil and bedrock.
- **Decay tests:** Store in topsoil, advance time past TTL, verify removal by GC.
- **Retrieval tests:** Store across all three layers, query, verify scoring weights subsoil highest.
- **Compaction tests:** Checkpoint, clear context, rehydrate, verify recovery.

---

## 9. Git Config Keys (Complete)

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.forge` | string | `github` | Forge type |
| `but-ai.forgeApiUrl` | string | per forge | Forge API URL |
| `but-ai.forgeToken` | string | env | Forge token |
| `but-ai.agent.tokenBudget` | int | `48000` | Max tokens per task |
| `but-ai.agent.marginFraction` | float | `0.05` | Emergency budget reserve |
| `but-ai.agent.maxRevisionCycles` | int | `2` | Max Tiller-Composter revision rounds |
| `but-ai.soil.branchPrefix` | string | `refs/but-ai/soil` | Soil branch prefix |
| `but-ai.soil.sharedBranch` | string | `refs/but-ai/soil/shared` | Shared compost heap |
| `but-ai.soil.topsoilTtl` | int | `259200` | Topsoil TTL (3 days) |
| `but-ai.soil.subsoilTtl` | int | `1209600` | Subsoil TTL (14 days) |
| `but-ai.soil.bedrockTtl` | int | `7776000` | Bedrock TTL (90 days) |
| `but-ai.soil.compostThreshold` | int | `2` | Observations needed for composting |
| `but-ai.soil.bedrockThreshold` | int | `5` | Observations needed for bedrock promotion |
| `but-ai.soil.maxRetrieval` | int | `5` | Max entries per query |
| `but-ai.signing.provider` | string | `openwallet` | Signing provider |
| `but-ai.signing.keyStore` | string | `~/.openwallet/keys` | Key store path |
| `but-ai.signing.rotationDays` | int | `90` | Key rotation interval |

---

## 10. Migration Path

CropOS is blunt about migration: we are a startup, not a migration consultancy.

1. **Install:** Put `but-ai` on PATH.
2. **Configure:** Set `gitbutler.aiModelProvider` and `but-ai.*` config keys.
3. **Run:** `but ai mcp` is a drop-in replacement for the legacy MCP server.
4. **Remove:** Delete the legacy MCP code when you are ready.

The `gitbutler_update_branches` shim is included for backward compatibility. It translates the old request format into calls against the new tool surface.

Zero downtime: old and new MCP servers can run in parallel. No database migrations, no state transfers. The soil memory system starts empty and populates organically.

---

*Soil composition: 30% durability / 30% velocity / 40% correctness*
*Growing season: Year 2*
*Field status: Mid-season growth*
