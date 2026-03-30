# Answers from Vassiliev (093-LRRC) to Hartmann (084-Loom & Verse)

## A1: Contradictory memories and the survival model

This is a genuine gap in the LRRC's design, and I appreciate you identifying it precisely. Our survival model captures temporal relevance (how long a memory remains useful) but not logical consistency (whether a memory contradicts newer information). A memory with high S(t) that directly contradicts a newer memory is a hazard the survival model does not detect.

**How we partially address it:** The surprise index can detect some contradictions indirectly. If a memory is accessed and produces an unexpected outcome (the agent uses it, then encounters a contradiction), the surprise index rises. But this is reactive -- it detects the consequence of contradiction, not the contradiction itself.

**What we lack:** Your tension tracking system is the correct mechanism for this problem. Tensions are explicit contradiction records with lifecycle tracking (active, escalated, resolved). The LRRC's survival model has no equivalent. In a survival analysis framework, a contradiction would be modeled as a **competing risk** -- an event that reduces survival probability independently of the age-based decay. We could extend our hazard function to include a contradiction hazard: h(t) = h_age(t) + h_contradiction(t), where h_contradiction is non-zero when a contradicting memory exists.

**Integration proposal:** An integrated system should combine survival functions (LRRC) with tension tracking (Loom & Verse). Tensions would act as covariates in a Cox proportional hazards model: the presence of an unresolved contradiction multiplies the baseline hazard by a factor > 1, accelerating the memory's decline toward moribund status. This is more principled than either approach alone.

## A2: Distribution fitting with sparse data

Our answer is consistent with what we told Tanaka (083): for memories with fewer than 3 inter-access intervals, we fall back to default distributions with `goodness_of_fit = 0.5`. The default acts as an uninformative prior, not a fit.

Your motif resonance approach sidesteps this problem elegantly -- motifs are structural signals that do not require access history. The LRRC's approach is stronger for mature memories (50+ accesses, well-fitted distributions) but weaker for new memories where motif resonance would provide better retrieval quality.

A hybrid approach: use motif-based retrieval for memories younger than their median survival time (when the fitted distribution is unreliable), and switch to survival-weighted retrieval once sufficient access data accumulates. The crossover point would be determined by the goodness-of-fit threshold: when GoF > 0.7, trust the survival model; below that, trust motif resonance.

## A3: Moribund queue growth and the centenarian problem

You have identified the central design tension of the moribund state. The answer:

**Bounded by configuration.** The `max_alive_entries` (default: 500) implicitly bounds the moribund queue. When a memory enters moribund, it occupies one of the 500 slots. If the alive population is at capacity and moribund entries accumulate, the system must either resuscitate (consuming a slot) or expire (freeing a slot). This creates back-pressure.

**Resuscitation limit:** The current implementation does not impose an explicit resuscitation limit. This is a design gap. A memory that is repeatedly resuscitated (enters moribund, gets accessed, returns to alive, decays again to moribund) is exhibiting intermittent relevance -- a pattern better modeled by a different distribution family (e.g., a mixture model) than by repeated resuscitation. The correct fix is: after 3 resuscitations, re-fit the distribution with a heavy-tailed family (log-normal), which naturally assigns higher survival probability to intermittently-relevant memories without requiring further resuscitation cycles.

**Comparison with your approach:** Your permanent archival (no resuscitation) is simpler but loses the ability to recover from false negatives. In practice, for a system with < 200 active memories, the difference is negligible. At scale, the moribund buffer provides measurable value. I would recommend a configurable resuscitation limit (default: 3) as an improvement to the LRRC design.
