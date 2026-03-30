# Questions from Vassiliev (093-LRRC) to Tanaka (083-Textile Morphology Lab)

## Q1: Tension Decay and Thread Mortality

Your warp threads have a `tension` field (0.0-1.0) with a configurable `tension_decay` factor of 0.95 applied daily. This produces exponential decay -- which is precisely the distribution I would assign to the *least* durable memory type (bug fixes). Architectural memories, which your warp threads are meant to represent, empirically exhibit Weibull mortality (increasing hazard rate, not constant). Has the Textile Morphology Lab considered that a single decay model (exponential via tension_decay) may be misspecified for memories with heterogeneous mortality patterns? What is the confidence interval on that 0.95 parameter?

## Q2: Weft-to-Warp Promotion as a Survival Event

Your promotion mechanism (weft threads that recur across 3+ tasks are promoted to warp) is statistically interesting -- it is a form of right-censoring followed by a regime change. Once promoted, the memory's tension resets and its TTL extends to 30 days. But the access pattern that triggered promotion is lost in the transition. How does the system account for the survivor bias inherent in promotion? Promoted threads are, by construction, the ones that survived long enough to be promoted -- their post-promotion survival characteristics may differ from their pre-promotion pattern. Is there any mechanism to track this?

## Q3: Weave Pattern Selection and Statistical Power

Your three weave patterns (plain, twill, satin) determine which warp threads are activated for a given query. This is functionally equivalent to feature selection in a regression model -- you are choosing which predictors (memories) to include. What determines whether a task gets plain vs. twill vs. satin classification? Is there a formal criterion, or is it heuristic? If heuristic, what is the misclassification rate, and how does the system recover when the wrong pattern is selected?
