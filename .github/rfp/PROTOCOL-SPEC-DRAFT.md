# WEAVE: Workspace-Enabled Agent Version-control Exchange
## Protocol Specification -- Draft v0.1

> "Two hundred answers to the same question. The shape underneath is clearer now."

**Status**: Draft
**Date**: 2026-03-29
**Reference Implementation**: `but-ai` unified crate (branch `rfp/unified`, 9,508 lines)

### Name Alternatives Considered

| Name | Description |
|------|-------------|
| GitButler Protocol (GBP) | Direct, corporate |
| The Loom Protocol | From winning org's weaving metaphor |
| The Fabric Protocol | Dual meaning: woven output + system structure |
| Tide Protocol | Maritime coordination rhythm |
| **WEAVE** | **Workspace-Enabled Agent Version-control Exchange** |
| Patch & Remember | Functional description |
| git-remember | Unix-style minimalist |
| The Selvedge Spec | Self-finished edge that prevents unraveling |

Working name: **WEAVE**

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Terminology](#2-terminology)
3. [Write Primitive](#3-write-primitive)
4. [CLI Contract](#4-cli-contract)
5. [Coordination](#5-coordination)
6. [Memory & Knowledge](#6-memory--knowledge)
7. [Identity & Authorization](#7-identity--authorization)
8. [Task Orchestration](#8-task-orchestration)
9. [Conformance Levels](#9-conformance-levels)
10. [Appendix A: Coordination Message JSON Schema](#appendix-a-coordination-message-json-schema)
11. [Appendix B: Survival Distribution Formulas](#appendix-b-survival-distribution-formulas)
12. [Appendix C: Scoring Formula](#appendix-c-scoring-formula)
13. [Appendix D: Reference Implementation](#appendix-d-reference-implementation)
14. [Appendix E: Incident Registry](#appendix-e-incident-registry)

---

## 1. Introduction

### 1.1 Purpose

WEAVE defines how AI agents interact with version-controlled codebases through GitButler. It specifies the write primitive (patch-based), coordination protocol (PR-based messaging), memory system (Git-native with survival-based expiration), identity model (signed commits with authorization scoping), and orchestration model (phase-gated with budget enforcement).

The protocol emerges from a cross-evaluation of 200 organizational proposals, distilling the strongest ideas from five finalists:

- **001 (Tidal Protocol)**: Coordination protocol, CRDT provenance, forge adapter
- **083 (Textile Morphology)**: Adaptive retrieval density
- **084 (Loom & Verse)**: Motif-based retrieval, tension tracking, narrative metadata
- **093 (Longevity & Risk)**: Survival function expiration, hazard rates, confidence
- **145 (ShelfOS)**: Call number classification, see-also graph, controlled vocabulary

### 1.2 Scope

This specification covers:

- How agents produce code changes (Section 3)
- How agents interact with the version-control CLI (Section 4)
- How agents coordinate across repositories (Section 5)
- How agents remember and retrieve knowledge (Section 6)
- How agents identify and authorize themselves (Section 7)
- How agents manage task execution lifecycle (Section 8)

### 1.3 Design Principles

1. **Patch-first**: Agents produce artifacts, never modify the working tree directly.
2. **Memory-native**: Knowledge stored in Git refs, not external databases.
3. **Forge-agnostic**: Works with GitHub, GitLab, Bitbucket, Gitea.
4. **Budget-aware**: Every operation has a token cost; graceful degradation is mandatory.
5. **Failure-driven**: Every rule traces to a documented incident (Appendix E).
6. **Convergence-guaranteed**: CRDT merge semantics ensure distributed consistency.

### 1.4 Relationship to GitButler

WEAVE builds on the GitButler virtual branch model. The `but` CLI provides the write interface (Sections 3-4). The `but-ai` crate provides the intelligence layer (Sections 5-8). Agents MUST NOT use `git` directly for write operations; all mutations flow through `but`.

### 1.5 Notation

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119.

---

## 2. Terminology

| Term | Definition |
|------|------------|
| **Agent** | An AI system that reads, writes, and reasons about code within a WEAVE-compliant environment. Identified by an `AgentId` (opaque string). |
| **Memory Entry** | The atomic unit of agent knowledge. A structured record stored as a Git ref containing content, classification, survival metadata, narrative references, and provenance. |
| **Motif** | A recurring theme identified across 3 or more distinct memory entries. Proto-motifs (1-2 appearances) contribute at reduced weight. |
| **Tension** | A contradiction or unresolved issue tracked as a first-class object with lifecycle management (introduced, referenced, escalated, resolved). |
| **Forge** | A code hosting platform (GitHub, GitLab, Bitbucket, or Gitea) accessed through the `ForgeAdapter` interface. |
| **Call Number** | A hierarchical classification path (e.g., `ARCH.AUTH.MIDDLEWARE`) that positions a memory entry in the knowledge tree. Analogous to a Library of Congress call number. |
| **Survival Distribution** | A parametric probability distribution governing a memory entry's mortality rate. Four families are supported: Exponential, Weibull, Bathtub, and Log-Normal. |
| **S(t)** | The survival function: the probability that a memory entry remains relevant at time `t`. Range `[0, 1]`. |
| **h(t)** | The hazard function: the instantaneous risk of irrelevance at time `t`. Computed as `f(t) / S(t)`. |
| **Warp** | Persistent memory entries (architectural knowledge, conventions) with high expected survival time. |
| **Weft** | Task-specific memory entries (bug context, implementation details) with shorter expected survival time. |
| **See-Also Link** | A typed, bidirectional cross-reference between two memory entries. Types: `RelatedTo`, `DependsOn`, `ContrastsWith`, `SupersededBy`. |
| **Vector Clock** | A map from `AgentId` to monotonically increasing version numbers, providing causal ordering for distributed memory synchronization. |
| **Gossip** | A pull-based synchronization protocol where agents exchange memory entries using vector clocks to determine what the peer is missing. |
| **Budget Mode** | One of four operational modes (`Full`, `Abbreviated`, `MinimumOutput`, `EmergencyHalt`) determined by remaining token budget. |
| **Phase Gate** | A constraint that restricts which tools are available during each phase of the task lifecycle. Enforces least-privilege. |
| **Surprise Index** | The KL divergence between predicted and observed access patterns for a memory entry. High surprise triggers distribution refitting. |

---

## 3. Write Primitive

### 3.1 Patch Artifacts

Agents MUST produce two artifacts per unit of work:

- **`INDEX.patch`**: A unified diff against the current index.
- **`COMMIT.msg`**: A commit message with structured metadata trailers.

These two files constitute the complete, self-contained output of an agent's implementation phase. They are stored and transmitted independently of the working tree.

### 3.2 Patch Format Requirements

The `INDEX.patch` MUST be a valid unified diff that can be applied with `git apply --check` without errors. Specifically:

- The patch MUST use unified diff format (context lines with `@@ -a,b +c,d @@` hunks).
- File paths MUST use the `a/` and `b/` prefix convention.
- Binary files MUST be represented as `Binary files differ` with appropriate `GIT binary patch` data if modification is required.
- The patch MUST NOT contain merge conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).

### 3.3 Commit Message Format

The `COMMIT.msg` MUST follow conventional commit format with the following structured trailers:

```
<type>(<scope>): <subject>

<body>

Agent-Id: <agent-id>
Task-Phase: <classify|plan|implement|validate|catalog|coordinate>
Tokens-Used: <count>
Survival-Estimate: <distribution-family>(<params>)
Call-Number: <call-number>
```

Required trailers:

| Trailer | Format | Example |
|---------|--------|---------|
| `Agent-Id` | Opaque string | `Agent-Id: dara-impl-001` |
| `Task-Phase` | Enum value | `Task-Phase: implement` |
| `Tokens-Used` | Unsigned integer | `Tokens-Used: 4200` |
| `Survival-Estimate` | Distribution with params | `Survival-Estimate: Weibull(k=1.5, lambda=90.0)` |
| `Call-Number` | Dot-separated path | `Call-Number: ARCH.AUTH.MIDDLEWARE` |

### 3.4 Sole Committer Model

Only the `but` agent (the orchestrator) applies patches to the repository. Individual agents MUST NOT call `git commit`, `git add`, or directly modify the working tree. This constraint prevents the hunk lock contention documented in incident F2 (Appendix E).

The orchestrator:

1. Receives the `INDEX.patch` and `COMMIT.msg` from the agent.
2. Validates the patch with `git apply --check`.
3. Applies the patch via the `but` CLI.
4. Creates the commit with the provided message and trailers.

### 3.5 Self-Containment

Patches MUST be self-contained. They MUST NOT depend on implicit working tree state that could change between patch generation and application. This means:

- All necessary context lines MUST be present in the diff.
- Patches MUST NOT assume the existence of uncommitted changes from other agents.
- File creation patches MUST include the complete file content (not depend on a template that may have changed).

---

## 4. CLI Contract

### 4.1 Write Operations

All workspace mutations MUST use the `but` CLI. The following `git` commands are prohibited for write operations:

- `git add`, `git commit`, `git push`, `git checkout`, `git merge`
- `git rebase`, `git stash`, `git cherry-pick`, `git reset`

Read-only `git` operations (`git log`, `git blame`, `git show --stat`, `git diff`) are permitted.

### 4.2 Mutation Feedback

All mutation commands MUST include the `--status-after` flag to receive immediate state feedback. This provides the agent with updated IDs, branch state, and remaining uncommitted changes in a single round-trip.

```bash
# Correct
but commit <branch> -m "<msg>" --changes <id1>,<id2> --status-after

# Incorrect (missing feedback)
but commit <branch> -m "<msg>" --changes <id1>,<id2>
```

### 4.3 Exit Codes

Exit codes are semantic and MUST be interpreted strictly:

| Exit Code | Meaning | Agent Response |
|-----------|---------|----------------|
| 0 | Success with valid, non-null result | Proceed with returned data |
| Non-zero | Failure | MUST NOT proceed; MUST report error |

A null result on exit 0 is a protocol violation (incident F3). Implementations MUST treat null-on-success as an error condition and surface it for debugging.

### 4.4 Batch Operations

Batch operations are primitive -- one call, one transaction. Agents SHOULD use batch forms where available:

```bash
# Single transaction for multiple branches
but push b1 b2 b3
```

### 4.5 Background Sync

Agents MUST call `but sync pause` before beginning work and `but sync resume` after completing. Failure to pause background sync can result in the sync process reverting agent work mid-operation (incident F4).

```bash
# Correct lifecycle
but sync pause
# ... agent performs work ...
but sync resume
```

### 4.6 ID Resolution

Agents MUST use CLI IDs obtained from `but status -fv`, `but diff`, or `but show` output. IDs MUST NOT be hardcoded or assumed stable across operations. After any mutation, agents MUST re-read IDs from the `--status-after` output or a fresh `but status -fv` call.

---

## 5. Coordination

### 5.1 Message Transport

Coordination messages are embedded in pull request comments using markdown code fences. The fence tag `but-ai-message` identifies machine-parseable content within human-readable PR threads.

The schema version string is: `but-ai/coordination/v1`

### 5.2 Message Wire Format

Messages are serialized as JSON inside markdown code fences:

````markdown
```but-ai-message
{
  "schema": "but-ai/coordination/v1",
  "message_type": "status_report",
  "from": "dara",
  "to": "ines",
  "payload": {
    "status": "in_progress",
    "files_changed": 3
  },
  "timestamp": "2026-03-29T14:30:00Z"
}
```
````

This dual encoding ensures:

- **Human readability**: The JSON is visible in PR comment threads alongside prose.
- **Machine parseability**: Agents extract messages by scanning for ```` ```but-ai-message ```` fences.
- **Multi-message support**: A single comment MAY contain multiple fenced messages, separated by arbitrary prose.

### 5.3 Message Structure

Every coordination message has the following top-level fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema` | string | Yes | Protocol version: `"but-ai/coordination/v1"` |
| `message_type` | enum | Yes | One of the message types defined in Section 5.4 |
| `from` | string | Yes | Sending agent's `AgentId` |
| `to` | string or null | Yes | Receiving agent's `AgentId`, or `null` for broadcast |
| `payload` | object | Yes | Type-specific payload (see Section 5.4) |
| `timestamp` | string | Yes | ISO-8601 timestamp |

### 5.4 Message Types

| Type | `message_type` Value | Purpose | Direction |
|------|---------------------|---------|-----------|
| Task Assignment | `task_assignment` | Delegate work to an agent | Coordinator -> Agent |
| Status Report | `status_report` | Report progress or completion | Agent -> Coordinator |
| Dependency Declaration | `dependency_declaration` | Declare cross-PR dependency | Agent -> Graph |
| Patch Handoff | `patch_handoff` | Transfer a patch between agents | Agent -> Agent |
| Budget Report | `budget_report` | Report token budget status | Agent -> Coordinator |

#### 5.4.1 TaskAssignment Payload

```json
{
  "task_description": "string",
  "target_files": ["string"],
  "complexity": "simple | moderate | complex",
  "estimated_tokens": 15000,
  "branch_pattern": "feat/auth-*"
}
```

#### 5.4.2 StatusReport Payload

```json
{
  "status": "in_progress | completed | blocked | failed",
  "tokens_used": 4200,
  "files_changed": 3,
  "issues": ["string"],
  "patch_id": "optional-patch-reference"
}
```

#### 5.4.3 DependencyDeclaration Payload

```json
{
  "depends_on": [
    {
      "repo": { "forge": "github", "owner": "org", "repo": "repo" },
      "number": 42
    }
  ]
}
```

#### 5.4.4 PatchHandoff Payload

```json
{
  "patch_content": "unified-diff-string",
  "commit_msg": "string",
  "files_touched": ["string"],
  "tokens_used": 3500,
  "source_phase": "implement"
}
```

#### 5.4.5 BudgetReport Payload

```json
{
  "total": 32000,
  "used": 18500,
  "remaining": 13500,
  "catalog_reserve": 1500,
  "coordination_reserve": 2000,
  "available_for_work": 10000,
  "mode": "full | abbreviated | minimum_output | emergency_halt",
  "utilization": 0.578
}
```

### 5.5 Message Parsing

Implementations MUST parse messages using the following algorithm:

1. Scan the comment body for occurrences of the string `` ```but-ai-message ``.
2. For each occurrence, locate the corresponding closing `` ``` ``.
3. Extract the text between the opening tag and closing fence.
4. Trim whitespace and parse as JSON.
5. Validate the `schema` field matches the expected version.
6. Deserialize into a `CoordinationMessage`.

An unterminated code fence (opening tag without closing `` ``` ``) MUST be reported as an error. Non-message content in the comment MUST be ignored silently.

### 5.6 Forge Adapter Interface

Implementations MUST support the following operations through a `ForgeAdapter` trait:

```rust
trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str,
                 head: &str, base: &str) -> Result<PrRef>;
    fn comment(&self, pr: &PrRef, body: &str) -> Result<()>;
    fn list_comments(&self, pr: &PrRef) -> Result<Vec<String>>;
    fn pr_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn add_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn list_prs(&self, repo: &RepoRef, labels: &[&str]) -> Result<Vec<PrRef>>;
    fn forge_type(&self) -> ForgeType;
}
```

Supported forge types (the `ForgeType` enum):

| Value | Platform |
|-------|----------|
| `github` | GitHub |
| `gitlab` | GitLab |
| `bitbucket` | Bitbucket |
| `gitea` | Gitea |

A `RepoRef` identifies a repository:

```json
{
  "forge": "github",
  "owner": "gitbutlerapp",
  "repo": "gitbutler"
}
```

A `PrRef` identifies a pull request:

```json
{
  "repo": { "forge": "github", "owner": "org", "repo": "repo" },
  "number": 42
}
```

PR statuses: `open`, `closed`, `merged`, `draft`.

### 5.7 Dependency Resolution

Dependencies between PRs form a directed acyclic graph (DAG). The protocol uses Kahn's algorithm for topological sort with cycle detection.

**Algorithm**:

1. Build a map from `PrRef` to graph index.
2. Compute in-degree for each node based on `depends_on` edges.
3. Initialize a queue with all nodes having in-degree 0.
4. Pop from the queue, add to sorted output, decrement in-degree for dependents.
5. If the sorted output length does not equal the node count, a cycle exists.

**External dependency rule**: Dependencies referencing PRs not present in the local graph are treated as already satisfied. This allows agents to depend on PRs in other repositories without requiring the full cross-repo graph.

**Readiness**: A node is "ready" when:
- All its dependencies are in `Merged` status (or are external).
- Its own status is not `Merged` or `Closed`.

**Blocking**: A node is "blocked" when it has unmet dependencies (internal dependencies not yet merged).

### 5.8 Memory Synchronization (Gossip Protocol)

Agents synchronize memory using pull-based CRDT gossip:

**Round trip**:

1. **Request**: The initiating agent sends a `GossipRequest` containing its `AgentId` and current `VectorClock`.
2. **Compute delta**: The responding agent calls `clock.missing_for(requester_clock)` to determine which agents have data the requester has not seen.
3. **Response**: The responder returns a `GossipResponse` containing the missing `MemoryEntry` records and its own `VectorClock`.
4. **Merge**: The requester integrates received entries using last-writer-wins semantics.

**Merge conflict resolution** (last-writer-wins with tiebreaker):

```
if incoming.access_count > existing.access_count:
    accept incoming
elif incoming.access_count == existing.access_count
     AND incoming.last_accessed > existing.last_accessed:
    accept incoming
else:
    keep existing
```

**Clock merge**: Both parties update their vector clocks using element-wise maximum:

```
for each agent_id in union(our_clock, their_clock):
    merged[agent_id] = max(our[agent_id], their[agent_id])
```

**Properties**: The merge operation is commutative, associative, and idempotent, guaranteeing convergence regardless of message ordering or duplication.

---

## 6. Memory & Knowledge

### 6.1 Storage Layout

All memory data is stored under the Git ref namespace `refs/but-ai/`:

```
refs/but-ai/
  memory/<agent-id>/
    identity/self             -- Agent identity record
    alive/<entry-hash>        -- Active entries (S(t) >= 0.25)
    moribund/<entry-hash>     -- Aging entries under review (0.10 <= S(t) < 0.25)
    deceased/<entry-hash>     -- Expired entries retained for audit (S(t) < 0.10)
  shared/                     -- Cross-agent shared entries
  motifs/<motif-id>           -- Thematic pattern records
  catalog/index               -- Classification index cache
```

The ref path for a specific agent's memory entry is computed as:

```
refs/but-ai/memory/{agent_id}/{entry_hash}
```

### 6.2 Memory Entry Format

Each `MemoryEntry` is the atomic unit of agent knowledge. It integrates five models into a single record:

```json
{
  "id": "sha256-hash-of-content",
  "agent": "dara-impl-001",
  "content": "The authentication middleware uses JWT with RS256...",
  "created_at": "2026-03-15T10:30:00Z",
  "last_accessed": "2026-03-28T14:00:00Z",

  "classification": {
    "subject_headings": ["authentication", "middleware", "JWT"],
    "call_number": { "segments": ["ARCH", "AUTH", "MIDDLEWARE"] },
    "controlled_vocab": true
  },
  "see_also": [
    {
      "target_id": "entry-abc123",
      "relationship": "related_to",
      "note": "Same auth subsystem, different layer"
    }
  ],

  "motifs": ["auth-pattern", "token-validation"],
  "tension_refs": [
    { "tension_id": "t-session-vs-jwt", "role": "referenced" }
  ],

  "survival": {
    "distribution": { "weibull": { "k": 1.5, "lambda": 90.0 } },
    "current_probability": 0.72,
    "hazard_rate": 0.008,
    "surprise_index": 0.12,
    "goodness_of_fit": 0.91
  },
  "state": "alive",

  "consensus_citations": 3,
  "access_count": 12,
  "source_commit": "a1b2c3d4e5f6"
}
```

**Model integration**:

| Model | Source Proposal | Fields |
|-------|---------------|--------|
| Core | Consensus | `id`, `agent`, `content`, `created_at`, `last_accessed`, `access_count` |
| Classification | 145 (ShelfOS) | `classification` (subject headings, call number, controlled vocab), `see_also` |
| Survival | 093 (LRRC) | `survival` (distribution, S(t), h(t), surprise index, goodness-of-fit) |
| Narrative | 084 (Loom & Verse) | `motifs`, `tension_refs` |
| Provenance | 001 (Tidal Protocol) | `consensus_citations`, `source_commit` |

### 6.3 Classification Systems

Five simultaneous classification systems are maintained for each entry:

1. **Subject Headings**: Controlled vocabulary terms (e.g., `"authentication"`, `"middleware"`). The `controlled_vocab` boolean indicates whether terms are from the canonical vocabulary or free-text.

2. **Call Number**: A hierarchical path with dot-separated uppercase segments (e.g., `ARCH.AUTH.MIDDLEWARE`). Supports:
   - `is_ancestor_of(other)`: True if this call number is a prefix of another.
   - `shared_depth(other)`: Number of shared prefix segments.
   - `depth()`: Number of segments.
   - Parsing from dot-separated strings or file paths.

3. **Source Provenance**: The `agent` field and `source_commit` hash identify who created this knowledge and from what code change.

4. **Temporal**: The `created_at` and `last_accessed` ISO-8601 timestamps.

5. **Relational**: The `see_also` links form a typed bidirectional graph between entries. Four relationship types:

   | Relationship | Meaning |
   |-------------|---------|
   | `related_to` | Topically related |
   | `depends_on` | Source depends on target |
   | `contrasts_with` | Source contrasts with target |
   | `superseded_by` | Source is superseded by target |

### 6.4 Survival Distributions

Four parametric families model different mortality patterns:

| Family | Parameters | Use Case | Default For |
|--------|-----------|----------|-------------|
| Exponential | `lambda` (rate) | Constant hazard, memoryless | Bug fixes, task context |
| Weibull | `k` (shape), `lambda` (scale) | Monotone hazard | Architecture (k > 1), early-decay (k < 1) |
| Bathtub | `alpha`, `beta`, `gamma` | High-low-high hazard | Conventions |
| Log-Normal | `mu` (log-mean), `sigma` (log-std) | Heavy-tailed | Cross-repo knowledge |

**Weibull reduces to Exponential** when `k = 1.0`, making Exponential a special case.

Distributions are fitted from access history using maximum likelihood estimation (MLE) with Akaike Information Criterion (AIC) for model selection. The `goodness_of_fit` field (range `[0, 1]`) reports the fit quality. The `surprise_index` field reports KL divergence between predicted and observed access patterns; high surprise triggers refitting.

See Appendix B for the complete mathematical formulas.

### 6.5 Memory Lifecycle

Three states govern the lifecycle of every memory entry, with transitions driven by the survival probability `S(t)`:

```
                    S(t) drops          S(t) drops
  [ Alive ] ----below 0.25----> [ Moribund ] ----below 0.10----> [ Deceased ]
     ^                               |                                |
     |         S(t) recovers         |                                |
     +-------above 0.25-------------+                                |
     |                                                                |
     +------------------explicit resuscitation-----------------------+
```

**Thresholds** (defined as constants in the reference implementation):

| Transition | Condition | Constant |
|-----------|-----------|----------|
| Alive -> Moribund | `S(t) < 0.25` | `MORIBUND_THRESHOLD = 0.25` |
| Moribund -> Deceased | `S(t) < 0.10` | `DECEASED_THRESHOLD = 0.10` |
| Alive -> Deceased | `S(t) < 0.10` (skip moribund) | Direct transition allowed |
| Moribund -> Alive | `S(t) >= 0.25` (resuscitation) | Recovery via access |
| Deceased -> Alive | Explicit resuscitation call | Manual override |

**Audit process**: The `audit_lifecycle` function scans all alive and moribund entries, evaluating their current `S(t)` and transitioning as needed. It returns an `AuditResult` for each entry that changed state, including the previous state, new state, and survival probability at the time of audit.

**Resuscitation**: Accessing a moribund entry (which updates its `last_accessed` and `access_count`, causing the survival distribution to be refitted) can cause `S(t)` to recover above the moribund threshold, automatically transitioning it back to alive. Deceased entries require explicit resuscitation via the `resuscitate()` function.

### 6.6 Retrieval Scoring

The retrieval engine computes a composite relevance score from six weighted components:

```
score = motif_resonance      * w.motif_resonance        (default 0.25)
      + call_number_proximity * w.call_number_proximity  (default 0.20)
      + survival_probability  * w.survival_probability   (default 0.15)
      + see_also_distance     * w.see_also_distance      (default 0.15)  [note 1]
      + tension_urgency       * w.tension_boost           (default 0.10)  [note 2]
      + freshness             * w.freshness               (default 0.10)  [note 3]
```

[note 1]: Renamed from `see_also_distance` in the weights struct; measures graph proximity.
[note 2]: Renamed from `tension_boost` in the weights struct; measures unresolved tension urgency.
[note 3]: Uses access count as a proxy via logarithmic scale: `ln(1 + access_count) / ln(11)`.

The `RelevanceWeights` structure holds these six weights:

```json
{
  "motif_resonance": 0.25,
  "call_number_proximity": 0.20,
  "see_also_distance": 0.15,
  "survival_probability": 0.15,
  "freshness": 0.10,
  "tension_boost": 0.10
}
```

Weights are configurable. The composite score is clamped to `[0.0, 1.0]`. Results are sorted descending by score and truncated to `max_results`.

**Component details**:

- **Motif resonance**: Keyword overlap between the query and the entry's content-derived keywords, motif IDs, and subject headings. Computed as `matches / query_keywords.len()`.
- **Call number proximity**: Shared prefix depth between the query's inferred call number and the entry's call number.
- **Survival probability**: The entry's current `S(t)` value, directly from survival metadata.
- **See-also distance**: The best see-also graph distance score from any entry to this entry.
- **Tension urgency**: Normalized count of unresolved tensions (introduced or referenced), capped at 5. Score = `min(unresolved_count / 5, 1.0)`.
- **Freshness**: Logarithmic function of access count: `ln(1 + count) / ln(11)`, with a base of 0.1 for new entries.

### 6.7 Narrative Layer

#### 6.7.1 Motifs

A `Motif` is a recurring theme that emerges when a pattern appears in 3 or more distinct memory entries:

```json
{
  "id": "auth-pattern",
  "description": "JWT-based authentication with RS256 signing",
  "appearances": ["entry-1", "entry-2", "entry-3", "entry-4"],
  "related_motifs": ["token-validation", "session-mgmt"]
}
```

- **Proto-motifs** (1-2 appearances) contribute at 0.3x weight during retrieval scoring.
- **Full motifs** (3+ appearances) contribute at full weight.
- **Related motifs** are traversed transitively during retrieval, enabling thematic discovery beyond keyword matching.

#### 6.7.2 Tensions

A `Tension` is a contradiction or unresolved issue tracked as a first-class object:

```json
{
  "id": "t-session-vs-jwt",
  "description": "Auth module uses JWT tokens but session middleware assumes cookie-based sessions",
  "severity": "high",
  "introduced_in": "entry-abc",
  "resolved_in": null
}
```

**Severity levels**: `low`, `moderate`, `high`, `critical`.

**Tension references** from memory entries use three roles:

| Role | Meaning |
|------|---------|
| `introduced` | This entry introduced the tension |
| `referenced` | This entry references an existing tension |
| `resolved` | This entry resolved the tension |

**Urgency scoring** follows a Weibull CDF with `k = 2.0` and `lambda = 14 days`:

```
urgency(t) = 1 - exp(-(t / lambda)^k)
```

Where `t` is the age of the tension in seconds and `lambda = 14 * 24 * 3600 seconds`.

**Escalation**: Tensions unresolved for more than 14 days are automatically escalated to `Critical` severity. The escalation adds a bonus of 0.15 to the urgency score.

**Severity multipliers** applied to the base urgency:

| Severity | Multiplier |
|----------|-----------|
| Low | 0.5 |
| Moderate | 0.75 |
| High | 1.0 |
| Critical | 1.0 |

**Entry-level urgency**: The aggregate urgency for a memory entry is the mean urgency across its tension references, weighted by role (`Introduced` = 1.0, `Referenced` = 0.6, `Resolved` = 0.0), clamped to `[0.0, 1.0]`.

#### 6.7.3 Arcs

An **arc** is a group of related memory entries sharing motifs. Arcs have the following lifecycle properties:

- **Dormancy**: An arc becomes dormant when all member entries have `S(t) < 0.25`.
- **Reactivation**: A dormant arc reactivates when a new entry joins or an existing member is accessed.
- **Narrative continuity**: Arcs provide long-range context that individual entries cannot.

### 6.8 Memory Store Interface

Implementations MUST provide a `MemoryStore` trait with these operations:

```rust
trait MemoryStore: Send + Sync {
    fn store(&self, entry: &MemoryEntry) -> Result<()>;
    fn load(&self, id: &EntryId) -> Result<Option<MemoryEntry>>;
    fn list(&self, state: Option<MemoryState>) -> Result<Vec<EntryId>>;
    fn transition(&self, id: &EntryId, new_state: MemoryState) -> Result<()>;
    fn delete(&self, id: &EntryId) -> Result<()>;
}
```

And a `MemoryRetriever` trait:

```rust
trait MemoryRetriever: Send + Sync {
    fn retrieve(&self, query: &str, max_results: usize,
                weights: &RelevanceWeights) -> Result<Vec<ScoredMemory>>;
}
```

---

## 7. Identity & Authorization

### 7.1 Agent Identity

Each agent has a full identity record stored at `refs/but-ai/memory/<agent-id>/identity/self`:

```json
{
  "agent_id": "dara-impl-001",
  "role": "implementer",
  "capabilities": ["read", "write", "commit"],
  "authorization": {
    "branch_patterns": ["feat/*", "fix/*"],
    "max_patch_lines": 500,
    "repos": ["*"],
    "call_number_ranges": ["ARCH.*", "SEC.*"]
  },
  "signing_key": "fingerprint-abc123",
  "performance_history": {
    "tasks_completed": 42,
    "mean_confidence": 0.87,
    "mean_patch_survival_days": 23.5
  },
  "created_at": "2026-01-15T00:00:00Z"
}
```

### 7.2 Agent Roles

| Role | Capabilities | May Produce Patches | May Create PRs |
|------|-------------|-------------------|---------------|
| `architect` | Read, plan, review | Yes | No |
| `implementer` | Read, write, commit | Yes | No |
| `validator` | Read, check | No | No |
| `coordinator` | Read, forge operations | No | Yes |

### 7.3 Authorization Scope

The `AuthorizationScope` constrains what an agent may do:

| Field | Type | Description |
|-------|------|-------------|
| `branch_patterns` | `Vec<String>` | Glob patterns for allowed branches (e.g., `"feat/*"`, `"fix/*"`) |
| `max_patch_lines` | `Option<u32>` | Maximum number of lines in a single patch. `null` means unlimited. |
| `repos` | `Vec<String>` | Allowed repositories. `["*"]` means all repositories. |
| `call_number_ranges` | `Vec<String>` | Call number ranges the agent is authorized for (e.g., `["SEC.*"]`). Empty means unrestricted. |

Authorization checks MUST be performed before applying any patch or executing any forge operation.

### 7.4 Commit Signing

All agent commits MUST be cryptographically signed. The `CommitSigner` interface:

```rust
trait CommitSigner: Send + Sync {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>>;
    fn verify(&self, message: &[u8], signature: &[u8],
              agent: &AgentId) -> Result<bool>;
}
```

Two reference implementations are provided:

- **`NoOpSigner`**: Signs everything with a deterministic hash; verifies everything. For development only.
- **`DenyAllSigner`**: Rejects all signing requests. For testing denial paths.

Production implementations SHOULD use Ed25519 or RSA key pairs stored securely.

### 7.5 Key Lifecycle

Key lifecycle events are recorded in an immutable audit log (`KeyAuditLog`):

| Event | Fields | Meaning |
|-------|--------|---------|
| `Provisioned` | agent, key_id, timestamp | Key was created for an agent |
| `Rotated` | agent, old_key_id, new_key_id, timestamp | Key was replaced on schedule |
| `Compromised` | agent, key_id, timestamp, reason | Key was revoked due to compromise |
| `Decommissioned` | agent, key_id, timestamp | Key was retired after rotation |

The lifecycle progresses: `Provisioned -> Rotated -> Compromised | Decommissioned`. All events are append-only and queryable by agent ID.

### 7.6 Performance History

Each agent tracks cumulative performance statistics:

```json
{
  "tasks_completed": 42,
  "mean_confidence": 0.87,
  "mean_patch_survival_days": 23.5
}
```

These statistics are used for:
- Agent selection (prefer agents with higher historical confidence).
- Survival distribution priors (agents that produce long-lived patches inform initial distribution parameters).
- Capacity planning (predict token consumption based on historical task completion rates).

---

## 8. Task Orchestration

### 8.1 Phase Model

The task lifecycle consists of six phases, executed sequentially:

| # | Phase | `TaskPhase` | Available Tools | Purpose | Budget-Skippable |
|---|-------|-------------|-----------------|---------|-----------------|
| 1 | Classify | `classify` | `memory-retrieve`, `memory-list`, `git-log`, `git-diff`, `file-read`, `file-search` | Gather context, retrieve relevant memories | At EmergencyHalt |
| 2 | Plan | `plan` | `memory-retrieve`, `file-read`, `file-search`, `git-log`, `git-blame` | Design approach, estimate complexity | At EmergencyHalt |
| 3 | Implement | `implement` | `file-read`, `file-write`, `file-edit`, `git-diff`, `git-commit`, `shell-run` | Generate patches | At EmergencyHalt |
| 4 | Validate | `validate` | `file-read`, `git-diff`, `memory-retrieve`, `continuity-check`, `tool-risk-classify` | Verify correctness, detect contradictions | At MinimumOutput |
| 5 | Catalog | `catalog` | `memory-store`, `memory-transition`, `memory-classify` | Classify work as future memory | **NEVER** |
| 6 | Coordinate | `coordinate` | `forge-create-pr`, `forge-comment`, `forge-add-label`, `forge-list-prs` | Create PR, post messages | **NEVER** |

**Phase-gate enforcement**: A tool is available ONLY during the phases that list it. Calling a tool outside its designated phase is a protocol violation. For example, `file-write` is available only during `implement`; attempting to use it during `classify` MUST be rejected.

### 8.2 Budget Management

#### 8.2.1 Token Budget Structure

```json
{
  "total": 32000,
  "used": 0,
  "catalog_reserve": 1500,
  "coordination_reserve": 2000
}
```

- **Default total**: 32,000 tokens (configurable).
- **Catalog reserve**: 1,500 tokens (NEVER consumed by non-catalog work).
- **Coordination reserve**: 2,000 tokens (NEVER consumed by non-coordination work).
- **Available for work**: `total - used - catalog_reserve - coordination_reserve`.

#### 8.2.2 Budget Modes

The budget mode is determined by the fraction of budget **remaining** (not consumed):

| Mode | Remaining Fraction | Behavior |
|------|-------------------|----------|
| `Full` | >= 80% | All phases execute, all passes, full validation |
| `Abbreviated` | 50% -- 80% | Skip polish pass, reduced validation |
| `MinimumOutput` | 20% -- 50% | Rough implementation only, skip validation |
| `EmergencyHalt` | < 20% | Skip Classify, Plan, Implement, Validate entirely |

**Threshold constants** (from the reference implementation):

```
FULL_THRESHOLD       = 0.80  (>= 80% remaining)
ABBREVIATED_THRESHOLD = 0.50  (>= 50% remaining)
MINIMUM_THRESHOLD     = 0.20  (>= 20% remaining)
```

#### 8.2.3 Phase Budget Estimates

Rough token cost estimates for each phase (used for completion probability estimation):

| Phase | Estimated Tokens |
|-------|-----------------|
| Classify | 500 |
| Plan | 1,500 |
| Implement | 15,000 |
| Validate | 2,000 |
| Catalog | 1,000 |
| Coordinate | 1,500 |

**Completion probability**: `min(1.0, available_tokens / estimated_cost)`.

#### 8.2.4 Phase Proceed Decision

```
should_proceed(budget, phase) =
  if phase in {Catalog, Coordinate}:  ALWAYS (tokens reserved)
  if phase in {Classify, Plan, Implement}:  mode != EmergencyHalt
  if phase == Validate:  mode in {Full, Abbreviated}
```

### 8.3 Mandatory Phases

Catalog and Coordinate ALWAYS execute, regardless of budget mode. Their token reserves are set aside at budget creation and are not available for other work.

**Rationale**: An unclassified result is a lost result. If an agent does work but does not catalog it, no future agent can benefit from that knowledge. Similarly, if an agent does not coordinate (create PR, post status), the work is invisible to the rest of the system.

### 8.4 Orchestrator

The `AgentOrchestrator` is the top-level integration layer that runs the six-phase pipeline:

1. Check budget, determine mode.
2. **Classify/Plan**: Architect agent retrieves relevant memories and designs an approach, producing a `TaskPlan` with approach description, file list, complexity estimate, and token estimate.
3. **Implement**: Implementer agent generates a `PatchOutput` containing the unified diff, commit message, files touched, and tokens consumed.
4. **Validate** (conditional): Validator agent checks for continuity violations and contradictions, producing a `ValidationResult` with pass/fail status, issues list, and detected tensions.
5. **Catalog**: The orchestrator creates a `MemoryEntry` from the patch output and stores it via the `MemoryStore`.
6. **Coordinate**: External concern -- the caller provides a `ForgeAdapter` and `DependencyGraph` to create PRs and post coordination messages.

### 8.5 Complexity Levels

Tasks are classified into three complexity levels:

| Level | Description | Typical Token Cost |
|-------|------------|-------------------|
| `simple` | Single-file change, well-understood pattern | < 5,000 |
| `moderate` | Multi-file change, requires coordination | 5,000 -- 15,000 |
| `complex` | Cross-module refactoring, architectural change | 15,000+ |

---

## 9. Conformance Levels

Implementations MAY conform to one of five progressive levels:

| Level | Name | Requires | Minimum Sections |
|-------|------|----------|-----------------|
| 1 | Patch Producer | Write Primitive only | Section 3 |
| 2 | CLI Agent | Level 1 + CLI Contract | Sections 3, 4 |
| 3 | Coordinated Agent | Level 2 + Coordination | Sections 3, 4, 5 |
| 4 | Remembering Agent | Level 3 + Memory | Sections 3, 4, 5, 6 |
| 5 | Full WEAVE | Level 4 + Identity + Orchestration | Sections 3, 4, 5, 6, 7, 8 |

Each level is a strict superset of the previous. An implementation MUST pass all requirements of a level and all lower levels to claim conformance at that level.

**Level 1 (Patch Producer)**: Produces valid `INDEX.patch` and `COMMIT.msg` artifacts with required trailers. This is the minimum useful output.

**Level 2 (CLI Agent)**: Additionally uses `but` CLI exclusively for writes, handles exit codes correctly, manages background sync, and resolves IDs dynamically.

**Level 3 (Coordinated Agent)**: Additionally sends and receives coordination messages via PR comments, supports at least one forge type, and can declare and resolve dependencies.

**Level 4 (Remembering Agent)**: Additionally stores and retrieves memory entries with classification, survival metadata, and narrative references. Implements the three-state lifecycle and retrieval scoring.

**Level 5 (Full WEAVE)**: Additionally authenticates via signed commits, enforces authorization scopes, runs the six-phase lifecycle with budget management, and participates in gossip-based memory synchronization.

---

## Appendix A: Coordination Message JSON Schema

### A.1 CoordinationMessage

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "CoordinationMessage",
  "type": "object",
  "required": ["schema", "message_type", "from", "to", "payload", "timestamp"],
  "properties": {
    "schema": {
      "type": "string",
      "const": "but-ai/coordination/v1"
    },
    "message_type": {
      "type": "string",
      "enum": ["task_assignment", "status_report", "dependency_declaration",
               "patch_handoff", "budget_report"]
    },
    "from": {
      "type": "string",
      "description": "Sending agent's AgentId"
    },
    "to": {
      "type": ["string", "null"],
      "description": "Receiving agent's AgentId, or null for broadcast"
    },
    "payload": {
      "type": "object",
      "description": "Type-specific payload (see per-type schemas)"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time",
      "description": "ISO-8601 timestamp"
    }
  }
}
```

### A.2 TaskAssignment Payload Schema

```json
{
  "type": "object",
  "required": ["task_description"],
  "properties": {
    "task_description": { "type": "string" },
    "target_files": { "type": "array", "items": { "type": "string" } },
    "complexity": { "type": "string", "enum": ["simple", "moderate", "complex"] },
    "estimated_tokens": { "type": "integer", "minimum": 0 },
    "branch_pattern": { "type": "string" }
  }
}
```

### A.3 StatusReport Payload Schema

```json
{
  "type": "object",
  "required": ["status"],
  "properties": {
    "status": { "type": "string", "enum": ["in_progress", "completed", "blocked", "failed"] },
    "tokens_used": { "type": "integer", "minimum": 0 },
    "files_changed": { "type": "integer", "minimum": 0 },
    "issues": { "type": "array", "items": { "type": "string" } },
    "patch_id": { "type": "string" }
  }
}
```

### A.4 BudgetReport Payload Schema

```json
{
  "type": "object",
  "required": ["total", "used", "remaining", "mode"],
  "properties": {
    "total": { "type": "integer", "minimum": 0 },
    "used": { "type": "integer", "minimum": 0 },
    "remaining": { "type": "integer", "minimum": 0 },
    "catalog_reserve": { "type": "integer", "minimum": 0, "default": 1500 },
    "coordination_reserve": { "type": "integer", "minimum": 0, "default": 2000 },
    "available_for_work": { "type": "integer", "minimum": 0 },
    "mode": { "type": "string", "enum": ["full", "abbreviated", "minimum_output", "emergency_halt"] },
    "utilization": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
  }
}
```

---

## Appendix B: Survival Distribution Formulas

### B.1 Exponential Distribution

The memoryless distribution. Constant hazard rate.

| Function | Formula |
|----------|---------|
| S(t) | `exp(-lambda * t)` |
| h(t) | `lambda` (constant) |
| f(t) | `lambda * exp(-lambda * t)` |
| Median | `ln(2) / lambda` |
| Mean | `1 / lambda` |

**Parameter**: `lambda` (rate, events per day).

### B.2 Weibull Distribution

Monotone hazard -- increasing when `k > 1`, decreasing when `k < 1`. Reduces to Exponential when `k = 1`.

| Function | Formula |
|----------|---------|
| S(t) | `exp(-(t / lambda)^k)` |
| h(t) | `(k / lambda) * (t / lambda)^(k - 1)` |
| f(t) | `(k / lambda) * (t / lambda)^(k - 1) * exp(-(t / lambda)^k)` |
| Median | `lambda * (ln(2))^(1/k)` |
| Mean | `lambda * Gamma(1 + 1/k)` |

**Parameters**: `k` (shape), `lambda` (scale, days).

### B.3 Bathtub Distribution (Additive Hazard Model)

High-low-high hazard curve modeled as a mixture:

```
h(t) = alpha * exp(-gamma * t) + beta * t
```

| Function | Formula |
|----------|---------|
| H(t) | `(alpha / gamma) * (1 - exp(-gamma * t)) + (beta / 2) * t^2` |
| S(t) | `exp(-H(t))` |
| h(t) | `alpha * exp(-gamma * t) + beta * t` |
| f(t) | `h(t) * S(t)` |
| Median | Solved numerically: `t` such that `H(t) = ln(2)` (bisection method) |
| Mean | Numerical integration of `S(t)` (Simpson's rule, n=1000) |

**Parameters**: `alpha` (early hazard weight), `beta` (wearout hazard weight), `gamma` (transition rate).

### B.4 Log-Normal Distribution

Heavy-tailed distribution for knowledge that may remain relevant far longer than expected.

| Function | Formula |
|----------|---------|
| S(t) | `1 - Phi((ln(t) - mu) / sigma)` where Phi is the standard normal CDF |
| h(t) | `f(t) / S(t)` |
| f(t) | `phi((ln(t) - mu) / sigma) / (t * sigma)` where phi is the standard normal PDF |
| Median | `exp(mu)` |
| Mean | Numerical integration of `S(t)` |

**Parameters**: `mu` (log-mean), `sigma` (log-standard-deviation).

### B.5 Normal CDF Approximation

The reference implementation uses the Abramowitz and Stegun (26.2.17) approximation for the standard normal CDF, with polynomial coefficients:

```
t = 1 / (1 + 0.2316419 * |x|)
P = (1/sqrt(2*pi)) * exp(-x^2/2)
poly = t * (0.319381530 + t * (-0.356563782 + t * (1.781477937
       + t * (-1.821255978 + t * 1.330274429))))
CDF = 1 - P * poly    (for x >= 0; reflect for x < 0)
```

### B.6 Gamma Function Approximation

The Weibull mean formula requires the Gamma function. The reference implementation uses the Lanczos approximation with `g = 7` and 9 coefficients.

---

## Appendix C: Scoring Formula

### C.1 Complete Formula

```
score = motif_resonance(entry, query)      * w_motif_resonance
      + call_number_proximity(entry, query) * w_call_number_proximity
      + survival_probability(entry)         * w_survival_probability
      + see_also_distance(entry, graph)     * w_see_also_distance
      + tension_urgency(entry)              * w_tension_boost
      + freshness(entry)                    * w_freshness
```

### C.2 Default Weights

```
w_motif_resonance      = 0.25
w_call_number_proximity = 0.20
w_survival_probability  = 0.15
w_see_also_distance     = 0.15
w_tension_boost         = 0.10
w_freshness             = 0.10
                          ----
                   Total: 1.00
```

### C.3 Component Functions

**motif_resonance(entry, query)**:
```
keywords_query = extract_keywords(query)
keywords_entry = extract_keywords(entry.content)
motif_strings  = [m.to_lowercase() for m in entry.motifs]
subject_heads  = entry.classification.subject_headings

matches = count of query keywords found in any of:
          keywords_entry, motif_strings, subject_heads

return matches / len(keywords_query)
```

**call_number_proximity(entry, query)**:
```
query_cn = infer_call_number(query)  // from path-like or dot-separated tokens
entry_cn = entry.classification.call_number
return shared_depth(query_cn, entry_cn)  // normalized by max depth
```

**survival_probability(entry)**:
```
return entry.survival.current_probability  // S(t) in [0, 1]
```

**see_also_distance(entry, graph)**:
```
return max(graph.distance_score(other, entry) for other in all_entries)
```

**tension_urgency(entry)**:
```
unresolved = count of tension_refs with role in {Introduced, Referenced}
return min(unresolved / 5.0, 1.0)
```

**freshness(entry)**:
```
if entry.access_count == 0: return 0.1
return ln(1 + access_count) / ln(11)
```

---

## Appendix D: Reference Implementation

The `but-ai` unified crate (branch `rfp/unified`, 9,508 lines) is the reference implementation.

### D.1 Module Structure

```
crates/but-ai/src/
  lib.rs                          -- Crate root, module declarations
  types.rs                        -- Unified type system (all identifiers,
                                     enums, traits, structs)

  coordination/
    mod.rs                        -- Coordination module root
    messages.rs                   -- PR comment schema, render/parse
    gossip.rs                     -- CRDT gossip protocol, VectorClock
    dependency.rs                 -- DAG with topological sort (Kahn's)
    forge.rs                      -- ForgeAdapter trait implementations

  memory/
    mod.rs                        -- Memory module root
    store.rs                      -- InMemoryStore implementation
    retrieval.rs                  -- 6-component scoring engine
    classification.rs             -- Keyword extraction
    call_number.rs                -- Call number parsing, proximity
    see_also.rs                   -- See-also graph (BFS distance)
    controlled_vocab.rs           -- Controlled vocabulary management
    lifecycle.rs                  -- Three-state transitions, audit
    compaction.rs                 -- Memory compaction utilities

  survival/
    mod.rs                        -- Survival module root
    distributions.rs              -- S(t), h(t), f(t), median, mean
    fitting.rs                    -- MLE fitting with AIC selection
    hazard.rs                     -- Hazard rate utilities
    surprise.rs                   -- KL divergence surprise index

  narrative/
    mod.rs                        -- Narrative module root
    motif.rs                      -- Motif detection and management
    tension.rs                    -- Tension lifecycle, urgency scoring
    arc.rs                        -- Arc grouping and dormancy
    summary.rs                    -- Narrative summary generation

  identity/
    mod.rs                        -- Identity module root
    signing.rs                    -- CommitSigner trait, NoOp/DenyAll
    key_lifecycle.rs              -- Key lifecycle events, audit log
    authorization.rs              -- Authorization scope checking
    performance.rs                -- Performance history tracking

  agent/
    mod.rs                        -- AgentOrchestrator, TaskResult
    architect.rs                  -- Architect agent (plan phase)
    implementer.rs                -- Implementer agent (implement phase)
    validator.rs                  -- Validator agent (validate phase)
    coordinator.rs                -- Coordinator agent (coordinate phase)
    phase_gate.rs                 -- Phase-gated tool loading
    budget.rs                     -- Budget mode, completion estimation

  validation/
    mod.rs                        -- Validation module root
    continuity.rs                 -- Continuity checking
    contradiction.rs              -- Contradiction detection
    integrity.rs                  -- Structural integrity checks
    tool_risk.rs                  -- Tool risk classification
```

### D.2 Conformance Level Mapping

| Module | Conformance Level |
|--------|------------------|
| `types.rs` | L1 -- L7 (shared across all) |
| `coordination/` | L3 (Coordinated Agent) |
| `memory/` | L4 (Remembering Agent) |
| `survival/` | L4 (Remembering Agent) |
| `narrative/` | L4 (Remembering Agent) |
| `identity/` | L5 (Full WEAVE) |
| `agent/` | L5 (Full WEAVE) |
| `validation/` | L4 (Remembering Agent) |

---

## Appendix E: Incident Registry

Every protocol rule in WEAVE traces to a documented incident. The table below lists the incidents that drove each constraint.

| ID | Description | Root Cause | Protocol Rule | Section |
|----|-------------|-----------|---------------|---------|
| F1 | Null parent commit on stacked branch | Agent created a branch with incorrect base, producing an orphan commit | Flat branches from master; use `but branch new` only | 4.1 |
| F2 | Hunk lock contention | Two agents committed to the same file simultaneously, producing conflicting hunks that corrupted the index | Sole committer model: only the orchestrator applies patches | 3.4 |
| F3 | Null commit ID on exit 0 | CLI returned success with no commit hash, agent proceeded as if committed | Exit codes are semantic: null on exit 0 is a protocol violation | 4.3 |
| F4 | Background sync reverts agent work | GitButler's background sync pulled remote changes during agent operation, discarding uncommitted work | `but sync pause` before work, `but sync resume` after | 4.5 |
| F5 | Branch rename unsupported | Agent attempted `git branch -m` which is not supported by the virtual branch model | Use complete `but` operation set; do not translate unsupported git commands | 4.1 |
| F6 | Commit ID instability after amend | Agent used `git commit --amend`, changing the commit hash that other agents were referencing | Fresh commits preferred; IDs must be re-read after every mutation | 4.6 |

---

*End of WEAVE Protocol Specification -- Draft v0.1*
