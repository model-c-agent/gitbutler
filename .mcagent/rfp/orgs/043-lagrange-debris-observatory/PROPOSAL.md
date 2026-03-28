# The Lagrange Debris Observatory — Proposal for `but-ai` Plugin

*Observation: Analytical + Radar*
*Confidence: 85% (based on design + pilot deployment data)*
*Independent observations: 3 (design review, pilot data, external consultation)*
*Epoch: 2026-03-28T00:00:00Z*

---

## 1. Plugin Architecture

### 1.1 Approach

`but-ai` is a Rust binary on PATH, discovered by `find_external_subcommand()` in `crates/but/src/alias.rs`. LDO treats the plugin as a scientific instrument: it must be calibrated, validated, and its outputs must include uncertainty quantification.

### 1.2 Design

```
but-ai (binary)
  |-- observe/       # CLI layer: argument parsing, output formatting
  |-- mcp/           # MCP layer: rmcp ServerHandler, tool registration
  |-- pipeline/      # Agent pipeline: Cataloger -> Aperture -> Tracker -> Correlator -> Validator -> Publisher
  |-- rcs-memory/    # Radar cross-section memory system
  |-- forge/         # Forge adapter layer
  |-- calibration/   # OpenWallet signing and validation
  |-- monitor/       # Sentinel-Ops monitoring
```

Subcommands:

| Command | Description |
|---------|-------------|
| `but ai observe <task>` | Execute a full observation campaign (task through pipeline) |
| `but ai plan <task>` | Produce campaign plan without execution (Cataloger only) |
| `but ai mcp` | Start MCP server on stdio |
| `but ai detect query <text>` | Query RCS memory |
| `but ai detect store <text>` | Store observation in RCS memory |
| `but ai detect gc` | Remove expired detections |
| `but ai detect survey` | Run deep survey mode (lower detection threshold) |
| `but ai status` | Pipeline status, budget, agent health |
| `but ai identity create` | Create agent identity with OpenWallet key |
| `but ai identity verify <commit>` | Verify signed commit (full validation chain) |
| `but ai identity rotate` | Rotate agent signing key |
| `but ai calibrate` | Run calibration checks against known-good test cases |

Environment variables:

| Variable | LDO Usage |
|----------|-----------|
| `BUT_WORKSPACE_DIR` | Observatory pointing direction — the Git workspace |
| `BUT_OUTPUT_FORMAT` | Data product format (`human`, `json`, `shell`) |
| `BUT_JSON` | JSON flag |

### 1.3 WASI Degradation

Under WASI:

- `but ai` unavailable as subcommand (no PATH discovery)
- MCP server operational as standalone module
- Memory queries operational (Git reads via `gix`)
- Patch generation and signing non-operational
- Monitoring (Sentinel-Ops) operational in read-only mode
- Degradation reported with `WASI_LIMITED` code and capability manifest

LDO's position: WASI support is important for long-term reproducibility (WASI binaries are deterministic), but full pipeline operation requires filesystem access that WASI restricts. The current plan is observation-only mode under WASI.

### 1.4 Trade-offs

**Considered:** Multi-binary architecture (one binary per agent).
**Rejected:** Seven binaries would create deployment complexity. A single binary with a pipeline architecture mirrors LDO's data processing pipeline (single codebase, multiple processing stages).

**Considered:** Python for rapid prototyping with a Rust rewrite later.
**Rejected:** The `but` ecosystem is Rust. A Python prototype would never be rewritten — "temporary" prototypes in scientific computing become permanent infrastructure.

---

## 2. Provider-Agnostic AI Interface

### 2.1 Approach

Use `but-llm` without modification. All four providers supported. New providers via PATH-based plugins. LDO does not modify upstream dependencies — this is a consortium rule.

### 2.2 Design

Provider routing (three-level):

1. Built-in providers via `but-llm` (OpenAI, Anthropic, Ollama, LMStudio)
2. External providers via `but-ai-provider-<name>` on PATH (JSON-RPC over stdio)
3. Fallback: structured error with installation instructions

Provider plugin interface:

```json
{
  "required_methods": ["initialize", "capabilities", "tool_calling_loop", "response"],
  "optional_methods": ["stream_response", "structured_output"],
  "capabilities_response": {
    "tool_calling": true,
    "streaming": true,
    "structured_output": true,
    "max_context": 200000,
    "supports_system_message": true
  }
}
```

LDO adds one non-standard field to the capabilities response: `deterministic_mode`. If a provider supports deterministic mode (fixed temperature, fixed seed), the pipeline uses it for reproducibility. This is critical for LDO's "given the same inputs, produce the same patch" requirement.

### 2.3 MCP Compatibility

Drop-in replacement for the legacy MCP server. All 10 workspace tools registered. Backward-compatible `gitbutler_update_branches` shim. The server reports:

```rust
ServerInfo {
    name: "GitButler MCP Server",
    version: "2.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

### 2.4 Trade-offs

**Considered:** Supporting multiple simultaneous providers for cross-validation (run the same query on two providers, compare outputs).
**Rejected for v1:** Doubles token cost. Interesting for scientific rigor but not justified until the pipeline is proven on a single provider. Noted as a v2 enhancement.

**Considered:** Embedding model support for memory retrieval.
**Rejected:** Adds a provider dependency. LDO's RCS-based memory scoring is provider-independent.

---

## 3. The But Agent

### 3.1 Approach

The agent operates as a scientific observation pipeline. Tasks are "observation campaigns." Subtasks are "observation sessions." Patches are "data products." The pipeline is seven stages, reflecting LDO's commitment to validation at every step.

### 3.2 Design: The Observation Pipeline

```
1. CAMPAIGN PLANNING  (Cataloger)    — Decompose task, assign catalog number
2. DETECTION          (Aperture)     — Memory retrieval, pattern detection
3. TRACKING           (Tracker)      — Patch generation (iterative)
4. CORRELATION        (Correlator)   — Multi-source review
5. VALIDATION         (Validator)    — Reproducibility check + signing
6. PUBLICATION        (Publisher)    — Cross-repo coordination
7. MONITORING         (Sentinel-Ops) — Continuous pipeline health
```

Budget allocation:

| Stage | Budget % | Purpose |
|-------|----------|---------|
| Campaign Planning | 8% | Task decomposition and cataloging |
| Detection | 10% | Memory retrieval and pattern detection |
| Tracking | 32% | Patch generation (iterative, 2-3 passes) |
| Correlation | 16% | Multi-source review |
| Validation | 5% | Signing and reproducibility check |
| Publication | 9% | Cross-repo coordination |
| Monitoring | 3% | Pipeline health checks |
| Reserve | 7% | For refinement cycles and re-detections |
| Calibration | 10% | Overhead for confidence metrics, error bars, reproducibility |

The 10% calibration overhead is LDO-specific: every stage spends tokens quantifying its own confidence. This is expensive but produces outputs that include uncertainty — essential for LDO's scientific credibility.

### 3.3 Branch Naming

LDO uses a catalog-based naming convention:

```
<agent>/<catalog-id>.<session-number>.<dependency>
```

Example: `tracker/LDO-2026-0042.s02.s01` — agent Tracker, catalog entry LDO-2026-0042, session 02 depending on session 01.

The catalog ID is persistent across sessions and repositories. It is the primary identifier for tracing a task from inception to completion.

### 3.4 Workspace Tools

Tools assigned by pipeline stage:

| Agent | Tools | Pipeline Stage |
|-------|-------|---------------|
| Cataloger | GetProjectStatus, GetBranchChanges, CreateBranch, GetCommitDetails | Planning |
| Aperture | GetProjectStatus, GetCommitDetails, GetBranchChanges | Detection |
| Tracker | GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit, CreateBranch, MoveFileChanges | Tracking |
| Correlator | GetBranchChanges, GetCommitDetails, GetProjectStatus | Correlation |
| Validator | GetCommitDetails, GetBranchChanges, Commit | Validation |
| Publisher | GetProjectStatus, GetBranchChanges, CreateBranch | Publication |
| Sentinel-Ops | GetProjectStatus, GetBranchChanges | Monitoring |

### 3.5 Confidence Metrics

Every agent output includes a confidence section:

```json
{
  "confidence": {
    "overall": 0.87,
    "components": {
      "task_clarity": 0.92,
      "codebase_familiarity": 0.78,
      "patch_quality": 0.91,
      "memory_support": 0.85
    },
    "observations": 3,
    "method": "multi-source correlation"
  }
}
```

In human mode:

```
Confidence: 0.87 (task: 0.92, familiarity: 0.78, quality: 0.91, memory: 0.85) | obs: 3
```

### 3.6 Trade-offs

**Considered:** Fewer agents (combine Cataloger+Publisher, combine Correlator+Validator).
**Rejected:** LDO's seven-agent pipeline is expensive but reflects the scientific method. Each stage is an independent verification step. Combining them weakens the verification chain.

**Considered:** Parallel agent execution for independent sessions.
**Rejected for v1:** Sequential execution is reproducible. Parallel execution introduces non-determinism in pipeline ordering. Revisit for v2 once sequential correctness is proven.

---

## 4. Polyrepo PR-Based Agent Coordination

### 4.1 Approach

PRs as publication channels. PR comments as structured observation reports. Cross-repo coordination as multi-observatory data sharing. No external dependencies.

### 4.2 Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, draft: &PrDraft) -> Result<PrId>;
    fn get_pr(&self, repo: &RepoRef, id: PrId) -> Result<PullRequest>;
    fn comment(&self, repo: &RepoRef, pr: PrId, body: &str) -> Result<CommentId>;
    fn list_comments(&self, repo: &RepoRef, pr: PrId, since: Option<DateTime>) -> Result<Vec<Comment>>;
    fn add_labels(&self, repo: &RepoRef, pr: PrId, labels: &[&str]) -> Result<()>;
    fn list_prs(&self, repo: &RepoRef, filter: &PrFilter) -> Result<Vec<PrSummary>>;
    fn get_diff(&self, repo: &RepoRef, pr: PrId) -> Result<String>;
    fn merge_status(&self, repo: &RepoRef, pr: PrId) -> Result<MergeStatus>;
}
```

Reference implementation: GitHub REST API. Forge config:

```ini
[but-ai]
    forge = github
    forgeApiUrl = https://api.github.com
    forgeToken = ${BUT_AI_FORGE_TOKEN}
```

### 4.3 PR Comment Schema

LDO uses an observation report format:

```markdown
<!-- ldo:v1 -->
```yaml
catalog_id: LDO-2026-0042
from: publisher@ldo
to: cataloger@ldo
type: observation_report
data_quality: GREEN
observation:
  type: patch_complete
  agent: tracker
  session: s02
  confidence: 0.87
  residuals: 0.03
  data:
    commit: abc1234
    lines: 142
    files: [src/auth/jwt.rs, src/middleware/mod.rs, src/middleware/auth.rs]
budget:
  consumed: 28400
  remaining: 36600
  calibration_overhead: 6500
dependencies:
  - forge://github.com/org/shared-lib#15
    status: complete
    confidence: 0.95
  - forge://gitlab.example.com/group/frontend#43
    status: in_progress
    confidence: 0.72
```
<!-- /ldo:v1 -->
```

Message types:

| Type | Originator | Description |
|------|-----------|-------------|
| `campaign_plan` | Cataloger | Task decomposition with session assignments |
| `observation_report` | Publisher | Session result with confidence metrics |
| `correlation_report` | Correlator | Review verdict with correlation coefficient |
| `validation_record` | Validator | Signing confirmation with reproducibility check |
| `dependency_declare` | Publisher | Cross-repo dependency with confidence |
| `anomaly_alert` | Sentinel-Ops | Pipeline anomaly notification |
| `data_request` | Any agent | Request for additional observations (context) |

Every message includes `data_quality` (GREEN/YELLOW/RED) and `confidence` (0.0-1.0). Recipients can filter by quality and confidence.

### 4.4 Cross-Repo References

Standard URI format:

```
forge://<host>/<owner>/<repo>#<number>
```

Cataloger maintains an observation log on the memory branch:

```yaml
# refs/but-ai/rcs/cataloger/campaigns/LDO-2026-0042.yaml
catalog_id: LDO-2026-0042
created: "2026-03-28T09:00:00Z"
status: in_progress
sessions:
  s01:
    agent: tracker
    pr: forge://github.com/org/backend#42
    status: complete
    confidence: 0.93
    lines: 87
  s02:
    agent: tracker
    pr: forge://github.com/org/frontend#43
    status: in_progress
    confidence: 0.78
    lines: null
  s03:
    agent: tracker
    pr: forge://gitlab.example.com/group/shared#17
    status: blocked
    blocked_by: s02
    confidence: null
budget:
  total: 65000
  consumed: 28400
  reserve: 4550
  calibration: 6500
```

### 4.5 Trade-offs

**Considered:** Real-time event streaming between repos via WebSockets.
**Rejected:** No proprietary dependencies. Forge polling with `since` parameter is sufficient and universally supported.

**Considered:** Git notes for metadata instead of PR comments.
**Rejected:** Git notes are not displayed by most forges. PR comments are human-readable and universally supported.

---

## 5. Agent Memory and Identity

### 5.1 Approach: Radar Cross-Section Memory

Memories are tracked objects detected by radar. Each memory has a radar cross-section (RCS) — how visible it is. Large RCS memories are obvious patterns that are easy to detect and retrieve. Small RCS memories are subtle patterns that require focused observation to detect. Each memory has a trajectory — how it evolves over time. And each memory has a confidence score based on the number of independent observations that confirm it.

This is not a metaphor stretched over a key-value store. The scoring model is derived from actual radar detection theory.

### 5.2 Storage

Memory branch: `refs/but-ai/rcs/<agent-id>/`

```
refs/but-ai/rcs/aperture/
  identity.yaml
  catalog/
    confirmed/       # Confirmed tracks (3+ observations)
      <hash>.yaml
    candidate/       # Candidate detections (1-2 observations)
      <hash>.yaml
    deprecated/      # Deprecated entries (source code deleted or pattern invalidated)
      <hash>.yaml
  index.yaml         # TLE-format index (sorted by RCS for fast sweep)
```

Each memory entry (tracked object):

```yaml
id: "rcs-28174"
first_detected: "2026-03-28T10:15:00Z"
last_observed: "2026-03-28T10:15:00Z"
status: candidate          # candidate | confirmed | deprecated
rcs: 0.8                   # Radar cross-section: 0.1 (subtle) to 10.0 (obvious)
trajectory:
  current_state: "active"  # active | stable | decaying
  evolution_rate: 0.05     # How fast the pattern is changing (0 = static, 1 = volatile)
ttl: 604800                # 7 days (candidate default)
confidence:
  score: 0.65              # Overall confidence (0-1)
  observations: 1          # Independent confirmations
  sources:                 # Which observations confirmed this
    - { agent: "tracker", commit: "abc1234", date: "2026-03-28" }
content: |
  The authentication module uses JWT with 15-minute access tokens.
  Refresh tokens last 7 days, stored in HTTP-only cookies.
  Endpoint: /api/auth/refresh
tags:
  primary: "authentication"
  secondary: ["jwt", "cookies", "middleware"]
detection_context:
  file: "src/auth/jwt.rs"
  branch: "feat/auth"
  commit: "abc1234"
snr: 4.2                   # Signal-to-noise ratio at detection
```

### 5.3 Detection and Confirmation

New pattern detection follows radar tracking protocol:

1. **Initial detection:** Pattern observed for the first time. Stored as `candidate` with `observations: 1` and `confidence: 0.3 + (snr / 20)` (higher SNR = higher initial confidence, max 0.8 for single detection).

2. **Confirmation:** Same pattern observed by a different agent or in a different context. `observations` increments, confidence recalculated:

```
confidence = 1.0 - (1.0 / (1.0 + observations * snr_avg / 5.0))
```

At 3+ observations, the entry is promoted from `candidate` to `confirmed` and TTL extends:

| Status | Required Observations | TTL | Confidence Range |
|--------|----------------------|-----|-----------------|
| Candidate | 1-2 | 7 days | 0.30-0.70 |
| Confirmed | 3+ | 30 days | 0.70-0.99 |
| Deprecated | N/A (explicit) | 7 days (grace) | N/A |

### 5.4 Retrieval: Radar Sweep

When Aperture retrieves memory:

1. **Determine sweep parameters:** Extract semantic keys from the query. Set minimum RCS threshold (default: 0.1 for broad sweep, configurable).
2. **Scan index:** Find entries whose tags match query keys and whose RCS >= threshold.
3. **Score each detection:**

```
score = tag_match * rcs_normalized * confidence * recency * snr_factor

where:
  tag_match      = 1.0 (primary), 0.5 (secondary), 0.2 (partial keyword match)
  rcs_normalized = min(entry.rcs, 5.0) / 5.0
  confidence     = entry.confidence.score
  recency        = exp(-age_days / (ttl_days * 0.5))
  snr_factor     = min(entry.snr / 10.0, 1.0)
```

4. **Return** top N results (default: 5) with scores and confidence intervals.

The RCS threshold is the key control: a narrow sweep (high RCS threshold) returns only obvious patterns. A deep survey (low RCS threshold) returns subtle patterns that may be noise. Aperture defaults to narrow sweep and only runs deep survey when explicitly requested or when the narrow sweep returns insufficient results.

### 5.5 Expiration

- **Candidate entries:** 7-day TTL. If not confirmed within 7 days, they enter deprecated status for a 7-day grace period, then are deleted.
- **Confirmed entries:** 30-day TTL. TTL resets on each new observation. Entries with 10+ observations have TTL extended to 90 days.
- **Deprecated entries:** 7-day grace period. No retrieval during grace period. Deleted after grace.
- **`but ai detect gc`:** Removes all entries past their TTL or grace period.

### 5.6 Compaction Survival

Before compaction:

1. Aperture runs a **rapid cataloging burst** — extracting critical context as `confirmed` entries with `rcs: 10.0` (maximum, ensuring detection in any sweep) and `snr: 20.0` (well above any threshold).
2. After compaction, Aperture's first action is a standard sweep, which naturally retrieves the high-RCS burst entries.
3. Burst entries are tagged `detection_context.source: compaction_burst` for traceability.

### 5.7 Long-Term Storage (Reference Catalog)

Shared memory on `refs/but-ai/rcs/reference/`:

```
refs/but-ai/rcs/reference/
  confirmed/       # Shared confirmed tracks
  index.yaml       # Cross-agent catalog
```

Promotion requirements: `confirmed` status, confidence > 0.85, observations > 5, confirmed by at least 2 different agents, SNR > 3.0. The reference catalog is LDO's shared knowledge base — the equivalent of the official debris catalog.

Cross-repo: `git fetch <remote> refs/but-ai/rcs/reference:refs/but-ai/rcs/reference-<remote>`.

### 5.8 Identity

```yaml
# refs/but-ai/rcs/tracker/identity.yaml
name: "tracker"
organization: "lagrange-debris-observatory"
role: "patch_generator"
created: "2026-03-28T09:00:00Z"
capabilities:
  - patch_generation
  - iterative_refinement
  - convention_matching
  - confidence_quantification
authorization:
  branches:
    allow: ["feat/*", "fix/*", "tracker/*", "LDO-*"]
    deny: ["main", "release/*"]
  repos:
    allow: ["org/*"]
    deny: []
  constraints:
    max_patch_lines: 500
    max_files: 10
    min_correlation_for_signing: 0.75
signing_key:
  fingerprint: "SHA256:jkl012..."
  provider: "openwallet"
  issued: "2026-03-28T09:00:00Z"
  expires: "2027-03-28T09:00:00Z"
  rotation_policy: "90d"
  validator_agent: "validator"
```

The `min_correlation_for_signing` field is LDO-specific: Validator will not sign a patch unless Correlator's correlation coefficient meets this threshold.

### 5.9 Trade-offs

**Considered:** Vector embeddings for retrieval.
**Rejected:** Embeddings are opaque. LDO requires explainable retrieval — every score must be decomposable into identifiable factors (tag match, RCS, confidence, recency, SNR). The RCS scoring model is fully transparent and auditable.

**Considered:** A relational database (PostgreSQL) for memory, matching LDO's existing data infrastructure.
**Rejected:** Must be Git-native for cross-repo sharing and reproducibility. YAML files on branches are versionable, diffable, and fetchable.

**Considered:** Single confidence score without decomposition.
**Rejected:** A single score hides information. LDO's multi-factor scoring (tag match, RCS, confidence, recency, SNR) tells you *why* a memory was retrieved, not just *that* it was retrieved.

---

## 6. Signed Commits via OpenWallet

### 6.1 Approach

All commits signed via OpenWallet. Validator is the signing authority and includes a reproducibility check: the patch being signed must match the patch that Correlator reviewed. This catches post-review modifications.

### 6.2 Key Lifecycle

**Provisioning:**
```bash
but ai identity create --name tracker --org lagrange-debris-observatory
```
Generates OpenWallet keypair. Public key in identity record. Private key in OpenWallet store.

**Rotation:**
```bash
but ai identity rotate --name tracker
```
New keypair generated. Old key archived with `reason: rotation`, `valid_through: <rotation-date>`.

**Revocation:**
```bash
but ai identity revoke --name tracker --reason compromise
```
Key archived with `reason: compromise`, no `valid_through`. All commits flagged suspect.

### 6.3 Validation Chain

Validator performs a six-point verification before signing:

| Check | Source | Threshold |
|-------|--------|-----------|
| 1. Identity valid | Identity record | Not expired, not revoked |
| 2. Branch authorized | authorization.branches | Pattern match |
| 3. Repo authorized | authorization.repos | Pattern match |
| 4. Constraints met | authorization.constraints | max_lines, max_files |
| 5. Correlator approved | Review record | correlation >= min_correlation_for_signing |
| 6. Patch integrity | Diff comparison | Patch at signing == patch at review |

Check 6 is LDO-unique. Validator computes a hash of the patch at review time (stored in the review record) and compares it against the hash of the patch at signing time. If they differ, signing is refused with a `POST_REVIEW_MODIFICATION` error.

### 6.4 Git Config Keys

```ini
[but-ai.calibration]
    provider = openwallet
    keyStore = ~/.openwallet/keys
    rotationDays = 90
    minCorrelation = 0.75
    verifyPatchIntegrity = true
```

### 6.5 Trade-offs

**Considered:** Delegated signing (Correlator signs upon approval, skipping Validator).
**Rejected:** Combining review and signing in one agent creates a conflict of interest. The reviewer's job is to assess; the signer's job is to authorize. Separation is a scientific control.

---

## 7. Token Budget

### 7.1 Budget Table

Model: Claude Opus 4 (200K context)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,600 | 0 | Once per session | Identity (300), role (200), tools (800), workspace (1,200), RCS schema (500), observation protocol (600) |
| **Campaign planning (Cataloger)** | 3,000 | 1,500 | Once per task | Task + workspace state. Output: campaign plan with catalog ID. |
| **Detection (Aperture)** | 3,500 | 1,200 | 1-2 per task | Multi-sweep retrieval (2,000), index scan (500), entry evaluation (1,000). Output: intelligence report with confidence. |
| **Tool call (per call)** | 1,600 | 500 | 6-9 per task | Tool result (1,000), context (600). Output: action + parameters. |
| **Tracking — per iteration (Tracker)** | 5,000 | 4,000 | 2-3 iterations per task | File contents (2,500), task spec (800), memory (800), tool results (900). Output: patch iteration. |
| **Commit message** | 1,200 | 300 | 1-2 per task | Patch summary. |
| **Correlation (Correlator)** | 6,000 | 2,500 | 1-2 per task | Diff (3,000), task spec (1,000), memory entries (1,000), style context (1,000). Output: correlation report. |
| **Validation (Validator)** | 1,500 | 600 | 1-2 per task | Authorization chain + integrity check. |
| **Publication (Publisher)** | 2,500 | 1,500 | 0-3 per task | PR context (1,500), dependency graph (1,000). Output: observation report. |
| **Monitoring (Sentinel-Ops)** | 1,000 | 300 | 2-4 per task | Agent health metrics. Output: status report. |
| **Calibration overhead** | 2,000 | 1,500 | Distributed | Confidence calculation, error bar generation, reproducibility metadata. |
| **TOTAL (typical task)** | **48,400** | **22,400** | -- | 200-line feature, 3 files, 2 sessions, 2 cross-repo deps, 8 tool calls, 2 Tracker iterations, 1 detection, 1 review, 2 coordination events |

**Total: ~70,800 tokens per typical task.**

At Claude Opus 4 pricing: ~$0.73 input + ~$1.68 output = **~$2.41 per task.**

LDO's budget is the highest of the five proposals because it includes: (a) iterative patch generation (2-3 Tracker passes), (b) multi-source review (Correlator cross-references against 3+ sources), (c) reproducibility validation (Validator integrity check), and (d) calibration overhead (confidence metrics at every stage). LDO considers this cost justified by the quality and traceability of the output.

### 7.2 Optimizations

1. **Per-agent tool filtering:** Each agent sees only its authorized tools. Savings: ~150 tokens per agent.
2. **RCS-threshold pre-filtering:** Aperture's narrow sweep (high RCS threshold) evaluates fewer entries. Savings: ~500 tokens per retrieval.
3. **Iteration early-stopping:** Tracker stops iterating when residuals drop below threshold, potentially saving one full iteration. Savings: ~9,000 tokens per early-stopped task.
4. **Correlation caching:** If a re-review after minor changes produces the same correlation coefficient (within 0.05), the full review is skipped. Savings: ~8,500 tokens per cached review.
5. **Sentinel-Ops compression:** Monitoring checks are compressed to threshold comparisons, minimizing LLM involvement. Savings: ~500 tokens per monitoring cycle.

### 7.3 Budget Enforcement

| Threshold | LDO Designation | Action |
|-----------|----------------|--------|
| 65% | Advisory | Reserve activated. Calibration overhead reduced to essentials. |
| 80% | Warning | Tracker limited to 2 iterations max. Correlator reduces cross-reference depth. |
| 90% | Critical | Forced convergence. Tracker must produce final patch in 1 more call. |
| 95% | Emergency | Produce partial patch. Mark `X-Partial: true`, `confidence: low`. |
| 100% | Stop | Hard stop. Submit whatever exists with `confidence: very_low`. |

---

## 8. Testing Strategy

### 8.1 Provider Testing

- **Mock provider:** `but-ai-provider-mock` with deterministic mode enabled. Canned responses keyed by input hash. All CI tests use this.
- **Determinism tests:** Run the same task 5 times with the mock provider. Verify identical patches and identical confidence scores each time. This is critical for LDO's reproducibility requirement.

### 8.2 Patch Workflow (Observation Pipeline Validation)

- **Full pipeline tests:** Task -> Cataloger -> Aperture -> Tracker -> Correlator -> Validator -> Publisher. End-to-end on a test repository. 10 test cases covering: single file, multi-file, iterative convergence, non-convergence (high residuals), cross-repo, partial (budget exhaustion), refinement cycle, trivial, blocked, and abort.
- **Round-trip with confidence:** Generate patch, verify confidence scores are within expected ranges for the test case, apply patch, verify result.
- **Iteration convergence tests:** Feed tasks of known complexity and verify that Tracker converges in the expected number of iterations (1 for trivial, 2-3 for moderate, non-convergence flagged for complex).

### 8.3 Cross-Repo Coordination

- **Mock forge:** HTTP server implementing `ForgeAdapter`. Validates observation report schema, data quality flags, and confidence values.
- **Multi-repo campaign test:** Three repos, one mock forge, one cross-repo task with cross-dependency. Verify: catalog ID consistency across repos, dependency tracking, data quality propagation (if an upstream dependency has `data_quality: YELLOW`, downstream sessions inherit the flag).

### 8.4 Token Budget

- **Budget enforcement tests:** Known budgets, known tasks, verify each threshold triggers correctly.
- **Calibration overhead measurement:** Run 20 tasks, measure what percentage of budget goes to confidence calculation and reproducibility metadata. Target: < 15%.
- **Accuracy benchmarks:** 50 test tasks, compare estimated vs actual usage, compute mean absolute error. Target: < 15% error per component.

### 8.5 Memory System

- **Detection tests:** Store a single observation, verify `candidate` status. Store a second observation of the same pattern, verify `observations` increments. Store a third, verify promotion to `confirmed`.
- **RCS scoring tests:** Store entries with varying RCS, query with known tags, verify scoring order matches expected (higher RCS, higher confidence = higher rank).
- **Sweep depth tests:** Verify narrow sweep (high RCS threshold) returns only obvious patterns. Verify deep survey returns candidate detections.
- **Expiration tests:** Store candidates, advance time past 7-day TTL, verify `deprecated` status. Advance past grace period, verify deletion.
- **Compaction burst tests:** Store burst entries, clear context, sweep, verify burst entries retrieved first (highest RCS).

---

## 9. Git Config Keys (Complete)

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.forge` | string | `github` | Forge type |
| `but-ai.forgeApiUrl` | string | per forge | Forge API URL |
| `but-ai.forgeToken` | string | env | Forge token |
| `but-ai.agent.tokenBudget` | int | `70000` | Max tokens per task (full pipeline) |
| `but-ai.agent.reserveFraction` | float | `0.07` | Budget reserve |
| `but-ai.agent.calibrationFraction` | float | `0.10` | Calibration overhead |
| `but-ai.agent.maxTrackerIterations` | int | `3` | Max Tracker refinement passes |
| `but-ai.agent.maxCorrelationRounds` | int | `3` | Max Correlator revision cycles |
| `but-ai.rcs.branchPrefix` | string | `refs/but-ai/rcs` | Memory branch prefix |
| `but-ai.rcs.referenceBranch` | string | `refs/but-ai/rcs/reference` | Shared reference catalog |
| `but-ai.rcs.candidateTtl` | int | `604800` | Candidate TTL (7 days) |
| `but-ai.rcs.confirmedTtl` | int | `2592000` | Confirmed TTL (30 days) |
| `but-ai.rcs.confirmedExtendedTtl` | int | `7776000` | Extended confirmed TTL, 10+ obs (90 days) |
| `but-ai.rcs.deprecatedGrace` | int | `604800` | Deprecated grace period (7 days) |
| `but-ai.rcs.confirmationThreshold` | int | `3` | Observations for candidate -> confirmed |
| `but-ai.rcs.maxRetrieval` | int | `5` | Max entries per sweep |
| `but-ai.rcs.defaultRcsThreshold` | float | `0.1` | Minimum RCS for narrow sweep |
| `but-ai.rcs.deepSurveyRcsThreshold` | float | `0.01` | Minimum RCS for deep survey |
| `but-ai.rcs.promotionMinConfidence` | float | `0.85` | Min confidence for reference catalog promotion |
| `but-ai.rcs.promotionMinObservations` | int | `5` | Min observations for reference catalog promotion |
| `but-ai.rcs.promotionMinAgents` | int | `2` | Min confirming agents for promotion |
| `but-ai.calibration.provider` | string | `openwallet` | Signing provider |
| `but-ai.calibration.keyStore` | string | `~/.openwallet/keys` | Key store path |
| `but-ai.calibration.rotationDays` | int | `90` | Key rotation interval |
| `but-ai.calibration.minCorrelation` | float | `0.75` | Min correlation coefficient for signing |
| `but-ai.calibration.verifyPatchIntegrity` | bool | `true` | Compare patch at review vs signing |

---

## 10. Migration Path

LDO approaches migration as a phased observation campaign:

**Phase 1 (Parallel Observation):** `but-ai mcp` runs alongside legacy MCP. Both endpoints active. Traffic to both is logged for comparison. Duration: 14 days minimum.

**Phase 2 (Validation):** Compare outputs from old and new servers across 100+ real tasks. Compute correlation coefficient. If correlation >= 0.95, proceed to Phase 3. If not, investigate discrepancies.

**Phase 3 (Cutover):** Legacy server redirects to `but-ai mcp`. Old code remains as fallback. Duration: 30 days.

**Phase 4 (Decommission):** After 30 days of clean operation, legacy code removed.

LDO does not rush migrations. Migrations are observation campaigns. The data tells you when it is safe to proceed.

---

*Observation: Analytical + Radar*
*Confidence: 85%*
*Independent observations: 3*
*Epoch: 2026-03-28T00:00:00Z*
