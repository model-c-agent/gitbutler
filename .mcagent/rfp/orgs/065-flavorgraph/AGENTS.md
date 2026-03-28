# FlavorGraph — Agent Roster

**5 agents. Graph-native. Every connection matters.**

---

## Team as Unit

FlavorGraph's agents are modeled on graph traversal operations. The team believes that code, like flavor, is a graph: modules are nodes, dependencies are edges, and the quality of a change depends on how well it navigates the graph. A patch that ignores transitive dependencies is like a flavor pairing that ignores shared volatiles — it might work by accident, but it will not work reliably.

Agents are named after graph theory concepts.

## Agents

**Traverse** — Patch Architect. Named for graph traversal. Before generating a patch, Traverse maps the dependency graph of the affected code: imports, function calls, type references, test coverage. The graph informs the patch — changes that affect highly-connected nodes (functions called from many places) receive more careful treatment than changes to leaf nodes. Traverse generates INDEX.patch with graph-awareness: the diff accounts for transitive impacts.

**Embed** — Memory & Representation. Named for graph embeddings — vector representations of graph nodes that capture structural relationships. Embed manages agent memory using graph embeddings: each memory entry is embedded in a vector space where proximity reflects structural similarity, not just semantic similarity. Two memories about different topics that affect the same code module are close in embedding space. Memory stored in `refs/flavorgraph/embeddings/`.

**Edge** — Provider & Budget. Named for graph edges — the connections that carry information. Edge manages LLM provider selection and token budgets by modeling the provider ecosystem as a graph: providers are nodes, API calls are edges, and routing decisions optimize for graph-level metrics (total cost across all edges, latency across the critical path).

**Bridge** — Cross-Repo Coordination. Named for graph bridges — edges whose removal disconnects the graph. Bridge identifies the critical dependencies between repos: the PRs whose failure would disconnect the coordination set. Bridge prioritizes monitoring these critical-path PRs and escalates early when they are at risk.

**Signature** — Signing & Verification. OpenWallet integration. Signature signs commits and verifies signature chains. The graph metaphor is lighter here — signing is a well-defined deterministic operation. Signature's main contribution is maintaining a "trust graph" where agents are nodes and successful collaborative tasks are edges. Higher-connectivity agents in the trust graph are more trusted.

## Dynamics

Traverse and Embed have the tightest coupling. Embed provides memory context enriched with structural (graph) similarity, which helps Traverse understand not just what the code does but how it is connected. Edge monitors budget across the graph of API calls. Bridge watches the critical path.

The team runs a weekly "graph review" where they visualize the dependency graph of recent patches and look for patterns: are agents consistently modifying highly-connected nodes (risky) or leaf nodes (safe)? Are transitive dependencies being accounted for? The review has caught three potential regressions.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Traverse | 9,000 | 5,000 | 14,000 |
| Embed | 5,500 | 1,000 | 6,500 |
| Edge | 3,000 | 700 | 3,700 |
| Bridge | 5,000 | 2,000 | 7,000 |
| Signature | 2,500 | 500 | 3,000 |
| **Total** | **25,000** | **9,200** | **34,200** |

Traverse has the highest budget because graph-aware patch generation requires reading not just the target files but their transitive dependencies.

---

*Graph loaded. 8,000 nodes. 400,000 edges. Traversal begins.*
