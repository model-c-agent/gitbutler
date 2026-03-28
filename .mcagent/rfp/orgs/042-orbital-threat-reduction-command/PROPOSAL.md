# Orbital Threat Reduction Command — Proposal for `but-ai` Plugin

*Orbital parameters: i=45deg, alt=500km, epoch=2026-03-28T00:00:00Z*
*"A patch is a guided munition. Aim it. Verify it. Fire it. Confirm the kill."*

---

## 1. Plugin Architecture

### 1.1 Approach

`but-ai` is a Rust binary on PATH, discovered by `find_external_subcommand()` in `crates/but/src/alias.rs`. OTRC treats the plugin as a weapons system: it has a defined interface, operational constraints, and a maintenance schedule. It does not improvise.

### 1.2 Design

```
but-ai (binary)
  |-- ops/        # Operational layer: CLI parsing, output formatting
  |-- mcp/        # MCP layer: rmcp ServerHandler, tool registration
  |-- agent/      # Agent pipeline: COMMAND -> SENTINEL -> STRIKER -> OVERWATCH -> KEYMASTER
  |-- orbit/      # Orbital memory system
  |-- forge/      # Forge adapter: PR-based coordination
  |-- armory/     # OpenWallet signing and key management
```

Subcommands:

| Command | Classification | Description |
|---------|---------------|-------------|
| `but ai mission <task>` | OPERATIONAL | Execute a full mission (task through the complete pipeline) |
| `but ai plan <task>` | PLANNING | Produce OPORD without execution (COMMAND only, dry run) |
| `but ai mcp` | SUPPORT | Start MCP server on stdio |
| `but ai intel query <text>` | INTELLIGENCE | Query orbital memory |
| `but ai intel store <text>` | INTELLIGENCE | Store intelligence in orbital memory |
| `but ai intel gc` | MAINTENANCE | Deorbit expired memory entries |
| `but ai status` | SITUATIONAL | Show operational status, budget, active missions |
| `but ai identity create` | SECURITY | Create agent identity with OpenWallet key |
| `but ai identity verify <commit>` | SECURITY | Verify signed commit authorization chain |
| `but ai identity rotate` | SECURITY | Rotate agent signing key |
| `but ai identity revoke --reason <r>` | SECURITY | Revoke agent signing key |

Environment variables:

| Variable | OTRC Usage |
|----------|-----------|
| `BUT_WORKSPACE_DIR` | Area of Operations (AO) — the Git workspace |
| `BUT_OUTPUT_FORMAT` | Report format (`human`, `json`, `shell`) |
| `BUT_JSON` | JSON mode flag |

### 1.3 WASI Degradation

Under WASI:

- `but ai` subcommand unavailable (no PATH discovery)
- MCP server can operate standalone via direct invocation on stdio
- Intelligence queries operational (Git reads via `gix`)
- Mission execution and signing non-operational
- Degradation reported as structured error: `WASI_DEGRADED` with capability manifest

OTRC's position: WASI is an operational constraint, not a failure. Agents operating under WASI are in "observe-only" mode — they can query intelligence and report observations but cannot engage (produce patches) or authorize (sign commits).

### 1.4 Trade-offs

**Considered:** Embedding `but-ai` as a library crate linked at compile time.
**Rejected:** Violates operational separation. The plugin must be independently deployable and revocable without modifying the host binary.

**Considered:** Containerized deployment (Docker).
**Rejected:** Adds runtime dependency. A static Rust binary on PATH has zero dependencies and deterministic behavior.

---

## 2. Provider-Agnostic AI Interface

### 2.1 Approach

Use `but-llm` without modification. The four existing providers are the primary weapons systems. New providers are supported via PATH-based plugin executables.

OTRC treats LLM providers like sensor systems: they all observe the same reality, they all have different characteristics (resolution, range, accuracy), and the mission uses whichever sensor is best suited to the conditions.

### 2.2 Design

Provider selection:

1. Read `gitbutler.aiModelProvider` from Git config
2. Built-in providers (`openai`, `anthropic`, `ollama`, `lmstudio`) route to `but-llm`
3. Non-built-in providers route to `but-ai-provider-<name>` on PATH
4. Provider plugins implement JSON-RPC over stdio:

```json
{
  "methods": {
    "required": ["initialize", "capabilities", "tool_calling_loop", "response"],
    "optional": ["stream_response", "structured_output"]
  }
}
```

The `capabilities` response includes:
- `tool_calling: bool` — can the provider handle tool calling loops?
- `streaming: bool` — does the provider support streaming?
- `structured_output: bool` — does the provider support JSON schema output?
- `max_context: int` — maximum context window size

`but-ai` adapts its behavior based on capabilities. If the provider does not support tool calling, the agent falls back to a plan-execute-assess loop using `response` and manual tool dispatch.

### 2.3 MCP Compatibility

Drop-in replacement for the existing MCP server. All 10 workspace tools registered. Backward-compatible `gitbutler_update_branches` shim included.

```rust
ServerInfo {
    name: "GitButler MCP Server",
    version: "2.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

### 2.4 Trade-offs

**Considered:** Supporting multiple simultaneous providers (ensemble).
**Rejected:** Interesting for threat assessment (multiple independent assessments of the same conjunction), but the complexity is not justified for v1. Revisit for v2 if redundancy requirements demand it.

**Considered:** Provider failover chain (if Anthropic fails, try OpenAI).
**Rejected:** Failover without explicit configuration is a security risk. The operator must explicitly configure backup providers. Automatic failover is mission creep.

---

## 3. The But Agent

### 3.1 Approach

The agent operates as a five-element fire team: COMMAND plans, SENTINEL provides intelligence, STRIKER executes, OVERWATCH verifies, KEYMASTER signs. The fire team follows a fixed operational sequence with no deviation. The agent produces INDEX.patch + COMMIT.msg as its sole kinetic output. No direct file edits. No `git commit`. No `but commit`.

### 3.2 Design: The Kill Chain

```
1. BRIEFING     (COMMAND)    — Receive task, produce OPORD
2. SWEEP        (SENTINEL)   — Intelligence retrieval from orbital memory
3. PLANNING     (COMMAND)    — Final mission plan with budget allocations
4. ENGAGEMENT   (STRIKER)    — Produce INDEX.patch + COMMIT.msg
5. ASSESSMENT   (OVERWATCH)  — Review patch, compute conjunction probability
6. AUTHORIZATION (KEYMASTER) — Verify authorization, sign via OpenWallet
```

Budget allocation:

| Phase | Budget % | Purpose |
|-------|----------|---------|
| Briefing | 8% | Task ingestion and OPORD production |
| Sweep | 10% | Memory retrieval (SENTINEL) |
| Planning | 7% | Final mission plan |
| Engagement | 40% | Patch generation (STRIKER) |
| Assessment | 20% | Review and risk scoring (OVERWATCH) |
| Authorization | 5% | Signing (KEYMASTER) |
| Reserve | 10% | Held for revision cycles and re-sweeps |

The 10% reserve is OTRC's **operational reserve** — doctrine requires maintaining reserve capacity at all times. An operation that commits 100% of its resources has no capacity to respond to the unexpected. Hartley: "If you are out of reserve, you are out of options."

### 3.3 Branch Naming

OTRC uses a military-style branch naming convention:

```
<callsign>/<mission-type>-<objective>.<dependency-chain>
```

Example: `striker/engage-auth.s01.s03` — agent STRIKER, engagement mission, objective "auth", task s03 depending on s01.

Mission types:
- `engage` — produce a patch (STRIKER)
- `recon` — read and analyze without changes (SENTINEL)
- `coord` — cross-repo coordination (COMMAND)

### 3.4 Workspace Tools

Tools assigned by role:

| Agent | Authorized Tools | Classification |
|-------|-----------------|----------------|
| COMMAND | GetProjectStatus, GetBranchChanges, CreateBranch | C2 (Command & Control) |
| SENTINEL | GetProjectStatus, GetCommitDetails, GetBranchChanges | ISR (Intelligence, Surveillance, Reconnaissance) |
| STRIKER | GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit, CreateBranch, MoveFileChanges | ENGAGEMENT |
| OVERWATCH | GetBranchChanges, GetCommitDetails, GetProjectStatus | ASSESSMENT |
| KEYMASTER | GetCommitDetails, GetBranchChanges, Commit | AUTHORIZATION |

Tool access is strict. An agent cannot use tools outside its classification. This is enforced programmatically, not by convention.

### 3.5 Progress Reporting

Agents report via SITREPs:

```json
{
  "type": "sitrep",
  "from": "striker",
  "phase": "engagement",
  "status": "complete",
  "mission": "engage-auth/s02",
  "details": {
    "target": "src/auth/jwt.rs",
    "lines_engaged": 87,
    "collateral": 0,
    "confidence": "high"
  },
  "budget": {
    "allocated": 20000,
    "consumed": 17400,
    "reserve": 5000
  },
  "timestamp": "2026-03-28T10:45:00Z"
}
```

In human mode:

```
[SITREP] STRIKER | engage-auth/s02 | COMPLETE | 87 lines | 0 collateral | budget 17400/20000
```

### 3.6 Trade-offs

**Considered:** Three agents instead of five (combine SENTINEL+COMMAND and OVERWATCH+KEYMASTER).
**Rejected:** OTRC does not compromise on separation of duties. The intelligence function (SENTINEL) must be independent of command (COMMAND). The verification function (OVERWATCH) must be independent of authorization (KEYMASTER). Combining them creates conflicts of interest.

**Considered:** Dynamic mission assignment (let COMMAND choose which agent handles each phase).
**Rejected:** The kill chain is fixed. Dynamic assignment introduces unpredictability. Predictability saves lives. (And tokens.)

---

## 4. Polyrepo PR-Based Agent Coordination

### 4.1 Approach

PRs are the communications channel. PR comments carry structured messages (OPORDs, SITREPs, INTSUMs). Only COMMAND communicates across repository boundaries. All other agents operate within their assigned AO (Area of Operations — the current repository).

### 4.2 Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, pr: &PullRequestDraft) -> Result<PrId>;
    fn get_pr(&self, repo: &RepoRef, id: PrId) -> Result<PullRequest>;
    fn post_comment(&self, repo: &RepoRef, pr: PrId, body: &str) -> Result<CommentId>;
    fn list_comments(&self, repo: &RepoRef, pr: PrId, since: Option<DateTime>) -> Result<Vec<Comment>>;
    fn set_labels(&self, repo: &RepoRef, pr: PrId, labels: &[&str]) -> Result<()>;
    fn list_prs(&self, repo: &RepoRef, filter: &PrFilter) -> Result<Vec<PrSummary>>;
    fn get_diff(&self, repo: &RepoRef, pr: PrId) -> Result<String>;
    fn check_merge_status(&self, repo: &RepoRef, pr: PrId) -> Result<MergeStatus>;
}
```

Reference implementation: GitHub REST API. Forge selection via Git config:

```ini
[but-ai]
    forge = github
    forgeApiUrl = https://api.github.com
    forgeToken = ${BUT_AI_FORGE_TOKEN}
```

### 4.3 PR Comment Schema

OTRC uses a military message format:

```markdown
<!-- otrc:v1 -->
```yaml
classification: UNCLASSIFIED
dtg: "2026-03-28T10:45:00Z"
from: command@otrc
to: striker@otrc
type: opord
mission: engage-auth/s03
body:
  situation:
    friendly: "s01 complete (backend#42), s02 in progress (frontend#43)"
    enemy: "Cross-repo dependency on shared-lib#17 not yet satisfied"
  mission: "Produce INDEX.patch for auth middleware integration"
  execution:
    target_files: [src/middleware/auth.rs, src/middleware/mod.rs]
    constraints: { max_lines: 200, max_files: 3 }
    budget: 18000
  admin:
    branch: "striker/engage-auth.s01.s03"
    depends_on:
      - forge://github.com/org/backend#42
      - forge://github.com/org/frontend#43
  signal:
    report_to: "forge://github.com/org/main-repo#10"
```
<!-- /otrc:v1 -->
```

Message types:

| Type | Originator | Description |
|------|-----------|-------------|
| `opord` | COMMAND | Operations order — mission assignment |
| `sitrep` | Any agent | Situation report — status update |
| `intsum` | SENTINEL | Intelligence summary — memory/context briefing |
| `fragord` | COMMAND | Fragmentary order — modification to existing OPORD |
| `review_assessment` | OVERWATCH | Review verdict with risk scoring |
| `auth_confirmation` | KEYMASTER | Signing confirmation with audit trail |
| `abort` | COMMAND | Mission abort with reason |

### 4.4 Cross-Repo References

Standard military-style reference format:

```
forge://<host>/<owner>/<repo>#<number>
```

COMMAND maintains a Combined Operations Tracker (COT) on the memory branch:

```yaml
# refs/but-ai/orbit/command/combined-ops.yaml
operation: auth-integration
classification: UNCLASSIFIED
dtg_start: "2026-03-28T09:00:00Z"
missions:
  engage-auth/s01:
    status: COMPLETE
    assigned_to: striker
    pr: forge://github.com/org/backend#42
    depends_on: []
    budget: { allocated: 18000, consumed: 15200 }
  engage-auth/s02:
    status: IN_PROGRESS
    assigned_to: striker
    pr: forge://github.com/org/frontend#43
    depends_on: [engage-auth/s01]
    budget: { allocated: 20000, consumed: 8400 }
  engage-auth/s03:
    status: BLOCKED
    assigned_to: striker
    pr: forge://gitlab.example.com/group/shared#17
    depends_on: [engage-auth/s01, engage-auth/s02]
    blocked_by: engage-auth/s02
    budget: { allocated: 18000, consumed: 0 }
total_budget: { allocated: 56000, consumed: 23600, reserve: 5600 }
```

### 4.5 Trade-offs

**Considered:** Decentralized coordination (each agent talks directly to its counterpart in other repos).
**Rejected:** Decentralized communication in military operations leads to fratricide. Only COMMAND communicates across boundaries. This is doctrine.

**Considered:** Shared state database for cross-repo coordination.
**Rejected:** Violates "no proprietary dependencies." The COT is a YAML file on a Git branch. Fetchable, diffable, auditable.

---

## 5. Agent Memory and Identity

### 5.1 Approach: Orbital Mechanics Memory

Memories are placed in orbits. Frequently accessed memories are in low orbit (LEO) — fast retrieval, short lifetime, high drag. Working memories are in medium orbit (MEO) — moderate retrieval, moderate lifetime. Archival memories are in geostationary orbit (GEO) — slow retrieval, long lifetime, stable. Deprecated memories are in graveyard orbit — no retrieval, pending deorbit (deletion).

The orbital analogy is not decorative. It maps directly to access patterns, TTL, and retrieval cost.

### 5.2 Storage

Memory branch: `refs/but-ai/orbit/<agent-id>/`

```
refs/but-ai/orbit/sentinel/
  identity.yaml
  leo/                 # Low Earth Orbit — active working memory
    <hash>.yaml
  meo/                 # Medium Earth Orbit — established patterns
    <hash>.yaml
  geo/                 # Geostationary — archival, foundational knowledge
    <hash>.yaml
  graveyard/           # Graveyard orbit — pending deorbit
    <hash>.yaml
  tle/                 # Two-Line Element sets (memory index)
    index.yaml
```

Each memory entry (modeled as a tracked object):

```yaml
id: "obj-42718"
cataloged: "2026-03-28T10:15:00Z"
orbit: leo
orbital_elements:
  altitude: 400              # Metaphor: 200-600km LEO, 2000-35786km MEO, 35786km GEO
  inclination: 65            # Relevance breadth: 90=universal, 0=narrow
  eccentricity: 0.02         # Stability: 0=stable, 0.5=variable access pattern
  period: 5400               # Access period in seconds (how often retrieved)
ttl: 259200                  # 3 days (LEO objects have high drag)
rcs: 1.2                     # Radar Cross Section (visibility/detectability)
content: |
  The authentication module uses JWT with 15-minute access tokens.
  Refresh tokens are 7-day, stored in HTTP-only cookies.
  Endpoint: /api/auth/refresh
tags:
  primary: "authentication"
  secondary: ["jwt", "cookies", "middleware"]
source:
  commit: "abc1234"
  branch: "feat/auth"
  file: "src/auth/jwt.rs"
confidence: 0.93
observations: 4              # Independent confirmations
last_observed: "2026-03-28T10:15:00Z"
```

### 5.3 Orbital Mechanics

**Orbit raising (promotion):** When a LEO memory accumulates 3+ observations and confidence > 0.8, it is promoted to MEO. When a MEO memory accumulates 5+ observations, it is promoted to GEO. Promotion extends TTL:

| Orbit | TTL | Promotion Threshold | Demotion Trigger |
|-------|-----|--------------------|-----------------------|
| LEO | 3 days | Entry point for all new memories | N/A (starting orbit) |
| MEO | 21 days | 3+ observations, confidence > 0.8 | 0 observations in 14 days |
| GEO | 90 days | 5+ observations, confidence > 0.9 | Explicit invalidation only |
| Graveyard | 7 days | N/A | TTL expired in any orbit |

**Orbital decay (demotion):** MEO memories with zero observations in 14 days decay back to LEO. LEO memories that expire their TTL enter graveyard orbit for 7 days (grace period) before deorbit (deletion).

**Deorbit (deletion):** `but ai intel gc` removes all entries in graveyard orbit past their grace period.

### 5.4 Retrieval: Conjunction Assessment

When SENTINEL retrieves memory, she performs a "conjunction assessment" against the query:

1. **Sweep all orbits** for entries whose tags match the query keywords
2. **Compute conjunction probability** for each match:

```
score = tag_match * rcs_weight * orbital_proximity * recency * confidence

where:
  tag_match         = 1.0 (primary), 0.5 (secondary), 0.2 (partial)
  rcs_weight        = min(entry.rcs, 2.0) / 2.0  [larger RCS = more visible = higher weight]
  orbital_proximity = 1.0 for same orbit as query, 0.7 for adjacent orbit, 0.4 for distant
  recency           = 1.0 - (age / ttl)
  confidence        = entry.confidence * log2(1 + entry.observations)
```

3. **Rank and return** top N (default 5) entries

The "orbital proximity" factor is the key innovation: SENTINEL retrieves from the orbit closest to the query's operational context. A tactical query (specific file, specific bug) targets LEO. A strategic query (architecture decision, pattern choice) targets GEO. This prevents strategic memories from cluttering tactical queries and vice versa.

### 5.5 Expiration

- **LEO:** 3-day TTL. High drag. Memories must be confirmed quickly or they decay.
- **MEO:** 21-day TTL. Moderate lifetime. Observations reset TTL.
- **GEO:** 90-day TTL. Long lifetime. Only expires if explicitly invalidated or if the source code is deleted.
- **Graveyard:** 7-day grace period. Last chance before permanent deorbit.

### 5.6 Compaction Survival

Before context compaction:

1. SENTINEL runs a **rapid cataloging sweep** — extracting critical context items and storing them as LEO entries with `rcs: 5.0` (maximum visibility) and `observations: 10` (instant MEO promotion eligibility).
2. After compaction, SENTINEL's first action is a full-orbit sweep, which naturally retrieves the high-RCS checkpoint entries.
3. Checkpoint entries are tagged `source: compaction_rescue` for traceability.

### 5.7 Long-Term Storage (Deep Space Network)

Shared long-term memory on `refs/but-ai/orbit/dsn/` (Deep Space Network):

```
refs/but-ai/orbit/dsn/
  geo/           # Shared geostationary entries
  index.yaml     # Cross-agent catalog
```

Only GEO entries can be contributed to the DSN. Requirements: confidence > 0.9, observations > 5, confirmed by at least 2 agents. The DSN is the highest-value intelligence resource and contamination is an operational failure.

Cross-repo: `git fetch <remote> refs/but-ai/orbit/dsn:refs/but-ai/orbit/dsn-<remote>`.

### 5.8 Identity

```yaml
# refs/but-ai/orbit/striker/identity.yaml
callsign: "STRIKER"
organization: "otrc"
classification: "ENGAGEMENT"
created: "2026-03-28T09:00:00Z"
capabilities:
  - patch_generation
  - precision_changes
  - convention_matching
authorization:
  branches:
    allow: ["feat/*", "fix/*", "striker/*"]
    deny: ["main", "release/*", "hotfix/*"]
  repos:
    allow: ["org/backend", "org/frontend"]
    deny: ["org/classified-*"]
  constraints:
    max_patch_lines: 500
    max_files: 10
    require_overwatch_approval: true
signing_key:
  fingerprint: "SHA256:ghi789..."
  provider: "openwallet"
  issued: "2026-03-28T09:00:00Z"
  expires: "2027-03-28T09:00:00Z"
  rotation_policy: "60d"
  custodian: "keymaster"
```

The `custodian: "keymaster"` field is OTRC-specific: the signing key is held by KEYMASTER, not by the identity's agent. STRIKER never touches the key. Only KEYMASTER does.

### 5.9 Trade-offs

**Considered:** Vector embeddings for retrieval.
**Rejected:** Embeddings are a black box. OTRC requires explainable retrieval — every memory returned must have a traceable score composed of identifiable factors. The orbital mechanics scoring is fully transparent.

**Considered:** Flat memory with recency-only scoring.
**Rejected:** Recency is a poor proxy for relevance. A two-week-old architectural decision is more relevant to a new feature than a two-minute-old debug log. Orbital classification separates access patterns from recency.

**Considered:** Redis or similar for fast memory access.
**Rejected:** External dependency. All memory is Git-native.

---

## 6. Signed Commits via OpenWallet

### 6.1 Approach

All agent commits signed via OpenWallet. KEYMASTER is the sole signing authority. Authorization is verified before every signing operation. A complete audit trail is maintained.

### 6.2 Key Lifecycle

**Provisioning:**
```bash
but ai identity create --callsign STRIKER --org otrc
```
KEYMASTER generates the keypair, stores the public key in the identity record, and retains custody of the private key in the OpenWallet store.

**Rotation (weapons maintenance):**
```bash
but ai identity rotate --callsign STRIKER
```
KEYMASTER generates a new keypair. The old key is moved to `revoked_keys` with:
- `reason: rotation`
- `valid_through: <rotation-date>`
- `custodian_at_rotation: keymaster`

Commits signed before rotation remain valid for the `valid_through` period.

**Revocation (munitions loss):**
```bash
but ai identity revoke --callsign STRIKER --reason compromise
```
The key is immediately revoked:
- `reason: compromise`
- `valid_through: null` (all commits suspect)
- `incident_report_required: true`

COMMAND is automatically notified. All in-progress missions assigned to the compromised agent are halted pending review.

### 6.3 Authorization Model

KEYMASTER verifies a five-point authorization chain before every signing:

| Check | Source | Failure Response |
|-------|--------|-----------------|
| 1. Identity valid | Identity record on orbit branch | REJECT: "Identity not found or expired" |
| 2. Key not revoked | Identity record revoked_keys list | REJECT: "Key revoked" |
| 3. Branch authorized | identity.authorization.branches | REJECT: "Branch outside authorization scope" |
| 4. Repo authorized | identity.authorization.repos | REJECT: "Repository outside authorization scope" |
| 5. OVERWATCH approval | Review approval token in commit metadata | REJECT: "No OVERWATCH approval on record" |

The authorization chain is encoded in the commit's signature metadata. A verifier can reconstruct the chain from the commit + identity record + review record.

### 6.4 Git Config Keys

```ini
[but-ai.armory]
    provider = openwallet
    keyStore = ~/.openwallet/keys
    rotationDays = 60
    requireOverwatchApproval = true
    auditLogBranch = refs/but-ai/orbit/audit
```

### 6.5 Trade-offs

**Considered:** Each agent holding its own signing key.
**Rejected:** Distributed key management is a distributed attack surface. Centralized custody by KEYMASTER is a single point of control — not a single point of failure, because KEYMASTER can be replaced, but a single point of accountability.

---

## 7. Token Budget

### 7.1 Budget Table

Model: Claude Opus 4 (200K context)

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,800 | 0 | Once per session | Agent identity (400), callsign/classification (200), tool descriptions (800), workspace state (1,400), orbital memory schema (500), OPORD protocol (500) |
| **Task ingestion (COMMAND)** | 2,500 | 800 | Once per task | Task order, PR body, issue description. Output: initial assessment. |
| **Intelligence sweep (SENTINEL)** | 3,500 | 1,200 | Once per task | Multi-orbit sweep (2,000), index scan (500), entry evaluation (1,000). Output: INTSUM (intelligence summary). |
| **Mission planning (COMMAND)** | 4,000 | 2,500 | Once per task | Task + INTSUM + workspace state. Output: OPORD with mission assignments and budget allocations. |
| **Tool call (per call)** | 1,700 | 550 | 6-8 per task | Tool result (1,100), accumulated context (600). Output: action + parameters. |
| **Engagement (STRIKER)** | 7,500 | 6,000 | 1-2 per task | Target files (4,000), mission brief (1,000), INTSUM (1,500), tool results (1,000). Output: INDEX.patch + strike assessment. |
| **Commit message** | 1,200 | 250 | 1-2 per task | Patch summary. Output: COMMIT.msg. |
| **Assessment (OVERWATCH)** | 6,000 | 2,500 | 1-2 per task | Full diff (3,500), surrounding context (1,500), INTSUM (1,000). Output: risk-scored review. |
| **Memory operation (SENTINEL)** | 1,500 | 500 | 1-2 per task | New pattern storage or update. Output: cataloged entry. |
| **Coordination (COMMAND)** | 1,800 | 800 | 0-3 per task | PR comments (1,200), COT update (600). Output: OPORD/FRAGORD/SITREP. |
| **Authorization (KEYMASTER)** | 1,200 | 400 | 1-2 per task | Authorization chain verification. Output: signing decision + audit entry. |
| **TOTAL (typical task)** | **45,700** | **20,500** | -- | 200-line feature, 3 files, 2 missions, 2 cross-repo deps, 7 tool calls, 1 intel sweep, 1 review, 2 coordination events |

**Total: ~66,200 tokens per typical task.**

At Claude Opus 4 pricing: ~$0.69 input + ~$1.54 output = **~$2.23 per task.**

OTRC's budget is the highest among the proposals because of the mandatory review step (OVERWATCH) and the five-agent structure. OTRC does not apologize for this. The redundancy buys correctness. A $2.23 correct patch is cheaper than a $1.50 patch that introduces a regression requiring three more patches to fix.

### 7.2 Budget Optimizations

1. **Role-scoped tool registration:** Each agent sees only its authorized tools. COMMAND sees 3 tools; STRIKER sees 6. Savings: ~120 tokens per agent.
2. **Orbit-filtered retrieval:** LEO sweep for tactical queries, GEO sweep for strategic queries. Avoids evaluating irrelevant entries. Savings: ~500 tokens per retrieval.
3. **INTSUM compression:** SENTINEL compresses intelligence summaries to key findings only — no reasoning, no hedging. Savings: ~400 tokens per INTSUM.
4. **Review skip for trivial patches:** Patches under 20 lines with a single file can be fast-tracked (OVERWATCH review reduced to header-only scan). Savings: ~4,000 tokens per trivial patch.
5. **OPORD caching:** If a task decomposition produces subtasks identical to a previous OPORD, COMMAND reuses the cached plan. Savings: ~3,000 tokens per reuse.

### 7.3 Budget Enforcement

| Threshold | OTRC Designation | Action |
|-----------|-----------------|--------|
| 70% | YELLOW | Reserve status. Reserve becomes available. |
| 85% | ORANGE | Mission shortening. Complete current mission, defer remaining. |
| 92% | RED | Forced completion. STRIKER must produce output within 2 LLM calls. |
| 97% | CRITICAL | Emergency. Produce partial patch. Mark `X-Partial: true`. |
| 100% | ABORT | Hard stop. Submit what exists. All agents halt. |

---

## 8. Testing Strategy

### 8.1 Provider Testing

- **Mock provider:** `but-ai-provider-mock` returns deterministic responses keyed by input hash. All integration tests use this. No live API calls in CI.
- **Provider conformance suite:** Validates JSON-RPC interface compliance for each provider plugin. 12 test cases covering tool calling, streaming, structured output, and error handling.

### 8.2 Patch Workflow (Kill Chain Validation)

- **Full kill chain tests:** Task -> COMMAND -> SENTINEL -> STRIKER -> OVERWATCH -> KEYMASTER. End-to-end on a test repository. 8 test cases: single file, multi file, cross-repo, partial (budget exhaustion), revision cycle, trivial (fast-track), blocked (missing dependency), abort.
- **Patch round-trip:** Generate INDEX.patch, apply to test repo, verify result. Test cases include: additions, deletions, renames, and mixed operations.
- **Strike assessment validation:** Verify STRIKER's collateral count is accurate (lines changed outside the target range = 0 for clean patches).

### 8.3 Cross-Repo Coordination

- **Mock forge:** HTTP server implementing `ForgeAdapter`. Returns canned PRs, validates OPORD/SITREP schema.
- **Combined operations test:** Three repos, one mock forge, one multi-mission task. Verify: COT accuracy, OPORD delivery, SITREP collection, dependency blocking behavior.

### 8.4 Token Budget

- **Budget enforcement at each threshold:** Inject known token counts, verify YELLOW/ORANGE/RED/CRITICAL/ABORT triggers correctly.
- **Reserve management:** Verify reserve activation at 70% and correct distribution to the active phase.
- **Accuracy benchmarks:** 30 test tasks, compare estimated vs actual usage, report mean absolute error per phase.

### 8.5 Memory System

- **Orbital promotion tests:** Store in LEO, simulate observations, verify promotion to MEO at 3 observations, to GEO at 5.
- **Orbital decay tests:** Store in MEO, advance time without observations, verify decay to LEO after 14 days.
- **Deorbit tests:** Store in LEO, advance past TTL, verify graveyard transition and eventual deletion.
- **Conjunction scoring tests:** Store entries across all orbits, query with known tags, verify scoring matches expected values.
- **Compaction rescue tests:** Checkpoint, clear context, rehydrate, verify high-RCS entries are recovered first.

---

## 9. Git Config Keys (Complete)

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.forge` | string | `github` | Forge type |
| `but-ai.forgeApiUrl` | string | per forge | Forge API URL |
| `but-ai.forgeToken` | string | env | Forge token |
| `but-ai.agent.tokenBudget` | int | `66000` | Max tokens per task |
| `but-ai.agent.reserveFraction` | float | `0.10` | Operational reserve |
| `but-ai.agent.maxRevisionCycles` | int | `3` | Max STRIKER-OVERWATCH cycles |
| `but-ai.orbit.branchPrefix` | string | `refs/but-ai/orbit` | Memory branch prefix |
| `but-ai.orbit.dsnBranch` | string | `refs/but-ai/orbit/dsn` | Deep Space Network shared branch |
| `but-ai.orbit.leoTtl` | int | `259200` | LEO TTL (3 days) |
| `but-ai.orbit.meoTtl` | int | `1814400` | MEO TTL (21 days) |
| `but-ai.orbit.geoTtl` | int | `7776000` | GEO TTL (90 days) |
| `but-ai.orbit.graveyardGrace` | int | `604800` | Graveyard grace period (7 days) |
| `but-ai.orbit.leoToMeoThreshold` | int | `3` | Observations for LEO->MEO promotion |
| `but-ai.orbit.meoToGeoThreshold` | int | `5` | Observations for MEO->GEO promotion |
| `but-ai.orbit.maxRetrieval` | int | `5` | Max entries per conjunction assessment |
| `but-ai.armory.provider` | string | `openwallet` | Signing provider |
| `but-ai.armory.keyStore` | string | `~/.openwallet/keys` | Key store path |
| `but-ai.armory.rotationDays` | int | `60` | Key rotation interval |
| `but-ai.armory.requireOverwatchApproval` | bool | `true` | Require OVERWATCH approval before signing |
| `but-ai.armory.auditBranch` | string | `refs/but-ai/orbit/audit` | Audit trail branch |

---

## 10. Migration Path

OTRC executes migrations as phased operations:

**Phase 1 (Parallel Operations):** `but-ai mcp` deployed alongside legacy MCP server. Both operational. `gitbutler_update_branches` available on both endpoints. Monitoring confirms functional equivalence.

**Phase 2 (Cutover):** Legacy MCP entry point updated to delegate to `but-ai mcp`. All traffic flows through new server. Old code remains in place as fallback.

**Phase 3 (Decommission):** After 30-day burn-in with zero incidents, legacy MCP code is removed.

Each phase has defined success criteria and abort conditions. Phase 3 does not execute without Phase 2 running clean for 30 days. This is how OTRC operates: trust is earned by performance, not assumed by schedule.

---

*Orbital parameters: i=45deg, alt=500km, epoch=2026-03-28T00:00:00Z*
*Classification: UNCLASSIFIED // FOUO*
*Track status: Proposal filed*
