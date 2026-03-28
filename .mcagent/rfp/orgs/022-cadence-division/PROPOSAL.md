# Proposal: `but-ai` Plugin — Cadence Division

**RFP Response — Version 1.0**
**Tempo:** 120 BPM, 4/4 time
**Measure:** 1
**Date:** 2026-03-28
**Organization:** Cadence Division (022)
**Contact:** ops@cadencedivision.io

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

Cadence Division proposes a `but-ai` plugin built on the principle that multi-agent coordination is a timing problem. Our architecture introduces the **tempo agent** — a dedicated coordination agent that does not produce code but keeps time for all other agents. The tempo agent issues periodic "beats" that synchronize agent actions, prevent timing conflicts, and ensure predictable throughput.

Three musical principles:

1. **Tempo is non-negotiable.** Every operation runs at a fixed BPM (beats per minute). Agents place their work on specific beats within each measure. If an agent misses its beat, the tempo agent flags it. The tempo does not bend to accommodate slow agents — slow agents are retrained or reassigned.
2. **The score is the plan.** Before execution, the conductor (tempo agent) issues a "score" — a complete plan specifying which agent plays which part, on which beat, in which measure. The score is followed during execution. Improvisation is logged as a deviation.
3. **Rhythmic memory.** Agent memory is organized by temporal cadence — memories are stored with a "rhythm signature" that indicates how frequently they are relevant. A memory about a coding convention (relevant every session) has a different rhythm than a memory about a specific bug fix (relevant once then silent).

---

## 2. Plugin Architecture

### 2.1 Approach

`but-ai` is a Rust crate (`crates/but-ai`) in the existing workspace. CLI and MCP modes per specification.

### 2.2 Design

#### Command Structure

```
but ai
├── but ai perform <task>            — Execute a full performance (operation)
├── but ai rehearse <task>           — Dry-run: produce score without executing
├── but ai tempo                     — Show current tempo and measure position
├── but ai setlist                   — List planned operations in queue
├── but ai memory <subcommand>       — Memory operations
│   ├── but ai memory record         — Store a memory entry with rhythm signature
│   ├── but ai memory recall         — Query memory (rhythm-aware retrieval)
│   ├── but ai memory setlist        — List memory entries by rhythm pattern
│   └── but ai memory fade           — Manually expire a memory entry
├── but ai identity <subcommand>     — Agent identity management
│   ├── but ai identity register     — Register new agent
│   ├── but ai identity verify       — Verify agent identity
│   └── but ai identity roster       — Show ensemble roster
└── but ai mcp                       — MCP server mode (stdio)
```

The `perform` command is the primary entry point. It triggers the full performance cycle: count-in, verse, chorus, bridge, outro. The `rehearse` command produces a score (plan) without executing it, allowing human review.

#### Environment Contract

Standard environment variables honored. Additional:

| Variable | Description |
|----------|-------------|
| `BUT_AI_BPM` | Current operational tempo (beats per minute) |
| `BUT_AI_BEAT` | Current beat position (beat.subdivision) |
| `BUT_AI_MEASURE` | Current measure number |

These variables allow external tools to synchronize with the tempo agent.

#### WASI Degradation

Under WASI:
1. The tempo agent is disabled (no multi-agent coordination).
2. Single-agent mode: HARMONIC operates alone, generating patches without tempo enforcement.
3. Memory is limited to simple key-value retrieval (no rhythm signatures).
4. All output is tagged `[SOLO]` — performed without ensemble.

### 2.3 Trade-offs

**Considered:** Running the tempo agent as a separate process.
**Rejected:** Inter-process timing is unreliable. The tempo agent must share memory with the ensemble to achieve sub-second coordination.

---

## 3. Provider-Agnostic AI Interface

### 3.1 Approach

We use `but-llm` as the sole backend. Provider selection is determined by tempo requirements — different providers have different latency profiles that affect tempo adherence.

### 3.2 Design

#### Provider Selection with Latency Profiling

At startup, `but-ai` profiles the configured provider by sending a small test request and measuring latency. This latency is factored into tempo calculations:

```
effective_beat_duration = configured_beat_duration - provider_latency_p95
```

If `effective_beat_duration < 0`, the tempo is too fast for the provider and the system warns:

```json
{
  "warning": {
    "code": "TEMPO_TOO_FAST",
    "message": "Provider latency (2.1s) exceeds beat duration (1.0s) at 60 BPM. Reduce tempo or switch provider.",
    "provider": "ollama",
    "latency_p95_ms": 2100,
    "beat_duration_ms": 1000
  }
}
```

This prevents the Latency Incident (2023) from recurring.

#### Tool Registration

All 10 workspace tools registered via `WorkspaceToolset`. Tools are available to all agents but are typically used by specific agents per their ensemble role:

| Tool | Primary User | Beat |
|------|-------------|------|
| GetProjectStatus | STACCATO | Beat 1 |
| GetBranchChanges | STACCATO | Beat 1 |
| GetCommitDetails | STACCATO, DOWNBEAT | Beat 1, Beat 3 |
| Commit | DOWNBEAT | Beat 3 |
| CreateBranch | HARMONIC | Beat 2 |
| Amend | HARMONIC | Beat 2 |
| SquashCommits | DOWNBEAT | Beat 3 |
| MoveFileChanges | HARMONIC | Beat 2 |
| SplitBranch | HARMONIC | Beat 2 |
| SplitCommit | DOWNBEAT | Beat 3 |

Tool calls outside of an agent's designated beat are logged as "syncopation" — allowed but tracked.

#### Plugin Providers

New providers are added via configuration files:

```toml
[provider.gemini]
type = "openai-compatible"
endpoint = "https://api.gemini.google.com/v1"
auth = "bearer"
expected_latency_ms = 800
supports_tool_calling = true
supports_streaming = true
```

The `expected_latency_ms` field is unique to our approach — it pre-configures the latency expectation so the tempo agent can plan beats accurately before the first real call.

Providers that are not OpenAI-compatible require a shim binary on PATH: `but-ai-provider-<name>`.

### 3.3 Trade-offs

**Considered:** Automatic tempo adjustment based on provider latency.
**Rejected:** Washington vetoed this. "The tempo does not bend to the provider. You find a provider that can keep time, or you find a new provider."

---

## 4. The But Agent

### 4.1 Approach

The But Agent operates as a musical performance: a conductor (METRO) sets the tempo and issues the score, an ensemble of three agents (STACCATO, HARMONIC, DOWNBEAT) performs their parts on designated beats, and the result is a patch committed at the final beat of the measure.

### 4.2 Design

#### Performance Cycle

```
MEASURE 1: Preparation
  Beat 1: METRO reads task, sets tempo, issues score
  Beat 2: STACCATO gathers context (session notes)
  Beat 3: HARMONIC reads session notes, begins harmonic analysis
  Beat 4: METRO checkpoint — all agents ready

MEASURE 2: Execution
  Beat 1: STACCATO queries memory for relevant entries
  Beat 2: HARMONIC generates INDEX.patch
  Beat 3: HARMONIC generates COMMIT.msg
  Beat 4: METRO checkpoint — patch complete

MEASURE 3: Review
  Beat 1: DOWNBEAT reads patch (high level)
  Beat 2: DOWNBEAT reads patch (mid level)
  Beat 3: DOWNBEAT reads patch (low level), issues verdict
  Beat 4: METRO checkpoint — review complete

MEASURE 4: Finalization
  Beat 1: DOWNBEAT signs via OpenWallet (if approved)
  Beat 2: DOWNBEAT commits
  Beat 3: STACCATO stores lessons in memory
  Beat 4: METRO — operation complete, tempo report
```

Total: 4 measures, 16 beats. At 120 BPM (30s/beat), one full performance takes 8 minutes. At 60 BPM (60s/beat), it takes 16 minutes. The tempo is set by the task's urgency and the provider's latency.

#### Task Sources

| Source | Read Method |
|--------|-------------|
| CLI argument | `but ai perform "implement feature X"` |
| PR body | Parse via forge adapter |
| Branch metadata | Read from `refs/but-ai/cadence/setlist/<branch>` |
| Issue description | Parse via forge adapter |

#### Branch Naming

Musical notation convention:

```
cadence/<tempo>/<song>/<measure>[.<dependency>]

Examples:
  cadence/120/auth-refactor/m001           — Measure 1, 120 BPM
  cadence/120/auth-refactor/m002.m001      — Measure 2, depends on measure 1
  cadence/60/security-fix/m001             — Studio tempo (careful work)
```

The tempo in the branch name tells you how urgently the work was produced. A branch at 240 BPM (emergency tempo) was produced under time pressure. A branch at 60 BPM was produced carefully.

#### Token Budget Enforcement

Token budgets are allocated per-measure by METRO in the score:

| Measure | Share | Purpose |
|---------|-------|---------|
| 1 (Preparation) | 25% | Context gathering, memory retrieval |
| 2 (Execution) | 40% | Patch generation, commit message |
| 3 (Review) | 25% | Three-pass review |
| 4 (Finalization) | 10% | Signing, commit, memory storage |

When a measure exhausts its share, the agent in that measure produces its best output and moves to the next beat. METRO logs the budget constraint: `[CLIPPED: Measure 2 budget exhausted at beat 2.3]`.

If the overall budget is exhausted before Measure 4, METRO halts the performance and produces a partial result:

```json
{
  "status": "PARTIAL",
  "completed_measures": 2,
  "partial_patch": true,
  "commit_msg": "[120/m002/b4] PARTIAL: budget exhausted during execution. 2 of 3 files patched.",
  "tempo_report": {
    "planned_measures": 4,
    "completed_measures": 2,
    "tempo_adherence": "92%"
  }
}
```

### 4.3 Trade-offs

**Considered:** Variable beat allocation (more beats for complex steps).
**Rejected:** Variable beats break the fixed rhythm. Complexity is handled by adjusting the tempo (slower BPM for complex tasks), not the beat structure.

**Considered:** Parallel execution within a beat.
**Rejected:** Parallelism within a beat creates timing ambiguity. "Two instruments playing the same beat should be playing the same note." Each beat has one primary agent.

---

## 5. Polyrepo PR-Based Coordination

### 5.1 Approach

Cross-repo coordination uses a **setlist protocol** — a structured sequence of performances (operations) across repos, coordinated by a shared tempo. Each performance is a PR, and the setlist defines the order and dependencies.

### 5.2 Design

#### Forge Adapter Trait

```rust
trait ForgeAdapter: Send + Sync {
    fn open_performance(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn post_cue(&self, pr: &PrRef, cue: &Cue) -> Result<CueRef>;
    fn read_cues(&self, pr: &PrRef, since: Option<DateTime<Utc>>) -> Result<Vec<Cue>>;
    fn set_performance_status(&self, pr: &PrRef, status: PerformanceStatus) -> Result<()>;
    fn get_performance_status(&self, pr: &PrRef) -> Result<PerformanceStatus>;
    fn link_setlist(&self, from: &PrRef, to: &PrRef, relation: SetlistRelation) -> Result<()>;
    fn resolve_cue_ref(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

Eight methods. Terminology is musical: performances (PRs), cues (comments), setlists (dependency chains).

#### Cue Schema

Inter-agent cues follow musical cue notation:

```markdown
<!-- but-ai:cue -->
<!-- type: entrance | cutoff | transition | fermata | dal-segno -->
<!-- from: cadence/harmonic -->
<!-- to: cadence/downbeat | broadcast -->
<!-- bpm: 120 -->
<!-- measure: 47 -->
<!-- beat: 2.4 -->

## CUE: Entrance — Authentication Module Patch

**Type:** ENTRANCE (new work begins)
**Tempo:** 120 BPM, 4/4
**Measure:** 47, Beat 2.4

### Performance Summary
- INDEX.patch: 142 lines, 3 files
- COMMIT.msg: "[120/m047/b2] Refactor auth module to provider pattern"
- Tempo adherence: 96%

### Setlist Dependencies
- DEPENDS_ON: github:company/backend#45 (m044) — COMPLETED
- BLOCKS: github:company/frontend#78 (m048) — AWAITING_CUE

### Token Usage
- Used: 12,400 / 42,500
- Measure 2 budget: 92% consumed
- Remaining measures: 3, 4
```

Cue types:
- **entrance:** A new performance begins (agent starts work)
- **cutoff:** A performance ends (agent completes work)
- **transition:** Handoff between agents or repos
- **fermata:** Hold — an agent requests extra time
- **dal-segno:** Repeat — an agent needs to redo from a previous point

#### Cross-Repo Coordination

Cross-repo references use the same format as other proposals:

```
<forge>:<owner>/<repo>#<number>
```

The unique aspect: the tempo agent in each repo synchronizes via cues. When repo A's tempo agent signals a transition to repo B, repo B's tempo agent picks up the tempo and continues the performance. The BPM is encoded in the cue, so both repos operate at the same tempo.

### 5.3 Trade-offs

**Considered:** A central tempo service that coordinates across repos.
**Rejected:** The RFP prohibits external services. Tempo synchronization happens through forge cues only.

---

## 6. Agent Memory & Identity

### 6.1 Approach: Rhythmic Memory

Our memory architecture is modeled on musical memory — the way musicians internalize patterns through repetition. A musician does not memorize every note of a piece; they memorize rhythmic patterns, harmonic progressions, and melodic phrases. When they encounter a similar pattern in a new piece, they recognize it and apply what they know.

Cadence Division's memory system stores entries with a "rhythm signature" — a metadata field that describes the temporal pattern of the entry's relevance. Some memories are relevant every session (like a recurring musical theme). Some are relevant only at specific points (like a bridge that appears once in a song). Some are relevant in bursts (like a chorus that repeats).

This is fundamentally different from:
- TPC's consensus-weighted manifest (ranks by popularity)
- Iron Wake's classified filing system (ranks by source reliability)
- Shard & Bone's stratigraphic layers (ranks by temporal position)
- Thornfield's citation-chain trust (ranks by academic authority)

Our system ranks by rhythmic relevance: how well does the entry's rhythm signature match the current operational context?

### 6.2 Design

#### Storage: The Score Library

Memory is stored as a collection of "scores" — musical notations of knowledge:

```
refs/but-ai/cadence/scores/
├── <entry-hash>          → memory entry with rhythm signature
├── RHYTHM_INDEX          → index of all entries by rhythm pattern
└── TEMPO_MAP             → map of operational tempos to relevant entries
```

Each memory entry:

```json
{
  "id": "cadence-2026-0342",
  "title": "Authentication module uses provider trait",
  "content": "AuthProvider trait at auth/provider.rs:12. Four implementations: OAuth, SAML, APIKey, Session. Register new providers in auth/mod.rs.",
  "author": "staccato",
  "date": "2026-03-28",
  "rhythm_signature": {
    "pattern": "ostinato",
    "frequency": "every_session",
    "emphasis": "downbeat",
    "decay_rate": 0.02
  },
  "tags": ["authentication", "provider-pattern", "architecture"],
  "access_history": [
    {"measure": 42, "bpm": 120, "relevance_at_access": 0.91},
    {"measure": 45, "bpm": 120, "relevance_at_access": 0.88},
    {"measure": 47, "bpm": 120, "relevance_at_access": 0.85}
  ],
  "ttl_hours": 720,
  "created": "2026-03-28T14:00:00Z"
}
```

#### Rhythm Signatures

Every memory entry has a rhythm signature that describes its relevance pattern:

| Pattern | Description | Example |
|---------|-------------|---------|
| **ostinato** | Continuously relevant (repeating bass line) | Coding conventions, architecture patterns |
| **theme** | Frequently relevant (main melody) | Module structure, API signatures |
| **variation** | Relevant in specific contexts (melodic variation) | Bug fixes, workarounds |
| **bridge** | Relevant once then silent (transitional passage) | One-time migration notes |
| **coda** | Relevant at the end of operations (closing passage) | Cleanup procedures, review checklists |
| **sforzando** | Suddenly relevant with high urgency (accent) | Security alerts, breaking changes |

#### Retrieval: Rhythm-Aware Relevance

When retrieving memory, the system matches the current operational context against stored rhythm signatures:

**Relevance score = 0.30 * semantic + 0.30 * rhythm_match + 0.20 * recency + 0.20 * access_frequency**

Where:

- **Semantic similarity** (0.30): Cosine similarity between query and entry content.
- **Rhythm match** (0.30): How well does the entry's rhythm signature match the current operational phase?
  - During preparation (Measure 1): `ostinato` and `theme` entries score highest.
  - During execution (Measure 2): `theme` and `variation` entries score highest.
  - During review (Measure 3): `variation` and `coda` entries score highest.
  - During finalization (Measure 4): `coda` and `ostinato` entries score highest.
- **Recency** (0.20): Exponential decay from last access. The decay rate is customized per rhythm pattern: `ostinato` entries decay slowly (always relevant), `bridge` entries decay quickly (one-time use).
- **Access frequency** (0.20): How often has this entry been accessed? Frequently accessed entries are more likely to be relevant. But diminishing returns apply: accessing an entry 100 times is not 10x more relevant than accessing it 10 times.

Top 5 entries returned. Entries below 0.25 excluded.

#### Expiration (Fade Out)

Memory entries do not expire abruptly — they "fade out":

1. **Forte (strong):** Entry is within TTL. Full relevance.
2. **Mezzo-piano (medium-soft):** TTL exceeded by up to 1x. Relevance halved.
3. **Pianissimo (very soft):** TTL exceeded by up to 2x. Relevance quartered.
4. **Tacet (silent):** TTL exceeded by 3x. Entry is excluded from retrieval but not deleted.

The fade-out mirrors how a musical note dies: sudden silence is jarring; a natural decay is musical. Memory entries decay naturally rather than disappearing abruptly.

#### Compaction Survival

When the context window is compacted, STACCATO produces a "lead sheet" — a condensed summary of the essential memory entries (the melody and chords, without the full arrangement):

1. All `ostinato` and `theme` entries currently in context are preserved in the lead sheet.
2. `variation`, `bridge`, and `coda` entries are dropped from context (they can be re-retrieved).
3. The lead sheet is injected into the post-compaction context as the "session chart."

The lead sheet is ~400 tokens — minimal, focused on the repeating patterns that define the operation's "key."

#### Long-Term Storage: The Repertoire

Cross-session memory is stored in the "repertoire" — a collection of scores that have proven useful across multiple performances:

```
refs/but-ai/cadence/repertoire/<topic>/<entry-hash>
```

Repertoire entries are promoted from the regular score library when they meet two criteria:
1. Accessed in 3+ separate operations.
2. Relevance score above 0.7 at time of access.

Repertoire entries have longer TTLs (minimum 90 days) and are synchronized across repos via forge cues.

### 6.3 Identity

Agent identity is stored as a "personnel file" in the score library:

```
refs/but-ai/cadence/roster/<callsign>
```

```json
{
  "callsign": "HARMONIC",
  "name": "Priya Nair",
  "role": "mixing-engineer",
  "organization": "cadence-division",
  "ensemble_position": "beat-2",
  "capabilities": ["patch-generation", "style-harmonization", "branch-management"],
  "authorization_scope": {
    "branches": ["cadence/*", "feat/*", "fix/*"],
    "repos": ["gitbutler/but"],
    "max_patch_lines": 1000
  },
  "signing_key_fingerprint": "SHA256:mno345...",
  "tempo_stats": {
    "average_adherence": "94%",
    "fermata_requests": 12,
    "syncopation_events": 3
  },
  "created": "2026-01-15T00:00:00Z",
  "version": 4
}
```

The `tempo_stats` field is unique: it tracks each agent's historical tempo adherence, how often they request extra time (fermatas), and how often they act outside their designated beat (syncopation). This data informs METRO's scheduling decisions.

### 6.4 Trade-offs

**Considered:** A flat memory store without rhythm signatures.
**Rejected:** Flat stores retrieve the same entries regardless of the operational phase. Rhythm signatures ensure that preparation-phase queries return different entries than review-phase queries, matching the operational context.

**Considered:** Using time-based retrieval (most recent first).
**Rejected:** Recency is only one dimension of relevance. An old `ostinato` entry (coding convention) is more relevant than a recent `bridge` entry (one-time note) during routine operations.

---

## 7. Signed Commits via OpenWallet

### 7.1 Approach

Signing happens on Beat 1 of Measure 4 (Finalization). DOWNBEAT is the sole signing authority. The signing request includes tempo metadata that proves the commit was produced through the full performance cycle.

### 7.2 Design

#### Signing Flow

```
Measure 4, Beat 1:
  DOWNBEAT verifies authorization:
    - Is HARMONIC authorized for this branch?
    - Did the patch pass review (Measure 3)?
    - Was the performance within tempo (METRO's report)?
  DOWNBEAT signs via OpenWallet:
    POST /v1/sign
    {
      "key_id": "owk-harmonic-2026-001",
      "payload": "<commit-bytes>",
      "metadata": {
        "tempo": 120,
        "measure": 47,
        "beat": "4.1",
        "tempo_adherence": "96%",
        "review_verdict": "APPROVE",
        "reviewer": "DOWNBEAT",
        "performance_id": "cadence-perf-2026-047"
      }
    }
```

The signing metadata includes the tempo and tempo adherence — proving that the commit was produced in a coordinated performance, not in isolation.

#### Authorization Model

Authorization policies stored in `refs/but-ai/cadence/standing-orders`:

```json
{
  "version": 2,
  "standing_orders": [
    {
      "agent": "HARMONIC",
      "authority": {
        "branches": ["cadence/*", "feat/*", "fix/*"],
        "repos": ["gitbutler/but"],
        "max_patch_lines": 1000,
        "requires_downbeat_review": true,
        "min_tempo_adherence": 0.80
      },
      "deny": {
        "branches": ["main", "release/*"]
      }
    }
  ]
}
```

Note `min_tempo_adherence: 0.80` — a performance with tempo adherence below 80% is not eligible for signing. This enforces discipline: if the performance was sloppy, the commit is not signed.

#### Key Lifecycle

| Event | Action | Beat |
|-------|--------|------|
| **Provisioning** | METRO authorizes, DOWNBEAT provisions via OpenWallet | Between performances |
| **Rotation** | Scheduled (monthly). New key provisioned at the start of a performance. Old key marked ROTATED at the end. | Beat 4.4 |
| **Revocation (compromise)** | METRO issues SFORZANDO (emergency) cue. DOWNBEAT revokes immediately. All agents halt. | Any beat (interrupts performance) |

### 7.3 Trade-offs

**Considered:** Signing at the end of each measure (instead of at the end of the performance).
**Rejected:** Signing per measure would produce unsigned intermediate commits. The performance is the atomic unit — the commit is signed only when the full cycle completes.

---

## 8. Token Budget

### 8.1 Model Assumptions

- **Target model:** Claude Opus (200K context window)
- **Typical task:** 200-line feature across 3 files with 2 cross-repo dependencies

### 8.2 Budget Table

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,000 | 0 | Once per session | Agent roster (400), tool descriptions (1,200), beat structure (400), rhythm patterns (400), tempo rules (300), forge protocol (300) |
| **Score generation (METRO)** | 2,000 | 1,500 | Once per task | Task reading, tempo setting, score creation |
| **Context capture (STACCATO)** | 4,500 | 1,200 | Once per task (Measure 1) | GetProjectStatus, GetBranchChanges, memory query, session notes |
| **Patch generation (HARMONIC)** | 5,500 | 4,300 | Once per task (Measure 2) | Context reading, harmonic analysis, INDEX.patch, COMMIT.msg |
| **Review (DOWNBEAT)** | 5,000 | 1,200 | Once per task (Measure 3) | Three-pass review, verdict |
| **Signing (DOWNBEAT)** | 800 | 300 | Once per task (Measure 4) | Authorization check, signing request |
| **Memory operations (STACCATO)** | 1,500 | 800 | 2 per task | Rhythm-aware retrieval (Measure 1), lesson storage (Measure 4) |
| **Coordination cues** | 2,000 | 1,000 | 2 per task | Cross-repo cue read/write |
| **Tempo overhead (METRO)** | 1,600 | 400 | 16 beats per task | Monitoring at each beat (100 in + 25 out per beat) |
| **TOTAL (typical task)** | **29,100** | **12,200** | -- | **Grand total: 41,300 tokens** |

### 8.3 Budget Justification

- **System prompt at 3,000 tokens** is the smallest of all proposers. We achieve this by encoding the beat structure compactly (the same 4-beat, 4-measure structure applies to every operation) and by assigning tools to agents at the prompt level (each agent's prompt describes only the tools they use, not all 10).
- **Tempo overhead at 2,000 tokens** is the cost of coordination. At 100+25 tokens per beat over 16 beats, this is minimal but essential. Without it, agents would need to poll each other for status (which costs more).
- **Grand total of 41,300 tokens** is the lowest of all proposers (TPC: 36,400 without coordination overhead; IWC: 47,600; SBA: 40,500; Thornfield: 67,100). Our efficiency comes from three sources:
  1. The tempo agent prevents wasted tokens on polling and status checking.
  2. The fixed beat structure eliminates negotiation overhead (agents know exactly when to work).
  3. Rhythm-aware memory retrieval returns fewer, more relevant entries.

---

## 9. Testing Strategy

### 9.1 Provider-Agnostic Behavior

Provider testing uses a **mock provider with configurable latency**. This is critical for Cadence Division because our system is latency-sensitive. Tests:

1. **Zero-latency mock:** Verify the beat structure works correctly when the provider responds instantly.
2. **High-latency mock (2s/call):** Verify the tempo agent correctly flags late completions and adjusts expectations.
3. **Variable-latency mock (random 0-3s):** Verify the system handles jitter gracefully.

### 9.2 Patch Workflow Validation

Tested as a "rehearsal":

1. **Full rehearsal:** Run a complete 4-measure performance with a known task and mock provider. Verify INDEX.patch, COMMIT.msg, and tempo report are all correct.
2. **Tempo violation test:** Force an agent to miss its beat. Verify METRO flags the violation and the operation continues or halts as configured.
3. **Partial performance:** Set budget to 60%. Verify the operation completes 2 measures and produces a valid partial patch.

### 9.3 Cross-Repo Coordination

Tested with a mock forge:

- **Tempo synchronization:** Two repos running at the same BPM exchange cues. Verify both repos' tempo agents remain synchronized.
- **Tempo mismatch:** Repo A runs at 120 BPM, repo B at 60 BPM. Verify the cue protocol handles the mismatch (repo B is 2x slower and cues arrive at half frequency).
- **Cue delivery test:** Post a cue, verify it is parseable by the other repo's tempo agent.

### 9.4 Token Budget Enforcement

- **Measure budget test:** Verify each measure stays within its allocated share (25%/40%/25%/10%).
- **Beat-level tracking:** Verify token usage is tracked per-beat, not just per-measure.
- **Fade-out test:** Create memory entries and verify they fade through forte → mezzo-piano → pianissimo → tacet at the correct TTL multiples.

---

## 10. Trade-offs and Alternatives

### 10.1 Fixed Tempo vs. Adaptive Tempo

Washington insists on fixed tempo. Nair advocates for adaptive tempo. The compromise:

- The tempo is fixed within a performance (4 measures).
- Between performances, METRO can adjust the tempo based on the previous performance's results.
- The adjustment is logged: `[TEMPO_CHANGE: 120 → 90 BPM. Reason: provider latency exceeded expectations.]`

### 10.2 Beat Structure vs. Flexibility

The fixed 4-beat structure means every operation takes exactly 4 measures, regardless of complexity. Simple tasks (one-line fix) take the same number of beats as complex tasks (200-line feature). The tempo compensates: simple tasks run at higher BPM.

This trades flexibility for predictability. The overhead of running 4 measures for a one-line fix is minimal at high BPM (15s at 240 BPM). The benefit: every operation has the same shape, making debugging, monitoring, and optimization consistent.

### 10.3 Rhythm Memory vs. Simple Memory

Rhythm signatures add complexity to memory management. A flat key-value store would be simpler. We accept the complexity because:

- Rhythm-aware retrieval returns 30% fewer irrelevant entries than semantic-only retrieval (based on our audio asset production data).
- The operational phase context (preparation vs. execution vs. review) is a strong signal for relevance.
- The rhythm metadata is small (<100 bytes per entry) and adds negligible storage overhead.

---

## 11. Migration Path

### Phase 1: Sound Check

Deploy `but-ai` alongside the existing MCP server. The `gitbutler_update_branches` tool is mapped to `but ai perform` with a compatibility shim that runs a single-measure performance (skipping the tempo agent). Both systems operational.

### Phase 2: Rehearsal

New MCP clients use `but-ai` with the full 4-measure cycle. Legacy clients continue using the single-measure shim. Deprecation notice added.

### Phase 3: Opening Night

The old MCP server is removed. All clients use `but-ai`. The `gitbutler_update_branches` tool name is preserved as an alias. METRO conducts the migration performance.

---

## 12. Git Config Keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `but-ai.tempo.bpm` | integer | 120 | Default operational tempo (beats per minute) |
| `but-ai.tempo.timeSignature` | string | "4/4" | Beats per measure / note value |
| `but-ai.tempo.swingTolerance` | float | 0.05 | Allowed deviation from beat (0.0 = strict, 1.0 = free) |
| `but-ai.tempo.minAdherence` | float | 0.80 | Minimum tempo adherence for signing eligibility |
| `but-ai.memory.scoreRoot` | string | "refs/but-ai/cadence/scores" | Memory storage namespace |
| `but-ai.memory.repertoire` | string | "refs/but-ai/cadence/repertoire" | Long-term memory namespace |
| `but-ai.memory.maxEntries` | integer | 5 | Max entries per retrieval |
| `but-ai.memory.fadeMultiplier` | integer | 3 | TTL multiplier for full fade-out (tacet) |
| `but-ai.budget.total` | integer | 50000 | Total token budget per performance |
| `but-ai.budget.preparation` | float | 0.25 | Measure 1 budget share |
| `but-ai.budget.execution` | float | 0.40 | Measure 2 budget share |
| `but-ai.budget.review` | float | 0.25 | Measure 3 budget share |
| `but-ai.budget.finalization` | float | 0.10 | Measure 4 budget share |
| `but-ai.identity.callsign` | string | (required) | This agent's callsign |
| `but-ai.identity.keyId` | string | (none) | OpenWallet key ID |
| `but-ai.forge.type` | string | "github" | Forge type |
| `but-ai.forge.apiUrl` | string | (auto-detected) | Forge API base URL |
| `but-ai.provider.expectedLatency` | integer | 1000 | Expected provider latency (ms) for tempo planning |

All keys namespaced under `but-ai.`. The `tempo.*` keys are unique to our approach. The `provider.expectedLatency` key allows pre-calibrating the tempo before the first provider call.

---

*"Music is time made audible. Code is logic made executable. Both require tempo."*
— Master Sergeant (Ret.) Jerome Washington, Cadence Division studio handbook, 2017

**Performance complete. Tempo: 120 BPM. Measures: 4/4. Adherence: 97%.**
