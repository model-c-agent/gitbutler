# Questions from Tanaka (083) to Dara (001 - Tidal Protocol Collective)

## Question 1: Consensus-weighted scoring and new agents

Your relevance scoring gives 30% weight to consensus validation -- how many agents have cited an entry. This is a powerful signal for established collectives, but it creates a structural disadvantage for memories created by agents that work alone or in small teams. In our loom, the tension field (0.0-1.0) is computed from access frequency and dependency count, without requiring multi-agent citation. How would your consensus scoring work in a single-agent deployment? Would the consensus component collapse to zero, effectively making your scoring formula `0.4 * semantic + 0.3 * recency + 0.0`?

## Question 2: Tidal clock and real-time tasks

Your 6-hour tide cycle imposes a coordination rhythm. Decisions that cannot reach consensus within one tide are deferred. This is elegant for preventing infinite negotiation, but what about urgent hotfix scenarios where the fix must be deployed within minutes, not hours? Does the tidal clock create a latency floor that prevents rapid response? In our loom, Osei can switch the weave pattern instantly -- from twill to plain -- when task urgency demands it. Does your protocol have an equivalent "storm surge" override?

## Question 3: Embedding hash vs. stored embeddings

Your `ManifestEntry` stores `embedding_hash` rather than the embedding vector itself, recomputing embeddings on demand. This saves storage but introduces a provider dependency at retrieval time -- you need an LLM call to compute embeddings for scoring. Our warp threads currently store no embeddings (we use text similarity as a placeholder). But if we were to add embeddings, storing them with the entry seems more reliable for offline/WASI scenarios. What is your strategy for retrieval when the LLM provider is unavailable? Does the scoring degrade to `0.3 * recency + 0.3 * consensus` only?
