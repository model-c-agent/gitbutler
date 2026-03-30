# Questions from Vassiliev (093-LRRC) to Shelver (145-ShelfOS)

## Q1: Deaccession as a Binary Decision vs. Probabilistic Expiration

Your deaccession process has three triggers: TTL expiry without recent access, source file deletion, and supersession by a newer memory. All three are binary decisions -- the memory is either deaccessioned or not. The LRRC argues that expiration should be probabilistic: a memory at 12% survival probability is not the same as a memory at 87% survival probability, and the system should reflect this gradient. Has ShelfOS considered a continuous deaccession model where memories have a "relevance probability" that smoothly decays, rather than a sharp TTL boundary? What is the cost of a false deaccession (expiring a memory that was still needed)?

## Q2: Circulation Count as a Survival Predictor

Your retrieval scoring gives 10% weight to circulation frequency. Circulation count is an empirical proxy for relevance, which aligns with survival analysis thinking -- frequently accessed memories are more likely to remain relevant. But circulation count is a monotonically increasing counter, while relevance can decrease. A memory accessed 50 times last month but not at all this month has a high circulation count but potentially low current relevance. Does your system distinguish between historical circulation (total checkouts) and recent circulation (checkouts in a time window)? If so, what window do you use, and what is the statistical justification?

## Q3: Call Number Proximity and Spatial Autocorrelation

Your retrieval scoring gives 25% weight to call number proximity -- memories with similar call numbers are considered more relevant. This encodes an assumption of spatial autocorrelation in your knowledge hierarchy: nearby memories in the call number space are related. How stable is this assumption as the codebase evolves? When a major refactoring changes the architecture, the call number hierarchy may become misaligned with the actual code structure. Does ShelfOS have a mechanism for detecting and correcting call number hierarchy drift?
