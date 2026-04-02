# Answers from Hartmann (084) to Tanaka (083)

## A1: Motif cold-start and proto-motifs

You have identified the single most significant weakness in our memory model. The cold-start problem is real. During the first ~10 chapters, motif_resonance (30% of the relevance score) returns 0.0 for all queries, and retrieval degrades to a weighted combination of embedding_similarity, arc_relevance, recency, and tension_urgency -- effectively a 70%-weighted subset of the full model.

Your suggestion of "proto-motifs" is one we should have thought of. In our current implementation, `MotifTracker::record_appearance` only promotes a theme to a motif at the emergence threshold. Before that, the theme exists only as a tag on individual chapters. The retrieval path in `NarrativeEngine::retrieve` calls `motifs.find_resonant()`, which only searches promoted motifs.

A concrete fix: the `MotifTracker` should expose a `find_proto_resonant` method that searches themes with 1-2 appearances, returning them with a reduced resonance weight (e.g., 0.3 instead of 1.0 per appearance). This way, even before formal motif emergence, thematic connections still influence retrieval. The cold-start period becomes a gradual ramp-up rather than a cliff.

This is a design change we would adopt in the integrated proposal.

## A2: Cross-arc tensions

You are correct that the `Tension` struct's `introduced_in` field is a chapter number, and chapters belong to a single arc. A tension about session timeout introduced in the auth arc has no explicit connection to the export arc.

However, the connection exists implicitly through motifs. If both the auth arc and the export arc share the motif "session-lifecycle", then the tension -- which is linked to a chapter that carries the "session-lifecycle" motif -- will surface when any task touching that motif is queried. The `tension_urgency` component of the relevance score boosts chapters with unresolved tensions regardless of which arc the query targets.

That said, this is implicit rather than explicit, and your point stands: the `Tension` struct should have an `affected_arcs: Vec<ArcId>` field that explicitly tracks cross-arc impact. The `TensionRegistry` should propagate tensions to arcs that share motifs with the originating chapter. This is a gap.

Your warp thread model handles this more naturally because warp threads are not scoped to categories -- they are structurally available to any task. Our arc-scoping is a constraint that the integrated proposal should relax for tensions specifically.

## A3: Dormancy with active tensions

This is a genuine design tension (a meta-tension, if you will). In the current implementation, `ArcManager::detect_dormancy` checks only the `last_chapter_at` timestamp against the dormancy threshold. It does not check whether the arc has active tensions.

The correct behavior: an arc with active tensions should **not** go dormant. The unresolved tension is evidence that the arc is not complete, even if no new chapters have been written. The dormancy check should be gated:

```
can_go_dormant = (time_since_last_chapter > dormancy_threshold)
                 AND (active_tensions.is_empty())
```

If an arc has active tensions but no recent chapters, it should enter a new state -- "stalled" -- which is distinct from dormant. Stalled arcs retain their individual chapters (no summarization, no information loss) but are flagged for attention. The tension escalation mechanism already handles the attention-drawing part: after 14 days, the tension escalates to Critical, which boosts it in every relevant query.

The Summarizer should refuse to summarize arcs with active tensions. This is a bug in our design that the integrated proposal must fix.
