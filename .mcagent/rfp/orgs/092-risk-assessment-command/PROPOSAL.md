# Proposal: `but-ai` Plugin -- Risk Assessment Command

**Submitted by:** Risk Assessment Command (Org 092)
**Domain:** Insurance Actuarial -- Tactical Risk Analysis
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

Risk Assessment Command proposes a `but-ai` plugin built on intelligence doctrine. Every piece of information entering the system is classified by threat level. Every agent operates within a command structure with clear authority and accountability. Every memory is reviewed on a mandatory cycle. Nothing is unassessed.

Our central innovation is **threat-assessment memory** -- a memory scheme where every entry is classified RED (high risk, mandatory 24-hour review), AMBER (moderate risk, weekly review), GREEN (routine, monthly review), or BLACK (declassified, archived). This classification drives review cycles, dissemination rules, and expiration timelines. The system does not wait for memories to become stale; it actively reviews them on schedule.

The plugin is implemented in Rust as `crates/but-ai`, using existing `but-llm` and `but-tools` crates. No existing crates are modified.

---

## 2. Plugin Architecture (RFP 3.1)

### Approach

The `but-ai` binary is a PATH-discovered Rust executable. It operates in two modes: CLI and MCP. The CLI is structured as an operational command system.

### Design

**Binary structure:**

```
but-ai
  ├── but ai brief <task>         -- Execute a task (issue operational orders)
  ├── but ai sitrep               -- Situation report (workspace state + threat board)
  ├── but ai mcp                  -- MCP server mode
  ├── but ai agent --task <desc>  -- Autonomous agent mode
  └── but ai classify <ref>       -- Classify a specific memory or change
```

The primary verb is `brief`. Every task execution is a briefing: the agent assesses the situation, identifies threats, issues orders, executes, and produces a SITREP (situation report) as output.

**Crate structure:**

```
crates/but-ai/
  src/
    lib.rs              -- Core library
    command/
      brief.rs          -- Task execution (operational orders)
      sitrep.rs         -- Situation reporting
      classify.rs       -- Threat classification
    intel/
      assessment.rs     -- Threat assessment engine
      classification.rs -- RED/AMBER/GREEN/BLACK classification
      review.rs         -- Mandatory review cycle management
    ops/
      patch.rs          -- INDEX.patch + COMMIT.msg production
      comms.rs          -- Cross-repo coordination (signals)
    security/
      signing.rs        -- OpenWallet integration
      authorization.rs  -- Branch-level access control
  bin/
    main.rs             -- Binary entry point
```

**Environment variables:**

`BUT_WORKSPACE_DIR` maps to AOR (Area of Responsibility). `BUT_OUTPUT_FORMAT` determines reporting style: `human` is a formatted briefing, `json` is structured intelligence, `shell` is telegraphic SITREP format.

**WASI degradation:**

Under WASI, the plugin operates in "garrison mode" -- a defensive posture where the agent can assess threats and generate patches within its AOR but cannot conduct cross-repo operations (no SIGINT -- signals intelligence). Garrison mode restricts the agent to local operations only: patch generation, memory management, and self-assessment. All coordination capabilities are suspended. The agent issues a clear SITREP indicating its reduced capability: `"STATUS: GARRISON MODE. CROSS-REPO OPS SUSPENDED. LOCAL OPS NOMINAL."`

**MCP compatibility:**

Drop-in replacement for the existing MCP server. Server name: `"GitButler Tactical AI"`, version `"2.0.0"`. Full backward compatibility with `gitbutler_update_branches`. The expanded tool surface is exposed as additional MCP tools, each with a threat classification indicating the risk level of its misuse.

### Trade-offs

**Alternative considered: Multiple binaries (one per agent).** Rejected. The command structure requires unified C2 (command and control). Four separate binaries would create coordination overhead and inconsistent state.

**Alternative considered: REST API instead of CLI.** Rejected. REST APIs introduce network complexity. A CLI with stdio is simpler, more secure, and consistent with the existing `but` architecture.

---

## 3. Provider-Agnostic AI Interface (RFP 3.2)

### Approach

We use `but-llm` exclusively. The provider is the communications channel; the intelligence product is ours. The channel does not determine the quality of the intelligence.

### Design

**Provider capability assessment:**

Before operations begin, Sharma assesses each provider's capability:

```rust
pub trait ProviderCapabilityAssessment {
    fn assess_provider(&self, provider: &LLMProvider) -> ProviderThreatProfile;
}

pub struct ProviderThreatProfile {
    pub supports_tool_calling: bool,
    pub supports_streaming: bool,
    pub max_context: usize,
    pub reliability_classification: Classification, // RED/AMBER/GREEN
    pub latency_classification: Classification,
    pub cost_per_token: f64,
}
```

RED-reliability providers (unreliable, high-latency) are restricted to GREEN-classified tasks. AMBER-reliability providers handle AMBER and GREEN tasks. Only GREEN-reliability providers (low error rate, predictable latency) are trusted with RED-classified tasks. This is need-to-know applied to infrastructure: unreliable channels do not carry critical intelligence.

**New provider registration:**

```ini
[but-ai "provider.gemini"]
    adapter = "/path/to/gemini-adapter"
    reliability = "AMBER"
    clearance = "GREEN"    # maximum classification level this provider may handle
```

The `clearance` field is unique to our design. It caps the classification level of tasks routed to this provider. A provider with GREEN clearance cannot process RED-classified tasks, regardless of its technical capability. This is a security control, not a capability limitation.

**Tool exposure:**

All 10 `WorkspaceToolset` tools are registered. Each tool is assigned a threat classification:

| Tool | Classification | Rationale |
|------|---------------|-----------|
| GetProjectStatus | GREEN | Read-only reconnaissance |
| GetBranchChanges | GREEN | Read-only reconnaissance |
| GetCommitDetails | GREEN | Read-only reconnaissance |
| CreateBranch | AMBER | Creates new attack surface |
| MoveFileChanges | AMBER | Alters codebase topology |
| Commit | RED | Irreversible write operation |
| Amend | RED | Modifies history |
| SquashCommits | RED | Modifies history |
| SplitBranch | AMBER | Structural change |
| SplitCommit | AMBER | Structural change |

RED tools require Sharma's validation before execution. AMBER tools require Voss's approval. GREEN tools are available to all agents at all times.

### Trade-offs

**Alternative considered: Provider-agnostic abstraction layer.** Rejected as unnecessary complexity. `but-llm` already provides provider abstraction. We add threat classification on top, not an additional abstraction layer.

---

## 4. The But Agent (RFP 3.3)

### Approach

The agent operates as a four-unit command structure (see AGENTS.md): Voss commands, Sharma analyzes, Reiter executes, Mbeki handles signals. Every task is an operation. Every operation has an operational order (OPORD).

### Design

**Operational lifecycle:**

```
1. INTEL PREP:    Voss reads task, Sharma assesses threats, both query memory
2. OPORD:         Voss issues numbered operational orders with assignments
3. EXECUTE:       Reiter generates patches per OPORD
4. VALIDATE:      Sharma validates patches against risk register
5. DEBRIEF:       INDEX.patch + COMMIT.msg produced; After Action Review logged
```

The After Action Review (AAR) is not optional. Every completed task produces an AAR that assesses: what was planned, what happened, what went right, what went wrong, and what should change. The AAR is stored in threat-assessment memory as an AMBER-classified entry (reviewed weekly).

**Patch production:**

Reiter generates unified diffs against the current index. Every patch includes a threat assessment in the commit message:

```
OPORD 042: Add session token validation middleware

THREAT ASSESSMENT:
- Risk of regression: AMBER (modifies auth pipeline)
- Risk of security vulnerability: GREEN (adds validation, does not remove it)
- Risk of performance impact: GREEN (single function call per request)

VALIDATED: Sharma (85% confidence)
TESTED: Unit tests passing
DEFERRED: Integration test for timeout edge case (filed as OPORD 043)
```

**Branch naming:**

```
ops/<classification>/<agent-id>/<opord-number>[.<dependency>]
```

Example: `ops/AMBER/reiter/042.039` -- AMBER-classified operation, executed by Reiter, OPORD 042, depends on OPORD 039.

The classification in the branch name serves a security function: it signals the sensitivity level of the work to anyone inspecting the branch structure. RED branches are treated with additional care.

**Token budget enforcement:**

Budget enforcement follows military logistics doctrine: when supplies run low, you conserve and prioritize.

| Budget Consumed | Protocol | Action |
|-----------------|----------|--------|
| 0-70% | Normal Ops | Full operational capability |
| 70-85% | Conservation | Reduce SITREP frequency, skip GREEN assessments |
| 85-95% | Austerity | Single-pass execution, minimal validation |
| 95%+ | Emergency Halt | Produce partial patch, issue final SITREP, halt |

At Emergency Halt, the partial INDEX.patch is valid and applies cleanly. The final SITREP lists all incomplete OPORDs, their priority, and the estimated tokens required to complete them. This information enables the next operation to pick up where this one left off.

**Progress reporting:**

```json
{
  "phase": "EXECUTE",
  "agent": "reiter",
  "rank": "operator",
  "opord": 42,
  "classification": "AMBER",
  "tokens_used": 22000,
  "tokens_budget": 50000,
  "threat_level": "AMBER",
  "patches_generated": 1,
  "patches_validated": 0,
  "risk_register_entries": 3,
  "operational_readiness": 0.85
}
```

### Trade-offs

**Alternative considered: Flat agent (single agent, no command structure).** Rejected. A single agent conflates intelligence analysis (what to do) with execution (doing it) and validation (was it done correctly). Conflating these roles is how intelligence failures happen. Separation of analysis, execution, and validation is foundational to RAC's methodology.

**Alternative considered: Democratic agent coordination (consensus-based).** Rejected. Consensus is slow. In a time-critical operation (token budget is time), someone must make decisions. The CO decides. The XO validates. The Operator executes. This is not authoritarianism; it is efficiency under constraint.

---

## 5. Polyrepo PR-Based Agent Coordination (RFP 3.4)

### Approach

Cross-repo coordination is modeled as **signals intelligence** (SIGINT). Mbeki monitors and manages all inter-repository communications with the discipline of a signals officer handling classified traffic.

### Design

**Forge adapter (Signals Interface):**

```rust
pub trait SignalsChannel: Send + Sync {
    fn transmit(&self, target: &RepoRef, message: &SignalsMessage) -> Result<MessageId>;
    fn receive(&self, since: DateTime<Utc>) -> Result<Vec<SignalsMessage>>;
    fn assess_channel(&self, pr: &PrId) -> Result<ChannelStatus>;
    fn classify_traffic(&self, pr: &PrId, classification: Classification) -> Result<()>;
    fn enumerate_contacts(&self, pr: &PrId) -> Result<Vec<RepoRef>>;
}
```

GitHub reference implementation provided. The trait maps to forge operations: `transmit` = create comment, `receive` = list comments, `assess_channel` = check PR status.

**SITREP schema (PR comment format):**

```json
{
  "$schema": "but-ai/signals/v1",
  "type": "opord | sitrep | intel | handoff | budget",
  "classification": "GREEN",
  "from": {
    "callsign": "mbeki",
    "unit": "092-rac",
    "station": "owner/repo"
  },
  "to": {
    "callsign": "target",
    "unit": "target-org",
    "station": "owner/other-repo"
  },
  "traffic": {
    "opord_ref": "ops/AMBER/reiter/042",
    "status": "EXECUTE",
    "dependencies": ["owner/other-repo#17"],
    "threat_level": "AMBER",
    "budget": { "consumed": 22000, "allocated": 50000 },
    "confidence": 0.85
  },
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Embedded in PR comments as:

````markdown
```but-ai-signals
{ ... }
```
````

**Information security in coordination:**

All SITREP messages include a classification level. Mbeki enforces dissemination rules:

- RED traffic: sent only to agents with RED clearance, never in public PR comments (uses draft PRs or private comments where available)
- AMBER traffic: sent to agents with AMBER or higher clearance
- GREEN traffic: unrestricted

If a forge does not support private comments, RED traffic is transmitted as a reference to a signed, encrypted blob stored on the memory branch, with the decryption key shared out-of-band.

**Dependency tracking:**

Mbeki maintains an **Order of Battle** (ORBAT) -- a structured map of all active operations across repositories, their dependencies, their classification levels, and their current status. The ORBAT is stored as a RED-classified memory entry (mandatory 24-hour review) and updated with every coordination event.

### Trade-offs

**Alternative considered: Unclassified coordination (all messages public).** Rejected. Some operational details (threat assessments, vulnerability information) should not be in public PR comments. Classification-aware coordination is a security feature, not overhead.

**Alternative considered: Webhook-based real-time coordination.** Rejected per RFP (no services beyond forge). Polling is less efficient but requires no infrastructure.

---

## 6. Agent Memory and Identity (RFP 3.5)

### Approach: Threat-Assessment Memory

Every memory entry in RAC's system is classified:

- **RED**: High-probability, high-impact information. Mandatory review every 24 hours. Disseminated only on a need-to-know basis.
- **AMBER**: Moderate-risk information. Mandatory review every 7 days. Available to all team agents.
- **GREEN**: Routine information. Review every 30 days. Available to all agents including external.
- **BLACK**: Declassified. Archived historical data. Available indefinitely for pattern analysis.

Classification is not a tag. It is an operational constraint that determines review frequency, dissemination scope, retention policy, and retrieval priority.

### Design

**Storage:**

Memory is stored on a classified branch structure:

```
refs/but-ai/intel/<agent-id>/
  red/
    threat-001.json    -- "Auth module has unpatched vulnerability" (24h review)
    threat-002.json    -- "Provider X returns inconsistent tool call format" (24h review)
  amber/
    assessment-001.json -- "Module X architecture uses middleware pattern" (7d review)
    assessment-002.json -- "Team prefers explicit error handling" (7d review)
    aar-001.json        -- "After Action Review: OPORD 040" (7d review)
  green/
    intel-001.json     -- "Function naming uses snake_case" (30d review)
    intel-002.json     -- "CI runs on every push to feat/* branches" (30d review)
  black/
    archive-001.json   -- Declassified: "Branch feat/old was deleted 2026-03-01"
    archive-002.json   -- Declassified: "Bug in parser fixed in commit abc123"
```

**Memory entry structure:**

```json
{
  "id": "threat-001",
  "classification": "RED",
  "content": "The authentication module has a known vulnerability in session token parsing",
  "source": "OPORD 038, validated by Sharma at 85% confidence",
  "created_at": "2026-03-25T10:00:00Z",
  "last_reviewed": "2026-03-28T08:00:00Z",
  "next_review": "2026-03-29T08:00:00Z",
  "review_count": 3,
  "access_count": 7,
  "dissemination": ["voss", "sharma", "reiter"],
  "declassification_date": "2026-04-25T00:00:00Z",
  "confidence": 0.85,
  "related_entries": ["assessment-001"],
  "embedding_vector": [0.12, -0.34, ...]
}
```

Key fields unique to our scheme:

- `next_review`: The date when this entry must be reviewed, regardless of access. This is not a TTL -- the entry does not expire on this date. It is a mandatory inspection.
- `declassification_date`: When the entry automatically drops one classification level (RED -> AMBER -> GREEN -> BLACK). Declassification is the normal lifecycle of intelligence: today's urgent threat is tomorrow's historical footnote.
- `confidence`: Sharma's assessed confidence in the entry's accuracy. Entries below 0.5 confidence are flagged for additional investigation.

**Relevance scoring:**

```
score = 0.30 * classification_priority(entry.classification)
      + 0.25 * embedding_similarity(query, entry)
      + 0.20 * confidence(entry)
      + 0.15 * review_recency(entry)
      + 0.10 * access_frequency(entry)
```

`classification_priority` is the key differentiator: RED entries always score higher than AMBER, which always score higher than GREEN. This means urgent, high-risk information is always surfaced first, regardless of semantic similarity. A RED-classified memory about a security vulnerability in an unrelated module will rank higher than a GREEN-classified memory about the exact module being worked on. This is intentional -- threats take priority.

**Review cycles:**

Sharma manages review cycles. At each review:

1. **Is the entry still accurate?** If not, update or declassify.
2. **Is the classification still appropriate?** Downgrade if the threat has been mitigated.
3. **Should the entry be consolidated?** Multiple entries about the same topic are merged.
4. **Is confidence still justified?** If the underlying evidence has changed, re-estimate.

Review cycles are budgeted separately from task execution. The review budget is a fixed allocation (5% of total budget per session) that Sharma uses exclusively for memory maintenance.

**Expiration (declassification timeline):**

Entries do not expire. They declassify. The timeline:

| Classification | Default Declassification Period | Trigger |
|---------------|-------------------------------|---------|
| RED | 30 days -> AMBER | Automatic unless re-certified by Voss |
| AMBER | 60 days -> GREEN | Automatic unless re-certified by Sharma |
| GREEN | 90 days -> BLACK | Automatic |
| BLACK | Permanent | Never declassifies further; archived |

Declassified (BLACK) entries remain in the archive indefinitely. They are not deleted because historical intelligence has pattern-analysis value. RAC's experience with the Dutch Flood Miss taught them that discarded historical data can contain early warnings of future threats.

**Compaction survival:**

During compaction, entries are retained in classification order: RED first, then AMBER, then GREEN. BLACK entries are already archived and not in active context. The compaction algorithm preserves all RED entries, as many AMBER entries as space permits, and summarizes GREEN entries into a single "GREEN BRIEFING" that lists entry titles and classifications without full content.

**Need-to-know compartmentalization:**

Not all agents see all memories. RED entries are compartmentalized:

- Voss sees all RED entries (CO has full picture)
- Sharma sees RED entries related to her analytical domain
- Reiter sees RED entries only when they affect his current OPORD
- Mbeki sees RED entries only when they affect coordination

This prevents a compromised agent from exfiltrating the entire RED memory store. Each agent's context window contains only the classified information it needs for its current task.

**Long-term storage:**

The BLACK archive serves as long-term storage. Cross-repo long-term memory:

```
refs/but-ai/shared-intel/
  patterns/       -- Declassified threat patterns observed across repos
  lessons/        -- Declassified After Action Reviews
  indicators/     -- Declassified early warning indicators
```

**Identity:**

Agent identity is encoded as a **security clearance document**:

```json
{
  "agent_id": "reiter",
  "unit": "092-rac",
  "rank": "operator",
  "clearance": "RED",
  "capabilities": ["patch_generation", "code_execution"],
  "authorization": {
    "branches": ["ops/*", "feat/*"],
    "max_patch_lines": 1000,
    "repos": ["owner/main-repo"],
    "classification_access": ["RED", "AMBER", "GREEN"]
  },
  "signing_key": "openwallet:092-rac:reiter",
  "issued_at": "2026-03-01T00:00:00Z",
  "clearance_review_date": "2026-06-01T00:00:00Z"
}
```

The `clearance_review_date` triggers a mandatory re-evaluation of the agent's clearance level. This is a standard security practice: clearances are not permanent.

### Trade-offs

**Alternative considered: TTL-based expiration.** Rejected. TTLs are passive -- they wait for time to pass. RAC's review cycles are active -- they inspect entries on schedule. Active review catches stale entries that TTL-based systems would retain (because the TTL has not expired yet) or discard prematurely (because the TTL was set too short).

**Alternative considered: Unclassified memory (all entries equal).** Rejected. Not all information is equally important. A memory about a security vulnerability must be handled differently from a memory about code formatting preferences. Classification enforces appropriate handling.

**Alternative considered: Per-entry TTL with renewal.** Considered and partially incorporated. Declassification timelines function similarly to TTLs but with a crucial difference: declassification does not delete the entry. It reduces its priority and moves it to the archive. In intelligence work, old data is not worthless -- it is historical context.

---

## 7. Signed Commits via OpenWallet (RFP 3.6)

### Approach

Every commit is signed. Every signature is verified. Every verification is logged. The signature chain is the authorization chain. No unsigned operations. No exceptions.

### Design

**Key hierarchy:**

```
Unit Key (092-rac)
  ├── CO Key (voss)            -- Strategic authority
  ├── XO Key (sharma)          -- Analytical authority
  ├── Operator Key (reiter)    -- Execution authority
  └── Signals Key (mbeki)      -- Communication authority
```

**Authorization model:**

RAC's authorization model is role-based with classification-aware constraints:

| Role | Branch Access | Classification Access | Write Operations |
|------|--------------|----------------------|-----------------|
| CO (Voss) | All branches | All classifications | Architecture, config |
| XO (Sharma) | All branches (read), ops/* (write) | All classifications | Validation annotations |
| Operator (Reiter) | ops/*, feat/* | AMBER, GREEN | Patches, commits |
| Signals (Mbeki) | None (read-only code) | GREEN (external) | PR comments only |

The Operator (Reiter) does not have RED classification access for code branches. If a task requires modifying RED-classified code (security-sensitive modules), the CO must explicitly authorize it by co-signing the OPORD. This dual-authorization requirement for sensitive operations is a direct adaptation of nuclear launch authorization protocols: two keys, two people, no single point of authority.

**Key lifecycle:**

| Event | Protocol | Duration |
|-------|----------|----------|
| Provisioning | CO authorizes, Unit key signs | Immediate |
| Rotation | Scheduled quarterly, XO coordinates | 48-hour overlap period |
| Compromise (suspected) | Key suspended pending investigation | Until cleared |
| Compromise (confirmed) | Key revoked, all signed work quarantined, forensic review | Immediate revocation, 72-hour review |
| Decommission | Clearance revoked, key archived | Immediate |

The distinction between "suspected" and "confirmed" compromise is operationally important. Suspected compromise triggers suspension (key cannot sign new operations but existing signatures remain valid). Confirmed compromise triggers revocation (all signatures are suspect and must be re-verified). This prevents both under-reaction (ignoring a real compromise) and over-reaction (invalidating good work based on a false alarm).

### Trade-offs

**Alternative considered: Single unit key for all agents.** Rejected for the same reason the military does not issue one key to an entire unit: if one person is compromised, the entire unit is compromised.

**Alternative considered: Per-operation signing (sign each tool call, not just commits).** Rejected as operationally impractical. The token overhead of signing every tool call would consume 10-15% of the budget. Signing commits (the final output) provides sufficient auditability.

---

## 8. Token Budget (RFP 3.7)

Estimates for Claude Opus on a typical task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,800 | 0 | Once per session | Agent identity, clearance, tool classifications, threat board summary |
| **Intel preparation (Voss)** | 3,500 | 1,200 | Once per task | Situation assessment, threat identification, critical path analysis |
| **Threat assessment (Sharma)** | 2,000 | 1,000 | Once per task | Probability estimates, risk classification, confidence intervals |
| **OPORD production (Voss)** | 1,000 | 800 | Once per task | Numbered operational orders with assignments |
| **Execution (per OPORD step)** | 1,200 | 1,500 | ~3 per task | Reiter executes orders, generates code |
| **Validation (Sharma)** | 2,000 | 800 | Once per task | Risk register update, patch risk assessment |
| **Commit message + threat assessment** | 500 | 600 | Once per task | Structured message with OPORD reference and risk classification |
| **Memory retrieval** | 1,200 | 400 | 2 per task | Classification-prioritized retrieval |
| **Memory review cycle** | 800 | 400 | 1 per task | Sharma's scheduled review (5% budget allocation) |
| **Coordination (Mbeki)** | 1,200 | 700 | 2 per task | SITREP transmission, dependency tracking |
| **After Action Review** | 500 | 600 | Once per task | Post-operation assessment |
| **TOTAL (typical task)** | **24,300** | **13,200** | -- | **37,500 total tokens** |

### Justification

The total of ~37,500 tokens is competitive with single-agent approaches because the command structure reduces wasted work. Voss's intel preparation identifies the critical path before Reiter spends tokens on execution, preventing the common pattern where an agent generates a patch, discovers a dependency, scraps the patch, and starts over.

The system prompt is 3,800 tokens -- slightly above the 4,000-token soft target but justified by the inclusion of the threat board summary and classification rules. Without these, agents would need to retrieve classification rules from memory on every tool call, which would cost more tokens overall.

The After Action Review adds ~1,100 tokens to every task. RAC considers this non-negotiable. An operation without a debrief is an operation that teaches nothing. The AAR feeds directly into threat-assessment memory, improving future operations.

The 5% budget allocation for memory review cycles (~1,200 tokens) ensures that classified memory remains current. This is a fixed cost that does not scale with task complexity.

---

## 9. Testing Strategy

### Provider-agnostic testing

A `MockProvider` implements the LLM provider interface with deterministic, classification-aware responses. Tests define a threat board state and verify that the agent correctly classifies new information, routes tool calls through the appropriate authorization checks, and refuses to execute RED operations without proper clearance.

### Patch workflow validation

INDEX.patch round-trip testing with threat annotations:

1. Establish a codebase state
2. Execute an OPORD with known threat classification
3. Capture INDEX.patch and COMMIT.msg
4. Verify the patch applies cleanly
5. Verify the commit message contains accurate threat assessment
6. Verify the AAR is produced and correctly classified

### Cross-repo coordination

A `MockSignals` implements `SignalsChannel` with classification-aware routing:

- Verify RED traffic is not sent in plain text
- Verify AMBER traffic is routed only to cleared agents
- Verify GREEN traffic is accessible to all
- Simulate coordination failures (lost messages, delayed responses)
- Verify ORBAT is updated correctly after each coordination event

### Token budget enforcement

Tests verify:

- Normal ops (0-70% consumption): full operational capability
- Conservation (70-85%): reduced SITREP frequency
- Austerity (85-95%): single-pass execution
- Emergency Halt (95%+): valid partial patch with final SITREP

### Threat-assessment memory

Memory tests verify:

- Classification assignment (correct initial classification)
- Review cycle enforcement (RED reviewed within 24h, AMBER within 7d, GREEN within 30d)
- Declassification timeline (RED -> AMBER -> GREEN -> BLACK on schedule)
- Compartmentalization (agents only see entries within their clearance)
- Confidence tracking (entries below 0.5 flagged for investigation)
- Need-to-know enforcement (RED entries not leaked to unauthorized agents)

---

## 10. Trade-offs and Alternatives

| Decision | Chosen | Alternative | Why |
|----------|--------|-------------|-----|
| Memory model | Threat classification (RED/AMBER/GREEN/BLACK) | Flat with TTL | Active review catches staleness faster than passive TTL |
| Agent structure | 4-unit command | Single agent | Separation of analysis, execution, and validation prevents intelligence failures |
| Expiration | Declassification (never deleted) | Deletion | Historical intelligence has pattern-analysis value |
| Coordination security | Classification-aware | All public | Sensitive operational details should not be in public PR comments |
| Authorization | Role-based with clearance | Flat per-agent | Rank-appropriate access prevents scope creep |
| Review cycles | Mandatory, scheduled | On-demand | Scheduled review catches issues that on-demand review misses |
| Key management | Dual-authorization for RED ops | Single signature | Critical operations require two authorizers |
| WASI fallback | Garrison mode (local only) | Disabled | Partial capability better than none |

---

## 11. Configuration Reference

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.ops.tokenBudget` | integer | 50000 | Total token budget per operation |
| `but-ai.ops.conservationThreshold` | float | 0.70 | Budget fraction triggering conservation mode |
| `but-ai.ops.austerityThreshold` | float | 0.85 | Budget fraction triggering austerity mode |
| `but-ai.ops.haltThreshold` | float | 0.95 | Budget fraction triggering emergency halt |
| `but-ai.intel.branch` | string | `refs/but-ai/intel` | Base ref for intel storage |
| `but-ai.intel.redReviewHours` | integer | 24 | Hours between mandatory RED reviews |
| `but-ai.intel.amberReviewDays` | integer | 7 | Days between mandatory AMBER reviews |
| `but-ai.intel.greenReviewDays` | integer | 30 | Days between mandatory GREEN reviews |
| `but-ai.intel.redDeclassifyDays` | integer | 30 | Days before RED -> AMBER declassification |
| `but-ai.intel.amberDeclassifyDays` | integer | 60 | Days before AMBER -> GREEN declassification |
| `but-ai.intel.greenDeclassifyDays` | integer | 90 | Days before GREEN -> BLACK declassification |
| `but-ai.intel.reviewBudgetPercent` | float | 0.05 | Budget fraction reserved for memory review |
| `but-ai.intel.minConfidence` | float | 0.50 | Minimum confidence before flagging for investigation |
| `but-ai.signals.schema` | string | `but-ai/signals/v1` | SITREP schema version |
| `but-ai.signals.pollInterval` | integer | 30 | Seconds between signals checks |
| `but-ai.identity.unitKey` | string | -- | OpenWallet unit key ID |
| `but-ai.identity.agentKeyPrefix` | string | -- | OpenWallet agent key prefix |
| `but-ai.provider.<name>.clearance` | string | `GREEN` | Maximum classification level for provider |

---

*"Intelligence work is not about knowing everything. It is about knowing what matters, when it matters, at the classification level appropriate to the consumer. Our plugin applies this principle to agent memory. Not every fact is equal. Not every memory deserves the same attention. Our agents know the difference."*
-- Colonel (Ret.) Marcus Voss, Commanding Officer
