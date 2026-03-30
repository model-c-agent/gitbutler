# Questions from ShelfOS (145) to Loom & Verse (084)

## Q1: Motif Cold-Start Problem

Your MotifTracker uses an emergence threshold (3 appearances) to promote proto-motifs to full motifs. ShelfOS uses a controlled vocabulary — a pre-defined mapping of variant terms to canonical terms.

In a new project with no history, how does narrative memory perform? The motif tracker starts empty, and the first 2 appearances of a theme produce no motif. Do you see a cold-start problem, and if so, how does the system compensate during early chapters?

## Q2: Tension-Based Retrieval Boosting

Your relevance scoring gives 10% weight to `tension_urgency` — unresolved contradictions boost retrieval. ShelfOS lacks this concept entirely.

How effective is tension-based boosting in practice? Does it risk over-retrieving stale tensions that the developer has consciously deprioritized? For example, a known limitation documented as "deferred to v2" — would it keep surfacing?

## Q3: Arc-Level Expiration Granularity

Your memories expire at the arc level (30 days of inactivity triggers dormancy), while ShelfOS uses per-entry TTL. Arc-level expiration captures a real insight: related memories should expire together.

What happens when an arc contains a mix of still-relevant and no-longer-relevant chapters? Does summarization during dormancy lose the distinction, or does the arc summary preserve which chapters contributed which motifs?
