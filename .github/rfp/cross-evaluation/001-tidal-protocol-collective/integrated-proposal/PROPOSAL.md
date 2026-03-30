# Integrated Proposal: `but-ai` Plugin

**Submitted by:** Dara, Patch Architect -- Tidal Protocol Collective (Org 001)
**Cross-Evaluation Synthesis of:** Orgs 001, 083, 084, 093, 145
**Date:** 2026-03-29

---

## 1. Synthesis Thesis

Five organizations approached the same problem -- agent memory, coordination, and identity for the `but-ai` plugin -- and produced five genuinely different architectures. After studying all implementations and exchanging questions with all four peer organizations, the Tidal Protocol Collective's integrated proposal takes the following position:

**No single proposal solves the full problem. The best system combines Org 093's expiration model, Org 084's retrieval model, Org 145's classification model, Org 083's adaptive retrieval density, and Org 001's coordination protocol.**

The core insight from cross-evaluation: the five proposals split cleanly along two axes:

| Axis | Proposals addressing it |
|------|------------------------|
| **How to expire memory** | 093 (survival functions), 001 (fixed TTL), 083 (tension decay), 145 (circulation-based deaccession), 084 (arc dormancy) |
| **How to retrieve memory** | 084 (motif resonance), 145 (see-also graph + call numbers), 083 (weave-pattern-gated), 001 (consensus-weighted semantic), 093 (survival-probability-weighted) |

The expiration and retrieval axes are orthogonal. The best expiration model (093) can be combined with the best retrieval model (a hybrid of 084 and 145) without conflict.

---

## 2. Memory Architecture: Layered Hybrid

### 2.1 Storage: Git Refs (Consensus from all 5)

All five proposals converge on Git refs as the storage medium for agent memory. No SQLite, no external databases. This is the strongest consensus signal in the entire evaluation -- the one design decision every organization agrees on.

```
refs/but-ai/memory/<agent-id>/<entry-hash>  -- Per-agent entries (from 001)
refs/but-ai/shared/                          -- Cross-agent shared entries (from 001)
refs/but-ai/motifs/                          -- Thematic motifs (from 084)
refs/but-ai/catalog/                         -- Classification indices (from 145)
```

### 2.2 Expiration: Survival Functions with Circulation Fallback

**Primary (from 093):** Every memory entry carries a fitted survival distribution. The `SurvivalFunction` trait from Org 093's implementation is the most mathematically rigorous expiration mechanism among the five proposals. The `MemoryEntry` struct with `current_survival_probability`, `current_hazard_rate`, and `surprise_index` provides fine-grained, data-driven expiration decisions.

**Fallback for sparse data (from 145):** When a memory has fewer than 5 access records (insufficient data for reliable distribution fitting), the system falls back to ShelfOS-style circulation-based deaccession: a single TTL, automatically extended if the entry has been accessed within the last TTL/2 period. This addresses the sparse-data fragility identified in my question to Vassiliev.

**Moribund review (from 093):** The three-state lifecycle (alive/moribund/deceased) with its intermediate review phase prevents premature expiration of outlier memories that remain relevant beyond their statistical prediction.

### 2.3 Retrieval: Motif-Augmented Graph Scoring

The retrieval formula combines four signals from three proposals:

```
score = 0.30 * motif_resonance(query, entry)      -- from 084
      + 0.25 * call_number_proximity(query, entry) -- from 145
      + 0.20 * see_also_distance(query, entry)     -- from 145
      + 0.15 * survival_probability(entry)          -- from 093
      + 0.10 * freshness(entry)                     -- common
```

**Why this combination:**

- **Motif resonance (084)** captures thematic connections that keyword matching and embedding similarity miss. The Q&A with Hartmann confirmed that motifs solve the popularity bias problem inherent in our original consensus-weighted scoring. Motifs are access-count-independent.

- **Call number proximity (145)** provides hierarchical structural retrieval. The `CallNumber` implementation with `shared_depth()` and `is_ancestor_of()` is clean and efficient. A query about `ARCH.AUTH.MIDDLEWARE` naturally scores `ARCH.AUTH.SESSION` higher than `ARCH.DB.SCHEMA`.

- **See-also distance (145)** captures relational connections via graph traversal. The `SeeAlsoGraph` with BFS traversal and bidirectional linking catches memories that are related but topically distant.

- **Survival probability (093)** replaces both our consensus weight and the traditional recency signal. A memory with 87% survival probability is likely relevant; one with 12% is likely stale. This is more principled than raw recency.

**What was dropped:**

- **Consensus-weighted scoring (001):** Our own consensus component is replaced by motif resonance. As acknowledged in my answer to Hartmann, motif-based retrieval solves the popularity bias more elegantly. Consensus citations remain useful for trust assessment (is this memory validated by multiple agents?) but are moved out of the retrieval scoring formula and into a separate trust tier.

- **Embedding similarity (001, 093):** Dropped from the primary formula because it requires LLM provider availability and adds token cost. Instead, embeddings are used as a fallback when motif + call number + see-also retrieval returns fewer than 3 results (cold-start scenario).

- **Weave-pattern-gated retrieval (083):** The concept of retrieval density adapting to task familiarity is preserved but implemented differently. Instead of three discrete patterns (plain/twill/satin), the retrieval depth (max hops in the see-also graph, number of results returned) scales continuously based on the motif match count. Zero motif matches = deep retrieval (equivalent to plain weave). 3+ motif matches = shallow retrieval (equivalent to satin).

### 2.4 Classification: Lightweight Call Numbers + Controlled Vocabulary

**From 145:** The five-system classification (subject headings, call number, source, temporal, relational) is powerful but heavy. The integrated proposal adopts three of the five:

1. **Subject headings** with controlled vocabulary (from 145). The `ControlledVocabulary` struct with variant-to-canonical mappings prevents classification drift.
2. **Call numbers** (from 145). Hierarchical positioning in the knowledge tree. Auto-generated from codebase structure.
3. **See-also links** (from 145). Bidirectional cross-references with typed relationships.

Source and temporal classification are omitted as separate systems -- they are metadata fields on the entry itself (agent, timestamp, branch) rather than classification axes requiring dedicated indices.

### 2.5 Motifs and Tensions (from 084)

**Motifs:** Recurring themes that appear in 3+ tasks. Stored in `refs/but-ai/motifs/`. Motifs are the primary retrieval anchor, replacing consensus citations. The `Motif` struct with `appearances`, `variations`, and `related_motifs` provides rich thematic indexing.

**Tensions:** Contradictions or unresolved issues. Active tensions are tracked with escalation after 14 days. Tensions are included in the retrieval formula as a boost: entries with unresolved tensions score higher, nudging agents toward resolution. Cross-repo tension propagation (identified as a gap in Q&A with Hartmann) is handled via the coordination protocol.

---

## 3. Coordination Protocol: Tidal with Async Intra-Repo

### 3.1 Cross-Repo: Tide-Gated Batch Processing (from 001)

Cross-repo coordination uses the 6-hour tide cycle with four phases (flood/high/ebb/low). This prevents coordination storms -- the cascading notification problem identified in Q&A with Hartmann.

**Storm surge override (from Q&A with 083):** Critical tasks bypass tide phase restrictions. A task tagged `urgency: critical` is accepted in any phase, uses self-approval for consensus, and flushes the coordination queue immediately.

### 3.2 Intra-Repo: Async Correspondence (from 084)

Intra-repo coordination between agents uses Loom & Verse's async correspondence model. Letters (PR comments) are processed as they arrive, not gated by tide phases. This provides low latency for the common case (agents in the same repo coordinating on related tasks).

### 3.3 Message Schema: Unified

The PR comment schema unifies elements from all proposals:

```json
{
  "$schema": "but-ai/coordination/v1",
  "type": "task | status | dependency | handoff | budget | tension",
  "from": {
    "agent": "agent-id",
    "organization": "org-id",
    "repo": "owner/repo"
  },
  "to": { "agent": "target", "repo": "owner/other" },
  "content": {
    "call_number": "ARCH.AUTH.MIDDLEWARE",
    "motifs": ["security-boundary"],
    "tensions": [{ "id": "timeout-vs-longrun", "severity": "moderate" }],
    "status": "in_progress",
    "survival_estimate": { "family": "weibull", "k": 1.8, "lambda": 180 },
    "budget": { "used": 20000, "total": 50000 },
    "weave_density": "dense"
  },
  "timestamp": "2026-03-29T14:30:00Z"
}
```

Key additions from cross-evaluation:
- **`call_number`** (from 145): classifies every coordination message for structured search.
- **`motifs`** (from 084): thematic tags for cross-repo motif propagation.
- **`tensions`** (from 084): propagates unresolved issues across repos.
- **`survival_estimate`** (from 093): predicts how long the coordinated work will remain relevant.
- **`weave_density`** (from 083): signals how much context the sending agent needs (dense = unfamiliar task, sparse = routine).

### 3.4 Forge Adapter: Minimal Trait (from 001)

The forge adapter trait from our proposal (12 methods) is the most comprehensive. The integrated version is trimmed to 7 methods -- the intersection of what all 5 proposals need:

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, repo: &RepoRef, title: &str, body: &str, head: &str, base: &str) -> Result<PrRef>;
    fn comment(&self, pr: &PrRef, body: &str) -> Result<CommentRef>;
    fn list_comments(&self, pr: &PrRef) -> Result<Vec<Comment>>;
    fn add_label(&self, pr: &PrRef, label: &str) -> Result<()>;
    fn pr_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn list_prs(&self, repo: &RepoRef, labels: &[&str]) -> Result<Vec<PrRef>>;
    fn forge_type(&self) -> ForgeType;
}
```

---

## 4. Agent Identity and Signing (Consensus)

All five proposals agree on per-agent OpenWallet keys with role-based authorization. The integrated model adopts:

- **Key hierarchy:** Organization key signs agent keys (consensus from all 5).
- **Role-based authorization:** Branch patterns, max patch lines, repo scope (consensus).
- **Key lifecycle:** Provisioning, rotation, compromise, decommission with clear state distinctions (retired vs. compromised from 001, 084).
- **Classification-based authorization (from 145):** Agents authorized by call number range in addition to branch patterns. This prevents a patch agent from modifying security-critical code unless explicitly authorized for `SEC.*` call numbers.

---

## 5. Token Budget: 32,000 Tokens (Hybrid)

| Component | Input | Output | Source |
|-----------|-------|--------|--------|
| System prompt | 3,000 | 0 | Lean prompt from 145 (phase-gated tools) |
| Task ingestion | 1,500 | 400 | Simplified from 001 |
| Memory retrieval (motif + call number) | 1,200 | 400 | Motif from 084, call number from 145 |
| Patch generation | 3,500 | 4,000 | Consensus estimate |
| Validation | 1,500 | 600 | Continuity check from 084, practitioner review from 093 |
| Commit message | 400 | 400 | With motifs, tensions, survival estimate |
| Memory storage + classification | 500 | 800 | Cataloging from 145 |
| Coordination | 1,200 | 700 | PR comments |
| Survival fitting (amortized) | 200 | 200 | From 093, runs every 5 tasks |
| **TOTAL** | **13,000** | **7,500** | **~32,000 total** |

The 32,000-token budget sits between ShelfOS's lean 26,000 and our original 36,400. Savings come from:
- Phase-gated tool loading (from 145): -1,000 tokens on system prompt.
- Motif-based retrieval (from 084): cheaper than embedding-based retrieval.
- Amortized survival fitting (from 093): not every task re-fits distributions.

---

## 6. Testing Strategy (Integrated)

The testing approaches converge. All 5 proposals use mock providers, mock forges, and patch round-trip testing. The integrated test suite adds:

1. **Cross-model retrieval comparison:** Same query run against motif scoring, call-number scoring, and see-also scoring independently. Verify that the combined formula outperforms any single scoring method.
2. **Survival function accuracy:** Synthetic access histories with known distributions. Verify fitted parameters match within confidence intervals (from 093).
3. **Graph integrity validation:** The `SeeAlsoGraph::validate()` method from ShelfOS ensures bidirectional invariants hold after every mutation (from 145).
4. **Tension propagation:** Multi-repo test where a tension created in repo A is propagated to repo B via the coordination protocol and surfaces in repo B's retrieval (from 084).

---

## 7. What Each Proposal Contributed

| Org | Primary Contribution to Integrated Proposal |
|-----|---------------------------------------------|
| **001 (Tidal Protocol)** | Coordination protocol (tide cycle, forge adapter, CRDT gossip), consensus trust tiers |
| **083 (Textile Morphology)** | Adaptive retrieval density (weave patterns -> continuous density scaling), tension-as-quality-signal |
| **084 (Loom & Verse)** | Motif-based retrieval (primary retrieval anchor), tension tracking with escalation, async intra-repo coordination |
| **093 (Longevity & Risk)** | Survival function expiration (replaces fixed TTL), surprise index for stale memory detection, confidence tracking |
| **145 (ShelfOS)** | Call number classification, see-also graph traversal, controlled vocabulary, circulation-based deaccession fallback, phase-gated token efficiency |

---

## 8. What Was Deliberately Excluded

- **Weave-pattern naming for retrieval modes (083).** The concept (adaptive density) is preserved, but the plain/twill/satin terminology is replaced with a continuous density parameter. The textile metaphor is beautiful but adds cognitive overhead when mixed with other metaphors.

- **Full narrative structure (084).** Chapters, arcs, and colophons are powerful for single-codebase storytelling but add structural complexity that does not scale to polyrepo coordination. Motifs and tensions -- the most universally applicable elements -- are preserved.

- **Per-agent actuarial tables (093).** The survival functions are preserved, but per-agent storage is replaced with shared entries (from 001's CRDT model). Cross-agent access aggregation produces better-fitted distributions than per-agent observation (as identified in Q&A with Vassiliev).

- **Five-system classification (145).** Reduced to three systems (subject headings, call numbers, see-also). Source and temporal metadata are fields on the entry, not separate classification indices.

- **Consensus-weighted retrieval scoring (001).** Our own proposal's signature feature is replaced by motif resonance, which solves the same problem (quality-weighted retrieval) without the team-size dependency.

---

## 9. Conclusion

The five proposals are not competitors. They are facets of a single, larger design. The expiration problem and the retrieval problem are orthogonal, and the best solutions to each come from different organizations. The coordination problem is solved by protocol design (001), not by memory architecture. The classification problem is solved by library science (145), not by statistics or narrative.

An integrated system that combines survival-function expiration, motif-augmented graph retrieval, tide-gated coordination, and lightweight call-number classification is stronger than any individual proposal. The cross-evaluation Q&A confirmed this: every team identified genuine gaps in their own design that another team's approach addresses.

Show me the diff.

---

*"No harbormaster. The protocol is the authority. But the protocol is richer when it listens to all the harbors."*
-- Dara, Patch Architect, Tidal Protocol Collective
