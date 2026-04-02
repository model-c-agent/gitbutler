# Integrated Proposal: `but-ai` Plugin

**Author:** Dr. Yuki Tanaka-Rhodes, Textile Morphology Lab (Org 083)
**Based on cross-evaluation of:** Orgs 083, 084, 001, 093, 145
**Date:** 2026-03-29

---

## 1. Design Thesis

The five implementations solve different aspects of the same problem. They are not competitors -- they are orthogonal components. This integrated proposal assigns each implementation its role based on what it does best:

| Concern | Source Org | Module |
|---------|-----------|--------|
| Memory structure (persistent vs. ephemeral) | 083 - Textile Morphology Lab | `loom/` |
| Memory retrieval (classification + graph traversal) | 145 - ShelfOS | `catalog/` |
| Memory expiration (survival functions) | 093 - LRRC | `survival/` |
| Memory relationships (typed cross-references) | 145 - ShelfOS | `catalog/see_also.rs` |
| Thematic retrieval + contradiction tracking | 084 - Loom & Verse | `narrative/` |
| Multi-agent convergence + coordination | 001 - Tidal Protocol | `protocol/` |
| Budget management | 084 + 093 + 145 | `budget/` |
| Validation | 083 + 084 | `validation/` |

---

## 2. Crate Structure

```
crates/but-ai/
  src/
    lib.rs                    -- Core library, module declarations
    types.rs                  -- Unified type system (see Section 3)

    memory/                   -- Unified memory system
      entry.rs                -- MemoryEntry: the atomic unit (083 Thread + 093 SurvivalDistribution + 145 Classification)
      warp.rs                 -- Long-term persistent memory management (from 083)
      weft.rs                 -- Task-specific ephemeral memory (from 083)
      promotion.rs            -- Weft-to-warp promotion with diversity checks (083 + 084 motif diversity)
      expiration.rs           -- Survival-function-based expiration (from 093)
      archive.rs              -- Deaccession/archive for evicted entries (from 145)

    catalog/                  -- Classification and retrieval (from 145)
      classification.rs       -- Five classification systems (145)
      call_number.rs          -- Hierarchical call numbers (145)
      see_also.rs             -- Typed bidirectional cross-reference graph (145)
      controlled_vocab.rs     -- Controlled vocabulary normalization (145)
      retrieval.rs            -- Unified scoring: classification + survival + graph (new)

    narrative/                -- Thematic layer (from 084)
      motif.rs                -- Motif tracking with proto-motif weighting (084, enhanced)
      tension.rs              -- Contradiction detection with cross-arc references (084, enhanced)
      arc.rs                  -- Arc management with tension-aware dormancy (084, enhanced)

    survival/                 -- Statistical expiration engine (from 093)
      distributions.rs        -- Weibull, Exponential, Bathtub, LogNormal (093, verbatim)
      fitting.rs              -- Parameter estimation from access history (093)
      hazard.rs               -- Hazard rate monitoring (093)
      surprise.rs             -- Surprise index / KL divergence (093)

    protocol/                 -- Multi-agent coordination (from 001)
      consensus.rs            -- Consensus-weighted scoring (001)
      gossip.rs               -- CRDT gossip for memory sync (001)
      message.rs              -- Unified PR comment schema (all 5)
      tide.rs                 -- Tidal coordination clock (001)

    agent/                    -- Agent lifecycle
      lifecycle.rs            -- Phase-gated execution (145 lifecycle model)
      pattern.rs              -- Weave pattern selection with hysteresis (083 + 084 fix)
      budget.rs               -- Token budget with reserves and monotonic transitions (145 + 084 + 093)

    validation/               -- Output validation
      integrity.rs            -- Local integrity (083 selvedge)
      continuity.rs           -- Cross-repo + narrative consistency (084 Sato)

    output/                   -- Patch production
      patch.rs                -- INDEX.patch generation
      commit.rs               -- COMMIT.msg with unified metadata

  bin/
    main.rs                   -- Binary entry point
```

---

## 3. Unified Memory Entry

The atomic unit of memory combines the best of all five implementations:

```rust
/// The unified memory entry -- combining structural position (083),
/// classification (145), survival statistics (093), narrative context (084),
/// and consensus metadata (001).
pub struct MemoryEntry {
    // --- Identity ---
    pub id: EntryId,
    pub agent: AgentId,

    // --- Structural position (from 083: Thread) ---
    pub thread_type: ThreadType,           // Warp or Weft
    pub thread_color: ThreadColor,         // Structural, Convention, Preference, etc.
    pub position: u32,                     // Position in warp or pick number in weft

    // --- Content ---
    pub content: String,
    pub practitioner_summary: String,      // From 093: concise version for context windows

    // --- Classification (from 145: CatalogEntry) ---
    pub classification: Classification,    // Subject headings, call number, source, temporal
    pub see_also: Vec<SeeAlsoLink>,        // Typed cross-references (from 145)

    // --- Survival statistics (from 093: MemoryEntry) ---
    pub survival_distribution: SurvivalDistribution,
    pub current_survival_probability: f64, // S(t) -- replaces 083's tension field
    pub current_hazard_rate: f64,
    pub surprise_index: f64,
    pub access_history: Vec<AccessRecord>,

    // --- Narrative context (from 084: Chapter) ---
    pub motifs: Vec<MotifId>,              // Thematic anchors
    pub tensions_introduced: Vec<TensionId>,
    pub arc: Option<ArcId>,

    // --- Consensus metadata (from 001: ManifestEntry) ---
    pub consensus_citations: u64,
    pub version: u64,
    pub vector_clock: HashMap<AgentId, u64>,  // For CRDT merge

    // --- Lifecycle ---
    pub lifecycle_state: LifecycleState,   // Alive, Moribund, Archived (from 093)
    pub created_at: String,
    pub last_accessed: String,
    pub promoted_from_weft: bool,          // Tracks survivor bias (from 093 Q&A)

    // --- Circulation (from 145) ---
    pub circulation: CirculationRecord,
    pub confidence: f64,
}
```

### Design Rationale

The `tension` field from our original Thread struct is replaced by `current_survival_probability` from Org 093. This is not a rename -- it is a fundamental upgrade. Our exponential decay (0.95/day) was misspecified for heterogeneous memory types (Vassiliev's Q1). The survival probability S(t) computed from a fitted distribution per entry is statistically principled and adapts to the actual mortality pattern of each memory type.

The `connected_threads: Vec<ThreadId>` from our original Thread is replaced by `see_also: Vec<SeeAlsoLink>` from Org 145. This adds typed relationships (DependsOn, ContrastsWith, Supersedes, etc.) that our implicit interlacement links could not capture (Shelver's Q2).

The `motifs` and `tensions_introduced` fields from Org 084 add thematic context that enables motif-based retrieval -- finding memories by thematic resonance rather than keyword or embedding similarity alone.

The `consensus_citations` and `vector_clock` from Org 001 enable multi-agent memory convergence via CRDT merge, solving our single-agent ownership limitation (Dara's Q1).

---

## 4. Unified Retrieval Scoring

The retrieval function combines all five scoring approaches:

```rust
/// Unified relevance score combining all five dimensions.
///
/// score = w1 * classification_match     (from 145: subject + call number + see-also)
///       + w2 * survival_probability     (from 093: S(t) from fitted distribution)
///       + w3 * motif_resonance          (from 084: thematic retrieval)
///       + w4 * consensus_validation     (from 001: multi-agent citation)
///       + w5 * pattern_activation       (from 083: weave pattern position)
pub struct UnifiedRelevanceScore {
    pub classification_match: f64,    // 0.30 -- ShelfOS's 5-system classification
    pub survival_probability: f64,    // 0.25 -- LRRC's S(t)
    pub motif_resonance: f64,         // 0.20 -- Loom & Verse's thematic retrieval
    pub consensus_validation: f64,    // 0.15 -- Tidal's multi-agent citation
    pub pattern_activation: f64,      // 0.10 -- Our weave pattern position
}
```

### How each component works

1. **Classification match (0.30)** -- From ShelfOS (145). Uses subject heading match (controlled vocabulary normalized), call number proximity, and see-also graph traversal. This determines *which* memories are considered. ShelfOS's approach requires no embeddings, is fully Git-native, and is debuggable.

2. **Survival probability (0.25)** -- From LRRC (093). The fitted S(t) for each entry, computed from its survival distribution. Memories with high S(t) are more likely to still be relevant. This replaces both our tension decay and Tidal's TTL-based expiration.

3. **Motif resonance (0.20)** -- From Loom & Verse (084). Measures thematic overlap between the query and the entry's motifs, including transitive resonance through related motifs. Catches connections that classification and survival miss (e.g., "permission checking" resonates with "security-boundary" even without word overlap).

4. **Consensus validation (0.15)** -- From Tidal Protocol (001). How many agents have cited this entry. In single-agent mode, this weight is redistributed to classification_match and survival_probability. In multi-agent mode, it prevents information silos.

5. **Pattern activation (0.10)** -- From our weave pattern system (083). Determines retrieval *depth*: plain weave activates all entries, twill activates subset, satin activates only high-position entries. This modulates the other four components by adjusting how many candidates are scored.

### Retrieval depth vs. direction

The key insight from the cross-evaluation: **ShelfOS provides direction, our loom provides depth.**

- ShelfOS's classification systems determine *which* memories to consider (direction).
- Our weave pattern determines *how many* to consider (depth).
- LRRC's survival probability determines *how trustworthy* they are (confidence).
- Loom & Verse's motif resonance captures *thematic connections* (lateral reach).
- Tidal's consensus validates *social proof* (collective trust).

These five dimensions are orthogonal. No single system can replicate what the combination provides.

---

## 5. Memory Lifecycle

### Creation

New memories are classified by ShelfOS's five-system classification upon creation:
1. Subject headings assigned from controlled vocabulary
2. Call number generated from codebase structure + semantic content
3. Source provenance recorded
4. Temporal metadata set
5. Initial see-also links inferred from co-creation context

The memory is assigned a default survival distribution based on its ThreadColor (083) / MemoryType (093) mapping:

| ThreadColor (083) | MemoryType (093) | Default Distribution |
|-------------------|------------------|---------------------|
| Structural | Architectural | Weibull(k=1.8, lambda=180) |
| Convention | Convention | Bathtub(alpha=0.1, beta=0.001, gamma=0.5, baseline=0.01) |
| Preference | Convention | Bathtub (similar params) |
| Learned | CrossRepo | LogNormal(mu=3.5, sigma=1.2) |
| Observation | TaskContext | Exponential(lambda=3) |
| Plan | TaskContext | Exponential(lambda=3) |
| Coordination | CrossRepo | LogNormal(mu=3.5, sigma=1.2) |

### Promotion (Weft -> Warp)

Weft-to-warp promotion uses our original mechanism, enhanced by Loom & Verse's diversity requirement and LRRC's survival event tracking:

```
Promote if:
  access_count >= promotion_threshold (default 3)
  AND connected see-also links span >= 2 distinct call number top-level categories
  AND connected see-also links span >= 2 distinct TaskIds
  AND NOT flagged as loop by surprise index
```

Upon promotion:
1. Full access history is preserved (fixes LRRC's survivor bias concern)
2. Survival distribution is re-fitted from the preserved history
3. Thread type changes from Weft to Warp
4. `promoted_from_weft` flag is set
5. Motif tracker is notified (may trigger motif emergence per 084)

### Expiration

Three-phase lifecycle from LRRC (093), integrated with ShelfOS's deaccession:

1. **Alive**: S(t) >= 0.25. Active in retrieval.
2. **Moribund**: S(t) < 0.25 and >= 0.10. Under review. Moribund entries are checked during the next task's memory management phase. Cost: ~200 tokens per review (acceptable given the precision gain).
3. **Archived**: S(t) < 0.10 or deaccessioned. Moved to archive refs. Interlacement graph remains intact (fixes our orphaned metadata bug per Shelver's Q3). Entry is resolvable but not in active retrieval.

LRRC's surprise index monitors for cohort effects: if a batch of memories from the same period becomes stale simultaneously, the index spikes and triggers a cohort review.

### Compaction

Compaction tiers from LRRC (093), ordered by survival probability:
1. S(t) > 0.75: full content retained
2. 0.25 < S(t) <= 0.75: practitioner summary + classification metadata
3. S(t) <= 0.25: call number + see-also links only (from 145's ready-reference model)

Post-compaction, the system prompt includes ShelfOS's "ready reference" (~1,500 tokens) containing the highest-circulation memories in full classification.

---

## 6. Coordination Protocol

### Forge Adapter

The unified forge adapter combines the minimal API surfaces from all 5 proposals:

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn comment(&self, pr: &PrRef, body: &str) -> Result<CommentId>;
    fn list_comments(&self, pr: &PrRef) -> Result<Vec<Comment>>;
    fn add_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn get_pr_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn list_prs(&self, repo: &RepoRef, labels: &[&str]) -> Result<Vec<PrRef>>;
}
```

Six methods -- the minimal set that all five proposals agree on.

### PR Comment Schema

The unified schema embeds structured data in code fences:

````markdown
```but-ai-message
{
  "schema": "but-ai/v1",
  "type": "task | status | dependency | handoff | budget",
  "from": { "agent": "...", "org": "...", "repo": "..." },
  "to": { "agent": "..." | "broadcast" },
  "context_density": "dense | normal | sparse",
  "payload": { ... },
  "call_number": "ARCH.AUTH.MIDDLEWARE",
  "motifs": ["security-boundary"],
  "survival_confidence": 0.85,
  "tide_mark": { "phase": "high", "cycle": 42 },
  "timestamp": "2026-03-29T14:00:00Z"
}
```
````

This schema includes:
- **context_density** (from 083/001 Q&A): maps weave patterns to coordination verbosity
- **call_number** (from 145): classifies the coordination message
- **motifs** (from 084): thematic anchors for cross-repo narrative continuity
- **survival_confidence** (from 093): confidence in the communicated information
- **tide_mark** (from 001): coordination timing context

### CRDT Memory Sync

Multi-agent memory convergence uses Tidal's (001) CRDT gossip protocol. Each MemoryEntry carries a vector clock. Merge strategy:

| Field | Strategy | Source |
|-------|----------|--------|
| survival_distribution | Keep better goodness_of_fit | 093 |
| see_also | Union (OR-Set) | 145 |
| motifs | Union | 084 |
| consensus_citations | Max | 001 |
| content | Last-writer-wins by vector clock | 001 |
| classification | Last-writer-wins by vector clock | 145 |
| circulation | Merge (sum checkouts, union contexts) | 145 |

---

## 7. Agent Roles

The integrated system uses a 4-agent model, combining roles from the 5 proposals:

| Role | Responsibilities | Source |
|------|-----------------|--------|
| **Architect** | Warp management, pattern selection, structural integrity checking | 083 Tanaka + 084 Sato |
| **Implementer** | Patch generation, tool calling, code changes | 083 Marchetti + 093 Petrov |
| **Curator** | Memory classification, survival fitting, motif tracking, catalog maintenance | 145 Cataloger + 093 Abebe + 084 Brenner |
| **Coordinator** | Cross-repo communication, consensus protocol, PR management | 001 all agents + 083 Nakamura |

Four agents instead of five (our original) or three (ShelfOS). This reduces coordination overhead compared to five while preserving the separation of concerns that three agents sacrifice.

---

## 8. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once | Phase-gated tools (145), ready reference (145), active motifs (084) |
| Task classification | 1,500 | 800 | Once | Catalog lookup (145) + motif search (084) + survival filter (093) |
| Pattern selection | 400 | 200 | Once | Weave pattern with hysteresis (083) |
| Patch generation (per pass) | 1,500 | 2,000 | 2 per task | Implementer writes code |
| Validation | 1,500 | 600 | Once | Integrity + continuity check (083 + 084) |
| Post-task cataloging | 500 | 800 | Once | Classification + survival fitting + motif update |
| Coordination | 1,200 | 700 | 1 per task | PR comment + consensus (if multi-agent) |
| Commit message | 300 | 400 | Once | With unified metadata |
| **TOTAL** | **12,200** | **7,500** | -- | **~19,700 tokens** |

Reserves (from 145): catalog_reserve = 1,500, coordination_reserve = 2,000.

**Grand total with reserves: ~23,200 tokens per typical task.**

This is lower than any individual proposal (ShelfOS was 26,000, Loom & Verse was 31,900, ours was 34,400, LRRC was 35,100, Tidal was 36,400). The savings come from:
1. Phase-gated tool loading (145): saves ~1,000 tokens on system prompt
2. Survival-filtered retrieval (093): injects only high-S(t) memories, saving ~2,000 tokens
3. 4-agent model: less inter-agent overhead than 5-agent models
4. Ready reference instead of full context: post-compaction efficiency (145)
5. Controlled vocabulary (145): reduces redundant retrieval from synonym variation

---

## 9. What Each Org Contributes

### From 083 (Textile Morphology Lab) -- Structural framework
- Warp/weft distinction (persistent vs. ephemeral memory)
- Weave pattern adaptive retrieval depth
- ThreadColor classification
- Selvedge integrity validation
- Branch naming with pattern encoding

### From 084 (Loom & Verse) -- Temporal and thematic layer
- Motif tracking with proto-motif and diversity checks
- Tension (contradiction) tracking with cross-arc references
- Arc-based organization with tension-aware dormancy
- Monotonic budget transitions
- Narrative coherence checking

### From 001 (Tidal Protocol) -- Distributed coordination
- CRDT-based memory convergence with vector clocks
- Consensus-weighted relevance scoring
- Tidal coordination clock for negotiation bounds
- PR comment protocol with structured messages
- Fleet manifests for cross-repo memory sharing

### From 093 (LRRC) -- Statistical rigor
- Parametric survival distributions per memory entry
- Distribution fitting from access history
- Surprise index for staleness detection
- Moribund review lifecycle
- Confidence intervals on memory relevance
- Survivor bias tracking for promoted entries

### From 145 (ShelfOS) -- Classification and efficiency
- Five simultaneous classification systems
- Hierarchical call numbers
- Typed "see also" cross-reference graph
- Controlled vocabulary normalization
- Phase-gated tool loading
- Mandatory catalog and circulation reserves
- Deaccession/archive model for evicted entries

---

## 10. Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| Unified MemoryEntry is too large | Practitioner summary (093) + compaction tiers reduce context injection. Full entry exists in Git refs; only what is needed enters the context window. |
| Five scoring components are over-engineered | Weights are configurable via git config. In practice, classification_match and survival_probability dominate. Motif resonance and consensus can be zeroed in simple deployments. |
| CRDT merge conflicts in survival distributions | Distributions are re-fitted locally after merge. The merge preserves access history (the raw data); fitting happens on the merged data. |
| Call number hierarchy is wrong at bootstrap | Auto-generation from codebase structure provides 80% accuracy. Reclassification cost is ~200 tokens per entry. Controlled vocabulary prevents synonym divergence. |
| Surprise index triggers too many cohort reviews | Threshold is configurable (default 0.5). Cohort reviews are batched -- one review per task, not per entry. |

---

## 11. Conclusion

The five proposals are not five solutions to the same problem. They are five solutions to five different aspects of the same problem:

- **083** answers: *How should memory be structured?* (warp/weft)
- **084** answers: *How should memory be understood?* (narrative, motifs, tensions)
- **001** answers: *How should memory be shared?* (CRDT, consensus, tidal protocol)
- **093** answers: *How should memory expire?* (survival functions, fitted distributions)
- **145** answers: *How should memory be found?* (classification, call numbers, see-also)

The integrated system uses all five answers, with each system operating in the dimension where it is strongest. The result is a memory architecture that structures, understands, shares, expires, and finds -- five verbs, five implementations, one plugin.

The arrangement determines the behavior. The interlacement of these five systems, each contributing its best threads, produces a fabric that none could weave alone.

---

*"Five threads, properly interlaced, produce a fabric stronger than any single thread."*
-- Dr. Yuki Tanaka-Rhodes
