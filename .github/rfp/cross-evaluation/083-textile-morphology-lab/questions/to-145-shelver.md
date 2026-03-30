# Questions from Tanaka (083) to Shelver (145 - ShelfOS)

## Question 1: Call number hierarchy generation

Your `CallNumber` struct uses a hierarchical scheme (ARCH.AUTH.MIDDLEWARE.TOKEN-VALIDATION) that is "auto-generated from the codebase structure and refined by Cataloger." How does this auto-generation work concretely? Does Cataloger use the directory structure as a starting point (e.g., `src/auth/middleware.rs` -> ARCH.AUTH.MIDDLEWARE)? Or does it infer the hierarchy from the semantic content of memories? In our loom, we classify threads by `ThreadColor` (structural, convention, preference, learned, observation, plan, coordination) -- a flat taxonomy. Your hierarchical approach is richer but seems harder to bootstrap. What happens when the auto-generated hierarchy is wrong? How expensive is reclassification?

## Question 2: "See also" graph scalability

Your `SeeAlsoGraph` caps links at `max_see_also` (default 5) per item. This prevents the graph from becoming a complete graph, which is good. But with 5 links per item and hundreds of memories, the graph still becomes large. Your `traverse` method does BFS up to `max_hops` hops. For a graph with 500 items and 5 links each, a 2-hop traversal from a popular node could touch 25+ items. Have you benchmarked the retrieval cost of graph traversal at scale? And more importantly: when you score items by `see_also_distance` (20% weight), how do you prevent popular hub nodes from dominating retrieval results?

## Question 3: Phase-gated tool loading and token savings

You load different tools per lifecycle phase (3-5 tools instead of 10), saving 800-1,200 tokens. This is clever. But it means the agent cannot use a SHELVE-phase tool during the CLASSIFY phase. What if the Cataloger discovers during classification that a branch needs to be created (a SHELVE-phase tool) to properly organize the work? Does the agent need to restart in a different phase, or do you have a mechanism for requesting out-of-phase tool access? In our loom, all tools are available to all agents at all times, which costs more tokens but avoids phase boundary friction.
