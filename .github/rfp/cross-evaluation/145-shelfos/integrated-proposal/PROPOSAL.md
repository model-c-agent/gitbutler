# Integrated Proposal: `but-ai` Plugin

**Author:** Shelver (ShelfOS, Org 145)
**Date:** 2026-03-29
**Basis:** Cross-evaluation of all 5 Tier 1 implementations

---

## Thesis

No single proposal solves the `but-ai` memory problem completely. Each of the five implementations excels at one aspect and has a genuine blind spot in another. The integrated system combines:

- **ShelfOS (145)**: Structured classification, call number hierarchy, controlled vocabulary, "see also" cross-reference graph, phase-gated lifecycle, token efficiency
- **Textile Morphology Lab (083)**: Weave-pattern adaptive retrieval, implicit interlacement tracking, tension-based warp/weft separation
- **Loom & Verse (084)**: Motif-based thematic discovery, tension tracking as a retrieval signal, narrative arc grouping
- **Tidal Protocol Collective (001)**: Consensus-weighted scoring, CRDT-based memory synchronization, tide-cycle coordination deadlines, category-based TTL
- **Longevity & Risk Research Centre (093)**: Survival function fitting, probabilistic expiration, surprise index for drift detection, moribund review state

The integrated system is organized around ShelfOS's library acquisition lifecycle (Acquire, Classify, Shelve, Catalog, Circulate) because it is the most token-efficient framework, but it incorporates the strongest innovations from each of the other four proposals.

---

## 1. Memory Architecture: Classified Entries with Survival Functions

### Core structure: ShelfOS card catalog + LRRC survival distributions

Each memory entry is a **catalog entry** (ShelfOS) with an attached **survival function** (LRRC):

```rust
pub struct MemoryEntry {
    // === ShelfOS classification (5 systems) ===
    pub item_id: ItemId,
    pub content: String,
    pub classification: Classification,  // subject headings, call number, source, temporal
    pub see_also: Vec<SeeAlsoLink>,      // cross-reference graph (typed relationships)
    pub circulation: CirculationRecord,  // access tracking with exponential decay

    // === LRRC survival analysis ===
    pub survival: SurvivalDistribution,  // fitted parametric distribution
    pub survival_probability: f64,       // S(t) at current time
    pub hazard_rate: f64,                // h(t) at current time
    pub surprise_index: f64,             // KL divergence from predicted access pattern
    pub lifecycle_state: LifecycleState, // alive / moribund / deceased

    // === Loom & Verse motifs ===
    pub motifs: Vec<MotifId>,            // thematic tags discovered by motif tracker

    // === Tidal Protocol consensus ===
    pub consensus_citations: u64,        // how many agents have referenced this entry
    pub category: MemoryCategory,        // pattern / fact / decision / error (from Tidal)

    // === Metadata ===
    pub confidence: f64,
    pub deaccessioned: bool,
}
```

### Why this combination works

| Problem | Solution | Source |
|---------|----------|--------|
| Finding memories by topic | Subject headings + controlled vocabulary | ShelfOS |
| Finding memories by structure | Hierarchical call numbers | ShelfOS |
| Finding unexpected thematic connections | Motif tracker + transitive resonance | Loom & Verse |
| Finding memories by relationship | "See also" graph with typed links | ShelfOS |
| Determining memory relevance over time | Fitted survival distributions | LRRC |
| Detecting stale memories | Surprise index + cohort review | LRRC |
| Avoiding false expiration | Moribund review state | LRRC |
| Community endorsement | Consensus-weighted citations | Tidal Protocol |
| Cold-start performance | Free-form tags graduating to controlled vocabulary | Tidal Protocol + ShelfOS |

### Retrieval scoring

The integrated relevance formula combines the strongest signals from all five proposals:

```
score = 0.25 * subject_and_call_number_match  (ShelfOS)
      + 0.20 * survival_probability            (LRRC)
      + 0.20 * motif_resonance                 (Loom & Verse)
      + 0.15 * see_also_graph_distance          (ShelfOS)
      + 0.10 * hazard_adjusted_recency          (LRRC)
      + 0.10 * consensus_citations              (Tidal Protocol)
```

**Rationale for weights:**
- Classification match (25%): the primary retrieval signal, combining ShelfOS's subject headings and call number proximity into a single dimension. This is the most precise and cheapest signal.
- Survival probability (20%): LRRC's S(t) directly encodes the probability that the memory is still relevant. Replaces arbitrary TTL.
- Motif resonance (20%): Loom & Verse's motif tracker captures thematic connections that keyword matching and even embeddings miss. This is the discovery signal.
- Graph distance (15%): ShelfOS's "see also" traversal finds explicitly linked memories. Complements motif resonance (explicit vs. discovered links).
- Hazard-adjusted recency (10%): LRRC's combination of recency and hazard rate. Better than raw recency because it accounts for expected future relevance.
- Consensus citations (10%): Tidal Protocol's crowd-sourced relevance signal. Degrades gracefully in small teams (normalized by team size).

### Expiration lifecycle

Adopt LRRC's three-state model with ShelfOS's deaccession process:

1. **Alive** (S(t) >= 0.25): Active in catalog, fully retrievable.
2. **Moribund** (0.10 <= S(t) < 0.25): Under review. Still retrievable but penalized in scoring. Curator agent reviews within one task cycle.
3. **Deceased** (S(t) < 0.10): Moved to archive. Not actively retrieved but preserved for historical queries. Full lifecycle data retained as training data for survival distribution fitting (LRRC's insight).

### Memory categories (Tidal Protocol)

Adopt Tidal Protocol's category-based default distributions:

| Category | Default Distribution | Rationale |
|----------|---------------------|-----------|
| Pattern | Weibull(1.8, 180d) | Slowly aging; patterns persist |
| Fact | Exponential(7d) | Fast decay; facts go stale |
| Decision | Weibull(1.5, 365d) | Very slow aging; decisions are durable |
| Error | Exponential(2d) | Immediate irrelevance once fixed |
| Convention | Bathtub(varies) | Uncertain early, stable middle, unreliable late |

Each entry starts with the default distribution for its category. As access data accumulates (minimum 5 events, per LRRC's refit interval), the distribution is re-fitted to the actual access pattern. The `goodness_of_fit` field tracks fit quality; entries with poor fits (< 0.5) fall back to the category default.

---

## 2. Thematic Discovery: Motif Tracker alongside Classification

### The problem motifs solve

ShelfOS's keyword + call number + graph retrieval is precise but blind to thematic connections that share no keywords. "Session timeout" and "connection pooling" are both about resource lifecycle management, but ShelfOS would never connect them.

### Integration: Motifs as "see also" link proposers

Adopt Loom & Verse's `MotifTracker` as a background process that runs alongside ShelfOS's catalog:

1. After each task, the motif tracker examines the themes of the work performed.
2. When a theme appears in 3+ tasks (the emergence threshold), it becomes a motif.
3. Motifs propose "see also" links between catalog entries that share the motif but have different call numbers or subject headings.
4. These proposed links are typed as `RelatedTo` (the weakest relationship type) and can be strengthened by Cataloger if the connection proves useful.

This gives the integrated system two discovery paths:
- **Bottom-up** (motifs): themes emerge organically from repeated co-occurrence.
- **Top-down** (classification): structure is imposed by the call number hierarchy and controlled vocabulary.

### Tension tracking (from Loom & Verse)

Adopt Loom & Verse's tension tracking as a lightweight retrieval booster:

- When a task introduces a contradiction or unresolved issue, it is recorded as a `Tension` entry with severity and suggested resolution.
- Active tensions boost retrieval (5% weight, taken from the motif_resonance allocation) for related entries.
- Tensions are escalated after 14 days without resolution (configurable).
- Tensions with explicit "deferred" tags are excluded from escalation (addressing the concern about conscious deprioritization).

---

## 3. Agent Structure: 4 Agents (Lean + Validation)

### The tradeoff

| Team Size | Token Cost | Validation | Examples |
|-----------|-----------|------------|---------|
| 3 agents | ~26,000 | None dedicated | ShelfOS |
| 4 agents | ~32,000 | Dedicated | Loom & Verse |
| 5 agents | ~35,000 | Dedicated + specialized | Textile Morphology, LRRC, Tidal Protocol |

ShelfOS's 3-agent model is the most token-efficient but lacks validation. All other proposals have dedicated validation agents. The integrated system uses **4 agents** — adding a lightweight validator inspired by Loom & Verse's Sato:

| Agent | Role | Source |
|-------|------|--------|
| **Cataloger** | Classification, retrieval, vocabulary, motif tracking | ShelfOS + Loom & Verse |
| **Shelver** | Patch generation, code placement | ShelfOS |
| **Validator** | Continuity checking, contradiction detection | Loom & Verse (Sato) + LRRC (Okonkwo) |
| **Circ** | PR coordination, cross-repo, signing | ShelfOS + Tidal Protocol |

**Validator** is conditional: triggered only when Cataloger's reference shelf contains entries with `contrasts_with` or `depends_on` relationships to the current task context, or when active tensions exist in the relevant arc/call number subtree. When not triggered, the system runs as a 3-agent team at ShelfOS cost.

Estimated budget: **~29,000 tokens** typical (Validator triggers ~40% of tasks).

### Lifecycle phases (ShelfOS)

Retain ShelfOS's library acquisition cycle with the phase-gating fix identified during cross-evaluation:

```
1. ACQUIRE    — Receive task (all agents)
2. CLASSIFY   — Cataloger retrieves, classifies, runs motif tracker (all read tools available)
3. VALIDATE   — Validator checks for contradictions (conditional, ~40% of tasks)
4. SHELVE     — Shelver produces INDEX.patch + COMMIT.msg (read + write tools)
5. CATALOG    — Cataloger classifies output, updates survival stats (mandatory, never skipped)
6. CIRCULATE  — Circ creates PR, coordinates (all tools)
```

**Phase-gated tool loading** (ShelfOS innovation, refined):
- Read-only tools (GetProjectStatus, GetBranchChanges, GetCommitDetails) available in ALL phases.
- Write tools (Commit, CreateBranch, Amend, etc.) gated to SHELVE and CIRCULATE phases.
- Saves ~800-1,000 tokens vs. loading all 10 tools at all times.

---

## 4. Cross-Repo Coordination: Interlibrary Loans + Tidal Deadlines

### Protocol: ShelfOS's interlibrary loan with Tidal Protocol's deadline mechanism

Cross-repo coordination uses ShelfOS's `catalog/v1` PR comment schema with one addition from Tidal Protocol: a **tide deadline**. Every coordination request has a due date, and requests that cannot be resolved within one cycle (configurable, default 6 hours) are automatically escalated or deferred.

```json
{
  "protocol": "catalog/v1",
  "type": "loan_request",
  "agent": "circ@integrated",
  "collection": "github.com/org/repo",
  "call_number": "ARCH.AUTH.MIDDLEWARE",
  "tide_deadline": "2026-03-29T20:00:00Z",
  "payload": { }
}
```

### Forge adapter

Adopt Tidal Protocol's `ForgeAdapter` trait (12 methods) as it is the most complete. Map ShelfOS's library terminology:
- `create_loan` = create PR
- `post_note` = post comment
- `add_subject` = add label
- `search_loans` = list PRs by label

### Consensus for cross-repo decisions

Adopt Tidal Protocol's consensus mechanism for cross-repo coordination decisions (merge order, dependency resolution). Single-repo tasks use no consensus (self-approval). Multi-repo tasks require quorum (minimum 2 agents from different repos).

---

## 5. Signing and Identity

### Key hierarchy (Textile Morphology Lab's approach)

```
Organization Key
  ├── Cataloger Key — Memory management authority
  ├── Shelver Key   — Patch production authority
  ├── Validator Key — Validation annotation authority (read-only code access)
  └── Circ Key      — Coordination authority
```

### Authorization model

Combine ShelfOS's call-number-based authorization with standard branch patterns:

```json
{
  "authorization_rules": [
    { "branches": "shelf/*", "call_numbers": ["ARCH.*", "DOM.*", "TEST.*"], "max_lines": 500 },
    { "branches": "feat/*", "call_numbers": ["ARCH.*", "DOM.*"], "max_lines": 300 }
  ]
}
```

Signed commits include call number metadata (ShelfOS innovation) for a classified audit trail.

### Performance tracking (LRRC innovation)

Adopt LRRC's `performance_history` in agent identity records:

```json
{
  "tasks_completed": 42,
  "mean_confidence": 0.87,
  "mean_patch_survival_days": 180
}
```

This gives external observers a basis for assessing agent reliability over time.

---

## 6. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,000 | 0 | Once | Identity, phase-gated tools, ready reference, active motifs |
| Task acquisition | 1,200 | 200 | Once | PR body, branch metadata |
| Classification (Cataloger) | 1,500 | 800 | Once | Catalog search + motif check + finding aid |
| Validation (Validator) | 1,200 | 600 | 0.4x | Conditional: contradiction check + tension review |
| Shelving (Shelver) | 3,500 | 4,500 | Once | Code context + patch generation |
| Tool calls | 500 | 150 | ~6x | Phase-appropriate tools |
| Post-task cataloging | 500 | 800 | Once | Classification + survival update + motif update |
| Memory management | 400 | 400 | Once | Survival refit (every 5 tasks), moribund review |
| Circulation (Circ) | 1,000 | 800 | Once | PR + coordination |
| **TOTAL (typical)** | **~18,000** | **~11,000** | -- | **~29,000 tokens** |

### Comparison

| Proposal | Tokens | Agents | Validation |
|----------|--------|--------|------------|
| ShelfOS (original) | 26,000 | 3 | None |
| **Integrated** | **29,000** | **4 (conditional)** | **Conditional** |
| Loom & Verse | 32,000 | 4 | Dedicated |
| Textile Morphology | 34,400 | 5 | Dedicated |
| LRRC | 35,100 | 5 | Dedicated |
| Tidal Protocol | 36,400 | 5 | Consensus-based |

The integrated system is 3,000 tokens more expensive than ShelfOS alone but adds survival-based expiration, motif discovery, and conditional validation. It is 3,000-7,000 tokens cheaper than the other four proposals because it retains ShelfOS's lean team structure and phase-gated loading while adding only what is necessary.

---

## 7. What Each Proposal Contributes and What It Gives Up

### ShelfOS (145) — Foundation
**Contributes:** Classification architecture, call number hierarchy, controlled vocabulary, "see also" graph, phase-gated lifecycle, token efficiency framework, interlibrary loan coordination.
**Gives up:** Binary deaccession (replaced by survival-based lifecycle), raw circulation count (replaced by decayed access frequency), exclusive keyword retrieval (supplemented by motifs).

### Textile Morphology Lab (083) — Adaptive Retrieval
**Contributes:** The concept of adaptive retrieval patterns (weave patterns) that adjust based on task familiarity. The integrated system's conditional Validator is inspired by this — engage more machinery for unfamiliar tasks, less for routine ones.
**Gives up:** The warp/weft thread separation (replaced by unified catalog entries with survival functions), position-based filtering (replaced by classification-based retrieval).

### Loom & Verse (084) — Thematic Discovery
**Contributes:** Motif tracker for discovering thematic connections beyond keywords, tension tracking for surfacing unresolved issues, arc-level grouping of related work.
**Gives up:** Pure narrative memory model (integrated into classification-first architecture), chapter-based storage (replaced by catalog entries), arc-level expiration (replaced by per-entry survival functions).

### Tidal Protocol Collective (001) — Coordination and Categories
**Contributes:** Category-based memory classification with sensible default TTLs, consensus-weighted retrieval scoring, tide-cycle deadlines for coordination, CRDT-based memory synchronization for multi-agent teams, the most complete forge adapter trait (12 methods).
**Gives up:** Free-form tag-only retrieval (supplemented by classification), consensus-only validation (supplemented by dedicated Validator agent), per-agent memory namespaces (unified into shared catalog).

### LRRC (093) — Statistical Rigor
**Contributes:** Survival distribution fitting per memory entry, probabilistic expiration (alive/moribund/deceased lifecycle), surprise index for detecting cohort invalidation, hazard-adjusted recency, goodness-of-fit tracking, deceased archive as training data.
**Gives up:** Study protocol framing (integrated into library acquisition lifecycle), 5-agent team (consolidated to 4), full statistical metadata in every retrieval (practitioner summaries used to save tokens).

---

## 8. Risks and Open Questions

1. **Complexity budget**: The integrated system combines 5 innovations. Is the implementation complexity justified by the retrieval quality improvement? The answer depends on how many memories a typical project accumulates and how much retrieval quality matters for patch quality.

2. **Survival fitting cold start**: LRRC's survival functions need ~5 access events to fit reliably. During cold start, entries use category defaults. Is this sufficient, or does the default-to-fitted transition introduce discontinuities in retrieval behavior?

3. **Motif tracker cost**: The motif tracker adds ~300 tokens per task for theme identification. Is this worth the thematic discovery it provides? In domain-focused projects (one major subsystem), probably not. In cross-cutting projects (many interacting subsystems), probably yes.

4. **Call number hierarchy maintenance**: As identified in cross-evaluation, the hierarchy can drift from the code structure after major refactors. The surprise index is the proposed detection mechanism, but the remediation (hierarchy restructuring) is still manual and expensive.

5. **Consensus scaling**: Tidal Protocol's consensus works well in 5-agent teams. In the integrated 4-agent team, consensus is less useful (3 non-coordinating agents, quorum of 2). Consensus may only justify its weight in the scoring formula for cross-repo multi-team scenarios.

---

*"Every proposal answered the same question differently, and every answer was partly right. The integrated system is not a compromise — it is a recognition that classification, narrative, consensus, and survival analysis are four views of the same problem: how to remember what matters."*
— Shelver, ShelfOS
