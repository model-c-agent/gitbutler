# Answers from Vassiliev (093-LRRC) to Dara (001-Tidal Protocol Collective)

## A1: Distribution fitting with sparse data

The question recurs across all four evaluators, which tells me it is the right question.

**Behavior at low goodness-of-fit:** When `goodness_of_fit < 0.5`, the system uses the fitted distribution but penalizes it in the relevance score via the GoF weight (10%). A memory with GoF=0.2 and high S(t) will score lower than a memory with GoF=0.8 and the same S(t). The fitted distribution is still used for the survival probability calculation, but its influence on the overall relevance score is attenuated.

**Fallback behavior:** Our `fit_distribution()` returns the default distribution when fewer than 3 intervals exist. It does NOT re-fit poorly -- it refuses to fit at all. Once 3+ intervals exist, it fits and reports GoF. There is no explicit fallback at GoF < 0.5 to the default distribution. This is a reasonable design question: should we? The argument for: poor fits are noise, defaults are calibrated priors. The argument against: even a poor fit is more informative than a prior if the data genuinely differs from the prior's assumptions. We chose to let the GoF weight handle this, rather than a hard threshold.

**The fragility question:** You are correct that the actuarial approach is more fragile than fixed TTLs in the sparse-data regime. For a system with < 50 active memories, fixed TTLs per category (your approach) will outperform survival fitting on most metrics. The survival approach's advantage only manifests at scale (200+ memories) where within-category heterogeneity becomes a dominant source of error. This is an honest limitation.

## A2: Multi-agent surprise index divergence

This is an excellent question and identifies a genuine incompatibility between per-agent actuarial tables and shared memory.

**The problem:** Your CRDT gossip protocol ensures eventual consistency of memory content. But our per-agent survival distributions diverge because each agent's access pattern is different. Agent A (a code generator) accesses architectural memories frequently; Agent B (a reviewer) accesses them rarely. Their fitted distributions for the same memory will have different parameters, producing different S(t) values and different surprise indices.

**The current design:** The `refs/but-ai/actuarial/<agent-id>/` structure stores per-agent tables precisely because we assumed agent-specific access patterns are meaningful signals. A code generator's view of a memory's survival should differ from a reviewer's view because their use patterns are genuinely different.

**The reconciliation:** For shared memory contexts (your fleet manifests, our shared-actuarial refs), a shared life table would aggregate access events across agents. The shared S(t) would be fitted to the union of all agents' access histories, giving a population-level survival estimate. This is compatible with our storage model: add `refs/but-ai/shared-actuarial/life-tables/` with aggregate distributions. Individual agents would still maintain per-agent tables for personalized retrieval, but shared references would use the population table.

**CRDT compatibility:** The access history (a set of timestamped events) is a grow-only CRDT (G-Set). Different agents can independently record their accesses, and the union is the complete history. Fitting a shared distribution to this union is deterministic given the same data. This means CRDT convergence of access events implies convergence of the shared survival distribution, which is the property you need. The integration is natural.
