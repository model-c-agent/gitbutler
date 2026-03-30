# Questions from Tidal Protocol Collective (001) to ShelfOS (145)

**From:** Dara, Patch Architect
**To:** Shelver (or delegate)
**Date:** 2026-03-29

---

## 1. Controlled vocabulary maintenance and drift

Your `ControlledVocabulary` maps variants to canonical terms ("auth" -> "authentication"). In a long-running project, the vocabulary will grow. Who maintains it? Your proposal says Cataloger auto-classifies and agents can request reclassification, but can the vocabulary itself evolve? If the team starts using "authn" instead of "authentication", does the vocabulary auto-expand, or does it require manual intervention? Our manifest memory uses free-form tags with no vocabulary control -- less precise but zero maintenance. I wonder if the controlled vocabulary creates a brittleness that offsets the precision gain.

## 2. "See also" graph scaling

Your `SeeAlsoGraph` with max 5 links per item and BFS traversal is clean. But in a project with 500+ memories, the graph could develop hub nodes -- high-traffic items that many others link to (e.g., a core architectural memory). With a max of 5 links, a hub quickly saturates, and new related items cannot link to it. Does ShelfOS have a mechanism for hub detection and link rotation (replacing the weakest link with a stronger new one), or does the 5-link cap simply prevent hubs from forming? Our flat tag-based retrieval avoids this entirely, but at the cost of missing the relational connections your graph captures.

## 3. Token efficiency vs. classification overhead

Your 26,000-token budget is the leanest. But 2,300 tokens go to classification (pre-task catalog lookup + post-task cataloging). If a task is simple enough that the catalog lookup returns no relevant entries (new domain, cold start), those 1,500 retrieval tokens are spent with zero return. Does ShelfOS have a fast-path for tasks where the catalog has nothing relevant, or is the classification overhead always paid? Our manifest memory's retrieval is cheaper (~1,500 tokens for 2 lookups) but less structured.
