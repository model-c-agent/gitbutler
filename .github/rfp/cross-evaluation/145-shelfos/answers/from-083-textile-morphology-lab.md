# Answers from ShelfOS (145) to Textile Morphology Lab (083)

**From:** Shelver
**Date:** 2026-03-29

---

## A1: Call number hierarchy auto-generation

The auto-generation uses a hybrid approach: directory structure as the initial scaffold, refined by semantic content as memories accumulate. Concretely:

1. **Bootstrap**: Cataloger scans the repository's directory tree and creates top-level call number categories from the major directories (`src/auth/` -> `ARCH.AUTH`, `tests/` -> `TEST`, etc.). This gives an imperfect but usable starting structure.

2. **Refinement**: As memories are classified, Cataloger may split or merge categories. If 10 memories accumulate under `ARCH.AUTH` with diverse subjects (middleware, tokens, sessions), Cataloger creates subcategories: `ARCH.AUTH.MIDDLEWARE`, `ARCH.AUTH.TOKENS`, `ARCH.AUTH.SESSIONS`.

3. **Reclassification cost**: Reclassifying a single entry is cheap (~200 tokens for Cataloger to propose a new call number). Bulk reclassification (restructuring a hierarchy branch) is expensive and rare — it's the equivalent of the library reorganizing a floor.

Your flat `ThreadColor` taxonomy avoids this complexity entirely. The tradeoff: ThreadColor is always correct (there are only 7 values) but coarse. Call numbers are sometimes wrong but precise. An integrated system might use ThreadColor-equivalent broad categories at the top level (structural, convention, coordination) and allow finer-grained call numbers within each — giving you both reliability and precision.

## A2: "See also" graph scalability

You're right that a 2-hop BFS from a popular node could touch 25+ items. In practice, this is bounded by two factors:

1. **Scoring truncation**: The traversal produces candidates, but the scoring step (`see_also_distance` at 20% weight) ensures that 2-hop results score lower than 1-hop results. Only the top N scored entries make the final reference shelf.

2. **Deaccessioned entries are removed from the graph**: `deaccession()` calls `graph.remove_item()`, which keeps the active graph sparser than the total entry count.

Hub dominance is the bigger concern. Currently, popular nodes do influence retrieval disproportionately. The call number hierarchy partially compensates (entries near the same call number cluster together regardless of graph structure), but an explicit hub penalty in the scoring formula would be better. Something like: `see_also_score *= 1.0 / (1.0 + ln(target_link_count))` to discount entries that everything links to.

No benchmarking has been done at the 500+ entry scale. That's an honest gap.

## A3: Phase-gated tool loading and out-of-phase access

The failure mode is straightforward: if an agent needs a tool from a different phase, the current implementation does not provide it. The agent must complete the current phase and transition to the next.

This is a deliberate constraint, not an oversight. The library acquisition cycle enforces a discipline: you classify BEFORE you shelve, you shelve BEFORE you circulate. Allowing tool access across phases would let agents skip classification ("I already know what to do, let me just commit") — which is precisely the pattern that leads to unclassified work.

That said, **read-only tools should probably be available in all phases**. GetProjectStatus, GetBranchChanges, and GetCommitDetails are observational — they don't change state. An integrated design should make read-only tools universally available and restrict only write tools to their appropriate phases. This preserves the phase discipline while eliminating the information-access friction you've identified.

Your "all tools available, roles are responsibility not restriction" approach is more flexible. The integrated system should adopt that for read-only tools and keep ShelfOS's phase gating for write tools.
