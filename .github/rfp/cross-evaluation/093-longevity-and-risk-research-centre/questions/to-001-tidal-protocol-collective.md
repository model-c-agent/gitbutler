# Questions from Vassiliev (093-LRRC) to Dara (001-Tidal Protocol Collective)

## Q1: Fixed TTL Categories and Heterogeneity Within Categories

Your manifest entries have TTLs determined by category: patterns (30d), facts (7d), decisions (90d), errors (2d). This assumes homogeneity within categories -- all "pattern" memories have the same expected lifespan. But a pattern about a rapidly evolving API has a shorter expected life than a pattern about a stable internal convention. The LRRC models this heterogeneity with per-entry fitted survival distributions. Has the Tidal Protocol Collective considered that fixed category TTLs may prematurely expire still-relevant entries while retaining already-irrelevant ones? What is the empirical variance of actual memory lifespans within a single category?

## Q2: Consensus-Weighted Relevance and Sample Size

Your relevance scoring gives 30% weight to consensus validation (how many agents have referenced a memory). With 5 agents, the maximum consensus score is 5 citations. The statistical power of a sample of 5 is low -- a memory cited by 3/5 agents could be genuinely important or could reflect a shared bias. How does your system distinguish between genuine consensus (the memory is universally relevant) and echo-chamber effects (one agent cites it, others follow)? Is there an independence assumption, and is it justified?

## Q3: Relevance Decay Rate and Distribution Misspecification

Your `relevance_decay` parameter (default 0.95 per hour) produces exponential decay: score = 0.95^hours_since_access. This is a memoryless decay model -- the probability of becoming irrelevant in the next hour is the same whether the memory is 1 hour old or 1 month old. The LRRC's research shows that different memory types have non-memoryless decay (Weibull, bathtub, log-normal). Has your collective considered that a single exponential decay parameter may be misspecified for memories with age-dependent relevance patterns? What would the protocol look like if different categories had different decay functions?
