# Questions from Hartmann (084) to Vassiliev (093)

## Q1: Survival Functions vs. Narrative Tension for Memory Relevance

Your survival function approach is the most mathematically rigorous memory management among all five proposals. The `SurvivalFunction` trait with S(t), f(t), h(t) implementations for four distribution families is impressive -- real actuarial mathematics, not metaphor.

**Question:** Our narrative memory uses "tension urgency" as 10% of the relevance score -- unresolved contradictions get boosted. Your model has no equivalent concept. A memory with high survival probability but known to be in contradiction with newer information would still score well in your system. How does the LRRC handle contradictory memories? Does the surprise index catch this case, or is contradiction detection outside the scope of the survival model?

## Q2: Distribution Fitting with Sparse Data

Your system re-fits survival distributions every 3-5 tasks based on accumulated access data. The `fitting.rs` module uses MLE with AIC-based model selection. This is statistically sound for memories with rich access histories.

**Question:** For new memories with fewer than 5 access records, how reliable is the distribution fitting? A Weibull fit with 3 data points has enormous confidence intervals. Our system avoids this problem by using motif resonance (a structural, not statistical, signal) for new content. Do you fall back to prior distributions (the defaults in ActuarialConfig) until sufficient data accumulates, or does Abebe attempt fitting even with minimal data?

## Q3: Moribund State and the Centenarian Problem

The alive/moribund/deceased lifecycle with Abebe reviewing moribund memories before expiration is a thoughtful design. It acknowledges that low-survival-probability memories can still be relevant (the centenarian analogy).

**Question:** What prevents the moribund queue from growing unboundedly? If Abebe reviews moribund memories and frequently resuscitates them, the moribund state becomes a de facto "permanent low-priority" tier rather than a transition to deceased. In our system, dormant arcs are summarized and the individual chapters are permanently archived -- there is no resuscitation. Is there a limit on how many times a memory can be resuscitated before it is forced to deceased?
