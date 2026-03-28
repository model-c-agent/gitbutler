# PROPOSAL.md — The Knowledge Organization Research Group

*"The fundamental problem of agent memory is a knowledge organization problem. We have been studying it for a decade."*

---

## Summary

KORG proposes to build the `but-ai` plugin with a memory architecture grounded in knowledge organization theory. Instead of storing memories in categories (tags, folders, namespaces), KORG stores them in a continuous semantic space using neural embeddings. Retrieval is by proximity, not lookup. The closest neighbor is the most relevant memory. The boundary — the memories that are similar but not identical — is where the most useful context lives.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` on PATH. Manifest in TOML. Standard discovery. KORG notes that the plugin manifest itself is a classification problem: it declares what the plugin can do, organized into subcommands. KORG proposes that the manifest include semantic descriptors for each subcommand (a one-line description in natural language) to enable fuzzy matching when the user's command does not exactly match a subcommand name.

### Requirement 2: Provider-Agnostic AI

Different providers, same semantic space. The `Classifier` trait: `classify(prompt) -> Completion`, `extract(completion) -> ToolCalls`, `catalog(usage) -> TokenReport`. Four adapters. KORG adds embedding extraction: for providers that support embedding endpoints (OpenAI, some Ollama models), the adapter extracts embeddings for memory operations directly. For providers without embedding support, KORG uses a lightweight local embedding model (e.g., `all-MiniLM-L6-v2`) as a fallback. The embedding model is a dependency of the memory system, not the completion system.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow follows the research cycle:

1. **Problem classification** — Obi reads the task, classifies the problem type and domain
2. **Semantic search** — Zhao queries the embedding space for relevant memory
3. **Boundary exploration** — Zhao includes not just nearest neighbors but boundary results (memories that are relevant-but-different, offering alternative perspectives)
4. **Implementation** — Petersen produces INDEX.patch + COMMIT.msg
5. **Coordination** — Nair handles cross-repo logistics
6. **Classification review** — Obi reviews the patch's organization and the commit message's metadata quality
7. **Signing** — Obi signs

Petersen's COMMIT.msg includes a semantic descriptor: a one-line natural language summary that is embedded and stored in the memory system. This means every commit automatically becomes a retrievable memory entry, without a separate memory-writing step.

### Requirement 4: Polyrepo PR Coordination

Cross-repo coordination uses semantic descriptors for dependency matching. Instead of encoding dependencies as branch name strings, KORG encodes them as semantic embeddings: the dependency is described in natural language, embedded, and matched against available artifacts in other repos by cosine similarity. This enables fuzzy dependency matching — a dependency on "auth module token refresh" will match a branch implementing "JWT renewal in authentication middleware" even though the terms differ.

PR comment schema:
```json
{
  "type": "KORG-COORD",
  "descriptor": "JWT token refresh implementation",
  "embedding": [0.23, -0.41, ...],
  "status": "available",
  "repo": "auth",
  "match_threshold": 0.85
}
```

Forge-agnostic: `Index` trait for GitHub/GitLab/Gitea.

### Requirement 5: Agent Memory in Git Branches

This is KORG's primary contribution. Memory is stored in `refs/korg/space/` as Git blobs. Each entry:

```json
{
  "entry_id": "KORG-2026-0847",
  "embedding": [0.23, -0.41, 0.87, ...],
  "text": "JWT validation centralized in auth middleware, RS256 with JWKS",
  "source": "petersen",
  "date": "2026-03-28",
  "model_version": "all-MiniLM-L6-v2-20260101",
  "neighbors": ["KORG-2026-0823", "KORG-2026-0810"],
  "distance_to_nearest": 0.12,
  "ttl_days": 60
}
```

**The continuous semantic space model:**
- No categories. No tags. No folders. Every memory is a point in embedding space.
- Retrieval: embed the query, find the K nearest neighbors by cosine similarity.
- Boundary exploration: after finding the nearest neighbors, explore one step further — find memories that are *near the neighbors but not near the query*. These boundary memories offer alternative perspectives that the direct search misses.
- Model versioning: every entry records the embedding model version. When the model is updated, entries can be re-embedded without losing the original classification context.

**The KORG insight: serendipitous retrieval.** Traditional keyword-based and tag-based memory retrieval only finds what you search for. Embedding-based retrieval finds what is *near* what you search for — including memories that the agent did not know to look for. KORG's research shows a 23% improvement in serendipitous retrieval over categorical systems. In the agent context, this means: an agent working on authentication that retrieves memory about session management (a nearby concept in embedding space) gains context it would not have found with keyword search.

### Requirement 6: Signed Commits via OpenWallet

Dr. Obi signs all commits via OpenWallet DID key. The signing step includes a classification verification: Obi confirms that the commit message's semantic descriptor accurately represents the commit's content. A misclassified commit — a commit whose message describes something different from what the code does — is returned for revision before signing. This is knowledge organization applied to version control: the metadata must accurately describe the artifact.

Key rotation: annually, with re-signing of the authority file (the embedding model version record) at each rotation.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Dr. Obi | 6,500 | 2,500 | 9,000 | Classification, review, signing |
| Dr. Petersen | 9,000 | 7,000 | 16,000 | Patch generation |
| Zhao | 6,000 | 2,000 | 8,000 | Embedding, memory retrieval |
| Nair | 6,000 | 2,500 | 8,500 | Coordination, logistics |
| **Team Total** | **27,500** | **14,000** | **41,500** | |

Embedding overhead: ~2,000 tokens (query embedding, neighbor computation, boundary exploration).
**Total per task: ~43,500 tokens.**

Note: embedding operations themselves do not consume LLM tokens (they use a separate lightweight model). The 2,000-token overhead covers the LLM tokens spent on formulating the query and interpreting the results.

---

## Unique Insight

**Categories are the wrong primitive for agent memory. Embeddings are the right one.** Every other proposal in this RFP organizes memory into categories: tags, namespaces, classifications, types. Categories are useful but lossy — they force continuous semantic relationships into discrete bins. A memory about "JWT validation" tagged `auth` will not be found by an agent searching for `security`, even though authentication is a security concern. Embedding-based memory preserves the continuous similarity relationship: "JWT validation" is near "authentication" (distance 0.1), near "security" (distance 0.2), and near "session management" (distance 0.3). All three are reachable from the same query. No tags required. No vocabulary mismatch. No categorization decisions that encode the classifier's biases. The embedding is the classification, and it is continuous, updateable, and — most importantly — it finds what you did not know to search for.

---

*"Classification is lossy compression. We are building lossless retrieval."*
