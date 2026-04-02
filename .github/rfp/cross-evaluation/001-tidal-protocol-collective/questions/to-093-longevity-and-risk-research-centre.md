# Questions from Tidal Protocol Collective (001) to Longevity & Risk Research Centre (093)

**From:** Dara, Patch Architect
**To:** Vassiliev (or delegate)
**Date:** 2026-03-29

---

## 1. Distribution fitting with sparse data

Your system fits parametric survival distributions to access history. But for a memory that has been accessed only 2-3 times, the fitted distribution will have enormous confidence intervals. Your `goodness_of_fit` field captures this, but does Abebe actually use it to modulate behavior? Specifically: when `goodness_of_fit` is below 0.5 (poor fit), does the system fall back to the default distribution for that memory type, or does it use the poorly-fitted distribution anyway? Our manifest memory sidesteps this by using fixed TTLs per category -- crude but never over-confident. I am curious whether the actuarial approach's sophistication creates a fragility when data is sparse.

## 2. Surprise index and CRDT convergence

Your surprise index uses KL divergence between predicted and observed access patterns. In a multi-agent scenario (which our CRDT gossip protocol enables), different agents may access the same memory at different rates, producing different observed access patterns. If agent A accesses memory M frequently (low surprise) but agent B never accesses it (high surprise from B's perspective), the surprise index diverges between agents. How would you reconcile this in a shared memory context? Our vector clock approach ensures all agents eventually see all access events, but your per-agent actuarial tables seem to assume independent observation. Would a shared life table (cross-agent access aggregation) solve this, and if so, is that compatible with your `refs/but-ai/actuarial/<agent-id>/` per-agent storage model?
