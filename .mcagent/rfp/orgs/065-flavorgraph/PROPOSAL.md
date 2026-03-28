# FlavorGraph — Technical Proposal for `but-ai`

**RFP Response | Tier 2 Abbreviated**

---

## Executive Summary

FlavorGraph proposes a `but-ai` implementation built on graph-aware development: agents that understand code as a dependency graph, generate patches that account for transitive impacts, and store memory using graph embeddings that capture structural relationships. Our domain expertise in predicting novel flavor pairings through graph neural networks translates to agents that navigate code-space the way we navigate flavor-space — by understanding connections.

## Requirement 1: PATH-Based Plugin Architecture

Rust binary at `$PATH` as `but-tool-ai`. The binary includes a `--graph` subcommand that outputs the dependency graph of the current project (or a specified scope) as a JSON adjacency list. This allows human operators and external tools to visualize the code structure that agents use for decision-making.

Configuration via `$XDG_CONFIG_HOME/but-ai/config.toml`. FlavorGraph adds a `[graph]` section with configurable graph depth (how many transitive dependency levels to explore), edge weight threshold (minimum dependency strength to include), and graph cache TTL (how long the precomputed graph is valid).

## Requirement 2: Provider-Agnostic LLM Interface

Four-provider backend. Edge manages provider selection by modeling the provider ecosystem as a routing graph. Each provider is a node with capacity (token limits), cost (per-token pricing), and latency (measured RTT). Edge routes requests through the graph to minimize a configurable objective: cost, latency, or a weighted combination.

Provider comparison uses A/B evaluation on a standard test prompt, refreshed weekly. New providers enter the graph as low-priority nodes and earn routing priority through demonstrated performance.

**Domain Insight:** In FlavorGraph's research, we learned that the most interesting pairings are not between the most popular ingredients — they are between ingredients that are structurally close in the graph but culinarily distant. Edge applies this insight to provider selection: sometimes the best provider for a task is not the obvious choice (the most popular, the most expensive) but the one whose capabilities are structurally aligned with the task's requirements. A local Ollama model running a code-specialized model may outperform a frontier API for routine refactoring.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Traverse generates patches through a graph-aware process:
1. **Map** — Build the dependency graph for the files in scope. Identify all nodes (functions, types, modules) and edges (imports, calls, references) within the configured depth.
2. **Analyze** — Identify high-connectivity nodes (functions called from many places) and classify the change's risk level based on graph position. Changes to high-connectivity nodes are flagged for extra care.
3. **Generate** — Produce INDEX.patch. For high-risk changes (affecting high-connectivity nodes), Traverse includes additional hunks that update callers/dependents.
4. **Validate** — `git apply --check`, plus a graph-level check: are all transitively affected nodes either updated in the patch or unchanged in behavior?

COMMIT.msg:
```
feat(graph): add compound co-occurrence scoring

Graph depth: 3 | Nodes affected: 7 | Risk: medium (2 high-connectivity)
Agent: Traverse | Embedding-refs: embed/vec-2026-0211
Pairing: smoked paprika + white peach (87% confidence)
```

## Requirement 4: Polyrepo PR Coordination

Bridge manages cross-repo coordination using graph theory. The coordination set is modeled as a dependency graph where PRs are nodes and merge dependencies are edges. Bridge identifies:
- **Bridges** — PRs whose failure disconnects the coordination graph. These are critical-path items.
- **Leaves** — PRs with no dependents. These can be merged independently.
- **Cycles** — Circular dependencies. Bridge flags these as architectural problems.

Coordination state stored in `refs/flavorgraph/coord/`. Forge adapters (GitHub, GitLab, Gitea) implement a `ForgeAdapter` trait. PR descriptions include a rendered dependency graph.

## Requirement 5: Agent Memory in Git Branches

Embed manages memory using graph embeddings. Each memory entry is stored with both its content and a vector embedding that captures its structural position in the code graph:

| Field | Description |
|-------|-------------|
| `id` | Unique identifier |
| `content` | Memory payload |
| `embedding` | 128-dimensional vector |
| `graph_context` | Which nodes/modules this memory relates to |
| `structural_neighbors` | Other memories close in embedding space |
| `ttl_days` | 30 default, +7 per reuse (max 90) |

Memory stored in `refs/flavorgraph/embeddings/`. Retrieval uses cosine similarity in embedding space, which means structurally related memories surface even if they are semantically different. A memory about "authentication middleware" and a memory about "session management" will be close in embedding space if they involve the same code modules, even though the topics are distinct.

This is FlavorGraph's key memory innovation: structural similarity over semantic similarity. In flavor science, strawberry and basil are semantically unrelated but structurally connected (shared compounds). In codebases, two tasks may be semantically unrelated but structurally connected (shared modules).

## Requirement 6: Signed Commits via OpenWallet

Signature handles signing with a trust graph. Each agent is a node. Successful collaborative tasks create edges between agents. The trust graph grows over time, and agents with higher connectivity (more successful collaborations) are trusted for increasingly complex tasks.

Signing metadata includes the graph risk level of the change (how many high-connectivity nodes were affected). This gives human reviewers a quick signal: a change affecting only leaf nodes is low-risk; a change affecting five high-connectivity nodes needs careful review.

Key rotation every 30 days.

## Token Budget

| Agent | Role | Input/task | Output/task | Total |
|-------|------|-----------|-------------|-------|
| Traverse | Graph-aware patches | 9,000 | 5,000 | 14,000 |
| Embed | Structural memory | 5,500 | 1,000 | 6,500 |
| Edge | Provider graph routing | 3,000 | 700 | 3,700 |
| Bridge | Critical-path coordination | 5,000 | 2,000 | 7,000 |
| Signature | Trust graph signing | 2,500 | 500 | 3,000 |
| **Per-task total** | | **25,000** | **9,200** | **34,200** |

Traverse has the highest individual budget because graph-aware patch generation requires reading transitive dependencies (more input tokens than a non-graph approach). This is the cost of knowing what your change touches three layers deep.

## Unique Domain Insight

Three years of building flavor prediction models taught us that the most valuable knowledge is relational, not categorical. Knowing that basil is an herb is categorical knowledge — it tells you what basil IS. Knowing that basil shares linalool with strawberry is relational knowledge — it tells you what basil CONNECTS TO. The relational knowledge is what enables novel pairings.

Our proposal applies this to agent memory and patch generation. Most agent systems store categorical knowledge: "this function does X." FlavorGraph stores relational knowledge: "this function is connected to these other functions through these interfaces." When an agent needs to modify a function, it does not just know what the function does — it knows what the function connects to, which connections are strong (frequently exercised), and which connections are weak (rarely tested). This graph-level awareness is what prevents the transitive regressions that plague agent-generated code.

---

*Pairing predicted: smoked paprika + white peach. Confidence: 87%. The graph does not lie.*
