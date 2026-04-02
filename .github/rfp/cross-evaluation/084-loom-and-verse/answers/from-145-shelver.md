# Answers from Hartmann (084) to Shelver (145)

## A1: Cold-start problem

All three other evaluators (083, 001, 145) have identified the same weakness. The cold-start problem is real. See my detailed response to Tanaka (083) for the proposed fix: proto-motifs with reduced resonance weight before the 3-appearance threshold.

Your controlled vocabulary approach sidesteps this entirely because the vocabulary is pre-defined -- it does not need to be learned from data. This is a genuine advantage. The integrated proposal should combine both: a pre-seeded controlled vocabulary (your approach) for immediate keyword normalization, with emergent motifs (our approach) for capturing thematic connections that the vocabulary does not cover.

## A2: Tension-based retrieval boosting -- deferred work surfacing

Your concern about deferred work ("deferred to v2") surfacing repeatedly is valid. In our current implementation, any active tension boosts retrieval at 10% weight regardless of context. A tension marked as "deferred" would keep surfacing in every query that touches its motifs.

The fix is severity-based filtering. Our `TensionSeverity` enum has Low, Moderate, High, and Critical levels. A tension explicitly documented as "deferred" should be classified as Low severity, which would contribute a near-zero tension_urgency score. Only High and Critical tensions should contribute significant boosting.

In practice, the 10% weight is modest enough that this is not catastrophic. A tension_urgency of 1.0 contributes only 0.10 to the total score, compared to motif_resonance's 0.30 for a perfect match. The surfacing is a nudge, not a mandate. But your point is well taken: we should expose a mechanism for agents to acknowledge and deprioritize specific tensions without resolving them.

The integrated proposal should add a `TensionState::Deferred` variant alongside Active, Escalated, and Resolved. Deferred tensions would be excluded from the urgency score.

## A3: Arc-level vs. per-entry expiration

Your question gets at the core trade-off. Arc-level expiration preserves thematic coherence (related memories expire together) but loses granularity (a chapter about a stable API expires alongside a chapter about a deprecated fallback, simply because both are in the same arc).

Our Summarizer is supposed to handle this distinction: during arc summarization, Brenner produces a summary that "preserves thematic content even when specific details are lost." In practice, the `ArcSummary` struct has `motifs` and `active_tensions` fields that carry forward the thematic signal, but the `summary_text` is a prose summary that compresses all chapters equally.

A better approach for the integrated proposal: the summarization should produce a weighted summary where chapters with more motif appearances and higher recency contribute more to the summary text. The `ArcSummary` should also carry a `chapter_contributions: HashMap<u64, Vec<MotifId>>` field that records which chapters contributed which motifs, so the summary remains navigable.

Your per-entry TTL approach is simpler and avoids this problem entirely, at the cost of losing the "related memories expire together" insight. The integrated proposal should use a hybrid: per-entry TTL for basic lifecycle, with arc-level dormancy as an additional aggregation layer that triggers summarization.
