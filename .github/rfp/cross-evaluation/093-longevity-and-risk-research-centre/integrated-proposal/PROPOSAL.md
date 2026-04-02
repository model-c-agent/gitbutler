# Integrated Proposal: `but-ai` Plugin

**From:** Professor Elena Vassiliev, Longevity & Risk Research Centre (Org 093)
**Cross-evaluation participants:** Orgs 083, 084, 001, 093, 145
**Date:** 2026-03-29

---

## Preamble

This integrated proposal is the product of cross-evaluation between all five Tier 1 organizations. I have read every proposal, every implementation, and every answer exchanged during the cross-evaluation. What follows is not a compromise. It is a synthesis built on observed convergence: every team independently acknowledged gaps that another team's approach fills.

The cross-evaluation produced three universal findings:

1. **Fixed expiration is misspecified.** All four non-LRRC teams (083, 084, 001, 145) acknowledged that their TTL or decay models are first-order approximations that fail for heterogeneous memory populations. The Tidal Protocol Collective's Dara stated: "our fixed TTL model is wrong in the general case." Tanaka (083) proposed replacing `tension_decay` with fitted survival distributions. Hartmann (084) conceded that fixed arc dormancy should be replaced by per-arc survival functions. ShelfOS acknowledged that binary deaccession should be replaced by continuous relevance probability. The convergence is unanimous: **survival-function-based expiration is the correct foundation.**

2. **Structural retrieval mechanisms beat statistical ones for young memories.** The LRRC's survival-function fitting requires 3-8 access records to become reliable. Before that, defaults govern. Loom & Verse's motif resonance (084) and ShelfOS's call number hierarchy + see-also graph (145) provide high-quality retrieval without access history. The integrated system needs both: **structural retrieval for young memories, survival-weighted retrieval for mature ones.**

3. **Contradiction detection is orthogonal to survival analysis.** The LRRC has no mechanism for detecting logically contradictory memories. Loom & Verse's tension tracking (084) fills this gap. Tensions should function as covariates in a survival model, accelerating the decline of contradicted memories. **The integrated system combines survival functions with tension tracking.**

---

## 1. Architecture

The integrated plugin is a single Rust crate (`crates/but-ai/`) within the GitButler workspace, compiling to a PATH-discovered binary with CLI and MCP modes.

**Binary structure:**
```
but-ai
  ├── but ai study <task>     -- Execute a task (LRRC study protocol)
  ├── but ai catalog <query>  -- Search memory (ShelfOS catalog interface)
  ├── but ai mortality         -- Show survival statistics (LRRC)
  ├── but ai mcp              -- MCP server mode
  ├── but ai agent --task <d> -- Autonomous agent mode
  └── but ai status           -- Agent state, budget, memory health
```

**Crate structure:**
```
crates/but-ai/
  src/
    lib.rs
    survival/           -- LRRC: survival distributions, fitting, surprise
      distributions.rs  -- Exponential, Weibull, Bathtub, LogNormal
      fitting.rs        -- MLE with AIC model selection
      hazard.rs         -- Hazard rate computation
      surprise.rs       -- KL divergence surprise index
    memory/             -- Integrated memory system
      entry.rs          -- Memory entry with survival + classification
      retrieval.rs      -- Hybrid retrieval (structural + survival-weighted)
      catalog.rs        -- ShelfOS: call numbers, subject headings, see-also
      tension.rs        -- Loom & Verse: contradiction tracking
      lifecycle.rs      -- Alive/moribund/deceased with cohort review
    protocol/           -- Tidal Protocol: coordination
      forge.rs          -- Forge adapter trait
      message.rs        -- PR comment schema
      consensus.rs      -- Consensus validation
    study/              -- Task execution
      protocol.rs       -- Study lifecycle
      patch.rs          -- INDEX.patch + COMMIT.msg
    identity/           -- Agent identity and signing
      agent.rs          -- Life record with performance history
      signing.rs        -- OpenWallet integration
  bin/
    main.rs
```

**WASI degradation:** In vitro mode (LRRC terminology). Survival analysis, memory management, and patch generation work fully. Cross-repo coordination disabled. This is the superset of all five proposals' WASI strategies.

**MCP compatibility:** Drop-in replacement for the existing MCP server. Backward compatible with `gitbutler_update_branches`.

---

## 2. Memory System: Survival-Classified Memory

The integrated memory system combines the LRRC's survival functions with ShelfOS's multi-dimensional classification, Loom & Verse's tension tracking, and the Tidal Protocol's consensus validation.

### 2.1 Memory Entry

```rust
pub struct MemoryEntry {
    // === Identity (common) ===
    pub id: MemoryId,
    pub content: String,
    pub created_at: String,
    pub last_accessed: String,

    // === LRRC: Survival analysis ===
    pub memory_type: MemoryType,         // Architectural, BugFix, Convention, etc.
    pub survival_distribution: SurvivalDistribution,
    pub current_survival_probability: f64,
    pub current_hazard_rate: f64,
    pub surprise_index: f64,
    pub access_history: Vec<AccessRecord>,
    pub lifecycle_state: LifecycleState,  // Alive, Moribund, Deceased
    pub goodness_of_fit: f64,

    // === ShelfOS: Classification ===
    pub call_number: CallNumber,          // Hierarchical position
    pub subject_headings: Vec<String>,    // Topical descriptors
    pub see_also: Vec<SeeAlsoLink>,       // Cross-references
    pub circulation: CirculationRecord,   // Access tracking
    pub source: SourceClassification,     // Provenance

    // === Loom & Verse: Narrative context ===
    pub motifs: Vec<MotifId>,            // Thematic anchors
    pub tensions: Vec<TensionId>,         // Active contradictions

    // === Tidal Protocol: Social validation ===
    pub consensus_citations: u64,         // Cross-agent citation count
    pub practitioner_summary: String,     // LRRC: simplified version for context injection
}
```

### 2.2 Retrieval: Two-Phase Scoring

The integrated retrieval uses **structural scoring** for all memories and **survival-weighted scoring** when the distribution fit is reliable.

**Phase 1: Structural retrieval (always active)**

```
structural_score = 0.25 * subject_match(query, entry)         -- ShelfOS
                 + 0.20 * call_number_proximity(query, entry)  -- ShelfOS
                 + 0.20 * motif_resonance(query, entry)        -- Loom & Verse
                 + 0.15 * see_also_distance(query, entry)      -- ShelfOS
                 + 0.10 * tension_urgency(entry)               -- Loom & Verse
                 + 0.10 * freshness(entry)                     -- Common
```

**Phase 2: Survival adjustment (when GoF > 0.7)**

```
final_score = structural_score * survival_weight(entry)

where survival_weight = 0.6 + 0.4 * survival_probability(entry)
```

When goodness-of-fit is below 0.7, `survival_weight = 1.0` (no adjustment). When GoF is high, survival probability modulates the structural score -- a memory with S(t) = 0.1 has its structural score multiplied by 0.64, while a memory with S(t) = 0.9 gets 0.96.

This addresses the universal finding that survival functions are unreliable for young memories but valuable for mature ones.

### 2.3 Expiration: Probabilistic with Moribund Buffer

Memories expire through the LRRC's three-state lifecycle:

- **Alive** (S(t) >= 0.25): Active in retrieval, full content in context.
- **Moribund** (0.10 <= S(t) < 0.25): Under review. Restricted to memory types with high re-learning cost (architectural, convention, dependency). Bug-fix and task-context memories skip moribund and go directly to deceased.
- **Deceased** (S(t) < 0.10): Archived with full lifecycle data. Available for distribution fitting calibration but excluded from retrieval.

**Resuscitation limit:** After 3 resuscitations, a memory is re-fitted with a heavy-tailed distribution (log-normal) rather than repeatedly resuscitated. This addresses the centenarian problem identified by Hartmann (084).

**Adaptive surprise threshold:** The surprise threshold auto-calibrates based on sample size: `threshold = 3 * expected_surprise_under_null(n, k)`. This addresses ShelfOS's concern about constant triggering in fast-moving codebases.

### 2.4 Tension Tracking (from Loom & Verse)

Contradictions are tracked as `Tension` entries with continuous urgency modeled by a Weibull hazard function (Hartmann's proposal, implemented with LRRC mathematics):

```
tension_urgency(t) = (k / lambda) * (t / lambda)^(k-1)
```

Default parameters: k=2.0, lambda=14 days. Urgency at day 7 is 0.071, at day 14 is 0.143, at day 28 is 0.286 -- a smooth gradient, not the binary flip of the original design.

Tensions act as competing risks in the survival model: the presence of an active tension multiplies the baseline hazard by a factor proportional to tension urgency, accelerating the memory's decline toward moribund.

### 2.5 Classification (from ShelfOS)

Every memory is classified by ShelfOS's five systems:
1. **Subject headings** from a controlled vocabulary
2. **Call number** hierarchy (auto-generated, refined by Cataloger)
3. **Source** provenance (task, agent, branch, tool)
4. **Temporal** metadata (created, accessed, validated)
5. **Relational** links ("see also" cross-references)

The call number hierarchy is monitored by the LRRC's surprise index. When surprise spikes for a call number subtree, a hierarchy reassessment is triggered -- addressing the architectural drift problem ShelfOS identified.

### 2.6 Motif-Based Retrieval (from Loom & Verse)

Recurring themes are detected across tasks. When a theme appears in 3+ tasks AND exceeds a frequency threshold of 5% of total tasks (addressing Hartmann's false-positive concern), it becomes a motif. Motifs serve as retrieval anchors that capture thematic resonance beyond keyword matching and embedding similarity.

### 2.7 Consensus Validation (from Tidal Protocol)

Cross-agent citation count is used as a **trust tier** (separate from retrieval scoring), following Dara's insight that trust is social but relevance is structural. A memory cited by 4/5 agents is more trusted for high-stakes decisions, but citation count does not inflate the retrieval score directly. This avoids the echo-chamber problem Dara identified.

---

## 3. Agent Protocol

The integrated agent follows the LRRC's study protocol lifecycle, enhanced with elements from all proposals:

```
1. LITERATURE REVIEW  -- Memory retrieval using hybrid structural+survival scoring
2. HYPOTHESIS         -- Approach design with uncertainty estimates (LRRC)
3. CLASSIFICATION     -- ShelfOS: classify task in the call number hierarchy
4. EXPERIMENT         -- Patch generation (common)
5. CONTINUITY CHECK   -- Loom & Verse: check for tensions with existing memories
6. PEER REVIEW        -- LRRC: validate output quality
7. PUBLICATION        -- INDEX.patch + COMMIT.msg
8. CATALOGING         -- ShelfOS: classify new work as future memory
```

**Token budget enforcement** uses the LRRC's survival-model approach: budget is a survival function with P(completion) estimated at each phase. When P(completion) < 50%, switch to minimum publishable unit. ShelfOS's mandatory cataloging reserve (1,500 tokens) is preserved -- an unclassified result is a lost result.

---

## 4. Coordination

The forge adapter trait follows the Tidal Protocol's design (12 methods, forge-agnostic). PR comments use a unified schema that includes:

- **Call number** (ShelfOS): classifies every coordination message
- **Survival metadata** (LRRC): confidence and surprise index
- **Motifs** (Loom & Verse): thematic context
- **Tide mark** (Tidal Protocol): coordination timing

```json
{
  "$schema": "but-ai/integrated/v1",
  "type": "status | dependency | handoff | budget",
  "from": { "agent": "petrov", "org": "integrated", "repo": "owner/repo" },
  "call_number": "ARCH.AUTH.MIDDLEWARE",
  "survival": { "confidence": 0.85, "surprise_index": 0.12 },
  "motifs": ["security-boundary"],
  "tide": "high",
  "timestamp": "2026-03-29T14:30:00Z"
}
```

---

## 5. Identity and Signing

Agent identity combines the LRRC's life record (with performance history) and ShelfOS's authorization model (call-number-based access control). Each agent is authorized to commit within specific call number ranges, preventing a code generator from modifying security configurations.

Key lifecycle follows the LRRC's actuarial model: usage-adjusted rotation schedules where high-use keys rotate more frequently.

---

## 6. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,200 | 0 | Once | Identity, phase-gated tools, survival summary, motif list |
| Literature review | 2,500 | 800 | Once/task | Hybrid structural+survival retrieval |
| Classification | 1,200 | 600 | Once/task | ShelfOS catalog classification |
| Hypothesis | 1,800 | 1,000 | Once/task | LRRC approach design |
| Experiment (per step) | 1,200 | 1,600 | ~3/task | Patch generation |
| Continuity check | 1,200 | 400 | Once/task | Tension detection |
| Peer review | 1,500 | 600 | Once/task | Validation |
| Commit message | 400 | 400 | Once/task | Scientific format with survival estimate |
| Memory management | 800 | 500 | Once/task | Survival fitting, moribund review |
| Post-task cataloging | 500 | 800 | Once/task | ShelfOS classification (mandatory reserve) |
| Coordination | 1,200 | 700 | 1-2/task | PR comments with unified schema |
| **TOTAL** | **19,700** | **10,600** | -- | **~30,300 tokens** |

The integrated budget (30,300 tokens) is comparable to Loom & Verse (31,900) and significantly lower than the Tidal Protocol (36,400) and Textile Morphology Lab (34,400). The savings come from ShelfOS's lean influences: phase-gated tool loading and efficient classification-based retrieval. The overhead comes from the LRRC's survival fitting and the integrated tension check -- both justified by the accuracy gains they produce.

---

## 7. What Each Proposal Contributed

| Component | Primary Source | Supporting Sources |
|-----------|---------------|-------------------|
| Survival distributions | **093 LRRC** | 083 (adopted for tension decay), 084 (adopted for arc dormancy) |
| Distribution fitting (MLE + AIC) | **093 LRRC** | 001 (acknowledged as correct replacement for fixed TTLs) |
| Surprise index | **093 LRRC** | 145 (adopted as hierarchy drift detector) |
| Moribund lifecycle | **093 LRRC** | 084 (improved with resuscitation limit) |
| Call number classification | **145 ShelfOS** | -- |
| See-also cross-references | **145 ShelfOS** | -- |
| Controlled vocabulary | **145 ShelfOS** | -- |
| Phase-gated tool loading | **145 ShelfOS** | -- |
| Mandatory cataloging reserve | **145 ShelfOS** | -- |
| Motif-based retrieval | **084 Loom & Verse** | 001 (replaced consensus in retrieval scoring) |
| Tension tracking | **084 Loom & Verse** | 093 (modeled as Weibull hazard, not step function) |
| Arc-based organization | **084 Loom & Verse** | 093 (dormancy replaced with survival function) |
| Forge adapter trait | **001 Tidal Protocol** | -- |
| CRDT-compatible access history | **001 Tidal Protocol** | 093 (access history as G-Set for shared life tables) |
| Consensus as trust tier | **001 Tidal Protocol** | 084 (motifs replaced consensus in retrieval) |
| Tide cycle for coordination timing | **001 Tidal Protocol** | -- |
| Warp/weft structural distinction | **083 Textile Morphology** | 093 (survival distributions per thread type) |
| Weave pattern as retrieval mode | **083 Textile Morphology** | 093 (surprise index as misclassification detector) |

---

## 8. Limitations

This section is longer than the capabilities section. That is by design.

1. **Cold start.** The survival-function advantage over simpler methods does not manifest until 2-3 weeks of operation. During cold start, the system operates on defaults and structural retrieval -- essentially equivalent to ShelfOS's approach. The sophistication of the integrated system is wasted during this period.

2. **Computational cost.** Survival distribution fitting (MLE with Newton-Raphson for Weibull) adds ~50-100ms per memory management cycle. For 500 memories, this is 25-50 seconds per full re-fit cycle. Re-fitting occurs every 3-5 tasks, not every task, but it is non-trivial.

3. **Sparse data fragility.** With fewer than 5 access records per memory, the fitted distributions have wide confidence intervals. The goodness-of-fit penalty mitigates this but does not eliminate it. For systems with < 50 active memories, the simpler approaches (fixed TTLs, circulation count) may perform equivalently at lower cost.

4. **Moribund overhead.** The moribund review cycle costs ~100 tokens per task. For budget-constrained environments, this may not be justified. The type-restricted moribund (only for high-value memory types) reduces this cost but does not eliminate it.

5. **Complexity.** The integrated system has more moving parts than any individual proposal. The memory entry struct carries survival metadata, classification metadata, narrative metadata, and social metadata. This is a lot of state per memory. The maintenance burden is real.

6. **No empirical validation.** All performance claims in this proposal are theoretical. No implementation has been deployed long enough to produce lifecycle data. The survival distributions are fitted to synthetic access patterns, not real agent behavior. The confidence intervals on every number in this proposal are wide.

---

*"The only honest way to manage uncertainty is to measure it. This integrated proposal measures more of the uncertainty than any individual proposal. It also introduces more uncertainty of its own. Both of these are true, and neither invalidates the other."*

-- Professor Elena Vassiliev, Principal Investigator
