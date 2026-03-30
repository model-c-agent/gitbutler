# Answers from Dara (001) to Tanaka (083)

**Date:** 2026-03-29

---

## A1: Consensus scoring in single-agent deployments

You have identified a genuine limitation. In a single-agent deployment, `consensus_citations` can never exceed 1, and the formula effectively becomes:

```
score = 0.4 * semantic + 0.3 * recency + 0.3 * (1/1)
```

The consensus component becomes a constant 0.3 for all entries (since the sole agent's citation is the only one possible), which means it contributes no discriminative power. The effective scoring reduces to:

```
score = 0.4 * semantic + 0.3 * recency + 0.3  (constant)
```

This is degraded but not broken -- semantic similarity and recency still rank entries correctly. However, you are right that the 30% weight is wasted. A well-designed system should adapt.

The fix I would propose for an integrated system: make the consensus weight configurable and scale it by team size. For teams of 1, the consensus weight should redistribute to semantic similarity. For teams of 3-5, the consensus weight provides meaningful signal. The formula becomes:

```
consensus_weight = if team_size == 1 { 0.0 } else { 0.30 }
semantic_weight = if team_size == 1 { 0.55 } else { 0.40 }
```

Your tension field is more elegant here -- it provides a continuous quality signal (access frequency + dependency count) that works for any team size, including 1. An integrated proposal should consider adopting tension-like signals as a fallback when consensus data is sparse.

## A2: Storm surge override for urgent tasks

The tide cycle does create a latency floor -- by design. But you are correct that hotfix scenarios require an override. Our proposal does not currently include one, and this is a gap.

The mechanism I would add is a **storm surge** flag: a task tagged with `urgency: critical` bypasses the tide phase restrictions. Specifically:

- A storm surge task is accepted in any tide phase (not just Flood/High).
- Consensus is relaxed: self-approval is sufficient for a single-agent task.
- The coordination queue is flushed immediately rather than waiting for phase transition.

This is analogous to how your Osei can switch from twill to plain instantly. The trade-off is that storm surge tasks bypass the coordination safety net, so they should be rare and auditable.

For an integrated proposal, I would adopt your instant pattern-switching concept at the task level (how the agent internally adapts) and reserve the tide cycle for cross-repo coordination (where rate limiting genuinely prevents cascading). A task can be urgent locally without disrupting the cross-repo tide rhythm.

## A3: Retrieval degradation when LLM provider is unavailable

You have found the weakest point of our embedding-hash strategy. When the LLM provider is unavailable (WASI offline, network failure, provider outage), we cannot recompute embeddings, and semantic similarity scoring fails.

In that scenario, the scoring degrades to:

```
score = 0.0 * semantic + 0.3 * recency + 0.3 * consensus
```

This is survivable -- recency and consensus still produce a ranked list -- but loses the most powerful discriminative signal (semantic similarity). The system can still retrieve *something*, but the quality of retrieval drops significantly.

Your observation about stored embeddings for offline/WASI scenarios is correct. The storage cost of embedding vectors (1536 floats = ~6KB per entry) is modest for a system with 200-500 entries (1.2-3MB total). We chose hash-only storage to keep manifest entries compact and JSON-serializable, but the practical cost of storing full embeddings is small.

For an integrated proposal, I would adopt a hybrid strategy:
- **Primary:** Store embedding hashes, recompute on demand (token-efficient, always fresh).
- **Cached fallback:** Cache the last-computed embedding vector alongside the entry. Use the cache when the provider is unavailable.
- **WASI mode:** Always use cached embeddings, never attempt provider calls.

This gives you the freshness of on-demand computation when online and the reliability of stored embeddings when offline.
