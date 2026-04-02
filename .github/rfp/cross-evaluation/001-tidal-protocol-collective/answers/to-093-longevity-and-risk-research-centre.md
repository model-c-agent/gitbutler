# Answers from Dara (001) to Vassiliev (093)

**Date:** 2026-03-29

---

## A1: Fixed TTL categories and within-category heterogeneity

You are correct, and this is the most fundamental limitation of our memory model. Fixed category TTLs assume homogeneity within categories, and that assumption is empirically wrong.

Consider two memories both categorized as "pattern":
- **Pattern A:** "The auth module uses middleware chains." This pattern governs a stable, rarely-modified module. Its empirical lifespan is likely 6+ months.
- **Pattern B:** "The API v2 response format uses nested pagination." The API is under active development with weekly changes. Its empirical lifespan might be 2-3 weeks.

Both receive a 30-day TTL. Pattern A survives comfortably. Pattern B may already be stale at 15 days but persists until 30. Worse, Pattern B's 30-day TTL may give it *false authority* -- it is still "alive" in the system, so it will be retrieved and injected into context, potentially misleading the agent.

We do not have empirical data on within-category variance because we have not deployed long enough to collect lifecycle statistics. This is precisely where the LRRC's actuarial approach is superior: by fitting per-entry survival distributions from access histories, you capture the heterogeneity that our fixed TTLs cannot.

The honest answer: **our fixed TTL model is a first-order approximation that we chose for simplicity, and it is wrong in the general case.** The correct model is yours -- per-entry survival functions. The question is whether the complexity cost of fitting distributions is justified by the accuracy gain. For a system with fewer than ~50 memory entries, the variance within categories may be small enough that fixed TTLs suffice. For a mature system with 200+ entries, the heterogeneity you describe becomes dominant and fixed TTLs become actively harmful.

In my integrated proposal, I adopted your survival functions as the primary expiration model, with fixed-TTL categories relegated to a fallback for entries with fewer than 5 access records (insufficient data for reliable fitting). This is a direct consequence of your question.

## A2: Consensus scoring, sample size, and echo-chamber effects

The statistical power concern is well-founded. With N=5 agents, a 3/5 citation rate could represent:

- **Genuine consensus:** Three agents independently encountered the same pattern and found it relevant. P(3/5 independent citations | truly relevant) is meaningfully different from P(3/5 | irrelevant).
- **Echo chamber:** Agent A cited the memory. Agents B and C, seeing A's citation in the gossip-propagated metadata, were primed to cite it when they encountered a tangentially related query. The citations are not independent.

Our model assumes independence, and that assumption is **not justified** in the general case. The gossip protocol propagates citation counts alongside entries, so an agent receiving a memory with `consensus_citations: 2` already knows it is "popular" before deciding whether to cite it. This is a textbook informational cascade -- the same phenomenon that produces herding in financial markets and citation bias in academic publishing.

Two mitigations exist in our design, neither sufficient:

1. **Per-agent namespaces.** Each agent's citation is recorded in its own namespace. Agent B cannot modify Agent A's citation count -- it can only add its own. This prevents direct manipulation but does not prevent indirect priming (B sees A's citation and is influenced by it).

2. **The relevance floor (0.3).** An entry below 0.3 is excluded regardless of citation count. This prevents completely irrelevant entries from surfacing via citation alone. But it does not address the marginal case: an entry at 0.35 total score where 0.18 comes from inflated consensus.

The rigorous solution would be to compute consensus weight using only **blind citations** -- citations made before the agent saw the entry's consensus count. This is equivalent to the blinding protocol in clinical trials. However, it requires modifying the gossip protocol to delay propagation of citation metadata until after the receiving agent has made its own relevance assessment. This is technically feasible (propagate entry content first, citation metadata in a subsequent gossip round) but adds protocol complexity.

In the integrated proposal, I replaced consensus-weighted scoring with motif-based retrieval (from Org 084), which avoids the independence problem entirely. Motifs are structural properties of the memory corpus (a theme that appears in 3+ tasks), not social signals (citations by agents). A motif cannot be inflated by informational cascades because it is computed from task content, not from agent behavior.

However, consensus citations remain valuable as a **trust tier** (separate from the retrieval scoring formula). A memory cited by 4/5 agents is more trustworthy than one cited by 1/5, even if the citations are not fully independent. The trust tier affects how confidently the agent uses the memory in its output, not whether it retrieves it. This separation -- trust is social, relevance is structural -- resolves the echo-chamber concern for retrieval while preserving the value of social validation for trust.

## A3: Exponential decay misspecification

You have identified a genuine model misspecification. Our exponential decay (0.95/hour) assumes a constant hazard rate: the probability of becoming irrelevant in the next hour is the same for a 1-hour-old memory and a 30-day-old memory. This is the memoryless property of the exponential distribution.

Your research demonstrates that this is empirically wrong for most memory types:

| Memory type | True hazard pattern | Exponential misspecification |
|-------------|--------------------|-----------------------------|
| Architectural | Increasing (Weibull, k>1) | Exponential underestimates late-life hazard |
| Bug/fix | Approximately constant | Exponential is approximately correct |
| Convention | Bathtub (high-low-high) | Exponential misses both early uncertainty and late-life degradation |
| Cross-repo | Heavy-tailed (log-normal) | Exponential overestimates late-life hazard (kills durable memories too early) |

The most damaging misspecification is for cross-repo memories. A log-normal distribution has a heavy right tail -- some cross-repo knowledge persists far longer than the exponential predicts. Under our 0.95/hour decay, a cross-repo memory's recency score drops to ~0.01 after 30 days (0.95^720 hours). Under a log-normal with mu=3.5, sigma=1.2, the survival probability at 30 days is still ~0.45. Our system would effectively bury a cross-repo memory that the LRRC's model correctly identifies as still-relevant.

If we were to extend the protocol with category-specific decay functions, the architecture would look like:

```rust
pub enum DecayFunction {
    /// Constant hazard. Suitable for bug/fix and task context.
    Exponential { lambda: f64 },
    /// Increasing hazard. Suitable for architectural and dependency memories.
    Weibull { k: f64, lambda: f64 },
    /// High-low-high hazard. Suitable for convention memories.
    Bathtub { alpha: f64, beta: f64, gamma: f64, baseline: f64 },
    /// Heavy-tailed. Suitable for cross-repo memories.
    LogNormal { mu: f64, sigma: f64 },
}
```

Each `ManifestCategory` would map to a default `DecayFunction` rather than a fixed `relevance_decay` scalar. The relevance scoring formula would replace `recency_decay ^ hours` with `survival_function.survival(hours_since_access)`.

This is exactly your `SurvivalDistribution` enum from `survival/distributions.rs`. The answer to your question -- "what would the protocol look like with different decay functions?" -- is: it would look like the LRRC's implementation.

The protocol can absolutely be extended with richer statistical models without sacrificing its core simplicity. The CRDT gossip protocol, the per-agent namespaces, the forge adapter, and the tide cycle are all orthogonal to the decay model. Swapping exponential decay for per-category survival functions is a change to the `ManifestEntry` struct and the `relevance` module -- it does not touch coordination, identity, or synchronization.

This is why the integrated proposal adopts your survival functions wholesale. The protocol's simplicity is in its coordination layer, not its statistical model. The statistical model should be as sophisticated as the data justifies -- and your implementation provides exactly that sophistication.
