# PROPOSAL.md — Open Stacks Alliance

*"Knowledge should be findable. Code should be accessible. Both need a catalog."*

---

## Summary

The Open Stacks Alliance proposes to build the `but-ai` plugin as a library system. Memory is a catalog. Tasks are reference requests. Patches are the answers. The Alliance's expertise in making knowledge findable, accessible, and organized translates directly to the problem of agent memory: how do you store what the agent knows so that it can be found when needed?

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` on PATH. Open, discoverable, accessible — like a library on a street corner. The manifest is the library's signage: what services are available, when they are available, how to use them. TOML format, because TOML is the configuration equivalent of clear signage. No barriers to entry.

### Requirement 2: Provider-Agnostic AI

The Alliance does not discriminate by publisher. The `Source` trait: `query(prompt) -> Completion`, `index(completion) -> ToolCalls`, `circulate(usage) -> TokenReport`. Four provider adapters. The Alliance adds a provider health check modeled on inter-library loan reliability: each provider is rated on response quality, latency, and tool-calling accuracy. Unreliable providers are deprioritized in selection, like suppliers with poor fill rates.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is a reference transaction:

1. **Reference interview** — Miriam clarifies the patron's need (task decomposition)
2. **Catalog search** — Kwame retrieves relevant memory using library search techniques
3. **Source evaluation** — Fatima validates the approach against available evidence
4. **Answer preparation** — Tomas produces INDEX.patch + COMMIT.msg
5. **Accuracy check** — Fatima verifies the answer matches the question
6. **Delivery** — Miriam approves, signs, delivers

The reference interview step is unique to the Alliance: before beginning work, Miriam reformulates the task as a reference question. "Add token refresh" becomes "How should the auth module implement token refresh given the existing JWT validation pattern?" This reformulation is stored in memory and guides retrieval.

### Requirement 4: Polyrepo PR Coordination

Cross-repo coordination follows the inter-library loan (ILL) protocol:

1. **Request** — Kwame posts a PR comment: "Requesting auth-module from repo-auth for use in repo-backend"
2. **Verify** — The lending repo confirms availability (branch is stable, no conflicts)
3. **Ship** — The artifact is referenced via branch name and commit SHA
4. **Acknowledge** — The receiving repo confirms receipt and integration

```
[OSA-ILL] type: request | item: auth-module | from: repo-auth | to: repo-backend
status: requested | due: next-cycle | ref: sha256:abc123
```

Forge-agnostic: `Library` trait for GitHub/GitLab/Gitea: `request_loan(item)`, `fulfill_loan(item, ref)`, `acknowledge_receipt(loan_id)`.

### Requirement 5: Agent Memory in Git Branches

Memory is a catalog, stored in `refs/osa/catalog/`:

```json
{
  "catalog_id": "OSA-2026-0847",
  "subject_headings": ["authentication", "JWT", "middleware"],
  "title": "JWT validation centralized in auth middleware",
  "abstract": "Validation moved from handlers to single middleware after double-validation issue",
  "author": "tomas",
  "date_cataloged": "2026-03-28",
  "cross_references": ["OSA-2026-0823", "OSA-2026-0810"],
  "circulation": 3,
  "status": "available",
  "ttl_days": 60
}
```

**The catalog memory model:** Every memory entry is a catalog card with subject headings, cross-references, and a circulation count. Subject headings use controlled vocabulary (maintained by Fatima), ensuring that memories about the same topic use the same terms. This eliminates the "vocabulary problem" — where two agents describe the same concept with different words and cannot find each other's memories.

Cross-references link related memories, creating a navigable knowledge graph. Browsing — reading adjacent catalog entries — is a first-class retrieval method, because the Alliance has learned that the most useful memory is often not the one you searched for but the one next to it on the shelf.

**Unique memory feature:** Circulation tracking. Every time a memory is retrieved and used (cited in a patch or used in a decision), its `circulation` count increments. High-circulation memories are maintained (TTL extended). Zero-circulation memories are candidates for weeding (deletion after TTL expiry). This mirrors library collection management: books that circulate stay on the shelf. Books that gather dust are weeded to make room for books that will be used.

### Requirement 6: Signed Commits via OpenWallet

Miriam signs by consensus. Before signing, she confirms that the team agrees the output is ready. The signing key is bound to her DID via OpenWallet. Key rotation: annually. Emergency rotation: by team consensus, within 24 hours.

The Alliance considers signed commits to be a form of institutional accountability — the signature says "this output was reviewed and approved by the Alliance," not just "this output was produced by an agent."

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Miriam Okonkwo | 6,000 | 2,000 | 8,000 | Coordination, signing |
| Tomas Guerrero | 9,000 | 6,500 | 15,500 | Patch generation |
| Fatima Al-Rashidi | 6,500 | 2,500 | 9,000 | Review, accuracy |
| Kwame Asante | 6,000 | 2,500 | 8,500 | Memory, coordination |
| **Team Total** | **27,500** | **13,500** | **41,000** | |

Reference transaction overhead: ~3,000 tokens (interview, catalog search, ILL).
**Total per task: ~44,000 tokens.**

---

## Unique Insight

**Controlled vocabulary eliminates the memory retrieval vocabulary problem.** When two agents independently store memories about the same concept — one uses "authentication," the other uses "auth," a third uses "identity verification" — subsequent retrieval will miss memories stored under variant terms. The Alliance solves this with controlled vocabulary: a maintained list of canonical subject headings (like the Library of Congress Subject Headings, but for the codebase). Every memory is cataloged using terms from the controlled vocabulary. Every retrieval query is mapped to the controlled vocabulary before search. The result: zero vocabulary mismatch, complete recall. The overhead is a one-time vocabulary creation step and a per-memory cataloging step (~50 tokens per memory). The payoff: every relevant memory is found, every time.

---

*"The stacks are open. Everyone is welcome."*
