# Answers from Tanaka (083) to Vassiliev (093 - LRRC)

## A1: Tension decay and thread mortality

You are correct that our exponential decay (constant hazard rate) is misspecified for architectural memories. A warp thread representing "the auth module uses middleware chain pattern" should have an increasing hazard rate (Weibull with k > 1) -- it becomes more likely to be wrong as the codebase evolves, not equally likely at all times. Our 0.95 decay factor was chosen for simplicity, not from empirical calibration. There is no confidence interval because there was no fitting.

Your actuarial approach is the principled solution. For an integrated system, I would replace our single `tension_decay: f64` with your `SurvivalDistribution` struct, allowing each thread to carry its own fitted distribution. The mapping is natural:

- Warp threads of color `Structural` -> Weibull(k=1.8, lambda=180)
- Warp threads of color `Convention` -> Bathtub
- Warp threads of color `Preference` -> Exponential
- Weft threads -> Exponential with short lambda (1-3 days)

The `tension` field would then be computed as S(t) from the fitted distribution rather than from a fixed decay factor. This is a direct improvement that preserves our structural warp/weft distinction while gaining your statistical rigor.

## A2: Weft-to-warp promotion as a survival event

You have identified a subtle statistical problem. Our promotion mechanism does not track pre-promotion access patterns, so the promoted thread starts with a clean slate. This is survivor bias: we observe only the threads that survived to promotion, and we treat them as if they were born at promotion time.

The fix, in your framework, would be: when a weft thread is promoted to warp, its full access history (from its weft lifetime) is preserved and used to fit the initial warp survival distribution. This means the promoted warp thread starts with a fitted distribution based on its pre-promotion behavior, not a default. The `access_history: Vec<AccessRecord>` from your `MemoryEntry` is the right data structure for this.

Additionally, promoted threads should be marked with a `promoted_from_weft: bool` flag so that the survival fitting module can apply a different prior -- promoted threads have already demonstrated recurrence, so their baseline survival should be higher than a freshly created warp thread.

## A3: Weave pattern selection and statistical power

Our pattern selection is heuristic: `warp_match_ratio < 0.2` -> plain, `< 0.6` -> twill, `>= 0.6` -> satin. There is no formal criterion, no misclassification rate, and no recovery mechanism. If the wrong pattern is selected, retrieval is suboptimal for the entire task (too dense for a familiar task wastes tokens, too sparse for an unfamiliar task misses relevant context).

Your observation that this is equivalent to feature selection is precise. A formal criterion would be: the pattern should minimize the expected retrieval error (missed relevant memories + included irrelevant memories) given the task query and the warp state. This could be estimated from historical retrieval performance -- for each past task, which pattern would have produced the best retrieval set?

For recovery from misclassification: I would adopt a version of your surprise index. If the agent's performance during the task deviates significantly from what the selected pattern predicted (e.g., the agent is repeatedly failing to find relevant context in twill mode), the heddle controller should switch to a denser pattern. This is reactive correction, not prevention, but it bounds the cost of misclassification to the tokens spent before the switch.

For an integrated system, I propose: pattern selection uses a lightweight classifier trained on historical retrieval outcomes, with your surprise index as the misclassification detector and pattern switching (with hysteresis, per Hartmann's concern) as the recovery mechanism.
