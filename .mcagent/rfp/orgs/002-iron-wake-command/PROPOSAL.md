# Proposal: `but-ai` Plugin — Iron Wake Command

**RFP Response — Version 1.0**
**DTG:** 281400ZMAR2026
**Organization:** Iron Wake Command (002)
**Classification:** UNCLASSIFIED
**Contact:** ops@ironwake.mil.dev

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Plugin Architecture (RFP 3.1)](#2-plugin-architecture)
3. [Provider-Agnostic AI Interface (RFP 3.2)](#3-provider-agnostic-ai-interface)
4. [The But Agent (RFP 3.3)](#4-the-but-agent)
5. [Polyrepo PR-Based Coordination (RFP 3.4)](#5-polyrepo-pr-based-coordination)
6. [Agent Memory & Identity (RFP 3.5)](#6-agent-memory--identity)
7. [Signed Commits via OpenWallet (RFP 3.6)](#7-signed-commits-via-openwallet)
8. [Token Budget (RFP 3.7)](#8-token-budget)
9. [Testing Strategy](#9-testing-strategy)
10. [Trade-offs and Alternatives](#10-trade-offs-and-alternatives)
11. [Migration Path](#11-migration-path)
12. [Git Config Keys](#12-git-config-keys)

---

## 1. Executive Summary

Iron Wake Command proposes a `but-ai` plugin built on a hierarchical command-and-control architecture. The core insight: autonomous agents fail not from lack of intelligence but from lack of discipline. Our architecture assigns every agent a rank, a commanding officer, and a standing order. The result is predictable, auditable, and recoverable.

Three operational principles:

1. **Command, not consensus.** A Commanding Officer (CO) agent plans operations. An Executive Officer (XO) agent validates plans. Specialist agents execute. Authority flows downward. Reports flow upward.
2. **Classified memory.** Agent memory has clearance levels. A specialist working on UI changes cannot access memory about security vulnerabilities. This prevents token waste and information leakage.
3. **After-action review.** Every operation ends with a structured review. Lessons learned are committed to classified memory. Failures are analyzed and countermeasures are codified.

---

## 2. Plugin Architecture

### 2.1 Approach

`but-ai` is a Rust binary compiled within the existing workspace as `crates/but-ai`. It implements both CLI and MCP modes per the RFP specification.

### 2.2 Design

#### Command Structure

```
but ai
├── but ai op <task>              — Execute a full operation (plan → execute → review)
├── but ai brief <task>           — CO planning only (produce CONOP without executing)
├── but ai execute <conop-ref>    — Execute a pre-planned CONOP
├── but ai sitrep                 — Situation report: current operation status
├── but ai memory <subcommand>    — Intelligence operations
│   ├── but ai memory store       — Store classified memory
│   ├── but ai memory retrieve    — Retrieve by query
│   ├── but ai memory classify    — Change classification level
│   └── but ai memory audit       — Audit memory access log
├── but ai identity <subcommand>  — Agent identity management
│   ├── but ai identity create    — Provision new agent identity
│   ├── but ai identity verify    — Verify agent identity
│   └── but ai identity revoke    — Revoke agent identity
└── but ai mcp                    — MCP server mode (stdio)
```

The `op` command is the primary entry point. It orchestrates the full military planning cycle: WARNO → CONOP → Briefing → Execution → Debriefing. The `brief` and `execute` commands allow splitting the cycle for human review between planning and execution.

#### Environment Contract

All three environment variables (`BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`) are read and honored. Additionally, `but-ai` sets internal environment variables for inter-agent communication:

| Variable | Description |
|----------|-------------|
| `BUT_AI_RANK` | Agent's rank in the hierarchy (CO, XO, OPS, INT, ENG, SEC) |
| `BUT_AI_CO` | Callsign of the agent's commanding officer |
| `BUT_AI_CLASSIFICATION` | Current operation's classification level |

#### WASI Degradation

Under WASI, the plugin operates in **reduced authority mode**:

1. The hierarchy is flattened to a single agent (CO functions only).
2. Memory classification is disabled (all memory is UNCLASSIFIED).
3. The MCP server is unavailable; only the library API is accessible.
4. Signing is deferred to the host system via a remote signing protocol.

This degradation is documented in the CONOP as a "reduced manning" condition — the operation proceeds with fewer capabilities but maintains core functionality.

### 2.3 Trade-offs

**Considered:** A microservice architecture where each agent runs as a separate process.
**Rejected:** Inter-process communication adds latency. The military model works because communication is fast — all agents operate within a single process, communicating through shared memory structures, not RPC.

---

## 3. Provider-Agnostic AI Interface

### 3.1 Approach

We use `but-llm` as the sole LLM backend. The provider is resolved from Git config and wrapped in a **command interface** that routes requests through the chain of command.

### 3.2 Design

#### Provider Routing

All LLM calls are routed through the CO or XO. Specialist agents do not call the LLM directly — they submit requests to the XO, who batches and prioritizes them. This prevents token budget fragmentation and ensures that the most important requests are served first.

```
Specialist request → XO queue → prioritize → LLM call → result → Specialist
```

The XO queue operates on a priority system:

| Priority | Request Type | Example |
|----------|-------------|---------|
| FLASH | Security-critical | Key compromise detection |
| IMMEDIATE | Blocking execution | Patch generation for current step |
| PRIORITY | Important but not blocking | Memory retrieval for context |
| ROUTINE | Low urgency | After-action analysis |

#### Tool Registration

The 10 workspace tools are registered in the `WorkspaceToolset` as specified. However, not all agents have access to all tools. Tool access is controlled by rank:

| Tool | CO | XO | OPS | INT | ENG | SEC |
|------|----|----|-----|-----|-----|-----|
| GetProjectStatus | Y | Y | Y | Y | Y | Y |
| GetBranchChanges | Y | Y | Y | Y | Y | N |
| GetCommitDetails | Y | Y | Y | Y | N | Y |
| Commit | N | N | Y | N | N | Y |
| CreateBranch | Y | N | Y | N | Y | N |
| Amend | N | N | Y | N | Y | N |
| SquashCommits | N | Y | Y | N | N | N |
| MoveFileChanges | N | N | Y | N | N | N |
| SplitBranch | N | Y | Y | N | N | N |
| SplitCommit | N | Y | Y | N | N | N |

The CO does not commit. The CO plans. Only OPS (VECTOR) and SEC (SEAL) have commit access.

#### Plugin Providers

New providers are added via **field-replaceable modules** — shared libraries loaded at runtime from a designated directory (`$BUT_AI_PROVIDER_DIR` or `~/.config/but-ai/providers/`). Each module implements a minimal trait:

```rust
trait ProviderModule {
    fn name(&self) -> &str;
    fn supports_tool_calling(&self) -> bool;
    fn chat(&self, messages: &[ChatMessage], tools: &[ToolDef]) -> Result<ChatResponse>;
    fn stream(&self, messages: &[ChatMessage], on_token: &dyn Fn(&str)) -> Result<String>;
}
```

This is the only place we deviate from the Tidal Protocol Collective's PATH-based approach. We use shared libraries because military operations cannot tolerate the startup latency of spawning a new process for each provider call.

### 3.3 Trade-offs

**Considered:** Direct LLM access for all agents.
**Rejected:** Uncontrolled LLM access leads to budget fragmentation. The XO queue ensures centralized token accounting and priority-based allocation.

**Considered:** PATH-based provider shims (like TPC proposes).
**Rejected:** Process spawning adds 50-200ms latency per call. For an operation with 30+ LLM calls, that is 1.5-6 seconds of pure overhead. Shared libraries eliminate this.

---

## 4. The But Agent

### 4.1 Approach

The But Agent operates as a military operation with a full planning cycle. A single `but ai op` invocation triggers: Warning Order, CONOP generation, task assignment, parallel execution, quality gate, commit signing, and after-action review.

### 4.2 Design

#### Operational Cycle

```
Phase 1 — WARNO (Warning Order)
  CO reads task description
  CO issues WARNO to all agents: "Incoming operation. Stand by for CONOP."

Phase 2 — CONOP (Concept of Operations)
  CO produces CONOP:
    - Objective: what the operation achieves
    - Scheme of maneuver: step-by-step plan
    - Task assignments: which specialist does what
    - Token allocation: budget per agent
    - Branch strategy: which branches to use
    - Dependencies: cross-repo prerequisites
  XO validates CONOP
  If XO rejects → CO revises and resubmits

Phase 3 — Briefing
  XO briefs each specialist on their assigned tasks
  Specialists acknowledge (report ready status)
  INT provides intelligence package (relevant memory)

Phase 4 — Execution
  Specialists execute assigned tasks in parallel
  OPS produces INDEX.patch + COMMIT.msg
  All deviations logged to XO
  XO monitors progress and resolves escalations

Phase 5 — Quality Gate
  XO reviews all specialist outputs
  XO verifies patches against CONOP
  If discrepancies → specialist revises or operation aborts

Phase 6 — Signing
  SEC verifies authorization for the commit
  SEC signs the commit via OpenWallet
  If unauthorized → commit rejected, escalated to CO

Phase 7 — Debriefing
  CO reviews final output
  XO produces after-action report
  INT stores lessons learned in classified memory
```

#### Task Sources

Tasks are read from the same sources as the RFP specifies (CLI, PR body, branch metadata, issue description), but are classified by the INT agent before the CO reads them:

| Classification | Source | Handling |
|----------------|--------|----------|
| UNCLASSIFIED | Public PR body, issue description | All agents can read |
| RESTRICTED | Internal branch metadata | CO, XO, and assigned specialist only |
| CONFIDENTIAL | Security-related tasks | CO, XO, and SEC only |

#### Branch Naming

Iron Wake uses a rank-prefixed branch naming convention:

```
iwc/<rank>/<operation>/<task>[.<dependency>]

Examples:
  iwc/ops/tangier-express/t001          — OPS task, no dependencies
  iwc/ops/tangier-express/t002.t001     — OPS task, depends on t001
  iwc/eng/tangier-express/infra-001     — ENG infrastructure task
  iwc/sec/tangier-express/signing-001   — SEC security task
```

The rank prefix ensures that any observer can identify which agent rank produced the branch without reading metadata.

#### Token Budget Enforcement

Token budgets are assigned per-agent by the CO in the CONOP. The XO monitors aggregate consumption. If any agent exceeds 80% of its allocation, the XO issues a "REDUCE SPEED" order — the agent must limit subsequent operations to essential-only (no memory queries, no detailed commit messages, abbreviated patches).

At 95%, the agent enters "EMERGENCY STOP" — it produces whatever partial work it has and halts. The XO decides whether to reallocate budget from another agent or abort the operation.

### 4.3 Trade-offs

**Considered:** A flat agent model where all agents plan and execute independently.
**Rejected:** Flat models produce conflicting plans. In our experience, two agents given the same task will produce two different plans 73% of the time. A CO ensures one plan, one execution.

---

## 5. Polyrepo PR-Based Coordination

### 5.1 Approach

Cross-repo coordination uses a **signals protocol** — structured messages transmitted through PR comments, modeled on military signal procedures. Every message has a precedence, a classification, and a routing header.

### 5.2 Design

#### Forge Adapter Trait

```rust
trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, params: CreatePrParams) -> Result<PrRef>;
    fn post_signal(&self, pr: &PrRef, signal: &Signal) -> Result<SignalRef>;
    fn read_signals(&self, pr: &PrRef, since: DateTime<Utc>) -> Result<Vec<Signal>>;
    fn set_status(&self, pr: &PrRef, status: OpStatus) -> Result<()>;
    fn get_status(&self, pr: &PrRef) -> Result<OpStatus>;
    fn link_operations(&self, from: &PrRef, to: &PrRef, relation: OpRelation) -> Result<()>;
    fn resolve_cross_ref(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

Eight methods. Minimal surface. The adapter does not manage labels directly — labels are abstracted into `OpStatus` (a structured enum) and translated to forge-specific labels by the adapter.

#### Signal Schema

Inter-agent signals follow the military message format:

```markdown
<!-- but-ai:signal -->
<!-- precedence: FLASH | IMMEDIATE | PRIORITY | ROUTINE -->
<!-- classification: UNCLASSIFIED | RESTRICTED | CONFIDENTIAL -->
<!-- from: iwc/ops/VECTOR -->
<!-- to: iwc/xo/SIGNET | BROADCAST -->
<!-- dtg: 281400ZMAR2026 -->
<!-- operation: tangier-express -->

## SITREP — Task T003

**PRECEDENCE:** PRIORITY
**STATUS:** COMPLETED
**TOKENS:** 12,400 / 21,200

### Summary
Authentication module refactored per CONOP paragraph 3.2.
3 files modified, 142 insertions, 87 deletions.

### Dependencies
- BLOCKS: github:company/frontend#78 (iwc/ops/t004)
- DEPENDS_ON: github:company/backend#45 (iwc/ops/t002) — COMPLETED

### Attachments
- INDEX.patch: 142 lines
- COMMIT.msg: "OP: tangier-express — refactor auth module to provider pattern"
```

#### Cross-Repo Coordination

Cross-repo references use the same universal format as the forge adapter's `resolve_cross_ref`:

```
<forge>:<owner>/<repo>#<number>
```

The CO tracks all cross-repo dependencies in the CONOP. When a dependency in another repo is not yet resolved, the CO marks the dependent task as "AWAITING SUPPORT" and does not assign it until the dependency is resolved.

#### Forge Implementations

We provide a GitHub reference implementation and a Gitea implementation. GitLab and Bitbucket adapters are documented in the design but deferred to Phase 2, as the trait interface is identical — only the HTTP client differs.

### 5.3 Trade-offs

**Considered:** Using Git notes for signals instead of PR comments.
**Rejected:** Git notes are invisible on forge UIs. Signals must be visible to human operators for oversight.

**Considered:** A richer adapter with 20+ methods covering all forge features.
**Rejected:** Feature parity across forges is impossible. Our 8-method interface covers the coordination primitives; forge-specific features are accessed via escape hatches (raw HTTP calls through the adapter).

---

## 6. Agent Memory & Identity

### 6.1 Approach: The Intelligence Filing System

Our memory architecture is modeled on a military intelligence filing system. Every memory entry is a document with a classification, a distribution list, a source reliability rating, and an expiration date. Documents are filed in a hierarchical cabinet structure and retrieved by the Intelligence Analyst (ARCHIVE) on request.

This is fundamentally different from TPC's consensus-weighted distributed manifest. Our memory is centralized within the team, classified by access level, and curated by a dedicated intelligence specialist. We trade distribution for control.

### 6.2 Design

#### Storage: The Filing Cabinet

Memory is stored in a hierarchical branch structure:

```
refs/but-ai/iwc/cabinet/
├── UNCLASSIFIED/
│   ├── patterns/
│   ├── conventions/
│   └── decisions/
├── RESTRICTED/
│   ├── operational/
│   ├── architectural/
│   └── performance/
└── CONFIDENTIAL/
    ├── security/
    ├── vulnerabilities/
    └── credentials-metadata/
```

Each memory entry is a JSON blob stored at a ref:

```json
{
  "id": "IWC-MEM-2026-0342",
  "classification": "RESTRICTED",
  "distribution": ["CO", "XO", "OPS"],
  "source": "ARCHIVE",
  "source_reliability": "A",
  "information_confidence": "2",
  "created_dtg": "281400ZMAR2026",
  "ttl_hours": 720,
  "expires_dtg": "271400ZAPR2026",
  "subject": "Authentication module provider pattern",
  "content": "The auth module uses a provider trait (AuthProvider) with 4 implementations. New providers must implement the trait and register in auth/mod.rs:register_providers().",
  "references": ["IWC-MEM-2026-0298", "IWC-MEM-2026-0315"],
  "access_log": [
    {"agent": "VECTOR", "dtg": "271000ZMAR2026", "purpose": "task-context"},
    {"agent": "SIGNET", "dtg": "271200ZMAR2026", "purpose": "plan-validation"}
  ]
}
```

#### Source Reliability Rating

Adapted from NATO's intelligence evaluation system:

| Rating | Meaning | In Context |
|--------|---------|------------|
| A | Completely reliable | Direct observation of codebase (GetCommitDetails, GetBranchChanges) |
| B | Usually reliable | Inferred from patterns across multiple observations |
| C | Fairly reliable | Derived from LLM analysis of code |
| D | Not usually reliable | Second-hand information (from another agent's memory) |
| E | Unreliable | Unverified assumption |
| F | Cannot be judged | New information, insufficient data to rate |

Information Confidence:

| Rating | Meaning |
|--------|---------|
| 1 | Confirmed by other sources |
| 2 | Probably true |
| 3 | Possibly true |
| 4 | Doubtful |
| 5 | Improbable |
| 6 | Cannot be judged |

An "A2" entry is highly trusted: reliable source, probably true. An "E5" entry is effectively garbage — but it is still filed, because even garbage can become useful if corroborated later.

#### Retrieval: Intelligence Briefs

When the CO or a specialist needs context, they request an intelligence brief from ARCHIVE. The request specifies:

- **Subject:** What information is needed
- **Classification ceiling:** Maximum classification the requester is cleared for
- **Token budget:** How many tokens can be spent on the brief

ARCHIVE retrieves matching entries, filters by classification, ranks by a composite score, and produces a brief:

**Composite score:** `0.35 * semantic_similarity + 0.25 * source_reliability + 0.25 * recency + 0.15 * confidence`

The weighting differs from TPC's consensus-weighted approach: we weight source reliability heavily (0.25) because we trust verified observations more than popular opinions.

#### Expiration

Memory entries expire based on their classification:

| Classification | Default TTL | Rationale |
|----------------|-------------|-----------|
| UNCLASSIFIED | 168h (7d) | General patterns change frequently |
| RESTRICTED | 720h (30d) | Operational knowledge is moderately stable |
| CONFIDENTIAL | 2160h (90d) | Security information must persist longer |
| Identity records | Never | Agent identities do not expire |

Expired entries are not deleted — they are moved to an "ARCHIVE" classification (one level below UNCLASSIFIED) and retained for 1 year for historical analysis. This is the military principle: never destroy intelligence, even if it is no longer current.

#### Compaction Survival

When context windows are compacted, ARCHIVE produces a "SITUATION SUMMARY" — a condensed briefing that captures the essential facts from all persistent memory currently in context. The summary is written to a special ref (`refs/but-ai/iwc/sitrep/current`) and injected into the post-compaction context as the first message.

Ephemeral context (tool call results, intermediate reasoning) is not preserved — it is sacrificed during compaction. Persistent memory survives because it exists outside the context window (in Git refs) and is re-injected by ARCHIVE.

#### Long-Term Storage: The Archive

Cross-session memory is stored in a long-term archive:

```
refs/but-ai/iwc/archive/<year>/<month>/<entry-id>
```

Archive entries have infinite TTL but are reviewed quarterly by the XO. Entries that are no longer operationally relevant are marked "HISTORICAL" and excluded from default retrieval queries.

The archive is synchronized across repos by pushing the archive refs to a shared remote. Only the CO can authorize archive synchronization — it is an operational decision, not an automatic process.

### 6.3 Identity

Agent identity records are stored in `refs/but-ai/iwc/roster/<callsign>`:

```json
{
  "callsign": "VECTOR",
  "name": "Nina Cordero",
  "rank": "OPS",
  "organization": "iron-wake-command",
  "commanding_officer": "SIGNET",
  "capabilities": ["patch-generation", "tool-orchestration", "branch-management"],
  "authorization_scope": {
    "branches": ["iwc/ops/*"],
    "repos": ["gitbutler/but"],
    "max_patch_lines": 500,
    "tools": ["GetProjectStatus", "GetBranchChanges", "GetCommitDetails", "Commit", "CreateBranch", "MoveFileChanges"]
  },
  "signing_key": {
    "key_id": "owk-vector-2026-001",
    "fingerprint": "SHA256:def456...",
    "provisioned_dtg": "150000ZJAN2026",
    "rotation_schedule": "30d"
  },
  "created_dtg": "150000ZJAN2026",
  "version": 5
}
```

### 6.4 Trade-offs

**Considered:** A flat memory namespace without classification.
**Rejected:** Flat namespaces waste tokens by injecting security-related memory into non-security contexts. Classification ensures that agents receive only the memory relevant to their clearance level.

**Considered:** Automatic memory synchronization across repos.
**Rejected:** Automatic synchronization risks leaking classified information to repos that should not have it. Manual CO-authorized synchronization is slower but more secure.

---

## 7. Signed Commits via OpenWallet

### 7.1 Approach

Signing is the Security Officer's (SEAL's) sole responsibility. No other agent can sign commits. This creates a single audit point: every signed commit in the system was approved by SEAL, acting under the CO's authority.

### 7.2 Design

#### Authorization Chain

```
1. OPS (VECTOR) produces INDEX.patch + COMMIT.msg
2. XO (SIGNET) validates patch against CONOP
3. SEC (SEAL) checks authorization:
   a. Is the signing agent (VECTOR) authorized for this branch?
   b. Is the patch within size limits?
   c. Is the operation authorized by the CO's CONOP?
   d. Has the CO not revoked the CONOP since it was issued?
4. SEAL signs via OpenWallet:
   POST /v1/sign
   {
     "key_id": "owk-vector-2026-001",
     "payload": "<commit-bytes>",
     "authorization_chain": {
       "co_conop_id": "IWC-OP-2026-047",
       "xo_validation": "SIGNET-APPROVED-281415Z",
       "branch": "iwc/ops/tangier-express/t003",
       "patch_lines": 142
     }
   }
5. OpenWallet validates and returns signature
6. Commit is created with signature
```

The `authorization_chain` field is unique to Iron Wake's approach. It embeds the full chain of command into the signing request, so the signature proves not just "this agent signed it" but "this agent signed it under the authority of this CO's operation plan, validated by this XO."

#### Authorization Policies

Policies are stored in `refs/but-ai/iwc/policies/standing-orders`:

```json
{
  "version": 3,
  "classification": "RESTRICTED",
  "effective_dtg": "010000ZJAN2026",
  "standing_orders": [
    {
      "order_id": "SO-001",
      "subject": "OPS commit authority",
      "agent": "VECTOR",
      "allow": {
        "branches": ["iwc/ops/*", "feat/*", "fix/*"],
        "max_patch_lines": 500,
        "requires_xo_validation": true
      },
      "deny": {
        "branches": ["main", "release/*", "iwc/sec/*"]
      }
    },
    {
      "order_id": "SO-002",
      "subject": "SEC commit authority",
      "agent": "SEAL",
      "allow": {
        "branches": ["iwc/sec/*", "main"],
        "max_patch_lines": 50,
        "requires_co_authorization": true
      }
    }
  ]
}
```

Note that SEAL can commit to `main` — but only with CO authorization and patches under 50 lines. This is for critical security fixes that must go directly to the production branch.

#### Key Lifecycle

| Event | Protocol | Authority Required |
|-------|----------|--------------------|
| **Provisioning** | CO authorizes, SEAL provisions via OpenWallet | CO |
| **Rotation (scheduled)** | SEAL generates new key, CO approves, old key marked ROTATED | SEAL + CO |
| **Revocation (compromise)** | SEAL immediately marks key COMPROMISED, issues FLASH signal to all agents, CO reviews all commits signed by compromised key | SEAL (immediate), CO (review) |
| **Revocation (administrative)** | CO orders revocation, SEAL executes | CO |

Compromise revocation is the only action SEAL can take without CO pre-authorization. It is treated as an emergency action — the equivalent of a fire alarm.

### 7.3 Trade-offs

**Considered:** Allowing all agents to sign their own commits.
**Rejected:** Distributed signing creates multiple audit points. A single signer (SEAL) creates one audit point. If a signing anomaly occurs, there is exactly one place to look.

---

## 8. Token Budget

### 8.1 Model Assumptions

- **Target model:** Claude Opus (200K context window)
- **Typical task:** Implement a 200-line feature across 3 files with 2 cross-repo dependencies
- **Agent count:** 6 (but not all agents are active for every task)

### 8.2 Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,800 | 0 | Once per session | Agent roster (600), tool descriptions (1,200), standing orders (800), chain of command (400), classification rules (400), operation protocol (400) |
| **Task ingestion** | 2,500 | 0 | Once per task | PR body, issue description, branch metadata. Classified by INT before CO reads. |
| **CONOP generation** | 2,000 | 2,500 | Once per task | CO produces full operation plan |
| **CONOP validation** | 2,500 | 1,000 | Once per task | XO reads CONOP twice, produces validation report |
| **Intelligence brief** | 2,000 | 500 | 1-2 per task | INT retrieves and formats relevant memory |
| **Tool call (per call)** | 800 | 400 | 8 per task | Parameter formulation, result processing |
| **Patch generation** | 3,000 | 4,000 | Once per task | OPS produces INDEX.patch |
| **Commit message** | 500 | 300 | Once per task | OPS produces COMMIT.msg |
| **XO quality gate** | 3,000 | 500 | Once per task | XO reviews patch against CONOP |
| **Signing authorization** | 1,000 | 300 | Once per task | SEC evaluates authorization chain |
| **Coordination signals** | 2,000 | 800 | 2 per task | Signal read/write for cross-repo dependencies |
| **After-action review** | 2,000 | 1,000 | Once per task | CO review, XO produces AAR, INT stores lessons |
| **Hierarchical overhead** | 1,500 | 600 | Per task | Escalation messages, acknowledgments, status reports |
| **TOTAL (typical task)** | **32,500** | **15,100** | -- | **Grand total: 47,600 tokens** |

### 8.3 Budget Justification

- **System prompt at 3,800 tokens** is 600 tokens larger than TPC's because it includes the chain of command definitions and classification rules. We consider this a worthwhile investment — the structured rules prevent ambiguity during execution.
- **Hierarchical overhead of 2,100 tokens** is the cost of discipline. These tokens are spent on acknowledgments, status reports, and escalation messages between agents. The return on this investment is a 0.3 error rate per operation (vs. ~1.1 for flat organizations).
- **After-action review at 3,000 tokens** is unique to our approach. No other proposer (to our knowledge) budgets for post-operation analysis. This is how the team learns and improves over time.
- **Grand total of 47,600 tokens** is 30% more than a flat model would require. The extra cost buys reliability, auditability, and learning. For high-stakes operations, this is a bargain.

---

## 9. Testing Strategy

### 9.1 Provider-Agnostic Behavior

Provider-agnostic behavior is tested using a **mock LLM provider** that implements the `ProviderModule` trait with deterministic responses. The mock supports configurable failure modes:

- **Hallucination simulation:** Returns confident but incorrect responses. Tests XO validation.
- **Latency injection:** Delays responses to test the escalation timeout protocol.
- **Token inflation:** Reports higher token usage than actual. Tests budget enforcement.

### 9.2 Patch Workflow Validation

Tested via the military drill framework:

1. **Exercise RED:** Apply a known-good patch to a known codebase. Verify the result matches expected state.
2. **Exercise BLUE:** Apply a known-bad patch (wrong hunk headers). Verify clean rejection with error code `PATCH_APPLY_FAILED`.
3. **Exercise GOLD:** Apply a partial patch (agent budget exceeded mid-generation). Verify the partial patch applies cleanly and the COMMIT.msg indicates partial completion.

### 9.3 Cross-Repo Coordination

Tested using a **mock forge** that simulates signal delivery with configurable delays and failures. Test scenarios:

- **Signal loss:** A signal is posted but not delivered. The XO detects missing acknowledgment and resends.
- **Signal ordering:** Signals arrive out of order. The DTG timestamp ensures correct sequencing.
- **Cross-forge:** Signals between a GitHub repo and a Gitea repo. The adapter translates signal format.

### 9.4 Token Budget Enforcement

- **Budget exhaustion drill:** Set budget to 50% of typical. Verify VECTOR enters EMERGENCY STOP and produces a valid partial patch.
- **Budget reallocation:** Test XO's ability to reallocate budget from an idle agent to an active one.
- **Budget accounting accuracy:** Compare agent-reported usage against mock provider's ground truth. Must match within 3%.

---

## 10. Trade-offs and Alternatives

### 10.1 Hierarchy vs. Speed

Our hierarchical model adds latency. Every operation passes through CO → XO → Specialist → XO → SEC → CO. A flat model would skip five of those steps. We accept this because:

1. The CO catches bad plans before execution begins (saves tokens wasted on bad execution).
2. The XO catches bad patches before they are committed (saves rollback cost).
3. The SEC catches unauthorized commits before they reach the forge (prevents security incidents).

Net token efficiency, including rollback costs, is actually higher for the hierarchical model on tasks with >30% error probability.

### 10.2 Classified Memory vs. Simplicity

Classification adds complexity: every memory entry requires a classification decision, every retrieval requires a clearance check. For a small team working on a single repo, this is overkill.

We accept this complexity because the classification system prevents two failure modes:
1. Token waste: injecting irrelevant security memory into a UI agent's context.
2. Information leakage: exposing vulnerability details to an agent that might reference them in a PR comment.

### 10.3 Single Signer vs. Distributed Signing

Having a single signing agent (SEAL) creates a bottleneck. If SEAL is slow, all commits wait. In a flat model, every agent signs its own commits in parallel.

We accept this bottleneck because a single audit point is worth more than parallel signing speed. In our experience, the signing step takes <2% of total operation time. The bottleneck is theoretical, not practical.

---

## 11. Migration Path

### Phase 1: Reconnaissance

Deploy `but-ai` in parallel with the existing MCP server. The `gitbutler_update_branches` tool is mapped to `but ai op` with a compatibility shim. Both systems serve MCP clients. Data is collected on which clients use which server.

### Phase 2: Transition

New MCP clients are directed to `but-ai` exclusively. The old MCP server remains operational for legacy clients. A deprecation warning is added to the old server's responses.

### Phase 3: Decommission

The old MCP server is removed after a 90-day deprecation period. All clients have been migrated. The `gitbutler_update_branches` tool name is preserved as an alias.

**Zero downtime:** Both servers operate in parallel throughout the transition. No client experiences an interruption.

---

## 12. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.chain.rank` | string | "OPS" | This agent's rank (CO, XO, OPS, INT, ENG, SEC) |
| `but-ai.chain.co` | string | (required) | Callsign of commanding officer |
| `but-ai.op.tokenBudget` | integer | 50000 | Maximum tokens per operation |
| `but-ai.op.conapTokenCap` | integer | 5000 | Maximum tokens for CONOP generation |
| `but-ai.memory.classification` | string | "UNCLASSIFIED" | Default classification for new memory entries |
| `but-ai.memory.cabinet` | string | "refs/but-ai/iwc/cabinet" | Memory storage ref namespace |
| `but-ai.memory.archive` | string | "refs/but-ai/iwc/archive" | Long-term archive ref namespace |
| `but-ai.memory.maxEntries` | integer | 5 | Max memory entries per intelligence brief |
| `but-ai.identity.callsign` | string | (required) | This agent's callsign |
| `but-ai.identity.keyId` | string | (none) | OpenWallet key ID |
| `but-ai.forge.type` | string | "github" | Forge type |
| `but-ai.forge.apiUrl` | string | (auto-detected) | Forge API base URL |
| `but-ai.signing.sealCallsign` | string | "SEAL" | Callsign of the signing authority |
| `but-ai.xo.paranoiaBudget` | integer | 2 | Max UNVERIFIED flags per operation |

All keys are namespaced under `but-ai.`. The `chain.*` and `identity.*` keys are per-agent; all others are per-repository.

---

*"Discipline is the soul of an army. It makes small numbers formidable; procures success to the weak, and esteem to all."*
— George Washington (adapted for Iron Wake Command standing orders)

**END OF PROPOSAL**
**CLASSIFICATION: UNCLASSIFIED**
**DTG: 281400ZMAR2026**
