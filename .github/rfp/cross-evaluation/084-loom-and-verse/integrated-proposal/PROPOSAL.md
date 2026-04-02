# Integrated Proposal: `but-ai` Plugin

**Author:** Kenji Hartmann, Coordinator/Publisher -- Loom & Verse (Org 084)
**Date:** 2026-03-29
**Based on cross-evaluation of:** Orgs 083, 084, 001, 093, 145

---

## Thesis

The five proposals each solved a different problem well:

- **Org 083 (Textile Morphology Lab)**: Best structural memory model (warp/weft separation, adaptive retrieval patterns)
- **Org 084 (Loom & Verse)**: Best thematic memory model (motifs, tensions, arc-based narrative)
- **Org 001 (Tidal Protocol)**: Best synchronization and coordination (CRDT gossip, consensus, forge adapter)
- **Org 093 (Longevity & Risk)**: Best memory lifecycle mathematics (survival functions, hazard rates, surprise index)
- **Org 145 (ShelfOS)**: Best retrieval ergonomics (card catalog, controlled vocabulary, "see also" graph, lean budget)

No single proposal is sufficient. The integrated system combines them, using each where it is strongest.

---

## 1. Memory Architecture: Three-Layer Model

The integrated memory system uses three layers, each drawn from a different proposal:

### Layer 1: Classification (from ShelfOS 145)

Every memory entry is classified using ShelfOS's five simultaneous classification systems:

```rust
pub struct Classification {
    pub subject_headings: Vec<String>,      // Topical (from 145)
    pub call_number: CallNumber,             // Hierarchical position (from 145)
    pub source: SourceClassification,        // Provenance (from 145)
    pub temporal: TemporalClassification,    // When created/accessed (from 145)
    // Note: relational ("see also") is in the graph layer below
}
```

**Controlled vocabulary** (from 145) normalizes variant terms immediately, solving the cold-start problem that plagues motif-based retrieval. The vocabulary is pre-seeded from the codebase structure and refined as memories accumulate.

**Call number hierarchy** (from 145) provides O(1) positional lookup. The hierarchy is auto-generated from the module structure.

### Layer 2: Lifecycle (from LRRC 093)

Every memory entry carries a fitted survival function:

```rust
pub struct MemoryLifecycle {
    pub survival_distribution: SurvivalDistribution,  // From 093
    pub current_survival_probability: f64,             // S(t) at current time
    pub current_hazard_rate: f64,                      // h(t) at current time
    pub surprise_index: f64,                           // KL divergence (from 093)
    pub lifecycle_state: LifecycleState,               // alive/moribund/deceased (from 093)
    pub resuscitation_count: u32,                      // Bounded at 3 (from Q&A)
}
```

The `SurvivalFunction` trait from Org 093's `survival/distributions.rs` is imported directly -- it is the most mathematically rigorous component across all five proposals. The four distribution families (Exponential, Weibull, Bathtub, LogNormal) are used as-is.

**Memory types map to distributions** (from 093):
- Architectural -> Weibull(k=1.8, lambda=180d)
- Bug/fix -> Exponential(lambda=3d)
- Convention -> Bathtub
- Cross-repo -> LogNormal(mu=3.5, sigma=1.2)

**Moribund state** (from 093) with a resuscitation limit of 3 (from Q&A). After 3 resuscitations, the distribution is re-fitted with a heavy-tailed family.

### Layer 3: Thematic (from Loom & Verse 084)

Memories are connected through motifs and tensions:

```rust
pub struct ThematicLayer {
    pub motifs: Vec<MotifId>,                // From 084
    pub tensions_introduced: Vec<TensionId>, // From 084
    pub tensions_resolved: Vec<TensionId>,   // From 084
    pub arc: ArcId,                          // From 084
    pub see_also: Vec<SeeAlsoLink>,          // From 145
}
```

**Motif emergence** uses a frequency-based threshold (from Q&A with 093):
```
promoted = appearances >= 3 AND (appearances / total_entries) >= 0.05
```

**Proto-motifs** (from Q&A): themes with 1-2 appearances contribute reduced resonance weight (0.3x).

**Tension lifecycle** uses continuous Weibull hazard (from Q&A with 093):
```
tension_urgency(t) = (k/lambda) * (t/lambda)^(k-1)
```
with k=2.0 and lambda=14 days, replacing the step-function escalation.

**"See also" graph** (from 145) provides explicit relational links between memories, complementing the implicit motif-based connections. The graph has a max of 5 links per entry and supports 2-hop traversal during retrieval.

---

## 2. Retrieval: Hybrid Scoring

The integrated relevance score combines signals from all five proposals:

```rust
pub struct IntegratedRelevanceScore {
    // From 145: structural matching
    pub subject_match: f64,           // 0.20 weight
    pub call_number_proximity: f64,   // 0.10 weight
    pub see_also_distance: f64,       // 0.10 weight

    // From 084: thematic resonance
    pub motif_resonance: f64,         // 0.20 weight
    pub tension_urgency: f64,         // 0.05 weight

    // From 093: survival-based
    pub survival_probability: f64,    // 0.15 weight
    pub hazard_adjusted_recency: f64, // 0.10 weight

    // From 001: social validation
    pub consensus_citations: f64,     // 0.05 weight

    // From 083: access pattern
    pub circulation_frequency: f64,   // 0.05 weight
}
```

**Total weights = 1.00.** The scoring prioritizes structural matching (0.40) over thematic resonance (0.25) over lifecycle signals (0.25) over social signals (0.10).

**Cold-start strategy:** For memories younger than their median survival time (when the fitted distribution is unreliable), the survival_probability weight is redistributed to motif_resonance and subject_match. This addresses both 084's motif cold-start and 093's sparse-data fitting problem.

---

## 3. Synchronization: CRDT Gossip (from Tidal Protocol 001)

The `GossipEngine` from Org 001 is the synchronization backbone. The vector clock + pull-based gossip protocol ensures eventual consistency without a central coordinator.

```rust
pub struct IntegratedManifestStore {
    agent: AgentId,
    entries: HashMap<EntryId, IntegratedMemoryEntry>,
    gossip: GossipEngine,  // From 001
    catalog: CardCatalog,  // From 145
    survival: SurvivalManager,  // From 093
    motifs: MotifTracker,  // From 084
    tensions: TensionRegistry,  // From 084
}
```

**CRDT merge strategy for integrated entries** (from Q&A between 083 and 001):
- `survival_distribution`: latest-fitted-wins (keyed by `fitted_at` timestamp)
- `classification`: merge subject headings as a G-Set (union)
- `see_also`: merge as OR-Set (union with removal markers)
- `motifs`: merge as G-Set (motifs only grow)
- `tensions`: merge by state precedence (Resolved > Escalated > Active)
- `content`: last-writer-wins keyed by vector clock

**Tide cycle** (from 001) is retained for gossip scheduling: gossip runs during Low tide phase, not during High tide execution.

---

## 4. Coordination: Forge Adapter (from Tidal Protocol 001)

The `ForgeAdapter` trait from Org 001 has the richest method set (12 methods) and cleanest forge-agnostic design:

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn comment(&self, pr: &PrRef, body: &str) -> Result<CommentRef>;
    fn list_comments(&self, pr: &PrRef) -> Result<Vec<Comment>>;
    fn add_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn remove_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn list_labels(&self, pr: &PrRef) -> Result<Vec<String>>;
    fn get_pr_body(&self, pr: &PrRef) -> Result<String>;
    fn pr_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn list_prs(&self, repo: &RepoRef, labels: &[&str]) -> Result<Vec<PrRef>>;
    fn cross_reference(&self, from: &PrRef, to: &PrRef) -> Result<()>;
    fn resolve_ref(&self, reference: &str) -> Result<PrRef>;
    fn forge_type(&self) -> ForgeType;
}
```

**PR comment schema:** The integrated schema combines ShelfOS's call_number classification with Org 001's structured headers:

```markdown
<!-- but-ai:message -->
<!-- type: task | status | dependency | handoff | budget -->
<!-- from: agent-id -->
<!-- call-number: ARCH.AUTH.MIDDLEWARE -->
<!-- timestamp: ISO-8601 -->

## [STATUS] Authentication middleware refactor

**Call Number:** ARCH.AUTH.MIDDLEWARE.TOKEN-VALIDATION
**Subject Headings:** authentication, jwt, middleware
**Motifs:** security-boundary, session-lifecycle
**Tensions:** timeout-vs-longrun (active, h(t)=0.08)
```

The call number in the comment enables classified cross-repo search (from 145). The motifs enable thematic cross-repo continuity (from 084). The tension hazard rate enables urgency-aware coordination (from 093).

---

## 5. Agent Roles: 4-Agent Workshop

The integrated team draws from three organizational models:

| Agent | Primary Role | Origin | Modules Owned |
|-------|-------------|--------|---------------|
| **Architect** | Context establishment, study design | 083 (Tanaka), 093 (Vassiliev) | Classification, survival fitting |
| **Author** | Patch generation, code writing | 083 (Marchetti), 084 (Orozco) | Patch generation, draft iteration |
| **Reviewer** | Validation, contradiction detection | 084 (Sato), 093 (Okonkwo), 083 (Lindqvist) | Tension registry, selvedge checks |
| **Coordinator** | Cross-repo, memory management | 084 (Hartmann), 001 (Dara), 145 (Circ) | Gossip, forge adapter, catalog updates |

**Why 4, not 3 or 5:** ShelfOS's 3-agent model is the most token-efficient but lacks dedicated validation (identified in Q&A). The 5-agent models (083, 093) add coordination overhead that exceeds the benefit for typical tasks. 4 agents provide validation without redundant coordination roles.

**Validation is not optional.** The cross-evaluation Q&A revealed that every proposal with a dedicated validator (083, 084, 093) produces higher-quality output descriptions. ShelfOS's absence of validation was the most consistently questioned design choice.

---

## 6. Token Budget

| Component | Input Tokens | Output Tokens | Notes |
|-----------|-------------|--------------|-------|
| System prompt | 2,800 | 0 | Phase-gated tools (from 145), ready reference (from 145) |
| Classification (Architect) | 1,500 | 800 | Catalog lookup + survival check |
| Patch generation (Author) | 3,500 | 4,500 | 2 passes (from 083's iterative model) |
| Tool calls (6 per task) | 3,000 | 900 | From 001's 8-call average, reduced by better classification |
| Validation (Reviewer) | 1,500 | 600 | Tension + selvedge check |
| Memory management | 800 | 600 | Survival re-fit + catalog update + motif recording |
| Coordination | 1,000 | 700 | Gossip + forge comments |
| Commit message | 400 | 500 | Including survival estimate + motifs + call number |
| **TOTAL** | **14,500** | **8,600** | **~23,100 tokens** |

**Phase-gated tool loading** (from 145) saves ~1,000 tokens. **Mandatory catalog reserve** (from 145) ensures post-task classification is never skipped. **Survival-filtered retrieval** (from 093) reduces wasted context by excluding moribund memories.

The 23,100-token budget is below ShelfOS's 26,000 because the integrated classification reduces redundant tool calls (the "132 redundant `but status` calls" problem cited in ShelfOS's proposal).

---

## 7. Expiration: Hybrid Model

The integrated expiration model combines three mechanisms:

1. **Per-entry survival function** (from 093): S(t) < 0.10 triggers deceased state.
2. **Arc-level dormancy** (from 084): fitted per arc (not fixed 30-day threshold -- improvement from Q&A with 093). Arcs with active tensions cannot go dormant (improvement from Q&A with 083).
3. **Deaccession review** (from 145): moribund entries reviewed before final expiration. Deceased entries archived (not deleted) as training data for improving future survival fits (from 093).

---

## 8. Compaction Survival

The compaction strategy merges approaches:

1. **Full retention** (S(t) > 0.75 AND circulation > 5): full entry in context.
2. **Summary retention** (0.25 < S(t) <= 0.75 OR circulation 1-5): call number + subject headings + practitioner summary (from 093's Okonkwo).
3. **Skeleton retention** (S(t) <= 0.25 OR uncirculated): call number only (from 145). Motifs and active tensions always preserved regardless of tier (from 084).

---

## 9. Crate Structure

```
crates/but-ai/src/
    lib.rs
    types.rs                    -- Unified types from all 5 proposals

    memory/
        mod.rs                  -- IntegratedManifestStore
        classification.rs       -- From 145: five classification systems
        call_number.rs          -- From 145: hierarchical call numbers
        controlled_vocab.rs     -- From 145: term normalization
        see_also.rs             -- From 145: cross-reference graph

    survival/
        mod.rs                  -- From 093: survival analysis core
        distributions.rs        -- From 093: Exponential, Weibull, Bathtub, LogNormal
        fitting.rs              -- From 093: MLE with AIC model selection
        hazard.rs               -- From 093: hazard classification
        surprise.rs             -- From 093: KL divergence surprise index

    narrative/
        mod.rs                  -- From 084: motifs and tensions
        motif.rs                -- From 084: motif emergence with frequency threshold
        tension.rs              -- From 084: tension lifecycle with Weibull urgency
        arc.rs                  -- From 084: arc management with survival-based dormancy

    sync/
        mod.rs                  -- From 001: CRDT synchronization
        gossip.rs               -- From 001: vector clock gossip protocol
        consensus.rs            -- From 001: quorum-based consensus
        tide.rs                 -- From 001: tidal cycle clock

    coordination/
        mod.rs                  -- Forge adapter and PR coordination
        forge.rs                -- From 001: ForgeAdapter trait (12 methods)
        message.rs              -- Integrated PR comment schema
        loan.rs                 -- From 145: interlibrary loan protocol

    agent/
        mod.rs                  -- 4-agent workshop
        architect.rs            -- Context + classification + survival
        author.rs               -- Patch generation (iterative)
        reviewer.rs             -- Validation + tension detection
        coordinator.rs          -- Cross-repo + memory management

    budget/
        mod.rs                  -- Token budget with mandatory reserves
        tracker.rs              -- From 001: per-call tracking
        circulation_budget.rs   -- From 145: catalog + circulation reserves

    identity/
        mod.rs                  -- Agent identity and signing
        signing.rs              -- From 001: OpenWallet integration
```

---

## 10. What Each Proposal Contributes

| Component | Primary Source | Supporting Sources |
|-----------|---------------|-------------------|
| Memory classification | **145 (ShelfOS)** | 084 (arc grouping) |
| Survival lifecycle | **093 (LRRC)** | 084 (arc dormancy triggers) |
| Thematic retrieval | **084 (Loom & Verse)** | 145 (see-also graph) |
| Synchronization | **001 (Tidal Protocol)** | 083 (warp CRDT merge strategy) |
| Retrieval scoring | **Hybrid** | All 5 contribute dimensions |
| Forge coordination | **001 (Tidal Protocol)** | 145 (call-number classified messages) |
| Token efficiency | **145 (ShelfOS)** | 083 (phase-gated tools) |
| Validation | **084 (Loom & Verse)** | 093 (practitioner review) |
| Structural memory | **083 (Textile Morphology)** | 001 (CRDT merge for threads) |
| Agent identity | **001 (Tidal Protocol)** | 093 (performance history) |

---

## 11. Key Improvements from Cross-Evaluation Q&A

These improvements were identified through the Q&A process and are not present in any single original proposal:

1. **Proto-motifs**: Themes with 1-2 appearances contribute reduced resonance (identified by 083, 001, 145 questioning 084's cold-start).
2. **Frequency-based motif threshold**: `appearances/total >= 0.05` in addition to absolute count >= 3 (identified by 093 questioning 084's statistical significance).
3. **Continuous tension urgency**: Weibull hazard function replacing step-function escalation (identified by 093 questioning 084's binary escalation).
4. **Arc dormancy gated by tensions**: Arcs with active tensions cannot go dormant (identified by 083 questioning 084's dormancy logic).
5. **Orphaned motif decay**: `0.95^months_since_last_active_arc` decay for motifs without active arcs (identified by 001 questioning 084's motif pollution).
6. **Survival-fitted arc dormancy**: Per-arc Weibull dormancy instead of fixed 30-day threshold (identified by 093 questioning 084's uniform TTL).
7. **Resuscitation limit**: Max 3 resuscitations before re-fitting with heavy-tailed distribution (identified by 084 questioning 093's moribund queue).
8. **Tension as competing risk**: Contradictions act as covariates in survival model, multiplying baseline hazard (identified by 084 questioning 093's lack of contradiction handling; answered by 093 proposing Cox proportional hazards).

---

*"The narrative must survive the machinery."*
*The machinery must be mathematically sound.*
*The mathematics must be practically retrievable.*
*The retrieval must be synchronized.*
*The synchronization must be classified.*

*Five principles. Five proposals. One plugin.*

-- Kenji Hartmann, 2026-03-29
