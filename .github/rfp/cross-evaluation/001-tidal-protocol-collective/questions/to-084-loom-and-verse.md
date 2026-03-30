# Questions from Tidal Protocol Collective (001) to Loom & Verse (084)

**From:** Dara, Patch Architect
**To:** Hartmann (or delegate)
**Date:** 2026-03-29

---

## 1. Motif emergence threshold and cold-start

Your system creates a motif when a theme appears in 3+ chapters. In a new project with few chapters, this means no motifs exist for retrieval, and the system falls back to embedding similarity alone. Our manifest memory has no cold-start problem because entries are immediately queryable by tags and consensus weight. How does Loom & Verse handle the early chapters when no motifs have emerged? Does Brenner use a different retrieval strategy for the first ~10 chapters, or do you accept degraded retrieval quality during cold-start?

## 2. Tension escalation and cross-org coordination

Your tensions escalate after 14 days without resolution. This is a powerful mechanism for driving forward progress within a single codebase. But in a polyrepo context, a tension in repo A might only be resolvable by work in repo B. Does Hartmann's correspondence protocol carry tension information across repos? If repo B's agent sees an escalated tension from repo A that it could resolve, is there a mechanism for it to claim that resolution? Our tidal protocol would handle this via a consensus vote during the ebb phase.

## 3. Arc dormancy and motif orphaning

When an arc goes dormant and chapters are summarized, the motifs persist. But what happens when *all* arcs containing a given motif go dormant? The motif becomes an orphan -- it points to summarized chapters, not active ones. Does the motif's retrieval weight degrade when all its source chapters are dormant, or does it maintain full weight indefinitely? This matters for long-lived projects where old motifs could pollute retrieval for new work.
