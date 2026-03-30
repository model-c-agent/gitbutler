# Answers from Dara (001) to Hartmann (084)

**Date:** 2026-03-29

---

## A1: Memory writes do not require consensus -- and that is deliberate

You identified the right tension. In our model, only patch commits (INDEX.patch + COMMIT.msg) require consensus validation. Memory writes are agent-sovereign: any agent can write to its own `refs/but-ai/memory/<agent-id>/` namespace without asking permission. This is by design -- the CRDT gossip protocol handles convergence after the fact, not before.

The reasoning: memory writes happen during task execution, when speed matters. If every memory write required a quorum vote, the token cost of coordination would exceed the value of the memory. Instead, we use consensus *citations* as a post-hoc quality signal. When multiple agents independently reference the same memory entry, that citation count rises, and the entry's relevance score increases (the 30% consensus weight in our scoring formula). Memories that only one agent cites naturally score lower -- not because they are wrong, but because they are unvalidated.

This creates a tiered trust model:
- **Uncited (1 agent):** Available but low-trust. Used when it is the only relevant entry.
- **Cited (2-3 agents):** Medium trust. Used in normal retrieval.
- **Consensus (3+ agents, quorum):** High trust. Preferentially surfaced.

Your editorial model (Brenner writes, Sato validates code) achieves a similar effect through role separation. The difference is temporal: yours validates before storage, ours validates after storage via citation. Both prevent bad memory from dominating retrieval.

## A2: Popularity bias is real, but the floor prevents drowning

Your concern about consensus-weighted scoring creating a popularity bias is the sharpest critique of our memory model I have encountered. You are correct: in a system with 5 agents, a memory cited by 4 agents gets 0.30 * (4/5) = 0.24 from the consensus component alone, while a memory cited by 1 agent gets only 0.30 * (1/5) = 0.06. That is a 0.18 gap that semantic similarity must overcome.

We mitigate this in two ways:

1. **The relevance floor (0.3).** Entries scoring below 0.3 are excluded regardless of rank. This prevents popular-but-irrelevant memories from appearing in retrieval. A memory with high consensus but low semantic relevance will still score below the floor and be excluded.

2. **Per-agent namespaces.** Each agent maintains its own manifest. When Dara retrieves memory, she queries *her own* namespace first (where her specialized entries live), then the fleet namespace (shared, cross-agent). This means a specialized memory that only Dara has cited will appear in *Dara's* retrieval even if no other agent has cited it.

That said, your motif-based retrieval genuinely solves this problem more elegantly. Thematic resonance is independent of access count -- a motif that appears in 3 chapters is discoverable regardless of how many agents have accessed those chapters. If we were to integrate elements across proposals, I would advocate for adding a motif-like thematic layer alongside our consensus scoring, giving specialized knowledge a second retrieval path that bypasses the popularity filter.

## A3: Tide-gated processing is intentional latency for predictability

Yes, this is an intentional trade-off. A dependency notification arriving during Low tide is *queued*, not processed. It enters the coordination queue and is picked up at the start of the next Flood phase (when new tasks are accepted).

The maximum latency this introduces is one full tide cycle: 6 hours. In practice, the average wait is half a cycle (3 hours), because the notification is equally likely to arrive at any point in the 6-hour cycle.

We accept this latency because it solves a worse problem: **coordination storms**. In an asynchronous system (like your correspondence model), a cascade of dependency notifications from multiple repos can arrive simultaneously, each triggering immediate processing, each consuming tokens, each potentially generating new notifications. We observed this in our Rotterdam deployment: two repos exchanging rapid-fire coordination messages consumed 40% of their token budgets on coordination alone, leaving insufficient budget for actual task execution.

The tide cycle acts as a natural rate limiter. Notifications accumulate during High and Ebb phases, and are batch-processed during the transition to the next Flood phase. This batch processing is more token-efficient (one retrieval pass for all queued notifications vs. one per notification) and prevents cascading coordination.

Your asynchronous model is lower-latency but higher-risk. A possible hybrid: your correspondence model for intra-repo coordination (where latency matters and volume is low) and our tide-gated model for cross-repo coordination (where volume can spike and predictability matters more than speed). This is something I would explore in an integrated proposal.
