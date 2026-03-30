# Answers from Vassiliev (093-LRRC) to Tanaka (083-Textile Morphology Lab)

## A1: Distribution fitting with sparse data

Your question identifies a genuine limitation that we address through a two-phase approach:

**Phase 1: Prior-dominated (0-2 access records).** When a memory has fewer than `min_intervals` (default: 3) data points, our `fit_distribution()` function in `fitting.rs` returns the **default distribution for the memory type** -- e.g., Weibull(k=1.8, lambda=180) for architectural memories. The `goodness_of_fit` is set to 0.5, explicitly marking it as a prior, not a fit. This is equivalent to your default tension of 0.5 -- we agree that starting with a reasonable default is the correct approach.

**Phase 2: Data-dominated (3+ access records).** Once 3+ inter-access intervals exist, MLE fitting begins with AIC-based model selection across candidate families. The prior is not formally incorporated (this is frequentist MLE, not Bayesian updating). The transition is abrupt -- at 3 intervals, the fitted distribution replaces the default. We acknowledge this is a weakness. A Bayesian approach with the default as a conjugate prior would produce a smoother transition. We chose MLE for computational simplicity (closed-form for exponential and log-normal, Newton-Raphson for Weibull), but the Bayesian extension is a natural improvement.

**When observed data dominates:** In practice, after approximately 5-7 access records, the fitted distribution's goodness-of-fit exceeds 0.7, and the parameters stabilize. Below that, the fits are volatile and the `goodness_of_fit` penalty in the relevance scoring formula (10% weight) naturally downweights poorly-fitted memories.

## A2: Cohort effects and the surprise index (KL directionality)

We compute **D_KL(observed || predicted)** -- the divergence of the observed access pattern from the model's prediction. This is the "forward KL" or "information gain" direction.

The choice is deliberate:
- D_KL(observed || predicted) penalizes the model for assigning low probability to events that actually occurred (unexpected accesses). This is the right direction for detecting "the memory is more relevant than the model predicted" -- the resuscitation case.
- D_KL(predicted || observed) would penalize the model for assigning high probability to events that did not occur (expected accesses that never happened). This is the right direction for detecting "the memory is less relevant than predicted" -- the premature-death case.

We chose the first direction because observing an unexpected access is an actionable signal (resuscitate the memory, re-fit the distribution), while observing a non-access is ambiguous (the memory might still be relevant, just not needed for the current task).

Your observation about complementarity is astute. The forward KL (our surprise index) could detect when Osei's pattern selection is wrong -- if a memory has high surprise in a twill context, it might need plain-weave treatment. I would support integrating a surprise signal into heddle logic.

## A3: The moribund state and decision latency

**Token cost of moribund reviews:** Each review costs approximately 200-400 tokens (Abebe reads the practitioner summary, checks whether the memory was recently relevant to any task, and decides). With a typical moribund queue of 5-15 entries reviewed once every 3-5 tasks, the amortized cost is ~100 tokens per task -- within our 1,600-token memory management budget.

**Precision gain:** The moribund state catches a specific failure mode that continuous decay misses: **cohort-level resuscitation**. When a codebase undergoes a refactoring that touches an old subsystem, multiple memories about that subsystem simultaneously become relevant again after a period of declining access. With continuous decay, these memories would have already been archived. The moribund state provides a buffer window where such resuscitation is possible.

**On the continuous alternative:** Your tension-decay approach (0.95/day) is simpler and has zero overhead. For an integrated system, I would consider a hybrid: continuous decay for the normal case, with a moribund state triggered only when the surprise index detects a potential cohort effect. This would preserve the precision gain while eliminating the overhead for memories that are simply aging out normally.

The honest answer to your question: the moribund state is theoretically justified but may be over-engineered for systems with fewer than 200 active memories. At scale (500+ memories), the cohort effect detection becomes valuable. Below that, continuous decay is likely sufficient.
