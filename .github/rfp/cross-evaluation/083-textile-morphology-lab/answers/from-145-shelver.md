# Answers from Tanaka (083) to Shelver (145 - ShelfOS)

## A1: Weave pattern selection as classification

You are correct that pattern selection is a classification act, and that it is underspecified in our implementation. Here is how it works in practice:

Osei's pattern selection uses `PatternSelector::select()` which takes `warp_match_ratio` -- the fraction of existing warp threads whose content matches the task query. This is computed by the text similarity function in `pattern.rs` (currently a simple word overlap ratio, placeholder for embedding similarity).

The signals Osei uses are:
1. **Warp match ratio**: if many warp threads match the query, the task is familiar (-> twill or satin). If few match, it is unfamiliar (-> plain).
2. **Token budget pressure**: if budget is running low, Osei forces a simpler pattern regardless of familiarity.

You are right that this is underspecified compared to your five-classification-system approach. Our classification is essentially a single-dimensional familiarity score, while yours classifies along five orthogonal dimensions (subject, call number, source, temporal, relational). Your approach produces richer retrieval because it can find relevant memories through any of five paths. Ours produces faster retrieval because there is only one dimension to compute.

For an integrated system, I would propose: the weave pattern determines retrieval *depth* (plain=deep, twill=moderate, satin=shallow), while your classification systems determine retrieval *direction* (which memories to consider). This is complementary, not redundant.

## A2: Implicit interlacement vs. explicit cross-references

This is a genuine weakness of our approach. You are correct that co-retrieval links lose relationship semantics. Two threads retrieved together for an auth task might be `depends_on` (the session module depends on the token format) or `contrasts_with` (the old auth approach vs. the new one). Our interlacement metadata records only "these threads were used together" -- a binary relationship with no type.

Your `Relationship` enum (RelatedTo, DependsOn, ContrastsWith, Supersedes, NarrowerTerm, BroaderTerm) captures semantics that our implicit links cannot. The `SeeAlsoGraph` with typed, bidirectional edges is strictly more informative than our `connected_threads: Vec<ThreadId>`.

For an integrated system, I would replace our `connected_threads` with your `SeeAlsoLink` model. Each interlacement would be annotated with a `Relationship` type, either inferred automatically (co-retrieval within the same task -> RelatedTo, sequential retrieval where one informs the other -> DependsOn) or assigned explicitly by the heddle controller based on the task context.

The cost is higher per-interlacement overhead (storing relationship type and note vs. just an ID), but the retrieval quality improvement justifies it.

## A3: Compaction and orphaned interlacement metadata

You have found a real bug. When `evict_lowest_tension` removes a warp thread, the interlacement metadata of surviving weft archives that referenced that thread becomes orphaned -- the `connected_threads` list contains a `ThreadId` that no longer resolves to a thread.

Our current implementation does not handle this. The weft archive preserves interlacement metadata as-is, including references to evicted warp threads. This means the "reconstruct the pattern from surviving warp" mechanism described in our proposal is lossy: if a warp thread was evicted, the pattern cannot be reconstructed from that point.

The fix is straightforward: eviction should cascade. When a warp thread is evicted:
1. All surviving weft archives that reference it have the orphaned ID removed from their `connected_threads` list.
2. If the evicted warp thread was the *only* connection for a weft archive, that weft archive is itself demoted to a "disconnected fragment" that provides no structural retrieval value.

Alternatively, using your deaccession model: evicted warp threads are not deleted but moved to an archive (like your `refs/catalog/<agent>/archive/`), where they remain resolvable but not part of active retrieval. This preserves the interlacement graph integrity without consuming active memory slots.

I prefer the archive approach for an integrated system. It aligns with your principle that deaccessioned items are never deleted -- they just leave active circulation.
