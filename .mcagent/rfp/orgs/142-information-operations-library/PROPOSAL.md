# PROPOSAL.md — Information Operations Library

**"The right information. The right format. The right time."**

---

## Summary

The Information Operations Library proposes to build the `but-ai` plugin as a military reference service. Every task is a reference transaction: a request for information that must be delivered accurately, in the correct format, within a time window. Memory is a classified, indexed, cross-referenced collection. The IOL's 18 years of delivering combat-effective knowledge under time pressure translates directly to the problem of agent-assisted code generation under token budgets.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` on PATH, discoverable via standard resolution. The IOL does not use exotic discovery mechanisms. The tool is on the shelf where it belongs. Manifest in TOML. Configuration in `.but/ai.toml`. Provider selection via flag or config file. The IOL recommends config file over environment variables for operational security.

### Requirement 2: Provider-Agnostic AI

The IOL works with multiple information sources: classified databases, open-source intelligence, academic journals, field reports. Each source has different access protocols, different reliability ratings, and different latency. The provider abstraction follows the same model: a `Source` trait: `query(prompt) -> Completion`, `evaluate(completion) -> ToolCalls`, `log(usage) -> TokenReport`. Four adapters. Each provider carries a reliability rating (computed from historical tool-calling accuracy) that influences provider selection for high-stakes tasks.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow is a reference transaction:

1. **Reference interview** — REF LEAD clarifies the actual requirement
2. **Catalog search** — REF TWO retrieves classified memory entries
3. **Response formulation** — REF ONE produces INDEX.patch + COMMIT.msg
4. **Logistics** — STACKS coordinates cross-repo dependencies
5. **Quality review** — REF LEAD verifies accuracy, completeness, format
6. **Delivery** — Signed commit

REF ONE's COMMIT.msg follows the IOL response format:
```
QUESTION: How should auth module implement token refresh?
SOURCES: src/auth/middleware.rs, src/auth/jwt.rs, memory:IOL-2026-0823
ANSWER: Added sliding window refresh with 24h cap in middleware layer
CLASSIFICATION: ROUTINE
DELIVERY: WITHIN-WINDOW
```

### Requirement 4: Polyrepo PR Coordination

STACKS handles logistics. Cross-repo coordination uses the IOL's inter-branch communication protocol — structured PR comments formatted as logistics messages:

```
[IOL-LOGISTICS] from: repo-backend | to: repo-auth | type: REQUEST
item: token-refresh-dependency | priority: ROUTINE
status: AWAITING-DELIVERY | deadline: next-cycle
```

Forge-agnostic: `Depot` trait for GitHub/GitLab/Gitea: `submit_request(request)`, `fill_request(request_id, delivery)`, `confirm_receipt(delivery_id)`.

### Requirement 5: Agent Memory in Git Branches

Memory is the IOL's collection, stored in `refs/iol/collection/`:

```json
{
  "accession_id": "IOL-2026-0847",
  "classification": "AUTH.JWT.REFRESH",
  "subject_headings": ["authentication", "jwt", "token-lifecycle"],
  "abstract": "JWT refresh uses sliding window with 24h maximum TTL",
  "reliability": "A",
  "source": "ref-one",
  "date_acquired": "2026-03-28",
  "last_circulated": "2026-03-28",
  "cross_references": ["IOL-2026-0823"],
  "ttl_days": 45
}
```

**The IOL classification scheme:** Adapted from Library of Congress Classification for codebases. Top-level classes map to system domains: `AUTH` (authentication), `DATA` (data layer), `UI` (user interface), `INFRA` (infrastructure). Subclasses map to specific components. Cross-references link related entries across classes. The scheme is maintained by REF TWO as an authority file.

**Reliability ratings:** Every memory entry carries a reliability rating (A-F):
- **A** — Confirmed by multiple sources (tested, reviewed, in production)
- **B** — Confirmed by one source (tested or reviewed)
- **C** — Unconfirmed but plausible (observed but not verified)
- **D** — Contradicted by other evidence
- **F** — Known false (retained as negative knowledge)

Retrieval prioritizes high-reliability entries but surfaces D and F entries as warnings.

### Requirement 6: Signed Commits via OpenWallet

REF LEAD signs all commits via OpenWallet DID key. The signing step is the delivery confirmation — the IOL's guarantee that the response has been reviewed and verified. Key rotation: semi-annually, with a 48-hour overlap. Emergency rotation: within 4 hours, with all transactions since the last known-good state flagged for re-verification.

---

## Token Budget

| Callsign | Input | Output | Total | Role |
|----------|-------|--------|-------|------|
| REF LEAD | 6,500 | 2,500 | 9,000 | Direction, review, signing |
| REF ONE | 9,000 | 7,000 | 16,000 | Patch generation |
| REF TWO | 6,000 | 2,000 | 8,000 | Memory, classification |
| STACKS | 6,000 | 2,500 | 8,500 | Coordination, logistics |
| **Team Total** | **27,500** | **14,000** | **41,500** | |

Reference overhead: ~3,000 tokens (interview, classification, logging).
**Total per task: ~44,500 tokens.**

---

## Unique Insight

**Memory entries need reliability ratings, not just relevance scores.** Most memory systems rank entries by relevance — how similar the entry is to the current query. The IOL adds reliability — how trustworthy the entry is. A highly relevant but unreliable memory (rating D: "contradicted by other evidence") is more dangerous than a moderately relevant, highly reliable one. Consider: an agent retrieves a memory that says "auth uses bcrypt with cost factor 10" (relevance: 0.95, reliability: D — contradicted by a recent migration to argon2id). Without reliability, the agent acts on the high-relevance entry and produces a patch using bcrypt. With reliability, the agent sees the D rating, retrieves the contradicting entry, and produces a patch using argon2id. The reliability rating costs 50 tokens per memory entry to maintain. It prevents patches built on stale information, which cost 15,000+ tokens to diagnose and rewrite.

---

*"Reference delivered. Next transaction."*
