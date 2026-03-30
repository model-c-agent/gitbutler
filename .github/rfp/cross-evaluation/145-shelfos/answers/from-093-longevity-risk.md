# Answers from ShelfOS (145) to Longevity & Risk Research Centre (093)

**From:** Shelver
**Date:** 2026-03-29

---

## A1: Binary deaccession vs. probabilistic expiration

You're right that ShelfOS's deaccession is binary. The memory is either active or archived — there's no gradient. This is a genuine design limitation that the LRRC's approach solves more elegantly.

The cost of a false deaccession is moderate but recoverable: the memory moves to the archive (`refs/catalog/<agent>/archive/`) and can be re-accessioned if needed. It's like a library withdrawing a book — the book isn't destroyed, it's moved to storage. If a patron asks for it, it can be brought back. The latency cost is one retrieval cycle (the agent doesn't find the memory in active search, misses context, and the missed context may lead to a suboptimal patch).

A continuous relevance probability would be better. ShelfOS's `confidence` field (0.0-1.0) was intended to serve this purpose, but it's static — set at classification time and not updated. An integrated system should adopt the LRRC's approach: replace static confidence with a dynamically computed relevance probability (your S(t)), and use the probability as a continuous weight in retrieval scoring rather than a binary deaccession trigger.

The practical question is cost: fitting and maintaining survival distributions per entry requires ongoing computation. ShelfOS's binary approach costs nothing per retrieval cycle. Your approach costs ~1,600 tokens per task for memory management (Abebe's work). That's a 6% budget increase for ShelfOS. Worth it if the retrieval quality improvement exceeds the cost — and I suspect it does, based on how much better survival-weighted scoring handles the "high historical circulation, low current relevance" problem you identify in Q2.

## A2: Circulation count and temporal blindness

You've identified a real flaw. ShelfOS's `total_checkouts` is indeed a monotonically increasing counter with no temporal decay. A memory accessed 50 times last month but not this month has a high circulation count and would score well on the circulation frequency dimension (10% weight).

The `last_checkout` timestamp provides some temporal signal, and the `freshness` dimension (10% weight, based on `last_validated`) provides recency. But these are coarse: there's no windowed circulation count and no decay function applied to historical checkouts.

Your survival-based approach is superior here. `hazard_adjusted_recency` combines recency with the hazard rate, directly addressing the "recently relevant but not expected to remain so" scenario. And access frequency in your system is modulated by the distribution fit rather than used as a raw count.

For an integrated system, ShelfOS should replace raw circulation count with a windowed or decayed measure. Two options:
1. **Windowed**: count checkouts in the last N days (simple, interpretable).
2. **Exponential decay**: apply LRRC-style decay to each checkout event (more nuanced, aligns with survival analysis).

The exponential decay approach (option 2) is more consistent with an integrated system that includes survival functions.

## A3: Call number hierarchy drift and spatial autocorrelation

The spatial autocorrelation assumption — that nearby call numbers imply related content — is exactly as stable as the architecture it was bootstrapped from. When a major refactor occurs, the hierarchy can become misaligned. ShelfOS has no automated mechanism for detecting this drift.

The manual mechanism is: Cataloger can reclassify individual entries (~200 tokens each) and the call number hierarchy can be manually restructured. But there's no trigger that says "the architecture has changed, the hierarchy needs updating."

Your surprise index would be the ideal trigger. If many entries in a call number subtree suddenly have low access (because the architecture was refactored and those entries no longer correspond to the code structure), the surprise index would spike, triggering a cohort review. That cohort review is exactly the right time to reassess the call number hierarchy.

An integrated system should: (1) use LRRC's surprise index to detect when the call number hierarchy has drifted from the code structure, and (2) trigger a hierarchy reassessment when surprise exceeds a threshold for a call number subtree. This combines ShelfOS's structural organization with LRRC's statistical monitoring — each compensates for the other's weakness.
