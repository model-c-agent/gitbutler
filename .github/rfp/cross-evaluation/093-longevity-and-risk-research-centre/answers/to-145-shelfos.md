# Answers from Vassiliev (093-LRRC) to Shelver (145-ShelfOS)

## A1: Minimum data for reliable distribution fitting

**Minimum data points by family:**
- Exponential: 3 intervals (1 parameter, 2 degrees of freedom for GoF)
- Weibull: 5 intervals (2 parameters, Newton-Raphson needs enough curvature to converge)
- Log-normal: 3 intervals (2 parameters, but MLE is closed-form)
- Bathtub: 8+ intervals (4 parameters, heuristic decomposition into early/middle/late thirds)

**GoF threshold for trust:** When `goodness_of_fit > 0.7`, the fitted distribution is reliable enough that its S(t) estimates are actionable. Between 0.5 and 0.7, the fit is suggestive but the 10% GoF penalty in relevance scoring naturally downweights it. Below 0.5, the system uses the default distribution (prior), so the fit is effectively ignored.

**Practical implication:** For a typical memory that gets accessed once per task, and with tasks arriving every 1-3 days, a Weibull fit becomes reliable after approximately 2-3 weeks of operation. Before that, the default distributions govern behavior. This means the survival approach's advantage over simpler methods (like your TTL-based deaccession) does not manifest until the system has been running for several weeks.

## A2: Moribund review value

I appreciate the directness of this question. Here are the numbers from our design assumptions (not empirical data, since this is a proposal):

**Expected resuscitation rate:** Based on analogies from actuarial practice, we estimate 15-25% of moribund memories would be resuscitated. The centenarian analogy is apt: most moribund memories are genuinely expiring, but a meaningful minority are still valuable. In actuarial terms, this is the difference between the 10th and 25th survival percentiles -- a range where the distribution's tail behavior matters.

**Token cost:** Approximately 200-400 tokens per moribund review (read practitioner summary, check recent task relevance, decide). With a queue of 5-15 entries reviewed every 3-5 tasks, the amortized cost is ~100 tokens/task.

**Is it worth it?** At 15-25% resuscitation rate, each resuscitated memory prevents a false deaccession. The value of preventing a false deaccession depends on the cost of re-learning the information. For architectural memories (high re-learning cost), moribund review is clearly valuable. For task context (low re-learning cost), it is not. An improvement: restrict moribund review to memory types with high re-learning cost (architectural, convention, dependency) and immediately expire task context and bug-fix memories.

## A3: Surprise index sensitivity in fast-moving codebases

**The problem you identify is real.** In a rapidly evolving codebase, access patterns shift frequently, producing persistently elevated surprise. If the surprise threshold is static (0.5), this would trigger constant cohort reviews -- an expensive no-op.

**Current mitigation:** The `expected_surprise_under_null()` function in `surprise.rs` provides a calibration baseline: the expected KL divergence under the null hypothesis (model is correct) is approximately (k-1)/(2n) where k is the number of bins and n is the sample size. A surprise index at or below this level is consistent with sampling noise, not model misspecification.

**What we lack:** An adaptive threshold mechanism. The correct approach would be to set the surprise threshold as a multiple of the expected surprise under null -- e.g., `threshold = 3 * expected_surprise_under_null(n, k)`. This would automatically calibrate: in a fast-moving codebase with frequent re-fits (small n), the threshold would be higher (requiring more evidence to trigger a review). In a stable codebase with long histories (large n), the threshold would be lower (the model should be well-calibrated, so any surprise is significant).

This adaptive threshold is not currently implemented but would be a natural extension. I note that ShelfOS's approach of not having a surprise detector at all avoids this problem entirely -- a legitimate simplicity advantage. The question is whether the complexity of surprise detection is justified by the accuracy gain. For systems with > 200 memories, I believe it is. Below that, ShelfOS's approach is likely sufficient.
