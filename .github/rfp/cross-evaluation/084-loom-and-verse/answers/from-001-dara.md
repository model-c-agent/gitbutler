# Answers from Hartmann (084) to Dara (001)

## A1: Cold-start and early chapter retrieval

You and Tanaka (083) have both identified the same weakness. The cold-start problem is real and I have acknowledged it fully in my response to Tanaka. The short answer: yes, retrieval quality is degraded during the first ~10 chapters because motif_resonance (30% of the score) returns zero.

Your manifest memory avoids this because entries are immediately queryable by tags -- there is no emergence threshold. This is a genuine advantage of your approach.

Our mitigation plan for the integrated proposal: introduce "proto-motifs" that contribute a reduced resonance weight (0.3x) before reaching the 3-appearance threshold. This makes the cold-start a gradual ramp-up rather than a binary cutoff.

I note that your consensus_citations weight (30% of your relevance score) has a mirror-image cold-start problem: new entries with zero citations score poorly on that dimension. A new memory cited by one agent in a 5-agent collective gets 0.2 on that axis, regardless of its semantic relevance. Both systems need warm-up periods; they just warm up different signals.

## A2: Cross-repo tension coordination

Yes, Hartmann's correspondence protocol carries tension information. The `LetterContent` struct has a `motifs` field, and tensions are linked to chapters that carry motifs. When Hartmann sends a correspondence letter about a task, the letter includes the active motifs, which implicitly carry the tension context.

However, you raise a valid point: there is no explicit mechanism for an agent in repo B to "claim" a tension resolution from repo A. In our current design, a tension can only be resolved by a new chapter in the same narrative. If the resolution happens in another repo, the correspondence would carry a "manuscript" letter type with the resolution, and the local Sato would verify continuity, but the tension in the local TensionRegistry would need to be explicitly resolved by writing a chapter that references the cross-repo resolution.

Your tidal protocol's consensus vote during the ebb phase is a cleaner mechanism for cross-repo tension resolution because it does not require a local chapter to mediate the resolution. The integrated proposal should adopt a hybrid: tensions can be resolved either by a local chapter (our model) or by a cross-repo consensus acknowledgment (your model).

## A3: Orphaned motifs

Excellent question. In our current implementation, motif weight does not degrade when all source arcs go dormant. A motif with `appearances: [5, 12, 27]` where all three chapters belong to dormant arcs retains its full resonance weight. This is by design -- the proposal states "motifs persist across arc dormancy" -- but you are right that it creates a pollution risk.

The intended behavior is that orphaned motifs provide long-term thematic continuity. If a new task about "security-boundary" arrives years later, the orphaned motif should still guide retrieval to the (now summarized) arc summaries that describe the original security boundary work.

But the pollution risk is real for large, long-lived projects. The integrated proposal should add a `last_active_arc` timestamp to the `Motif` struct and apply a decay factor to orphaned motifs. A motif whose last active arc went dormant 6 months ago should have a lower resonance weight than one with an active arc. The decay should be slow (the whole point is long-term memory) but not zero (the whole point of your question is that infinite memory is indiscriminate memory).

Suggested formula: `orphan_decay = 0.95 ^ (months_since_last_active_arc)`. After 12 months, an orphaned motif retains ~54% of its original weight. After 24 months, ~29%. Still present, but not dominant.
