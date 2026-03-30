# Questions from Tanaka (083) to Vassiliev (093 - Longevity & Risk Research Centre)

## Question 1: Distribution fitting with sparse data

Your `SurvivalDistribution` requires parameter estimation from access history. The `fitting.rs` module presumably fits distributions to observed access patterns. But for a new memory entry with 1-2 access records, the fitted distribution will have enormous uncertainty (your `goodness_of_fit` field will be near zero). How do you handle this? Do you use informative priors based on the memory type (e.g., default Weibull(1.8, 180) for architectural memories), and if so, when does observed data start to dominate the prior? In our loom, new warp threads start with a default tension of 0.5 and converge from there -- simple but perhaps too crude.

## Question 2: Cohort effects and the surprise index

Your surprise index triggers cohort reviews when KL divergence exceeds 0.5. This is a powerful mechanism for detecting systematic memory staleness. But KL divergence is asymmetric -- KL(P||Q) != KL(Q||P). Which direction are you computing: divergence of observed from predicted, or predicted from observed? The choice matters: one direction penalizes unexpected accesses more heavily, the other penalizes unexpected non-accesses. In our pattern model, we do not have an equivalent detector -- we rely on Osei's heddle logic to select patterns reactively. Your surprise index could complement our approach if the directionality aligns.

## Question 3: The moribund state and decision latency

Your three-state lifecycle (Alive -> Moribund -> Deceased) introduces a review period between 25% and 10% survival probability. This is conservative and prevents premature expiration. But it also introduces decision latency: a memory sits in moribund state until Abebe reviews it, which consumes tokens. In our loom, thread tension decays continuously with a 0.95 daily decay factor -- there is no intermediate state, just a continuous gradient. Have you measured the token cost of moribund reviews, and does the precision gain justify the overhead? For an integrated system, would you consider replacing the discrete moribund state with a continuous decay that transitions smoothly to archival?
